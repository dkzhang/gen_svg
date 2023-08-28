use crate::config::AppConfig;
use crate::element::{Coordinate, PointLogical, Project, ProjectRect};
use crate::parse::{C2PS, PointScreen};
use crate::shape::{Draw, Polygon, Rectangle, Text};
use log::log;
use std::collections::HashMap;
use std::fmt;

pub fn convert_project(
    project: &Project,
    c2ps: &C2PS,
    ac: &AppConfig,
) -> Vec<Box<dyn Draw>> {
    let polygons = project
        .rects
        .iter()
        .map(|r| convert_rect_to_polygon(&r))
        .collect::<Vec<ProjectPolygon>>();

    let merged = merge(&polygons);

    let extended_polygons = merged
        .iter()
        .map(|p| extend_merge_edges(&p))
        .collect::<Vec<ProjectPolygon>>();


    for p in extended_polygons.iter() {
        println!("extended_polygons: {}", p)
    }

    // covert each polygon to shape Rectangle or Polygon
    let mut result: Vec<Box<dyn Draw>> = Vec::new();

    for (i, p) in extended_polygons.iter().enumerate() {
        result.append(&mut convert_to_vd(
            &p,
            &c2ps,
            &format!("project_{}_{}",project.id, i),
            &project.name,
            ac,
        ));
    }

    return result;
}

fn convert_to_vd(
    p: &ProjectPolygon,
    c2ps: &C2PS,
    id: &String,
    name: &String,
    ac: &AppConfig,
) -> Vec<Box<dyn Draw>> {
    let mut result: Vec<Box<dyn Draw>> = Vec::new();
    let turn_map = turn_analysis(p);
    let spacing = PointScreen {
        x: ac.parameters.project_spacing_width,
        y: ac.parameters.project_spacing_height,
    };

    if is_rectangle(p) {
        println!("p.points: {:?}", p.points);

        let top_left = coordinate_conversion(&p.points[0], &turn_map[&p.points[0]], &spacing,&c2ps);
        let bottom_right = coordinate_conversion(&p.points[2], &turn_map[&p.points[2]], &spacing,&c2ps);

        let (width, height) = (
            bottom_right.x - top_left.x,
            bottom_right.y - top_left.y,
        );

        let rect = Box::new(Rectangle {
            id: None,
            class: vec![ProjectClass::Project.to_string()],
            x: top_left.x,
            y:top_left.y,
            width,
            height,
        });
        result.push(rect);

        let text = Box::new(Text {
            id: Some(id.clone()),
            class: vec![ProjectClass::ProjectText.to_string()],
            x: top_left.x + width / 2,
            y: top_left.y + height / 2,
            content: name.clone(),
        });
        result.push(text);
    } else {
        let polygon = Box::new(Polygon {
            id: None,
            class: vec![ProjectClass::Project.to_string()],
            points: p.points.iter().map(|p| coordinate_conversion(p, &turn_map[p], &spacing,&c2ps)).collect(),
        });

        println!("polygon: {:?}", polygon.points);
        let rc = find_suitable_rect_center(&polygon.points);

        result.push(polygon);

        let text = Box::new(Text {
            id: Some(id.clone()),
            class: vec![ProjectClass::ProjectText.to_string()],
            x: rc.x,
            y: rc.y,
            content: name.clone(),
        });
        result.push(text);
    }

    return result;
}

fn convert_rect_to_polygon(rect: &ProjectRect) -> ProjectPolygon {
    let points = vec![
        PointLogical {
            x: rect.top_left.x,
            y: rect.top_left.y,
        },
        PointLogical {
            x: rect.top_left.x,
            y: rect.bottom_right.y,
        },
        PointLogical {
            x: rect.bottom_right.x,
            y: rect.bottom_right.y,
        },
        PointLogical {
            x: rect.bottom_right.x,
            y: rect.top_left.y,
        },
    ];

    return ProjectPolygon { points };
}

fn eliminate_merge_edges(
    polygon1: &ProjectPolygon,
    polygon2: &ProjectPolygon,
) -> Option<ProjectPolygon> {
    let mut result: Vec<PointLogical> = Vec::new();

    let points1 = &polygon1.points;
    let points2 = &polygon2.points;

    // check if there is at least 3 points
    if points1.len() < 3 || points2.len() < 3 {
        return None;
    }

    let (n1, n2) = (points1.len(), points2.len());

    for i in 0..n1 {
        for j in 0..n2 {
            if points1[i] == points2[j] && points1[(i + 1) % n1] == points2[(n2 + j - 1) % n2] {
                // found a merge edge

                // eliminate this edge
                for k in 0..=i {
                    result.push(points1[k].clone());
                }

                for k in j + 1..points2.len() {
                    result.push(points2[k].clone());
                }

                if j > 1 {
                    for k in 0..(j - 1) {
                        result.push(points2[k].clone());
                    }
                }

                for k in i + 1..points1.len() {
                    result.push(points1[k].clone());
                }

                return Some(ProjectPolygon { points: result });
            }
        }
    }
    return None;
}

fn eliminate_merge_edges2(
    polygon1: &ProjectPolygon,
    polygon2: &ProjectPolygon,
) -> Option<ProjectPolygon> {
    let mut result: Vec<PointLogical> = Vec::new();

    let points1 = &polygon1.points;
    let points2 = &polygon2.points;

    // check if there is at least 3 points
    if points1.len() < 3 || points2.len() < 3 {
        return None;
    }

    let (n1, n2) = (points1.len(), points2.len());

    for i in 0..n1 {
        for j in 0..n2 {
            if detect_congruence_overlap_inverse(
                &points1[i],
                &points1[(i + 1) % n1],
                &points2[j],
                &points2[(j + 1) % n2],
            ) {
                // found a merge edge
                // eliminate this edge
                for k in 0..=i {
                    result.push(points1[k].clone());
                }

                if points1[i] != points2[(j + 1) % n2] {
                    result.push(points2[(j + 1) % n2].clone());
                }

                for k in j + 2..points2.len() {
                    result.push(points2[k].clone());
                }

                for k in 0..j {
                    result.push(points2[k].clone());
                }

                if points1[(i + 1) % n1] != points2[j] {
                    result.push(points2[j].clone());
                }

                for k in i + 1..points1.len() {
                    result.push(points1[k].clone());
                }

                return Some(ProjectPolygon { points: result });
            };
        }
    }
    return None;
}

fn extend_merge_edges(p: &ProjectPolygon) -> ProjectPolygon {
    if p.points.len() < 3 {
        return p.clone();
    }

    let mut points: Vec<PointLogical> = p.points.clone();

    let (mut i, mut n, mut exist_remove) = (0, points.len(), false);

    loop {
        let p1 = &points[i];
        let p2 = &points[(i + 1) % n];
        let p3 = &points[(i + 2) % points.len()];

        if is_congruence(p1, p2, p3) || *p1 == *p2 {
            points.remove((i + 1) % n);
            exist_remove = true;
            n -= 1;
        } else {
            i += 1;

            if i >= n {
                if exist_remove {
                    // is exist remove, scan again
                    i = 0;
                    exist_remove = false;
                } else {
                    // break after a no remove full scan
                    break;
                }
            }
        }
    }

    return ProjectPolygon { points };
}

fn is_congruence(p1: &PointLogical, p2: &PointLogical, p3: &PointLogical) -> bool {
    return p1.x * (p2.y - p3.y) + p2.x * (p3.y - p1.y) + p3.x * (p1.y - p2.y) == 0;
}

fn is_rectangle(p: &ProjectPolygon) -> bool {
    if p.points.len() != 4 {
        return false;
    }

    return true;
}

fn merge(polygons: &Vec<ProjectPolygon>) -> Vec<ProjectPolygon> {
    let mut result: Vec<ProjectPolygon> = Vec::new();

    if polygons.len() == 0 {
        return result;
    }

    result.push(polygons[0].clone());

    for i in 1..polygons.len() {
        let mut to_merge = true;
        result.push(polygons[i].clone());

        while to_merge {
            to_merge = false;
            for j in 0..result.len() - 1 {
                let merged = eliminate_merge_edges2(&result[j], &result[result.len() - 1]);
                if let Some(p) = merged {
                    result.remove(j);
                    result.remove(result.len() - 1);
                    result.push(p);
                    to_merge = true;

                    break;
                }
            }
        }
    }

    return result;
}

fn turn_analysis(p: &ProjectPolygon) -> HashMap<PointLogical, (Direction, Direction)> {
    let mut result = HashMap::new();

    let n = p.points.len();
    for i in 0..n {
        let prev = &p.points[(i + n - 1) % n];
        let next = &p.points[(i + 1) % n];
        let cur = &p.points[i];

        result.insert(
            cur.clone(),
            (compute_direction(prev, cur), compute_direction(cur, next)),
        );
    }

    return result;
}

fn compute_direction(p1: &PointLogical, p2: &PointLogical) -> Direction {
    if p1.x < p2.x {
        return Direction::Right;
    } else if p1.x > p2.x {
        return Direction::Left;
    } else {
        if p1.y < p2.y {
            return Direction::Down;
        } else if p1.y > p2.y {
            return Direction::Up;
        } else {
            panic!("p1 == p2");
        }
    }
}

fn is_in_range(x1: i32, x2: i32, x: i32) -> bool {
    return if x1 < x2 {
        x1 <= x && x <= x2
    } else {
        x2 <= x && x <= x1
    };
}

fn detect_inverse(
    p1: &PointLogical,
    p2: &PointLogical,
    p3: &PointLogical,
    p4: &PointLogical,
) -> bool {
    if (p2.x - p1.x) * (p4.y - p3.y) + (p2.y - p1.y) * (p3.x - p4.x) != 0 {
        return false;
    };

    if p1.y == p2.y && (p1.x - p2.x) * (p3.x - p4.x) < 0 {
        return true;
    };

    if p1.x == p2.x && (p1.y - p2.y) * (p3.y - p4.y) < 0 {
        return true;
    };

    return false;
}
fn detect_congruence_overlap_inverse(
    p1: &PointLogical,
    p2: &PointLogical,
    p3: &PointLogical,
    p4: &PointLogical,
) -> bool {
    // detect inverse
    if detect_inverse(p1, p2, p3, p4) == false {
        return false;
    };

    // detect congruence
    if is_congruence(p1, p2, p3) == false {
        return false;
    };

    // detect overlap
    return if p1.x == p2.x {
        is_in_range(p1.y, p2.y, p3.y)
            || is_in_range(p1.y, p2.y, p4.y)
            || is_in_range(p3.y, p4.y, p1.y)
            || is_in_range(p3.y, p4.y, p2.y)
    } else {
        is_in_range(p1.x, p2.x, p3.x)
            || is_in_range(p1.x, p2.x, p4.x)
            || is_in_range(p3.x, p4.x, p1.x)
            || is_in_range(p3.x, p4.x, p2.x)
    };
}

fn coordinate_conversion_aux(d2: &(Direction, Direction)) -> (i32, i32, i32) {
    // 0,1,2,3 -> top_left, bottom_left, bottom_right, top_right
    let (mut dx, mut dy, mut i) = match d2 {
        (Direction::Up, Direction::Left) => (-1, 0, 3),
        (Direction::Up, Direction::Right) => (-1, -1, 2),
        (Direction::Down, Direction::Left) => (0, 0, 0),
        (Direction::Down, Direction::Right) => (0, -1, 1),
        (Direction::Left, Direction::Up) => (-1, 0, 3),
        (Direction::Left, Direction::Down) => (0, 0, 0),
        (Direction::Right, Direction::Up) => (-1, -1, 2),
        (Direction::Right, Direction::Down) => (0, -1, 1),
        _ => {
            panic!("impossible")
        }
    };
    return (dx, dy, i);
}

fn coordinate_conversion(
    point: &PointLogical,
    d2: &(Direction, Direction),
    spacing: &PointScreen,
    c2ps: &C2PS,
) -> PointScreen {
    let (dx, dy, i) = coordinate_conversion_aux(d2);
    let c = Coordinate {
        x: point.x + dx,
        y: point.y + dy,
    };

    let p = c2ps.convert(&c).expect("point does doesn't in C2PS");

    println!("c: {:?} ||  p: {:?}",c, p);

    let ps = match i {
        0 =>{
            PointScreen{
                x: p[0].x + spacing.x,
                y: p[0].y + spacing.y,
            }
        }
        1 =>{
            PointScreen{
                x: p[1].x + spacing.x,
                y: p[1].y - spacing.y,
            }
        }
        2 =>{
            PointScreen{
                x: p[2].x - spacing.x,
                y: p[2].y - spacing.y,
            }
        }
        3=>{
            PointScreen{
                x: p[3].x - spacing.x,
                y: p[3].y + spacing.y,
            }
        }
        _ => {panic!("impossible")}
    };

    return ps
}

// finding a suitable rectangle in a polygon, return it's center
fn find_suitable_rect_center(vps: &Vec<PointScreen>) -> PointScreen{
    let mut upwards:Vec<(&PointScreen, &PointScreen)> = Vec::new();
    let mut downwards:Vec<(&PointScreen, &PointScreen)> = Vec::new();

    for i in 0..vps.len(){
        let p0 = &vps[i];
        let p1 = &vps[(i+1)%vps.len()];
        if p0.x == p1.x{
            if p0.y < p1.y{
                downwards.push((p0, p1));
            }else{
                upwards.push((p0, p1));
            }
        }
    }

    let mut result = PointScreen{x:0,y:0};
    let mut max_width = 0;

    for up in upwards.iter(){
        for down in downwards.iter(){
            // the down line must on the left of the up line
            if up.0.x <= down.0.x{
                continue;
            }

            let width = up.0.x - down.0.x;
            if width <= max_width{
                continue;
            }

            // find intersection of y
            if down.0.y >= up.0.y || down.1.y <= up.1.y{
                // no intersection
                continue;
            }

            let y = std::cmp::max(down.0.y, up.1.y);
            let yy = std::cmp::min(down.1.y, up.0.y);

            max_width = width;
            result = PointScreen{x:(up.0.x + down.0.x)/2, y:(y + yy)/2};
        }
    }
    assert_ne!(max_width, 0);

    return result;
}

#[derive(Debug, Clone, Hash)]
pub struct ProjectPolygon {
    pub points: Vec<PointLogical>,
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl fmt::Display for ProjectPolygon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for p in self.points.iter() {
            result.push_str(&format!("{}->", p));
        }
        write!(f, "{}", result)
    }
}

pub enum ProjectClass{
    Project,
    ProjectText,
    StatusRunning,
}

impl ProjectClass{
    pub fn to_string(&self) -> String{
        match self{
            ProjectClass::Project => "project".to_string(),
            ProjectClass::StatusRunning => "status_running".to_string(),
            ProjectClass::ProjectText => "project-text".to_string(),
        }
    }
}
