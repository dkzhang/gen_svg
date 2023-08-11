mod element;
mod parse;
mod shape;

use crate::element::{ColumnHeader, ColumnHeaderCell, Grid};
use crate::parse::Parameters;
use crate::shape::Draw;
use svg::Document;
use svg::Node;

fn main() {
    let path_style: shape::PathStyle = shape::PathStyle {
        fill: Some(String::from("green")),
        stroke: Some(String::from("black")),
        stroke_width: Some(1),
        stroke_opacity: Some(1.0),
        fill_opacity: Some(0.5),
        stroke_linecap: None,
        stroke_linejoin: None,
        stroke_dasharray: None,
        stroke_dashoffset: None,
        transform: None,
    };

    let para: Parameters = Parameters {
        head_width: 20,
        head_height: 8,
        cell_width: 20,
        cell_height: 5,
        project_spacing_width: 0,
        project_spacing_height: 0,
        shape_scale_width: 0.0,
        shape_scale_height: 0.0,
        group_spacing_height: 2,
    };

    let rect_style = shape::RectangleStyle {
        fill: Some(String::from("white")),
        stroke: Some(String::from("black")),
        stroke_width: Some(1),
        stroke_opacity: Some(1.0),
        fill_opacity: Some(0.5),
        rx: None,
        ry: None,
        transform: None,
    };

    let text_style = shape::TextStyle {
        font_family: Some(String::from("Arial")),
        font_size: Some(7),
        text_anchor: Some(String::from("middle")),
        fill: Some(String::from("black")),
        font_weight: Some(String::from("normal")),
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


    let mut x = 10;
    let mut y = 10;
    let vd2 = parse::convert_column_header(ch, x, y, &para, &rect_style, &text_style);
    // x = 10;
    y += para.head_height + para.group_spacing_height;
    let vd1 = parse::convert_grid(g, x, y, &para, &path_style);



    let mut document = Document::new()
        .set("width", "400")
        .set("height", "300")
        .set("viewBox", (0, 0, 400, 300))
        .set("preserveAspectRatio", "xMidYMid meet");

    for d in vd1 {
        document = document.add(d.draw());
    }

    for d in vd2 {
        document = document.add(d.draw());
    }

    svg::save("image.svg", &document).unwrap();
}
