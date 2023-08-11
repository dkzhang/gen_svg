use svg::node::element::tag::Rectangle;
use crate::shape::{Draw, Path, PathStyle, Rectangle, RectangleStyle};
use crate::element::{ColumnHeader, Grid, Table};
use crate::parse::Parameters;
use crate::shape::text::{Text, TextStyle};

pub fn convert_grid<'a>(
    g: Grid,
    x:i32,
    y:i32,
    para: &'a Parameters,
    path_style: &'a PathStyle,
) -> Vec<Box<dyn Draw+ 'a>> {
    let mut result: Vec<Box<dyn Draw>> = Vec::new();
    let mut path = Box::new(Path {
        id: g.id,
        d: String::new(),
        style: path_style,
    });

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

pub fn convert_column_header<'a>(
    column_header: ColumnHeader,
    x:i32,
    y:i32,
    para: &'a Parameters,
    rect_style: &'a RectangleStyle,
    text_style: &'a TextStyle,
) -> Vec<Box<dyn Draw+ 'a>> {
    let mut result: Vec<Box<dyn Draw>> = Vec::new();
    let mut hy = y;
    for cr in column_header.rows {
        let mut hx = x;
        for c in cr{
            let rect = Box::new(Rectangle {
                id: None,
                x: hx,
                y: hy,
                width: c.iw * para.head_width,
                height: para.head_height,
                style: rect_style,
            });
            result.push(rect);

            let text = Box::new(Text{
                id: None,
                x: hx + c.iw * para.head_width / 2,
                y: hy + para.head_height * 100 /114,
                content: c.text.clone(),
                style: &text_style,
            });
            result.push(text);


            hx += c.iw * para.head_width;
        }
        hy += para.cell_height;
    };
    return result;
}

// pub fn convert_table<'a>(
//     table: Table,
//     para: &'a Parameters,
//     path_style: &'a PathStyle,
// ) -> Vec<Box<dyn Draw+ 'a>> {
//     let mut result: Vec<Box<dyn Draw>> = Vec::new();
// }
