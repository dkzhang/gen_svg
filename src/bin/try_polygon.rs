use std::fmt;
use log::log;

fn main() {
    let rects = vec![
        Coordinate { x: 0, y: 2 },
        Coordinate { x: 1, y: 2 },
        Coordinate { x: 2, y: 0 },
        Coordinate { x: 2, y: 1 },
        Coordinate { x: 2, y: 2 },
        Coordinate { x: 0, y: 0 },
        Coordinate { x: 0, y: 1 },
    ];

    let polygons = convert_to_polygon(&rects);

    for p in polygons.iter() {
        println!("{}", p);
    }

    println!("----------------------eliminate_merge_edges----------------------");

    let merged = merge(&polygons);

    for p in merged.iter() {
        println!("{}", p);
    }

    println!("----------------------extend_merge_edges----------------------");
    let extended_polygons = merged.iter().map(|p| extend_merge_edges(&p)).collect::<Vec<Polygon>>();
    for p in extended_polygons.iter() {
        println!("{}", p);
    }
}

fn convert_to_polygon(rects: &Vec<Coordinate>) -> Vec<Polygon> {
    let mut polygons: Vec<Polygon> = Vec::new();

    for r in rects.iter() {
        let mut edges: Vec<EdgeLogical> = Vec::new();

        edges.push(EdgeLogical {
            from: PointLogical { x: r.x, y: r.y },
            to: PointLogical { x: r.x , y: r.y +1 },
        });

        edges.push(EdgeLogical {
            from: PointLogical { x: r.x, y: r.y + 1 },
            to: PointLogical { x: r.x + 1, y: r.y + 1 },
        });

        edges.push(EdgeLogical {
            from: PointLogical { x: r.x + 1, y: r.y + 1 },
            to: PointLogical { x: r.x + 1, y: r.y },
        });

        edges.push(EdgeLogical {
            from: PointLogical { x: r.x + 1, y: r.y },
            to: PointLogical { x: r.x, y: r.y },
        });

        polygons.push(Polygon { edges });
    }
    return polygons;
}

fn eliminate_merge_edges(
    polygon1: &Polygon,
    polygon2: &Polygon,
) -> Option<Polygon> {
    let mut result: Vec<EdgeLogical> = Vec::new();
    let edges1 = &polygon1.edges;
    let edges2 = &polygon2.edges;

    for i in 0..edges1.len() {
        for j in 0..edges2.len() {
            if edges1[i].from == edges2[j].to && edges1[i].to == edges2[j].from {
                // found a merge edge

                // eliminate this edge
                for k in 0..i {
                    result.push(edges1[k].clone());
                }

                for k in j + 1..edges2.len() {
                    result.push(edges2[k].clone());
                }

                for k in 0..j {
                    result.push(edges2[k].clone());
                }

                for k in i + 1..edges1.len() {
                    result.push(edges1[k].clone());
                }

                return Some(Polygon { edges: result });
            }
        }
    }
    return None;
}

fn extend_merge_edges(p : &Polygon) -> Polygon{
    if p.edges.len() == 0{
        return Polygon{edges:Vec::new()};
    }

    let mut new_edges :Vec<EdgeLogical> = Vec::new();

    let mut i = 0;
    new_edges.push(p.edges[0].clone());
    for j in 1..p.edges.len(){
        let p1 = &new_edges[i].from.clone();
        let p2 = &new_edges[i].to.clone();
        let p3 = &p.edges[j].to;

        print!("edge1: {}, edge2: {} ==> ", &p.edges[i], &p.edges[j]);
        if is_congruence(p1, p2, p3) {
            println!("can merge,remove edge: {}, push edge: {}", p.edges[i], EdgeLogical { from: p1.clone(), to: p3.clone() });
            new_edges.remove(i);
            new_edges.push(EdgeLogical { from: p1.clone(), to: p3.clone() });
        }else{
            println!("can not merge, push edge: {}", p.edges[j]);

            new_edges.push(p.edges[j].clone());
            i+=1;
        }
    };

    // check the last edge
    let n = p.edges.len();
    let p1 = &p.edges[n-1].from;
    let p2 = &p.edges[n-1].to;
    let p3 = &p.edges[0].to;
    if is_congruence(p1, p2, p3) {
        new_edges.remove(n - 1);
        new_edges.remove(0);
        new_edges.push(EdgeLogical { from: p1.clone(), to: p3.clone() });
    };

    return Polygon{edges:new_edges};
}

fn is_congruence(p1: &PointLogical, p2: &PointLogical, p3: &PointLogical) -> bool {
    return p1.x*(p2.y-p3.y) + p2.x*(p3.y-p1.y) + p3.x*(p1.y-p2.y) == 0;
}

fn merge(polygons :&Vec<Polygon>) -> Vec<Polygon>{
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
             for j in 0..result.len()-1{
                 let merged = eliminate_merge_edges(&result[j], &result[result.len()-1]);
                 if let Some(p) = merged {
                     result.remove(j);
                     result.remove(result.len()-1);
                     result.push(p);
                     to_merge = true;
                     break;
                 }
             }
        }


    }

    return result;
}

#[derive(Debug, Clone, Hash)]
pub struct Polygon {
    pub edges: Vec<EdgeLogical>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PointLogical {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EdgeLogical {
    pub from: PointLogical,
    pub to: PointLogical,
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

impl fmt::Display for PointLogical {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl fmt::Display for EdgeLogical {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})->({},{})", self.from.x, self.from.y, self.to.x, self.to.y)
    }
}

impl fmt::Display for Polygon{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for e in self.edges.iter() {
            result.push_str(&format!("{} ", e));
        }
        write!(f, "{}", result)
    }
}
