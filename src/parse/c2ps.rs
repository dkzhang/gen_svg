use crate::element::Coordinate;
use crate::parse::{PointScreen, ScreenUnit};

// Coordinate to PointScreen
#[derive(Debug)]
pub struct C2PS {
    pub regions: Vec<Region>,
}

#[derive(Debug)]
pub struct Region{
    pub top_left_c: Coordinate,
    pub top_left_ps: PointScreen,
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

    pub fn get_ps_min_max(&self) -> (ScreenUnit,ScreenUnit,ScreenUnit,ScreenUnit){
        assert_ne!(self.regions.len(), 0);

        let (mut min_x, mut min_y, mut max_x, mut max_y) = (self.regions[0].top_left_ps.x, self.regions[0].top_left_ps.y, self.regions[0].top_left_ps.x, self.regions[0].top_left_ps.y);
        for r in self.regions.iter(){
            if r.top_left_ps.x < min_x{
                min_x = r.top_left_ps.x;
            }

            if r.top_left_ps.y < min_y{
                min_y = r.top_left_ps.y;
            }

            if r.top_left_ps.x + r.cell_wh_ps.x * r.wh_c.x> max_x{
                max_x = r.top_left_ps.x + r.cell_wh_ps.x * r.wh_c.x;
            }

            if r.top_left_ps.y + r.cell_wh_ps.y * r.wh_c.y> max_y{
                max_y = r.top_left_ps.y + r.cell_wh_ps.y * r.wh_c.y;
            }
        }
        return (min_x, min_y, max_x, max_y);
    }

    pub fn push(&mut self, r:Region) {
        self.regions.push(r);
    }
}

fn convert(r :&Region ,c:&Coordinate) -> Option<[PointScreen;4]>{
    if c.x >= r.top_left_c.x && c.x <  (r.top_left_c.x+r.wh_c.x)&&
        c.y>= r.top_left_c.y && c.y < (r.top_left_c.y+r.wh_c.y) {
        return Some([
            PointScreen{x: r.top_left_ps.x + r.cell_wh_ps.x * c.x, y: r.top_left_ps.y + r.cell_wh_ps.y * c.y},
            PointScreen{x: r.top_left_ps.x + r.cell_wh_ps.x * c.x, y: r.top_left_ps.y + r.cell_wh_ps.y * (c.y+1)},
            PointScreen{x: r.top_left_ps.x + r.cell_wh_ps.x * (c.x+1), y: r.top_left_ps.y + r.cell_wh_ps.y * (c.y+1)},
            PointScreen{x: r.top_left_ps.x + r.cell_wh_ps.x * (c.x+1), y: r.top_left_ps.y + r.cell_wh_ps.y * c.y},
        ]);
    }else {
        return None;
    }
}

// HashMap<Coordinate, Vec<PointScreen>>