mod parameters;
mod gradient;

pub use parameters::*;
pub use gradient::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub parameters: Parameters,
}
