use crate::config::AppConfig;
use crate::element::{ColumnHeaders, Coordinate, Grid, RowGroup, RowHeader};
use crate::parse::PointScreen;
use crate::shape::{Draw, Path, Rectangle, Text};
use std::collections::HashMap;

pub fn convert_grid(
    g: &Grid,
    top_left: &PointScreen,
    cr: i32,
    ac: &AppConfig,
) -> (
    Vec<Box<dyn Draw>>,
    HashMap<Coordinate, Vec<PointScreen>>,
    i32,
) {
    let para = &ac.parameters;
    let (x, y) = (top_left.x, top_left.y);

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

    let mut pm: HashMap<Coordinate, Vec<PointScreen>> = HashMap::new();
    for i in 0..g.iw {
        for j in 0..g.ih {
            pm.insert(
                Coordinate { x: i, y: j + cr },
                vec![
                    PointScreen {
                        x: top_left.x + i * para.cell_width,
                        y: top_left.y + j * para.cell_height,
                    },
                    PointScreen {
                        x: top_left.x + i * para.cell_width,
                        y: top_left.y + (j + 1) * para.cell_height,
                    },
                    PointScreen {
                        x: top_left.x + (i + 1) * para.cell_width,
                        y: top_left.y + (j + 1) * para.cell_height,
                    },
                    PointScreen {
                        x: top_left.x + (i + 1) * para.cell_width,
                        y: top_left.y + j * para.cell_height,
                    },

                ],
            );
        }
    }

    return (result, pm, height);
}

pub fn convert_column_header(
    header: &ColumnHeaders,
    origin_point: &PointScreen,
    ac: &AppConfig,
) -> Vec<Box<dyn Draw>> {
    let para = &ac.parameters;

    let height = para.head_height;
    let width = para.cell_width;

    let c_top_left = PointScreen {
        x: origin_point.x,
        y: origin_point.y - header.rows.len() as i32 * height,
    };

    let mut result: Vec<Box<dyn Draw>> = Vec::new();
    let mut hy = c_top_left.y;
    for cr in header.rows.iter() {
        let mut hx = c_top_left.x;
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
        }
        hy += height;
    }

    return result;
}

pub fn convert_row_header(
    row_header: &RowHeader,
    top_right: &PointScreen,
    style: &AppConfig,
) -> Vec<Box<dyn Draw>> {
    let para = &style.parameters;

    let height = para.cell_height;
    let width = para.head_width;

    let top_left = PointScreen {
        x: top_right.x - row_header.cols.len() as i32 * width,
        y: top_right.y,
    };

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
        }
        hx += width;
    }
    return result;
}

pub fn compute_row_header_width(row_groups: &Vec<RowGroup>, style: &AppConfig) -> i32 {
    let mut maxc: i32 = 0;
    for rg in row_groups.iter() {
        let c = rg.header.cols.len() as i32;
        if c > maxc {
            maxc = c;
        }
    }
    return maxc * style.parameters.head_width;
}

pub fn compute_column_header_height(column_header: &ColumnHeaders, style: &AppConfig) -> i32 {
    return column_header.rows.len() as i32 * style.parameters.head_height;
}
