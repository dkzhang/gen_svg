use crate::element::PointLogical;
use crate::get_projects::project::Project0;
use crate::my_utils::date::{Date70, int_to_date70};
use crate::my_utils::device::expand_abbreviation;
use std::collections::HashMap;
use std::fmt;
use serde::{Deserialize, Serialize};

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
    pub rects: Vec<ProjectRect>,

    pub metering: Vec<f64>,
}

impl Project {
    pub fn new(
        project0: &Project0,
        row_index_map: &HashMap<String, i32>,
        col_index_map: &HashMap<Date70, i32>,
    ) -> Self {
        let mut project = Project {
            id: project0.id.clone(),
            name: project0.name.clone(),
            rects: vec![],
            metering: project0.metering.clone(),
        };

        let date_min = col_index_map.keys().min().unwrap().clone();
        let date_max = col_index_map.keys().max().unwrap().clone();

        for rect in project0.rects.iter() {
            let mut date_from = int_to_date70(rect.date_from).unwrap();
            let mut date_to = int_to_date70(rect.date_to).unwrap();

            // check if the rect is out of date range
            if date_from > date_max || date_to < date_min {
                continue;
            }

            // change the date range to the date range of the table
            if date_from < date_min {
                date_from = date_min;
            }
            if date_to > date_max {
                date_to = date_max;
            }

            let date_from_index = col_index_map.get(&date_from).unwrap().clone();
            let date_to_index = col_index_map.get(&date_to).unwrap().clone();

            let d = expand_abbreviation(&rect.devices);
            log::info!("row_index_map = {:?}", row_index_map);
            log::info!("col_index_map = {:?}", col_index_map);
            log::info!("date_min = {}, date_max = {}", date_min, date_max);
            log::info!("date_from = {}, date_to = {}", date_from, date_to);
            log::info!("expand_abbreviation(&rect.devices) = {:?}", d);

            let mut devices_index = expand_abbreviation(&rect.devices)
                .iter()
                .map(|d| row_index_map.get(d).unwrap())
                .cloned()
                .collect::<Vec<_>>();
            devices_index.sort();

            // merge devices_index
            let mut a : Option<i32> = None;
            let mut b : Option<i32> = None;

            for i in 0..devices_index.len(){
                if let Some(b0) = b{
                    if devices_index[i] == b0 + 1 {
                        b = Some(devices_index[i]);
                    }else{
                        project.rects.push(ProjectRect::new2(
                            Coordinate {
                                x: date_from_index,
                                y: a.unwrap(),
                            },
                            &Coordinate {
                                x: date_to_index,
                                y: b.unwrap(),
                            },
                            rect.status.clone(),
                        ));
                        a = Some(devices_index[i]);
                        b = Some(devices_index[i]);
                    }
                } else{
                    a = Some(devices_index[i]);
                    b = Some(devices_index[i]);
                }
            }
            // push the last rect
            if let Some(a0) = a{
                project.rects.push(ProjectRect::new2(
                    Coordinate {
                        x: date_from_index,
                        y: a0,
                    },
                    &Coordinate {
                        x: date_to_index,
                        y: b.unwrap(),
                    },
                    rect.status.clone(),
                ));
            }
        }
        return project;
    }

    pub fn new_vec(
        projects0: &Vec<Project0>,
        row_index_map: &HashMap<String, i32>,
        col_index_map: &HashMap<Date70, i32>,
    ) -> Vec<Project> {
        return projects0
            .iter()
            .map(|gp| Project::new(gp, row_index_map, col_index_map))
            .collect::<Vec<_>>();
    }
}

impl ProjectRect {
    pub fn new(c: &Coordinate, status: ProjectStatus) -> ProjectRect {
        ProjectRect {
            top_left: PointLogical { x: c.x, y: c.y },
            bottom_right: PointLogical {
                x: c.x + 1,
                y: c.y + 1,
            },
            status,
        }
    }
    pub fn new2(c_tl: Coordinate, c_br: &Coordinate, status: ProjectStatus) -> ProjectRect {
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
pub const PROJECT_EXPEDITED: u64 = 1 << 1;
pub const PROJECT_HISTORICAL: u64 = 1 << 2;
pub const PROJECT_RUNNING: u64 = 1 << 3;
pub const PROJECT_QUEUED: u64 = 1 << 4;
pub const PROJECT_DELAYED: u64 = 1 << 5;
pub const PROJECT_FAULT: u64 = 1 << 6;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectStatus(u64);

impl ProjectStatus {
    pub fn new(cs: Vec<u64>) -> Self {
        Self(cs.iter().fold(0, |acc, c| acc | c))
    }

    pub fn add(mut self, c: u64) -> Self {
        Self(self.0 | c)
    }

    pub fn remove(mut self, c: u64) -> Self {
        Self(self.0 & !c)
    }

    pub fn has(&self, c:u64) -> bool{
        self.0 & c != 0
    }

    pub fn to_vs(&self) -> Vec<String> {
        let mut classes = Vec::new();
        if self.0 & PROJECT_NORMAL != 0 {
            classes.push("dk-project-normal".to_string());
        }
        if self.0 & PROJECT_EXPEDITED != 0 {
            classes.push("dk-project-expedited".to_string());
        }
        if self.0 & PROJECT_HISTORICAL != 0 {
            classes.push("dk-project-historical".to_string());
        }
        if self.0 & PROJECT_RUNNING != 0 {
            classes.push("dk-project-running".to_string());
        }
        if self.0 & PROJECT_QUEUED != 0 {
            classes.push("dk-project-queued".to_string());
        }
        if self.0 & PROJECT_DELAYED != 0 {
            classes.push("dk-project-delayed".to_string());
        }
        if self.0 & PROJECT_FAULT != 0 {
            classes.push("dk-project-fault".to_string());
        }
        return classes;
    }

    pub fn to_string(&self) -> String {
        return self.to_vs().join(" ");
    }
}
