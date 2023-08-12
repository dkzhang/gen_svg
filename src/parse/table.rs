use crate::config::{AppConfig};
use crate::element::Table;
use crate::parse::grid::*;
use crate::shape::Draw;

pub fn convert_table<'a>(
    t: &'a Table,
    x: i32,
    y: i32,
    ac: &'a AppConfig,
) -> Vec<Box<dyn Draw + 'a>> {
    let para = &ac.parameters;

    let (mut cx, mut cy) = (x, y);
    let mut vd: Vec<Box<dyn Draw>> = Vec::new();

    // firstly compute row header position
    let (row_header_xx, row_header_yy) =
        compute_row_header_pos(&t.row_groups, x, y, &ac);

    log::info!(
        "row_header_xx: {}, row_header_yy: {}",
        row_header_xx,
        row_header_yy
    );

    // firstly convert the column header
    let (mut col_header_vd, col_header_xx, col_header_yy) = convert_column_header(
        &t.col_headers,
        row_header_xx,
        y,
        &ac,
    );
    vd.append(&mut col_header_vd);

    cy = col_header_yy;
    cy += para.group_spacing_height;

    // convert each row group
    for rg in t.row_groups.iter() {
        let (mut row_header_vd, row_header_xx, row_header_yy) =
            convert_row_header(&rg.header, cx, cy, &ac);
        vd.append(&mut row_header_vd);

        let (mut row_grid_vd, _, _) = convert_grid(&rg.grid, row_header_xx, cy, &ac);
        vd.append(&mut row_grid_vd);

        cy = row_header_yy;
        cy += para.group_spacing_height;
    }

    return vd;
}
