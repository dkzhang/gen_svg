use crate::element::Project;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Defs {
    #[serde(rename = "linearGradient")]
    pub linear_gradients: Option<Vec<LinearGradient>>,

    #[serde(rename = "radialGradient")]
    pub radial_gradients: Option<Vec<RadialGradient>>,
}

#[derive(Debug, Deserialize)]
pub struct LinearGradient {
    pub stop: Vec<Stop>,

    #[serde(flatten)]
    pub attrs: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct RadialGradient {
    pub stop: Vec<Stop>,

    #[serde(flatten)]
    pub attrs: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct Stop {
    #[serde(flatten)]
    pub attrs: HashMap<String, String>,
}

impl Defs {
    fn add_gra_for_project(project: &Project) -> LinearGradient {
        let mut lg = LinearGradient {
            stop: vec![],
            attrs: HashMap::new(),
        };

        lg.attrs
            .insert(String::from("id"), format!("gra_proj_{}", project.id));
        lg.attrs.insert(String::from("x1"), String::from("0%"));
        lg.attrs.insert(String::from("y1"), String::from("0%"));
        lg.attrs.insert(String::from("x2"), String::from("100%"));
        lg.attrs.insert(String::from("y2"), String::from("0%"));

        // color mapping: from white to red
        let color_mapping = ColorMapping::new((255, 255, 255), (255, 0,0));

        let n = project.metering.len();
        for i in 0..n {
            let mut stop = Stop {
                attrs: {
                    let mut attrs = HashMap::new();
                    attrs.insert(String::from("offset"), format!("{}%", i * 100 / (n - 1)));
                    attrs.insert(
                        String::from("stop-color"),
                        color_mapping.get_color(project.metering[i]),
                    );
                    attrs
                },
            };
            lg.stop.push(stop);
        }
        return lg;
    }

    pub fn add_gra_for_projects(self, projects: &Vec<Project>) -> Self {
        let mut lgs = self.linear_gradients.unwrap_or(vec![]);
        for p in projects.iter() {
            if p.metering.len() != 0 {
                lgs.push(Defs::add_gra_for_project(p));
            }
        }
        return Defs {
            linear_gradients: Some(lgs),
            radial_gradients: self.radial_gradients,
        };
    }
}

fn u8_to_hex_color_string(red: u8, green: u8, blue: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", red, green, blue)
}

struct ColorMapping{
    pub from: (u8, u8, u8),
    pub to: (u8, u8, u8),
}

impl ColorMapping{
    pub fn new(from: (u8, u8, u8), to: (u8, u8, u8)) -> Self{
        Self{
            from,
            to,
        }
    }
    
    pub fn get_color(&self, ratio: f64) -> String{
        let (r1, g1, b1) = self.from;
        let (r2, g2, b2) = self.to;
        let r = (r1 as f64 + (r2 as f64 - r1 as f64) * ratio) as u8;
        let g = (g1 as f64 + (g2 as f64 - g1 as f64) * ratio) as u8;
        let b = (b1 as f64 + (b2 as f64 - b1 as f64) * ratio) as u8;
        u8_to_hex_color_string(r, g, b)
    }
}

