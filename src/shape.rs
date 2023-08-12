pub mod line;
pub mod path;
pub mod polygon;
pub mod rectangle;
pub mod text;

pub use line::*;
pub use path::*;
pub use polygon::*;
pub use rectangle::*;
pub use text::*;

pub trait Draw {
    fn draw(&self) -> Box<dyn svg::Node>;
}
