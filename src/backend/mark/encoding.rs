

use backend::mark::fill::{Fill};
use backend::mark::visualization::Visualization;


#[derive(Serialize)]
pub struct Encoding {
    pub enter: Visualization,
    pub update: Fill,
    pub hover: Fill,
}


impl Encoding {

    pub fn new() -> Encoding {
        Encoding {
            enter: Visualization::new(),
            update: Fill::new("steelblue"),
            hover: Fill::new("red"),
        }
    }
}
