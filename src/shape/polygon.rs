
use crate::shape::Draw;
use svg::node::element::Polygon as svg_Polygon;
use crate::parse::PointScreen;

pub struct Polygon {
    pub id: Option<String>,
    pub class: Vec<String>,


    pub points: Vec<PointScreen>,
}

impl Draw for Polygon {
    fn draw(&self) -> Box<dyn svg::Node> {
        let mut mp = svg_Polygon::new().set("points", vec_point_to_string(&self.points));

        /////////////////////////////////////////////////////////////
        if let Some(id) = &self.id {
            mp = mp.set("id", id.clone());
        };

        if self.class.is_empty() == false {
            mp = mp.set("class", self.class.join(" "));
        };

        /////////////////////////////////////////////////////////////
        return Box::new(mp);
    }
}

fn vec_point_to_string(vec: &Vec<PointScreen>) -> String {
    let mut s = String::new();
    for p in vec {
        s.push_str(&format!("{},{} ", p.x, p.y));
    }
    return s;
}
