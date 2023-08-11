pub mod rectangle;
pub mod line;
pub mod polygon;
pub mod path;
pub mod text;

pub use rectangle::*;
pub use line::*;
pub use polygon::*;
pub use path::*;
pub use text::*;

pub trait Draw {
    fn draw(&self) -> Box<dyn svg::Node>;
}


