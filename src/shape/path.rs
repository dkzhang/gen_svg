
use crate::shape::Draw;
use svg::node::element::Path as svg_Path;

pub struct Path{
    pub id: Option<String>,
    pub class: Vec<String>,

    pub d: String,
}

impl Draw for Path {
    fn draw(&self) -> Box<dyn svg::Node> {
        let mut mp = svg_Path::new().set("d", self.d.clone());

        /////////////////////////////////////////////////////////////
        if let Some(id) = &self.id {
            mp = mp.set("id", id.clone());
        };

        if self.class.is_empty() == false {
            mp = mp.set("class", self.class.join(" "));
        };

        return Box::new(mp);
    }
}
