
pub mod grid;
pub mod project;

pub use grid::*;
pub use project::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct PointLogical {
    pub x: i32,
    pub y: i32,
}