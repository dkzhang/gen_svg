// use crate::shape::rectangle;
//
// struct grid<'a> {
//     frame:  rectangle<'a>,
//     // h_lines: Vec<line>,
//     // v_lines: Vec<line>,
// }
//

pub mod grid;
pub mod table;

pub use grid::*;




#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct PointScreen {
    pub x: i32,
    pub y: i32,
}