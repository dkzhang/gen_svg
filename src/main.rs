mod config;
mod element;
mod parse;
mod shape;

use crate::config::{Parameters, PolygonStyle};
use crate::element::{
    ColumnHeader, ColumnHeaderCell, Grid, RowGroup, RowHeader, RowHeaderCell, Table,
};
use crate::shape::Draw;
use svg::Document;
use svg::Node;

use simplelog::*;
use std::fs::File;
use serde_json;

use crate::config::StyleConfig as MyConfig;
use crate::parse::table::convert_table;
use std::io::{Read, Write};
use std::path::Path;

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

    let config = load_config_style("./config/style.toml").unwrap();
    let para: Parameters = config.parameters;
    let path_style = config.path_style;
    let polygon_style = config.polygon_style;
    let rect_style = config.rectangle_style;
    let text_style = config.table_header_text_style;

    let table = Table {
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

    let (x, y) = (0, 0);
    let mut vd = convert_table(&table, x, y, &para, &path_style, &rect_style, &text_style);

    let table_json = serde_json::to_string_pretty(&table).expect("Failed to serialize data");

    // Write the JSON string to a file.
    let mut file = File::create("table.json").expect("Failed to create file");
    file.write_all(table_json.as_bytes()).expect("Failed to write data");

    let mut document = Document::new()
        .set("width", "400")
        .set("height", "300")
        .set("viewBox", (0, 0, 400, 300))
        .set("preserveAspectRatio", "xMidYMid meet");

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
