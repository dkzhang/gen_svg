use std::collections::HashMap;
use crate::config::{AppConfig};
use crate::element::{PointLogical, Table};
use crate::parse::grid::*;
use crate::parse::PointScreen;
use crate::shape::Draw;

pub fn convert_table<'a>(
    t: &'a Table,
    top_left: PointScreen,
    ac: &'a AppConfig,
) -> Vec<Box<dyn Draw + 'a>> {
    let para = &ac.parameters;

    let points_map:HashMap<PointLogical, PointScreen> = HashMap::new();

    let (mut cx, mut cy) = (top_left.x, top_left.y);
    let mut vd: Vec<Box<dyn Draw>> = Vec::new();

    // firstly compute row header position
    let row_header_bottom_right = compute_row_header_pos(&t.row_groups, top_left, &ac);

    log::info!("row_header_bottom_right: {:?}", row_header_bottom_right);

    // firstly convert the column header
    let (mut col_header_vd, col_header_bottom_right) = convert_column_header(
        &t.col_headers,
        PointScreen {
            x: row_header_bottom_right.x,
            y: top_left.y,
        },
        &ac,
    );
    vd.append(&mut col_header_vd);

    cy = col_header_bottom_right.y;
    cy += para.group_spacing_height;

    // convert each row group
    for rg in t.row_groups.iter() {
        let (mut row_header_vd, row_header_bottom_right) =
            convert_row_header(&rg.header, PointScreen{ x: cx, y: cy }, &ac);
        vd.append(&mut row_header_vd);

        let (mut row_grid_vd, _) = convert_grid(&rg.grid, PointScreen{x:row_header_bottom_right.x, y:cy} , &ac);
        vd.append(&mut row_grid_vd);

        cy = row_header_bottom_right.y;
        cy += para.group_spacing_height;
    }

    return vd;
}
