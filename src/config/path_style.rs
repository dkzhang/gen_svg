use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct PathStyle {
    pub fill: Option<String>,
    pub fill_opacity: Option<f32>,
    pub stroke: Option<String>,
    pub stroke_width: Option<i32>,
    pub stroke_linecap: Option<String>,
    pub stroke_linejoin: Option<String>,
    pub stroke_dasharray: Option<String>,
    pub stroke_dashoffset: Option<i32>,
    pub stroke_opacity: Option<f32>,

    pub transform: Option<String>,
}