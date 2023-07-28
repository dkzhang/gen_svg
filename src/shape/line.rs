use svg::node::element::Line as svg_Line;
use crate::shape::Draw;

pub struct Line<'a> {
    pub id: Option<String>,

    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,

    pub style: &'a LineStyle,
}

pub struct LineStyle {
    pub stroke: Option<String>,
    pub stroke_width: Option<i32>,
    pub stroke_opacity: Option<f32>,
    pub stroke_linecap: Option<String>,
    pub stroke_dasharray: Option<String>,
    pub stroke_dashoffset: Option<i32>,

    pub transform: Option<String>,
}

impl Draw for Line<'_>{
    fn draw(&self) -> Box<dyn svg::Node>{
        let mut ml = svg_Line::new()
            .set("x1", self.x1)
            .set("y1", self.y1)
            .set("x2", self.x2)
            .set("y2", self.y2);

        /////////////////////////////////////////////////////////////
        if let Some(id) = &self.id {
            ml = ml.set("id", id.clone());
        };

        /////////////////////////////////////////////////////////////
        if let Some(stroke) = &self.style.stroke {
            ml = ml.set("stroke", stroke.clone());
        };

        if let Some(stroke_width) = self.style.stroke_width {
            ml = ml.set("stroke-width", stroke_width);
        };

        if let Some(stroke_opacity) = self.style.stroke_opacity {
            ml = ml.set("stroke-opacity", stroke_opacity);
        };

        if let  Some(stroke_linecap) = &self.style.stroke_linecap {
            ml = ml.set("stroke-linecap", stroke_linecap.clone());
        };

        if let Some(stroke_dasharray) = &self.style.stroke_dasharray {
            ml = ml.set("stroke-dasharray", stroke_dasharray.clone());
        };

        if let Some(stroke_dashoffset) = self.style.stroke_dashoffset {
            ml = ml.set("stroke-dashoffset", stroke_dashoffset);
        };

        if let Some(transform) = &self.style.transform {
            ml = ml.set("transform", transform.clone());
        };

        return Box::new(ml);
    }
}
