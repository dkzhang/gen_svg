use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Parameters {
    pub head_width: i32,
    pub head_height: i32,

    pub cell_width: i32,
    pub cell_height: i32,

    pub project_spacing_width: i32,
    pub project_spacing_height: i32,

    pub shape_scale_width: f32,
    pub shape_scale_height: f32,

    pub group_spacing_height: i32,
}