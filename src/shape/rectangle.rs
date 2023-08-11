use svg::node::element::Rectangle as svg_Rectangle;
use crate::config::RectangleStyle;
use crate::shape::Draw;

pub struct Rectangle<'a> {
    pub id: Option<String>,

    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,

    pub style: &'a RectangleStyle,
}


impl Draw for Rectangle<'_>{
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

        /////////////////////////////////////////////////////////////
        if let Some(fill) = &self.style.fill {
            mr = mr.set("fill", fill.clone());
        }

        if let Some(stroke) = &self.style.stroke {
            mr = mr.set("stroke", stroke.clone());
        }

        if let Some(stroke_width) = self.style.stroke_width {
            mr = mr.set("stroke-width", stroke_width);
        }

        if let Some(stroke_opacity) = self.style.stroke_opacity {
            mr = mr.set("stroke-opacity", stroke_opacity);
        }

        if let Some(fill_opacity) = self.style.fill_opacity {
            mr = mr.set("fill-opacity", fill_opacity);
        }

        if let Some(rx) = self.style.rx {
            mr = mr.set("rx", rx);
        }

        if let Some(ry) = self.style.ry {
            mr = mr.set("ry", ry);
        }

        if let Some(transform) = &self.style.transform {
            mr = mr.set("transform", transform.clone());
        }
        /////////////////////////////////////////////////////////////
        return Box::new(mr);
    }
}
