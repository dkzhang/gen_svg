use crate::config::AppConfig;
use crate::element::{Coordinate, PointLogical, Table};
use crate::parse::grid::*;
use crate::parse::{C2PS, PointScreen};
use crate::shape::Draw;
use std::collections::HashMap;

pub fn convert_table(
    t: &Table,
    ac: &AppConfig,
) -> (Vec<Box<dyn Draw>>, C2PS) {
    let para = &ac.parameters;

    let mut vd: Vec<Box<dyn Draw>> = Vec::new();

    // compute origin position
    let origin_point = PointScreen {
        x: para.x0 + get_row_header_width(&t.row_headers, ac),
        y: para.y0 + get_column_header_height(&t.col_headers, ac),
    };

    log::info!("origin_point: {:?}", origin_point);

    // convert column headers
    let mut col_header_vd =
        convert_column_header(&t.col_headers, &origin_point, &t.grid.x_segments, &ac);
    vd.append(&mut col_header_vd);

    // convert row headers
    let mut row_header_vd: Vec<Box<dyn Draw>> =
        convert_row_header(&t.row_headers, &origin_point, &t.grid.y_segments, &ac);
    vd.append(&mut row_header_vd);

    // convert grid
    let (mut grid_vd, c2ps) = convert_grid(&t.grid, &origin_point, &ac);
    vd.append(&mut grid_vd);

    return (vd, c2ps);
}

pub enum TableClass{
    Grid,
    RowHeader,
    RowHeaderText,
    ColumnHeader,
    ColumnHeaderText,
    TodayLine,
}

impl TableClass{
    pub fn to_string(&self) -> String{
        match self{
            TableClass::Grid => "dk-grid".to_string(),
            TableClass::RowHeader => "dk-row-header".to_string(),
            TableClass::RowHeaderText => "dk-row-header-text".to_string(),
            TableClass::ColumnHeader => "dk-col-header".to_string(),
            TableClass::ColumnHeaderText => "dk-col-header-text".to_string(),
            TableClass::TodayLine => "dk-today-line".to_string(),
        }
    }
}
