use crate::shape::{Draw, Path, PathStyle};
use crate::element::Grid;
use crate::parse::Parameters;

pub fn convert_grid<'a>(
    g: Grid,
    para: &'a Parameters,
    path_style: &'a PathStyle,
) -> Vec<Box<dyn Draw+ 'a>> {
    let mut result: Vec<Box<dyn Draw>> = Vec::new();
    let mut path = Box::new(Path {
        id: g.id,
        d: String::new(),
        style: path_style,
    });

    let x = para.x + para.head_width * para.head_rows_n + g.ix * para.cell_width;
    let y = para.y + para.head_height * para.head_column_n + g.iy * para.cell_height
        + (g.i_group + 1) * para.group_spacing_height;

    let width = para.cell_width * g.iw;
    let height = para.cell_height * g.ih;

    let mut d = String::new();
    d.push_str(&format!("M{},{} H{} V{} H{} V{} Z  ", x, y, x + width, y + height, x, y));
    for i in 1..g.iw{
        d.push_str(&format!("M{},{} V{}  ",x + i * para.cell_width, y,y + height))
    }
    for j in 1..g.ih{
        d.push_str(&format!("M{},{} H{}  ",x, y + j * para.cell_height,x + width))
    }

    path.d = d;
    result.push(path);
    return result;
}
