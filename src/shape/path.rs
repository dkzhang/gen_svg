use crate::config::PathStyle;
use crate::shape::Draw;
use svg::node::element::Path as svg_Path;

pub struct Path<'a> {
    pub id: Option<String>,

    pub d: String,

    pub style: &'a PathStyle,
}

impl Draw for Path<'_> {
    fn draw(&self) -> Box<dyn svg::Node> {
        let mut mp = svg_Path::new().set("d", self.d.clone());

        /////////////////////////////////////////////////////////////
        if let Some(id) = &self.id {
            mp = mp.set("id", id.clone());
        };

        /////////////////////////////////////////////////////////////
        if let Some(fill) = &self.style.fill {
            mp = mp.set("fill", fill.clone());
        };

        if let Some(fill_opacity) = self.style.fill_opacity {
            mp = mp.set("fill-opacity", fill_opacity);
        };

        if let Some(stroke) = &self.style.stroke {
            mp = mp.set("stroke", stroke.clone());
        };

        if let Some(stroke_width) = self.style.stroke_width {
            mp = mp.set("stroke-width", stroke_width);
        };

        if let Some(stroke_linecap) = &self.style.stroke_linecap {
            mp = mp.set("stroke-linecap", stroke_linecap.clone());
        };

        if let Some(stroke_linejoin) = &self.style.stroke_linejoin {
            mp = mp.set("stroke-linejoin", stroke_linejoin.clone());
        };

        if let Some(stroke_dasharray) = &self.style.stroke_dasharray {
            mp = mp.set("stroke-dasharray", stroke_dasharray.clone());
        };

        if let Some(stroke_dashoffset) = self.style.stroke_dashoffset {
            mp = mp.set("stroke-dashoffset", stroke_dashoffset);
        };

        if let Some(stroke_opacity) = self.style.stroke_opacity {
            mp = mp.set("stroke-opacity", stroke_opacity);
        };

        if let Some(transform) = &self.style.transform {
            mp = mp.set("transform", transform.clone());
        };

        return Box::new(mp);
    }
}
