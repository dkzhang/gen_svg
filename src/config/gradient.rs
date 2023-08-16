use std::collections::HashMap;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct Defs{
    #[serde(rename = "linearGradient")]
    pub linear_gradients: Option<Vec<LinearGradient>>,

    #[serde(rename = "radialGradient")]
    pub radial_gradients: Option<Vec<RadialGradient>>,
}

#[derive(Debug, Deserialize)]
pub struct LinearGradient{
    pub stop: Vec<Stop>,

    #[serde(flatten)]
    pub attrs: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct RadialGradient{
    pub stop: Vec<Stop>,

    #[serde(flatten)]
    pub attrs: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct Stop {
    #[serde(flatten)]
    pub attrs: HashMap<String, String>,
}