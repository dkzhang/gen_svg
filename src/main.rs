mod element;
mod shape;
mod parse;

use svg::Document;
use svg::Node;
use crate::shape::Draw;
use crate::element::Grid;
use crate::parse::Parameters;

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

    let para: Parameters = Parameters{
        x: 0,
        y: 0,
        head_width: 10,
        head_height: 5,
        head_column_n: 2,
        head_rows_n: 1,
        cell_width: 20,
        cell_height: 5,
        project_spacing_width: 0,
        project_spacing_height: 0,
        shape_scale_width: 0.0,
        shape_scale_height: 0.0,
        group_spacing_height: 1,
    };

    let g = Grid{
        id: None,
        i_group: 0,
        ix: 0,
        iy: 0,
        iw: 10,
        ih: 5,
    };

    let vd = parse::convert_grid(g, &para, &path_style);

    let mut document = Document::new()
        .set("width", "400")
        .set("height", "300")
        .set("viewBox", (0, 0, 400, 300))
        .set("preserveAspectRatio", "xMidYMid meet");

    for d in vd {
        document = document.add(d.draw());
    }

    svg::save("image.svg", &document).unwrap();

}