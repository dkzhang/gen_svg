mod element;
mod parse;
mod shape;
mod config;

use crate::element::{ColumnHeader, ColumnHeaderCell, Grid};
use crate::config::{Parameters, PolygonStyle};
use crate::shape::Draw;
use svg::Document;
use svg::Node;

use simplelog::*;
use std::fs::File;


use std::io::Read;
use std::path::Path;
use crate::config::Config as MyConfig;

fn load_config_style<P: AsRef<Path>>(path: P) -> Result<MyConfig, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: MyConfig = toml::from_str(&contents)?;
    Ok(config)
}

fn main() {

    let log_file = File::create("gen_svg.log").unwrap();

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), log_file)
        ]
    ).unwrap();


    let config = load_config_style("./config/style.toml").unwrap();
    let para: Parameters = config.parameters;
    let path_style = config.path_style;
    let rect_style = config.rectangle_style;
    let text_style = config.text_style;

    let pl = PolygonStyle {
        fill: Some(String::from("red")),
        stroke: Some(String::from("blue")),
        stroke_width: Some(1),
        stroke_opacity: Some(0.5),
        fill_opacity: Some(0.5),
        transform: Some(String::from("rotate(45)")),
    };

    let ch = ColumnHeader {
        rows: vec![vec![
            ColumnHeaderCell {
                iw: 1,
                text: String::from("A"),
            },
            ColumnHeaderCell {
                iw: 2,
                text: String::from("B"),
            },
            ColumnHeaderCell {
                iw: 3,
                text: String::from("C"),
            },
        ]],
    };



    let g = Grid {
        id: None,
        iw: 10,
        ih: 5,
    };


    // let mut x = 10;
    // let mut y = 10;
    // let vd2 = parse::convert_column_header(ch, x, y, &para, &rect_style, &text_style);
    // // x = 10;
    // y += para.head_height + para.group_spacing_height;
    // let vd1 = parse::convert_grid(g, x, y, &para, &path_style);



    let mut document = Document::new()
        .set("width", "400")
        .set("height", "300")
        .set("viewBox", (0, 0, 400, 300))
        .set("preserveAspectRatio", "xMidYMid meet");

    // for d in vd1 {
    //     document = document.add(d.draw());
    // }
    //
    // for d in vd2 {
    //     document = document.add(d.draw());
    // }

    svg::save("image.svg", &document).unwrap();


    log::info!("This is an information message from file {} at line {} .", file!(), line!());
    log::warn!("This is a warning message from file {} at line {} .", file!(), line!());
    log::error!("This is an error message from file {} at line {} .", file!(), line!());
}
