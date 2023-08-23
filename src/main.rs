mod config;
mod element;
mod gen_element;
mod parse;
mod shape;

use crate::element::{
    ColumnHeaderCell, ColumnHeaders, Coordinate, Grid, Project, ProjectRect, RowHeaderCell,
    RowHeaders, Table,
};
use crate::shape::Draw;
use std::{fmt, fs};
use svg::Document;
use svg::Node;

use serde_json;
use simplelog::*;
use std::fs::File;

use crate::config::{AppConfig, Defs};
use crate::gen_element::col_header::from_date;
use crate::gen_element::row_headers::{from_devices, DeviceGroup, DeviceList};
use crate::parse::table::convert_table;
use crate::parse::{convert_project, PointScreen};
use serde_json::from_reader;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::str::FromStr;
use svg::node::element::{Definitions, Link, Style};

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
    response::Response,
    body::Body,
};
use axum::extract::Query;
use axum::http::header;

use serde::{de, Deserialize, Deserializer, Serialize};

fn load_config_style<P: AsRef<Path>>(path: P) -> Result<AppConfig, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: AppConfig = toml::from_str(&contents)?;
    Ok(config)
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/svg", get(get_svg));

    tracing::info!("Listening on 0.0.0.0:8080");

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> impl IntoResponse  {
    let svg = create_svg();

    let mut response = Response::new(Body::from(svg));

    response.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("image/svg+xml"),
    );

    return response
}

async fn get_svg(Query(params): Query<Params>) -> String {
    format!("{:?}", params)
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Params {
    #[serde(default)]
    start_date: Option<i32>,
    end_date: Option<i32>,
    location: i32,
}


fn create_svg() -> String {

    let app_config = load_config_style("./config/style.toml").unwrap();

    let (col_headers, col_index_map, x_segments) = from_date("20230701", "20231001");
    let (row_headers, row_index_map, y_segments) =
        from_devices(&DeviceList::load_from_json("./config/devices.json").expand_abbreviation());

    let table_origin = element::Table {
        col_headers,
        row_headers,
        grid: Grid {
            id: None,
            x_segments,
            y_segments,
        },
    };

    let table_json = "table.json";
    table_origin.save_to_json(table_json);

    // read css file
    let css_content = fs::read_to_string("./config/style.css")
        .expect("Something went wrong reading the css file");
    let css_style_def = Definitions::new().add(Style::new(css_content));

    // read gradient xml file
    let gradient_filename = "./config/gradient.xml";
    let file_gradient = File::open(gradient_filename).expect("Failed to open json file");
    let gradient_defs_struct: Defs =
        serde_xml_rs::from_reader(file_gradient).expect("Failed to read gradient from xml file");
    let gradient_defs = parse::gradient::convert_to_gradient(gradient_defs_struct);

    // create svg document
    let mut document = Document::new()
        // .set("width", "38400")
        // .set("height", "21600")
        // .set("viewBox", (0, 0, 38400, 21600))
        // .set("preserveAspectRatio", "xMidYMid meet")
        .set("xmlns", "http://www.w3.org/2000/svg")
        .set("xmlns:xlink", "http://www.w3.org/1999/xlink")
        .add(css_style_def)
        .add(gradient_defs);

    // write shape in svg
    // write table
    let table = Table::load_from_json(table_json);

    let (mut vd, c2ps) = convert_table(&table, &app_config);

    let (min_x, min_y, max_x, max_y) = c2ps.get_ps_min_max();
    let margin = 100;
    document = document
        .set("width", (max_x + margin).to_string())
        .set("height", (max_y + margin).to_string())
        .set("viewBox", (0, 0, max_x + margin, max_y + margin))
        .set("preserveAspectRatio", "xMinYMim meet");

    for d in vd {
        document = document.add(d.draw());
    }

    //write project
    let project1 = Project {
        id: String::from("001"),
        name: String::from("Project1"),
        rects: vec![
            ProjectRect::new2(Coordinate { x: 0, y: 2 }, &Coordinate { x: 1, y: 3 }),
            ProjectRect::new2(Coordinate { x: 2, y: 0 }, &Coordinate { x: 2, y: 3 }),
        ],
    };
    let mut project1_vd = convert_project(&project1, &c2ps, &app_config);
    for d in project1_vd {
        document = document.add(d.draw());
    }

    let project2 = Project {
        id: String::from("002"),
        name: String::from("Project2"),
        rects: vec![ProjectRect::new2(
            Coordinate { x: 3, y: 1 },
            &Coordinate { x: 6, y: 2 },
        )],
    };
    let mut project2_vd = convert_project(&project2, &c2ps, &app_config);
    for d in project2_vd {
        document = document.add(d.draw());
    }

    let project3 = Project {
        id: String::from("003"),
        name: String::from("Project3"),
        rects: vec![
            ProjectRect::new2(Coordinate { x: 3, y: 3 }, &Coordinate { x: 6, y: 3 }),
            ProjectRect::new2(Coordinate { x: 7, y: 0 }, &Coordinate { x: 8, y: 3 }),
            ProjectRect::new2(Coordinate { x: 9, y: 0 }, &Coordinate { x: 11, y: 0 }),
        ],
    };
    let mut project3_vd = convert_project(&project3, &c2ps, &app_config);
    for d in project3_vd {
        document = document.add(d.draw());
    }

    // svg::save("image.svg", &document).unwrap();

    log::info!(
        "This is an information message from file {} at line {} .",
        file!(),
        line!()
    );
    log::warn!(
        "This is a warning message from file {} at line {} .",
        file!(),
        line!()
    );
    log::error!(
        "This is an error message from file {} at line {} .",
        file!(),
        line!()
    );

    return document.to_string();
}
