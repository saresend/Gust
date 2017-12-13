
use backend::mark::scaler::Scaler;
use backend::constants::*;

#[derive(Serialize)]
pub struct Visualization {
    pub x: Scaler,
    pub width: Scaler,
    pub y: Scaler,
    pub y2: Scaler,
}


impl Visualization {
    pub fn new() -> Visualization {
        Visualization {
            x: Scaler::new(XSCALE,"field", XAXIS),
            width: Scaler::new(XSCALE, "band", "1"),
            y: Scaler::new(YSCALE, "field", YAXIS),
            y2: Scaler::new(YSCALE, "value", "0"),
        }
    }
}