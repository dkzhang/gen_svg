
pub mod table;
pub mod project;

use std::fmt;
pub use table::*;
pub use project::*;

pub type LogicalUnit = i32;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct PointLogical {
    pub x: LogicalUnit,
    pub y: LogicalUnit,
}

impl fmt::Display for PointLogical {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
