use log::log;
use std::collections::HashMap;
use std::fmt;
use svg::node::element::tag::Rectangle;

fn main() {
    let rects = vec![
        // Rectangle::new(Coordinate { x: 0, y: 2 }),
        // Rectangle::new(Coordinate { x: 1, y: 2 }),
        // Rectangle::new(Coordinate { x: 2, y: 0 }),
        // Rectangle::new(Coordinate { x: 2, y: 1 }),
        // Rectangle::new(Coordinate { x: 2, y: 2 }),
        // Rectangle::new(Coordinate { x: 0, y: 0 }),
        // Rectangle::new(Coordinate { x: 0, y: 1 }),
        Rectangle::new2(&Coordinate { x: 0, y: 0 }, &Coordinate { x: 0, y: 2 }),
        Rectangle::new2(&Coordinate { x: 1, y: 2 }, &Coordinate { x: 2, y: 2 }),
        Rectangle::new2(&Coordinate { x: 2, y: 0 }, &Coordinate { x: 2, y: 1 }),
        Rectangle::new(&Coordinate { x: 1, y: 0 }),
        Rectangle::new(&Coordinate { x: 1, y: 1 }),
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

fn eliminate_merge_edges2(polygon1: &Polygon, polygon2: &Polygon) -> Option<Polygon> {
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

                return Some(Polygon { points: result });
            };
        }
    }
    return None;
}

fn extend_merge_edges(p: &Polygon) -> Polygon {
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

fn turn_analysis(p: &Polygon) -> HashMap<PointLogical, (Direction, Direction)> {
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
        (Direction::Up, Direction::Right) => (0, 0, 0),
        (Direction::Down, Direction::Left) => (-1, -1, 2),
        (Direction::Down, Direction::Right) => (0, -1, 1),
        (Direction::Left, Direction::Up) => (0, -1, 1),
        (Direction::Left, Direction::Down) => (0, 0, 0),
        (Direction::Right, Direction::Up) => (-1, -1, 2),
        (Direction::Right, Direction::Down) => (-1, 0, 3),
        _ => {
            panic!("impossible")
        }
    };
    return (dx, dy, i);
}

fn coordinate_conversion(
    point: &PointLogical,
    d2: &(Direction, Direction),
    c2p: &HashMap<Coordinate, Vec<PointScreen>>,
) -> PointScreen {
    let (dx, dy, i) = coordinate_conversion_aux(d2);
    let c = Coordinate {
        x: point.x + dx,
        y: point.y + dy,
    };

    return c2p.get(&c).expect("point does doesn't in HashMap")[i as usize].clone();
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
    pub fn new(c: &Coordinate) -> Rectangle {
        Rectangle {
            top_left: PointLogical { x: c.x, y: c.y },
            bottom_right: PointLogical {
                x: c.x + 1,
                y: c.y + 1,
            },
        }
    }
    pub fn new2(c_tl: &Coordinate, c_br: &Coordinate) -> Rectangle {
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
