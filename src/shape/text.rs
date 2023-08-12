
use crate::shape::Draw;
use svg::node::element::Text as svg_Text;
pub struct Text{
    pub id: Option<String>,
    pub class: Vec<String>,

    pub x: i32,
    pub y: i32,
    pub content: String,


}

impl Draw for Text {
    fn draw(&self) -> Box<dyn svg::Node> {
        let mut mt = svg_Text::new().set("x", self.x).set("y", self.y);

        if let Some(id) = &self.id {
            mt = mt.set("id", id.clone());
        };

        if self.class.is_empty() == false {
            mt = mt.set("class", self.class.join(" "));
        };


        mt = mt.add(svg::node::Text::new(self.content.clone()));

        return Box::new(mt);
    }
}
