use crate::element::Coordinate;
use crate::parse::PointScreen;

// Coordinate to PointScreen
#[derive(Debug)]
pub struct C2PS {
    pub regions: Vec<Region>,
}

#[derive(Debug)]
pub struct Region{
    pub top_left_c: Coordinate,
    pub top_right_ps: PointScreen,
    pub wh_c: Coordinate,
    pub cell_wh_ps: PointScreen,
}

impl C2PS{
    pub fn new() -> Self{
        Self{
            regions: Vec::new(),
        }
    }
    pub fn convert(&self, c:&Coordinate) -> Option<[PointScreen;4]>{
        for r in &self.regions{
            if let Some(ps) = convert(r,c){
                return Some(ps);
            }
        }
        return None;
    }

    pub fn push(&mut self, r:Region) {
        self.regions.push(r);
    }
}

fn convert(r :&Region ,c:&Coordinate) -> Option<[PointScreen;4]>{
    if c.x >= r.top_left_c.x && c.x <  (r.top_left_c.x+r.wh_c.x)&&
        c.y>= r.top_left_c.y && c.y < (r.top_left_c.y+r.wh_c.y) {
        return Some([
            PointScreen{x: r.top_right_ps.x + r.cell_wh_ps.x * c.x, y: r.top_right_ps.y + r.cell_wh_ps.y * c.y},
            PointScreen{x: r.top_right_ps.x + r.cell_wh_ps.x * c.x, y: r.top_right_ps.y + r.cell_wh_ps.y * (c.y+1)},
            PointScreen{x: r.top_right_ps.x + r.cell_wh_ps.x * (c.x+1), y: r.top_right_ps.y + r.cell_wh_ps.y * (c.y+1)},
            PointScreen{x: r.top_right_ps.x + r.cell_wh_ps.x * (c.x+1), y: r.top_right_ps.y + r.cell_wh_ps.y * c.y},
        ]);
    }else {
        return None;
    }
}

// HashMap<Coordinate, Vec<PointScreen>>