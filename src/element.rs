
pub mod grid;
pub mod project;

use std::fmt;
pub use grid::*;
pub use project::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct PointLogical {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for PointLogical {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct LogicalUnit (i32,i32);