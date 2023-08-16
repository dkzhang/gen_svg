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
pub mod project;
pub mod gradient;
pub mod c2ps;

use std::fmt;
pub use grid::*;
pub use table::*;
pub use project::*;
pub use c2ps::*;



pub type ScreenUnit = i32;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct PointScreen {
    pub x: ScreenUnit,
    pub y: ScreenUnit,
}

impl fmt::Display for PointScreen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}