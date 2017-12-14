

use backend::mark::fill::{Fill};
use backend::mark::visualization::Visualization;

use backend::mark::scaler::Scaler;
use backend::constants::*;


#[derive(Serialize)]
pub struct Encoding {
    pub enter: Visualization,
    pub update: Fill,
    pub hover: Fill,
    pub fill: Scaler<String>,
}


impl Encoding {

    pub fn new() -> Encoding {
        Encoding {
            enter: Visualization::new(),
            update: Fill::new("steelblue"),
            hover: Fill::new("red"),
            fill: Scaler::new(ZSCALE, "field", ZCOORD.to_string()),
        }
    }
}
