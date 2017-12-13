
use backend::mark::scaler::Scaler;
use backend::constants::*;

#[derive(Serialize)]
pub struct Visualization {
    pub x: Scaler<String>,
    pub width: Scaler<i32>,
    pub y: Scaler<String>,
    pub y2: Scaler<i32>,
}


impl Visualization {
    pub fn new() -> Visualization {
        Visualization {
            x: Scaler::new(XSCALE,"field", XAXIS.to_string()),
            width: Scaler::new(XSCALE, "band", 1),
            y: Scaler::new(YSCALE, "field", YAXIS.to_string()),
            y2: Scaler::new(YSCALE, "value", 0),
        }
    }
}