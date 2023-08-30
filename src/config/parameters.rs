use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Parameters {
    pub x0: i32,
    pub y0: i32,
    pub head_width: i32,
    pub head_height: i32,

    pub cell_width: i32,
    pub cell_height: i32,

    pub project_spacing_width: i32,
    pub project_spacing_height: i32,

    pub shape_scale_width: f32,
    pub shape_scale_height: f32,

    pub segment_spacing_height: i32,
    pub segment_spacing_width: i32,

}
