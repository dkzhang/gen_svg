mod config;
mod element;
mod parse;
mod shape;

use crate::config::{Parameters, PolygonStyle};
use crate::element::{
    ColumnHeader, ColumnHeaderCell, Grid, RowGroup, RowHeader, RowHeaderCell, Table,
};
use crate::shape::Draw;
use std::fs;
use svg::Document;
use svg::Node;

use serde_json;
use simplelog::*;
use std::fs::File;

use crate::config::StyleConfig as MyConfig;
use crate::parse::table::convert_table;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use serde_json::from_reader;
use svg::node::element::tag::Definitions;
use svg::node::element::{Definitions, Link, Style};

fn load_config_style<P: AsRef<Path>>(path: P) -> Result<MyConfig, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: MyConfig = toml::from_str(&contents)?;
    Ok(config)
}

fn main() {
    let log_file = File::create("gen_svg.log").unwrap();

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(LevelFilter::Info, Config::default(), log_file),
    ])
    .unwrap();

    let style_config = load_config_style("./config/style.toml").unwrap();

    let table_origin = Table {
        col_headers: ColumnHeader {
            rows: vec![
                vec![ColumnHeaderCell {
                    iw: 10,
                    text: String::from("Oct"),
                }],
                vec![
                    ColumnHeaderCell {
                        iw: 1,
                        text: String::from("1"),
                    },
                    ColumnHeaderCell {
                        iw: 1,
                        text: String::from("2"),
                    },
                    ColumnHeaderCell {
                        iw: 1,
                        text: String::from("3"),
                    },
                    ColumnHeaderCell {
                        iw: 1,
                        text: String::from("4"),
                    },
                    ColumnHeaderCell {
                        iw: 1,
                        text: String::from("5"),
                    },
                    ColumnHeaderCell {
                        iw: 1,
                        text: String::from("6"),
                    },
                    ColumnHeaderCell {
                        iw: 1,
                        text: String::from("7"),
                    },
                    ColumnHeaderCell {
                        iw: 1,
                        text: String::from("8"),
                    },
                    ColumnHeaderCell {
                        iw: 1,
                        text: String::from("9"),
                    },
                    ColumnHeaderCell {
                        iw: 1,
                        text: String::from("10"),
                    },
                ],
            ],
        },
        row_groups: vec![
            RowGroup {
                header: RowHeader {
                    cols: vec![
                        vec![RowHeaderCell {
                            ih: 3,
                            text: String::from("F301-3"),
                        }],
                        vec![
                            RowHeaderCell {
                                ih: 1,
                                text: String::from("F301"),
                            },
                            RowHeaderCell {
                                ih: 1,
                                text: String::from("F302"),
                            },
                            RowHeaderCell {
                                ih: 1,
                                text: String::from("F303"),
                            },
                        ],
                    ],
                },
                grid: Grid {
                    id: None,
                    iw: 10,
                    ih: 3,
                },
            },
            RowGroup {
                header: RowHeader {
                    cols: vec![
                        vec![RowHeaderCell {
                            ih: 3,
                            text: String::from("F401-3"),
                        }],
                        vec![
                            RowHeaderCell {
                                ih: 1,
                                text: String::from("F401"),
                            },
                            RowHeaderCell {
                                ih: 1,
                                text: String::from("F402"),
                            },
                            RowHeaderCell {
                                ih: 1,
                                text: String::from("F403"),
                            },
                        ],
                    ],
                },
                grid: Grid {
                    id: None,
                    iw: 10,
                    ih: 3,
                },
            },
        ],
    };

    let table_json = serde_json::to_string_pretty(&table_origin).expect("Failed to serialize data");

    let json_filename = "table.json";
    // Write the JSON string to a file.
    let mut file_json_w = File::create(json_filename).expect("Failed to create file");
    file_json_w.write_all(table_json.as_bytes())
        .expect("Failed to write data");

    let file_json_r = File::open(json_filename).expect("Failed to open json file");
    let table = from_reader(BufReader::new(file_json_r)).expect("Failed to read table from json file");

    // read css file
    let css_content =
        fs::read_to_string("./style.css").expect("Something went wrong reading the css file");

    let css_style_def = Definitions::new().add(Style::new(css_content));

    let mut document = Document::new()
        .set("width", "400")
        .set("height", "300")
        .set("viewBox", (0, 0, 400, 300))
        .set("preserveAspectRatio", "xMidYMid meet")
        .set("xmlns", "http://www.w3.org/2000/svg")
        .set("xmlns:xlink", "http://www.w3.org/1999/xlink")
        .add(css_style_def);

    // write shape in svg
    let (x, y) = (0, 0);
    let mut vd = convert_table(&table, x, y, &style_config);

    for d in vd {
        document = document.add(d.draw());
    }

    svg::save("image.svg", &document).unwrap();

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
}
