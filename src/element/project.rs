use std::fmt;
use crate::element::PointLogical;

// pub enum Status {
//     Running,
//     Finished,
//     Waiting,
//     Fault,
// }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: CoordinateUnit,
    pub y: CoordinateUnit,
}

pub type CoordinateUnit = i32;

#[derive(Debug, Clone, Hash)]
pub struct ProjectRect {
    pub top_left: PointLogical,
    pub bottom_right: PointLogical,
    pub status: ProjectStatus,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub rects : Vec<ProjectRect>,

    pub metering: Vec<f64>,
}

impl ProjectRect {
    pub fn new(c: &Coordinate, status:ProjectStatus) -> ProjectRect {
        ProjectRect {
            top_left: PointLogical { x: c.x, y: c.y },
            bottom_right: PointLogical {
                x: c.x + 1,
                y: c.y + 1,
            },
            status,
        }
    }
    pub fn new2(c_tl: Coordinate, c_br: &Coordinate, status:ProjectStatus) -> ProjectRect {
        ProjectRect {
            top_left: PointLogical {
                x: c_tl.x,
                y: c_tl.y,
            },
            bottom_right: PointLogical {
                x: c_br.x + 1,
                y: c_br.y + 1,
            },
            status,
        }
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

pub const PROJECT_NORMAL: u64 = 1;
pub const PROJECT_EXPEDITED:u64 = 1<<1;
pub const PROJECT_HISTORICAL:u64 = 1<<2;
pub const PROJECT_RUNNING:u64 = 1<<3;
pub const PROJECT_QUEUED:u64 = 1<<4;
pub const PROJECT_DELAYED:u64 = 1<<5;
pub const PROJECT_FAULT:u64 = 1<<6;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ProjectStatus(u64);

impl ProjectStatus {
    pub fn new(cs: Vec<u64>) -> Self{
        Self(cs.iter().fold(0, |acc, c| acc | c))
    }

    pub fn add(mut self, c: u64) -> Self{
        Self(self.0 | c)
    }

    pub fn remove(mut self, c: u64)-> Self{
        Self(self.0 & !c)
    }

    pub fn to_vs(&self) -> Vec<String>{
        let mut classes = Vec::new();
        if self.0 & PROJECT_NORMAL != 0{
            classes.push("dk-project-normal".to_string());
        }
        if self.0 & PROJECT_EXPEDITED != 0{
            classes.push("dk-project-expedited".to_string());
        }
        if self.0 & PROJECT_HISTORICAL != 0{
            classes.push("dk-project-historical".to_string());
        }
        if self.0 & PROJECT_RUNNING != 0{
            classes.push("dk-project-running".to_string());
        }
        if self.0 & PROJECT_QUEUED != 0{
            classes.push("dk-project-queued".to_string());
        }
        if self.0 & PROJECT_DELAYED != 0{
            classes.push("dk-project-delayed".to_string());
        }
        if self.0 & PROJECT_FAULT != 0{
            classes.push("dk-project-fault".to_string());
        }
        return classes;
    }

    pub fn to_string(&self) -> String{
        return self.to_vs().join(" ");
    }
}
