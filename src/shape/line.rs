use crate::shape::{Draw};
use svg::node::element::Line as svg_Line;

pub struct Line{
    pub id: Option<String>,
    pub class: Vec<String>,

    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,

}

impl Draw for Line {
    fn draw(&self) -> Box<dyn svg::Node> {
        let mut ml = svg_Line::new()
            .set("x1", self.x1)
            .set("y1", self.y1)
            .set("x2", self.x2)
            .set("y2", self.y2);

        /////////////////////////////////////////////////////////////
        if let Some(id) = &self.id {
            ml = ml.set("id", id.clone());
        };

        if self.class.is_empty() == false {
            ml = ml.set("class", self.class.join(" "));
        };
        /////////////////////////////////////////////////////////////

        return Box::new(ml);
    }
}
