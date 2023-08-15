use crate::config::AppConfig;
use crate::element::{Coordinate, PointLogical, Table};
use crate::parse::grid::*;
use crate::parse::PointScreen;
use crate::shape::Draw;
use std::collections::HashMap;

pub fn convert_table(
    t: &Table,
    top_left: PointScreen,
    ac: &AppConfig,
) -> (Vec<Box<dyn Draw>>, HashMap<Coordinate, Vec<PointScreen>>) {
    let para = &ac.parameters;

    let mut points_map: HashMap<Coordinate, Vec<PointScreen>> = HashMap::new();

    let (mut cx, mut cy) = (top_left.x, top_left.y);
    let mut vd: Vec<Box<dyn Draw>> = Vec::new();

    // compute origin position
    let origin_point = PointScreen {
        x: top_left.x + compute_row_header_width(&t.row_groups, &ac),
        y: top_left.y + compute_column_header_height(&t.col_headers, &ac),
    };

    log::info!("origin_point: {:?}", origin_point);

    // firstly convert the column header
    let mut col_header_vd= convert_column_header(&t.col_headers, &origin_point,&ac);
    vd.append(&mut col_header_vd);

    cy = origin_point.y;
    cy += para.group_spacing_height;
    let mut cr = 0; //current row index

    // convert each row group
    for rg in t.row_groups.iter() {
        let mut row_header_vd =
            convert_row_header(&rg.header, &PointScreen{x:origin_point.x, y:cy}, &ac);
        vd.append(&mut row_header_vd);

        let (mut row_grid_vd,pm, grid_height) = convert_grid(
            &rg.grid,
            &PointScreen {
                x: origin_point.x,
                y: cy,
            },
            cr,
            &ac,
        );
        cr += rg.grid.ih;
        vd.append(&mut row_grid_vd);
        points_map.extend(pm);

        cy += grid_height;
        cy += para.group_spacing_height;
    }


    return (vd, points_map);
}
