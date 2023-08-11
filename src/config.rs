

mod parameters;
mod line_style;
mod path_style;
mod polygon_style;
mod rectangle_style;
mod text_style;

pub use parameters::*;
pub use line_style::*;
pub use path_style::*;
pub use polygon_style::*;
pub use rectangle_style::*;
pub use text_style::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config{
    pub parameters:Parameters,
    pub line_style:LineStyle,
    pub path_style:PathStyle,
    pub polygon_style:PolygonStyle,
    pub rectangle_style:RectangleStyle,
    pub text_style:TextStyle,
}