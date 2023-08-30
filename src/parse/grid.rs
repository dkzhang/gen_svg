use crate::config::AppConfig;
use crate::element::{ColumnHeaders, Coordinate, Grid, LogicalUnit, PointLogical, RowHeaders};
use crate::parse::{C2PS, c2ps, PointScreen, ProjectClass, ScreenUnit, TableClass};
use crate::shape::{Draw, Path, Rectangle, Text};
use std::collections::HashMap;

pub fn convert_grid(
    grid: &Grid,
    top_left: &PointScreen,
    ac: &AppConfig,
) -> (Vec<Box<dyn Draw>>, C2PS) {
    let para = &ac.parameters;
    let mut vds_all: Vec<Box<dyn Draw>> = Vec::new();
    let mut c2ps = C2PS::new();

    let mut current_col_row = Coordinate { x: 0, y: 0 };
    let mut current_top_left = top_left.clone();

    for &y in grid.y_segments.iter() {
        current_col_row.x = 0;
        current_top_left.x = top_left.x;

        for &x in grid.x_segments.iter() {

            let vds =
                convert_grid_seg_path(&current_top_left, &PointLogical{ x, y }, ac);

            c2ps.push(c2ps::Region{
                top_left_c: current_col_row.clone(),
                top_left_ps: current_top_left.clone(),
                wh_c: Coordinate{x, y},
                cell_wh_ps: PointScreen{x: para.cell_width, y: para.cell_height},
            });

            current_col_row.x += x;
            current_top_left.x += x * para.cell_width + para.segment_spacing_width;

            vds_all.extend(vds);
        }
        current_col_row.y += y;
        current_top_left.y += y * para.cell_height + para.segment_spacing_height;
    }

    return (vds_all, c2ps);
}
fn convert_grid_seg_path(
    top_left: &PointScreen,
    wh: &PointLogical,
    ac: &AppConfig,
) -> (Vec<Box<dyn Draw>>) {
    let para = &ac.parameters;
    let (x, y) = (top_left.x, top_left.y);
    let (iw, ih) = (wh.x, wh.y);

    let mut vds: Vec<Box<dyn Draw>> = Vec::new();
    let mut path = Box::new(Path {
        id: None,
        class: vec![TableClass::Grid.to_string()],
        d: String::new(),
    });

    let width = para.cell_width * iw;
    let height = para.cell_height * ih;

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
    for i in 1..iw {
        d.push_str(&format!(
            "M{},{} V{}  ",
            x + i * para.cell_width,
            y,
            y + height
        ))
    }
    for j in 1..ih {
        d.push_str(&format!(
            "M{},{} H{}  ",
            x,
            y + j * para.cell_height,
            x + width
        ))
    }

    path.d = d;
    vds.push(path);

    return vds;
}

pub fn convert_column_header(
    header: &ColumnHeaders,
    origin_point: &PointScreen,
    x_segments: &Vec<i32>,
    ac: &AppConfig,
) -> Vec<Box<dyn Draw>> {
    let para = &ac.parameters;

    let height = para.head_height;
    let width = para.cell_width;
    let spacing = para.segment_spacing_width;

    let c_top_left = PointScreen {
        x: origin_point.x,
        y: origin_point.y - get_column_header_height(header, ac),
    };

    // x map
    let mut x_map:Vec<(ScreenUnit,ScreenUnit)> = Vec::new();
    {
        let mut cx = c_top_left.x;
        for &xs in x_segments{
            for i in 0..xs {
                x_map.push((cx+width*i, cx+width*(i+1))) ;
            }
            cx += width * xs + spacing;
        }
    }
    println!("x_map: {:?}", x_map);

    let mut result: Vec<Box<dyn Draw>> = Vec::new();
    let mut hy = c_top_left.y;

    for cr in header.rows.iter() {
        let mut ci: usize= 0;
        for c in cr.iter() {
            let rect = Box::new(Rectangle {
                id: None,
                class: vec![TableClass::ColumnHeader.to_string()],
                x: x_map[ci].0,
                y: hy,
                width: x_map[ci+c.iw as usize -1].1 - x_map[ci].0,
                height: height,
            });
            result.push(rect);

            let text = Box::new(Text {
                id: None,
                class: vec![TableClass::ColumnHeaderText.to_string()],
                x: (x_map[ci].0 + x_map[ci+c.iw as usize -1].1) / 2,
                y: hy + height / 2,
                content: c.text.clone(),
            });
            result.push(text);

            ci+=c.iw as usize;
        }
        hy += height;
    }
    return result;
}

pub fn convert_row_header(
    header: &RowHeaders,
    origin_point: &PointScreen,
    y_segments: &Vec<i32>,
    ac: &AppConfig,
) -> Vec<Box<dyn Draw>> {
    let para = &ac.parameters;

    let height = para.head_height;
    let width = para.cell_width;

    let c_top_left = PointScreen {
        x: origin_point.x - get_row_header_width(header, ac),
        y: origin_point.y,
    };

    // y map
    let mut y_map:Vec<(ScreenUnit,ScreenUnit)> = Vec::new();
    {
        let mut cy = c_top_left.y;
        for &ys in y_segments{
            for j in 0..ys {
                y_map.push((cy +para.cell_height* j, cy +para.cell_height*(j +1))) ;
            }
            cy += para.cell_height * ys + para.segment_spacing_height;
        }
    }

    let mut result: Vec<Box<dyn Draw>> = Vec::new();
    let mut hx = c_top_left.x;

    for cr in header.cols.iter() {
        let mut cj = 0;
        for c in cr.iter() {
            let rect = Box::new(Rectangle {
                id: None,
                class: vec![TableClass::RowHeader.to_string()],
                x: hx,
                y: y_map[cj].0,
                height: y_map[cj +c.ih as usize -1].1 - y_map[cj].0,
                width: width,
            });
            result.push(rect);

            let text = Box::new(Text {
                id: None,
                class: vec![TableClass::RowHeaderText.to_string()],
                x: hx + width / 2,
                y: (y_map[cj].0 + y_map[cj +c.ih as usize -1].1) / 2,
                content: c.text.clone(),
            });
            result.push(text);

            cj += c.ih as usize;
        }
        hx += width;
    }
    return result;
}

pub fn get_column_header_height(header: &ColumnHeaders, ac: &AppConfig) -> i32 {
    let para = &ac.parameters;
    return header.rows.len() as i32 * para.head_height;
}

pub fn get_row_header_width(header: &RowHeaders, ac: &AppConfig) -> i32 {
    let para = &ac.parameters;
    return header.cols.len() as i32 * para.cell_width;
}
