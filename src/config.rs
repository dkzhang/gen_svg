mod parameters;

pub use parameters::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub parameters: Parameters,
}
