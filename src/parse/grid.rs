use crate::config::{AppConfig};
use crate::element::{ColumnHeader, Grid, RowGroup, RowHeader, Table};
use crate::shape::{Draw, Path, Rectangle, Text};
use svg::node::element::tag::Rectangle;
use crate::parse::PointScreen;

pub fn convert_grid<'a>(
    g: &'a Grid,
    top_left: PointScreen,
    ac: &'a AppConfig,
) -> (Vec<Box<dyn Draw + 'a>>, PointScreen) {
    let para = &ac.parameters;
    let (x,y) = (top_left.x, top_left.y);

    let mut result: Vec<Box<dyn Draw>> = Vec::new();
    let mut path = Box::new(Path {
        id: g.id.clone(),
        class: Vec::new(),
        d: String::new(),
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

    let bottom_right = PointScreen {
        x: x + width,
        y: y + height,
    };
    return (result, bottom_right);
}

pub fn convert_column_header<'a>(
    column_header: &'a ColumnHeader,
    top_left: PointScreen,
    ac: &'a AppConfig,
) -> (Vec<Box<dyn Draw + 'a>>, PointScreen) {
    let para = &ac.parameters;

    let height = para.head_height;
    let width = para.cell_width;

    let mut xx = top_left.x;
    let mut yy = top_left.y;

    let mut result: Vec<Box<dyn Draw>> = Vec::new();
    let mut hy = top_left.y;
    for cr in column_header.rows.iter() {
        let mut hx = top_left.x;
        for c in cr.iter() {
            let rect = Box::new(Rectangle {
                id: None,
                class: Vec::new(),
                x: hx,
                y: hy,
                width: c.iw * width,
                height: height,
            });
            result.push(rect);

            let text = Box::new(Text {
                id: None,
                class: Vec::new(),
                x: hx + c.iw * width / 2,
                y: hy + height / 2,
                content: c.text.clone(),
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
    let bottom_right = PointScreen {
        x: xx,
        y: yy,
    };
    return (result, bottom_right);
}

pub fn convert_row_header<'a>(
    row_header: &'a RowHeader,
    top_left: PointScreen,
    style: &'a AppConfig,
) -> (Vec<Box<dyn Draw + 'a>>, PointScreen) {
    let para = &style.parameters;

    let height = para.cell_height;
    let width = para.head_width;

    let mut xx = top_left.x;
    let mut yy = top_left.y;

    let mut result: Vec<Box<dyn Draw>> = Vec::new();
    let mut hx = top_left.x;
    for cc in row_header.cols.iter() {
        let mut hy = top_left.y;
        for r in cc {
            let rect = Box::new(Rectangle {
                id: None,
                class: Vec::new(),
                x: hx,
                y: hy,
                width: width,
                height: r.ih * height,
            });
            result.push(rect);

            let text = Box::new(Text {
                id: None,
                class: Vec::new(),
                x: hx + width / 2,
                y: hy + r.ih * height / 2,
                content: r.text.clone(),
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

    let bottom_right = PointScreen {
        x: xx,
        y: yy,
    };
    return (result, bottom_right);
}

pub fn compute_row_header_pos<'a>(
    row_groups: &'a Vec<RowGroup>,
    top_left: PointScreen,
    style: &'a AppConfig,
) -> PointScreen {
    let para = &style.parameters;

    let height = para.cell_height;
    let width = para.head_width;

    let mut xx = top_left.x;
    let mut yy = top_left.y;

    let mut gy = top_left.y;

    for rg in row_groups.iter() {
        let mut hx = top_left.x;
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

    let bottom_right = PointScreen {
        x: xx,
        y: yy,
    };
    return bottom_right;
}
