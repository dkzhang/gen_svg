use crate::config::AppConfig;
use crate::element::{Coordinate, CoordinateUnit};
use crate::parse::{C2PS, TableClass};
use crate::shape::{Draw, Line};

pub fn convert_today_line(
    today: CoordinateUnit,
    c2ps: &C2PS,
    ac: &AppConfig,
) -> Vec<Box<dyn Draw>> {
    let para = &ac.parameters;

    let p = c2ps.convert(&Coordinate{x: today, y: 0}).unwrap()[3]; // the top right corner

    let x = p.x;
    let y1 = p.y - para.head_height;
    let y2 = c2ps.get_ps_min_max().3;   //max_y

    let line = Box::new(Line{
        id: None,
        class: vec![TableClass::TodayLine.to_string()],
        x1: x,
        y1: y1,
        x2: x,
        y2: y2,
    });

    return vec![line];
}