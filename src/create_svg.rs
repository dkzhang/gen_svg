use std::fs;
use std::fs::File;
use svg::Document;
use svg::node::element::{Definitions, Style};
use crate::{DateDateLoc, element, load_config_style, parse};
use crate::config::Defs;
use crate::element::Grid;
use crate::gen_element::col_header::from_date70;
use crate::gen_element::int_to_date70;
use crate::gen_element::row_headers::{DeviceList, from_devices};
use crate::get_projects::get_projects;
use crate::parse::{convert_project, convert_table};
use crate::parse::today_line::convert_today_line;
use crate::shape::Draw;

pub fn create_svg(dl: &DateDateLoc) -> String {
    let app_config = load_config_style("./config/style.toml").unwrap();

    let (col_headers, x_segments) = from_date70(
        int_to_date70(dl.start_date).unwrap(),
        int_to_date70(dl.end_date).unwrap(),
    );

    let (row_headers, y_segments) =
        from_devices(&DeviceList::load_from_json("./config/devices.json").expand_abbreviation());

    let table = element::Table {
        col_headers,
        row_headers,
        grid: Grid {
            id: None,
            x_segments,
            y_segments,
        },
        today: 10,
    };

    // read css file
    let mut css_content = fs::read_to_string("./config/style.css")
        .expect("Something went wrong reading the css file");
    css_content += "\n.dk-project{\n    fill: url(#gradient_blue);\n    fill-opacity : 1;\n} \n";
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

    let (mut vd, c2ps) = convert_table(&table, &app_config);

    println!("c2ps: {:?}", c2ps);

    let (min_x, min_y, max_x, max_y) = c2ps.get_ps_min_max();
    let margin = 100;
    document = document
        .set("width", (max_x + margin).to_string())
        .set("height", (max_y + margin).to_string())
        .set("viewBox", (0, 0, max_x + margin, max_y + margin))
        .set("preserveAspectRatio", "xMinYMin meet");

    for d in vd {
        document = document.add(d.draw());
    }

    // add projects
    let projects = get_projects();

    let mut projects_vd = projects
        .iter()
        .map(|p| convert_project(p, &c2ps, &app_config))
        .flatten()
        .collect::<Vec<Box<dyn Draw>>>();

    for d in projects_vd {
        document = document.add(d.draw());
    }

    // today line
    let mut today_line_vd = convert_today_line(10, &c2ps, &app_config);
    for d in today_line_vd {
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