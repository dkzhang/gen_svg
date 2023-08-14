use log::log;
use std::collections::HashMap;
use std::fmt;

fn main() {
    let rects = vec![
        Rectangle::new(Coordinate { x: 0, y: 2 }),
        Rectangle::new(Coordinate { x: 1, y: 2 }),
        Rectangle::new(Coordinate { x: 2, y: 0 }),
        Rectangle::new(Coordinate { x: 2, y: 1 }),
        Rectangle::new(Coordinate { x: 2, y: 2 }),
        Rectangle::new(Coordinate { x: 0, y: 0 }),
        Rectangle::new(Coordinate { x: 0, y: 1 }),
    ];

    let polygons = rects
        .iter()
        .map(|r| convert_rect_to_polygon(&r))
        .collect::<Vec<Polygon>>();

    for p in polygons.iter() {
        println!("{}", p);
    }

    println!("----------------------eliminate_merge_edges----------------------");

    let merged = merge(&polygons);

    for p in merged.iter() {
        println!("{}", p);
    }

    println!("----------------------extend_merge_edges----------------------");
    let extended_polygons = merged
        .iter()
        .map(|p| extend_merge_edges(&p))
        .collect::<Vec<Polygon>>();
    for p in extended_polygons.iter() {
        println!("{}", p);
    }

    println!("----------------------turn_analysis----------------------");
    for p in extended_polygons.iter() {
        let turn_map = turn_analysis(p);
        for (k, v) in turn_map.iter() {
            println!("{}: {:?}", k, v);
        }
    }

}

fn convert_rect_to_polygon(rect: &Rectangle) -> Polygon {
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

    return Polygon { points };
}

fn eliminate_merge_edges(polygon1: &Polygon, polygon2: &Polygon) -> Option<Polygon> {
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

                return Some(Polygon { points: result });
            }
        }
    }
    return None;
}

fn extend_merge_edges(p: &Polygon) -> Polygon {
    if p.points.len() < 3 {
        return p.clone();
    }

    let mut points: Vec<PointLogical> = p.points.clone();

    let (mut i, mut n) = (0, points.len());
    while i < n {
        let p1 = &points[i];
        let p2 = &points[(i + 1) % n];
        let p3 = &points[(i + 2) % points.len()];

        if is_congruence(p1, p2, p3) {
            points.remove((i + 1) % n);
            n -= 1;
        } else {
            i += 1;
        }
    }

    return Polygon { points };
}

fn is_congruence(p1: &PointLogical, p2: &PointLogical, p3: &PointLogical) -> bool {
    return p1.x * (p2.y - p3.y) + p2.x * (p3.y - p1.y) + p3.x * (p1.y - p2.y) == 0;
}

fn merge(polygons: &Vec<Polygon>) -> Vec<Polygon> {
    let mut result: Vec<Polygon> = Vec::new();

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
                let merged = eliminate_merge_edges(&result[j], &result[result.len() - 1]);
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

fn turn_analysis(p: &Polygon) -> HashMap<PointLogical, (Direction, Direction)> {
    let mut result = HashMap::new();

    let n = p.points.len();
    for i in 0..n {
        let prev = &p.points[(i + n - 1) % n];
        let next = &p.points[(i + 1) % n];
        let cur = &p.points[i];

        result.insert(
            cur.clone(),
            (ComputeDirection(prev, cur), ComputeDirection(cur, next)),
        );
    }

    return result;
}

fn ComputeDirection(p1: &PointLogical, p2: &PointLogical) -> Direction {
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

#[derive(Debug, Clone, Hash)]
pub struct Rectangle {
    pub top_left: PointLogical,
    pub bottom_right: PointLogical,
}

#[derive(Debug, Clone, Hash)]
pub struct Polygon {
    pub points: Vec<PointLogical>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PointLogical {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PointScreen {
    pub x: i32,
    pub y: i32,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Rectangle {
    pub fn new(c: Coordinate) -> Rectangle {
        Rectangle {
            top_left: PointLogical { x: c.x, y: c.y },
            bottom_right: PointLogical {
                x: c.x + 1,
                y: c.y + 1,
            },
        }
    }
    pub fn new2(c_tl: Coordinate, c_br: Coordinate) -> Rectangle {
        Rectangle {
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

impl fmt::Display for PointLogical {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl fmt::Display for Polygon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for p in self.points.iter() {
            result.push_str(&format!("{}->", p));
        }
        write!(f, "{}", result)
    }
}
