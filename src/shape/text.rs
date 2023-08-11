use svg::node::element::Text as svg_Text;
use crate::shape::Draw;
pub struct Text<'a> {
    pub id: Option<String>,

    pub x: i32,
    pub y: i32,
    pub content: String,

    pub style: &'a TextStyle,
}

pub struct TextStyle {
    pub font_family: Option<String>,
    pub font_size: Option<i32>,
    pub text_anchor: Option<String>,
    pub fill: Option<String>,
    pub font_weight: Option<String>,
}

impl Draw for Text<'_> {
    fn draw(&self) -> Box<dyn svg::Node> {
        let mut mr = svg_Text::new()
            .set("x", self.x)
            .set("y", self.y);

        if let Some(id) = &self.id {
            mr = mr.set("id", id.clone());
        };

        if let Some(font_family) = &self.style.font_family {
            mr = mr.set("font-family", font_family.clone());
        };

        if let Some(font_size) = self.style.font_size {
            mr = mr.set("font-size", font_size);
        };

        if let Some(text_anchor) = &self.style.text_anchor {
            mr = mr.set("text-anchor", text_anchor.clone());
        };

        if let Some(fill) = &self.style.fill {
            mr = mr.set("fill", fill.clone());
        };

        if let Some(font_weight) = &self.style.font_weight {
            mr = mr.set("font-weight", font_weight.clone());
        };

        mr = mr.add(svg::node::Text::new(self.content.clone()));

        return Box::new(mr);
    }
}