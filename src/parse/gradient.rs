use svg::Node;
use crate::config::{Defs};
pub fn convert_to_gradient(defs: Defs) -> svg::node::element::Definitions {
    let mut result = svg::node::element::Definitions::new();

    if let Some(lgs) = defs.linear_gradients{
        for lg in lgs.iter(){
            let mut gradient = svg::node::element::LinearGradient::new();
            for (k, v) in lg.attrs.iter(){
                gradient.assign(k.clone(), v.clone());
            }
            for s in lg.stop.iter(){
                let mut stop = svg::node::element::Stop::new();
                for (k, v) in s.attrs.iter(){
                    stop.assign(k.clone(), v.clone());
                }
                gradient.append(stop);
            }
            result.append(gradient);
        }
    }

    if let Some(rgs) = defs.radial_gradients{
        for rg in rgs.iter(){
            let mut gradient = svg::node::element::RadialGradient::new();
            for (k, v) in rg.attrs.iter(){
                gradient.assign(k.clone(), v.clone());
            }
            for s in rg.stop.iter(){
                let mut stop = svg::node::element::Stop::new();
                for (k, v) in s.attrs.iter(){
                    stop.assign(k.clone(), v.clone());
                }
                gradient.append(stop);
            }
            result.append(gradient);
        }
    }
    return result
}
