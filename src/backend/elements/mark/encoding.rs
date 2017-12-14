

use backend::elements::mark::fill::{Fill};
use backend::elements::mark::visualization::Visualization;
use backend::elements::graph::GraphType;

#[derive(Serialize)]
pub struct Encoding {
    pub enter: Visualization,
    pub update: Fill,
    pub hover: Fill,
    
}


impl Encoding {

    pub fn new(graph_type: GraphType) -> Encoding {
        Encoding {
            enter: Visualization::new(graph_type),
            update: Fill::new("steelblue"),
            hover: Fill::new("red"),
        }
    }
}
