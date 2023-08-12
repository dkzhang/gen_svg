use crate::shape::Draw;
use svg::node::element::Rectangle as svg_Rectangle;

pub struct Rectangle {
    pub id: Option<String>,
    pub class: Vec<String>,

    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Draw for Rectangle {
    fn draw(&self) -> Box<dyn svg::Node> {
        let mut mr = svg_Rectangle::new()
            .set("x", self.x)
            .set("y", self.y)
            .set("width", self.width)
            .set("height", self.height);

        /////////////////////////////////////////////////////////////
        if let Some(id) = &self.id {
            mr = mr.set("id", id.clone());
        }

        if self.class.is_empty() == false {
            mr = mr.set("class", self.class.join(" "));
        };

        /////////////////////////////////////////////////////////////

        return Box::new(mr);
    }
}
