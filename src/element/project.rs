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
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Hash)]
pub struct ProjectRect {
    pub top_left: PointLogical,
    pub bottom_right: PointLogical,
}

#[derive(Debug, Clone, Hash)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub rects : Vec<ProjectRect>,
}

impl ProjectRect {
    pub fn new(c: &Coordinate) -> ProjectRect {
        ProjectRect {
            top_left: PointLogical { x: c.x, y: c.y },
            bottom_right: PointLogical {
                x: c.x + 1,
                y: c.y + 1,
            },
        }
    }
    pub fn new2(c_tl: Coordinate, c_br: &Coordinate) -> ProjectRect {
        ProjectRect {
            top_left: PointLogical {
                x: c_tl.x,
                y: c_tl.y,
            },
            bottom_right: PointLogical {
                x: c_br.x + 1,
                y: c_br.y + 1,
            },
        }
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
