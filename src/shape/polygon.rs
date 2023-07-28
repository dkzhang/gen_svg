use svg::node::element::Polygon as svg_Polygon;
use crate::shape::Draw;

pub struct Polygon<'a> {
    pub id: Option<String>,

    pub points: Vec<(i32, i32)>,

    pub style: &'a PolygonStyle,
}

pub struct PolygonStyle {
    pub fill: Option<String>,
    pub stroke: Option<String>,
    pub stroke_width: Option<i32>,
    pub stroke_opacity: Option<f32>,
    pub fill_opacity: Option<f32>,

    pub transform: Option<String>,
}

impl Draw for Polygon<'_>{
    fn draw(&self) -> Box<dyn svg::Node>{
        let mut mp = svg_Polygon::new()
            .set("points", vec_to_string(&self.points));

        /////////////////////////////////////////////////////////////
        if let Some(id) = &self.id {
            mp = mp.set("id", id.clone());
        };

        /////////////////////////////////////////////////////////////
        if let Some(fill) = &self.style.fill {
            mp = mp.set("fill", fill.clone());
        };

        if let Some(stroke) = &self.style.stroke {
            mp = mp.set("stroke", stroke.clone());
        };

        if let Some(stroke_width) = self.style.stroke_width {
            mp = mp.set("stroke-width", stroke_width);
        };

        if let Some(stroke_opacity) = self.style.stroke_opacity {
            mp = mp.set("stroke-opacity", stroke_opacity);
        };

        if let Some(fill_opacity) = self.style.fill_opacity {
            mp = mp.set("fill-opacity", fill_opacity);
        };

        if let Some(transform) = &self.style.transform {
            mp = mp.set("transform", transform.clone());
        };

        return Box::new(mp);
    }
}

fn vec_to_string(vec: &Vec<(i32, i32)>) -> String {
    let mut s = String::new();
    for (x, y) in vec {
        s.push_str(&format!("{},{} ", x, y));
    }
    return s;
}
