use serde::{Deserialize, Serialize};
use crate::element::ProjectStatus;
use crate::my_utils::date::DateInt;

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct Project0{
    pub id: String,
    pub name: String,
    pub rects : Vec<ProjectRect0>,

    pub metering: Vec<f64>,
}

#[derive(Debug, Clone,Hash,Serialize,Deserialize)]
pub struct ProjectRect0 {
    pub date_from: DateInt,
    pub date_to: DateInt,
    pub devices: Vec<String>,
    pub status: ProjectStatus,
}