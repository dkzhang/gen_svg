use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct PolygonStyle {
    pub fill: Option<String>,
    pub stroke: Option<String>,
    pub stroke_width: Option<i32>,
    pub stroke_opacity: Option<f32>,
    pub fill_opacity: Option<f32>,

    pub transform: Option<String>,
}