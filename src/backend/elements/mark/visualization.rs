
use backend::elements::mark::scaler::Scaler;
use backend::elements::constants::*;

use backend::elements::graph::{GraphType};

#[derive(Serialize)]
pub struct Visualization {
    pub x: Scaler<String>,
    pub width: Scaler<i32>,
    pub y: Scaler<String>,
    pub y2: Scaler<i32>,
    pub fill: Option<Scaler<String>>,
}


impl Visualization {
    pub fn new(graph_type: GraphType) -> Visualization {
        match graph_type {
            GraphType::Bar => {
                Visualization {
                x: Scaler::new(XSCALE,"field", XCOORD.to_string()),
                width: Scaler::new(XSCALE, "band", 1),
                y: Scaler::new(YSCALE, "field", YCOORD.to_string()),
                y2: Scaler::new(YSCALE, "value", 0),
                fill: Some(Scaler::new(ZSCALE, "field", ZCOORD.to_string())),
            }
            },
            GraphType::StackedBar => {
                    Visualization {
                    x: Scaler::new(XSCALE,"field", XCOORD.to_string()),
                    width: Scaler::new(XSCALE, "band", 1),
                    y: Scaler::new(YSCALE, "field", YCOORD.to_string()),
                    y2: Scaler::new(YSCALE, "value", 0),
                    fill: Some(Scaler::new(ZSCALE, "field", ZCOORD.to_string())),
                }
            }
        }
        
    }
}