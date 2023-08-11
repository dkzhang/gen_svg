use crate::config::{Parameters, PathStyle, RectangleStyle, TextStyle};
use crate::element::Table;
use crate::shape::Draw;
use crate::parse::grid::*;

pub fn convert_table<'a>(
    t: Table,
    x:i32,
    y:i32,
    para: &'a Parameters,
    path_style: &'a PathStyle,
    rect_style: &'a RectangleStyle,
    text_style: &'a TextStyle,
) -> Vec<Box<dyn Draw+ 'a>> {
    let (mut cx, mut cy) = (x,y);

    let mut vd: Vec<Box<dyn Draw>> = Vec::new();
    let (mut col_header_vd, col_header_xx, col_header_yy) =
        convert_column_header(t.col_headers, x, y, &para, &rect_style, &text_style);
    vd.append(&mut col_header_vd);
    cy = col_header_yy;

    cy += para.group_spacing_height;
   for rg in t.row_groups {
        let (mut row_header_vd, row_header_xx, row_header_yy) =
            convert_row_header(rg.header, cx, cy, &para, &rect_style, &text_style);
        vd.append(&mut row_header_vd);

       let (mut row_grid_vd, _, _ ) =
            convert_grid(rg.grid, row_header_xx, cy, &para, &path_style);
       vd.append(&mut row_grid_vd);

       cy = row_header_yy;
       cy += para.group_spacing_height;
    }
    return vd;
}