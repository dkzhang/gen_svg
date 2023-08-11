use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct LineStyle {
    pub stroke: Option<String>,
    pub stroke_width: Option<i32>,
    pub stroke_opacity: Option<f32>,
    pub stroke_linecap: Option<String>,
    pub stroke_dasharray: Option<String>,
    pub stroke_dashoffset: Option<i32>,

    pub transform: Option<String>,
}