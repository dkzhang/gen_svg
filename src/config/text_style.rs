use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct TextStyle {
    pub font_family: Option<String>,
    pub font_size: Option<i32>,
    pub text_anchor: Option<String>,
    pub fill: Option<String>,
    pub font_weight: Option<String>,
}