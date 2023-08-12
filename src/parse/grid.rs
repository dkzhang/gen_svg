use crate::config::PathStyle;
use crate::config::{Parameters, RectangleStyle, TextStyle};
use crate::element::{ColumnHeader, Grid, RowGroup, RowHeader, Table};
use crate::shape::{Draw, Path, Rectangle, Text};
use svg::node::element::tag::Rectangle;

pub fn convert_grid<'a>(
    g: &'a Grid,
    x: i32,
    y: i32,
    para: &'a Parameters,
    path_style: &'a PathStyle,
) -> (Vec<Box<dyn Draw + 'a>>, i32, i32) {
    let mut result: Vec<Box<dyn Draw>> = Vec::new();
    let mut path = Box::new(Path {
        id: g.id.clone(),
        d: String::new(),
        style: path_style,
    });

    let width = para.cell_width * g.iw;
    let height = para.cell_height * g.ih;

    let mut d = String::new();
    d.push_str(&format!(
        "M{},{} H{} V{} H{} V{} Z  ",
        x,
        y,
        x + width,
        y + height,
        x,
        y
    ));
    for i in 1..g.iw {
        d.push_str(&format!(
            "M{},{} V{}  ",
            x + i * para.cell_width,
            y,
            y + height
        ))
    }
    for j in 1..g.ih {
        d.push_str(&format!(
            "M{},{} H{}  ",
            x,
            y + j * para.cell_height,
            x + width
        ))
    }

    path.d = d;
    result.push(path);
    return (result, x + width, y + height);
}

pub fn convert_column_header<'a>(
    column_header: &'a ColumnHeader,
    x: i32,
    y: i32,
    para: &'a Parameters,
    rect_style: &'a RectangleStyle,
    text_style: &'a TextStyle,
) -> (Vec<Box<dyn Draw + 'a>>, i32, i32) {
    let height = para.head_height;
    let width = para.cell_width;

    let mut xx = x;
    let mut yy = y;

    let mut result: Vec<Box<dyn Draw>> = Vec::new();
    let mut hy = y;
    for cr in column_header.rows.iter() {
        let mut hx = x;
        for c in cr.iter() {
            let rect = Box::new(Rectangle {
                id: None,
                x: hx,
                y: hy,
                width: c.iw * width,
                height: height,
                style: rect_style,
            });
            result.push(rect);

            let text = Box::new(Text {
                id: None,
                x: hx + c.iw * width / 2,
                y: hy + height / 2,
                content: c.text.clone(),
                style: &text_style,
            });
            result.push(text);

            hx += c.iw * width;

            if hx > xx {
                xx = hx;
            }
        }
        hy += height;
        if hy > yy {
            yy = hy;
        }
    }
    return (result, xx, yy);
}

pub fn convert_row_header<'a>(
    row_header: &'a RowHeader,
    x: i32,
    y: i32,
    para: &'a Parameters,
    rect_style: &'a RectangleStyle,
    text_style: &'a TextStyle,
) -> (Vec<Box<dyn Draw + 'a>>, i32, i32) {
    let height = para.cell_height;
    let width = para.head_width;

    let mut xx = x;
    let mut yy = y;

    let mut result: Vec<Box<dyn Draw>> = Vec::new();
    let mut hx = x;
    for cc in row_header.cols.iter() {
        let mut hy = y;
        for r in cc {
            let rect = Box::new(Rectangle {
                id: None,
                x: hx,
                y: hy,
                width: width,
                height: r.ih * height,
                style: rect_style,
            });
            result.push(rect);

            let text = Box::new(Text {
                id: None,
                x: hx + width / 2,
                y: hy + r.ih * height / 2,
                content: r.text.clone(),
                style: &text_style,
            });
            result.push(text);

            hy += r.ih * height;

            if hy > yy {
                yy = hy;
            }
        }
        hx += width;
        if hx > xx {
            xx = hx;
        }
    }
    return (result, xx, yy);
}

pub fn compute_row_header_pos<'a>(
    row_groups: &'a Vec<RowGroup>,
    x: i32,
    y: i32,
    para: &'a Parameters,
    rect_style: &'a RectangleStyle,
    text_style: &'a TextStyle,
) -> (i32, i32) {
    let height = para.cell_height;
    let width = para.head_width;

    let mut xx = x;
    let mut yy = y;

    let mut gy = y;

    for rg in row_groups.iter() {
        let mut hx = x;
        for cc in rg.header.cols.iter() {
            let mut hy = gy;
            for r in cc {
                hy += r.ih * height;

                if hy > yy {
                    yy = hy;
                }
            }
            hx += width;
            if hx > xx {
                xx = hx;
            }
        }

        gy = yy;
        gy += para.group_spacing_height;
    }

    return (xx, yy);
}
