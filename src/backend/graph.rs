/* Author: Samuel Resendez
 * The Top level vega object, but as a rust struct!
 */

use serde::ser::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;

use backend::signal::Signal;
use backend::scale::*;
use backend::data::data::Data;
use backend::axis::*;
use backend::constants::*;
use backend::mark::mark::Mark;


/// Graph is the primary structure which encompasses all of the different possible graphs 
/// we can build with this crate.
pub struct Graph {
    schema: &'static str,
    pub description: String,
    pub width: u16,
    pub height: u16,
    pub padding: u16,
    pub autosize: AutoSize,
    pub data: Vec<Data>,
    pub signals: Vec<Signal>,
    pub scales: Vec<Scale>,
    pub axes: Vec<Axis>,
    pub marks: Vec<Mark>,
}

/// Since Grust is built to use the Vega visual specification, Autosize gives the different options
/// available in how to pad the final visualization
/// https://vega.github.io/vega/docs/specification/#autosize
#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AutoSize {
    Pad,
    Fit,
    None,
}


///Provides the different supported GraphTypes by Grust (and the list continues to grow!).
pub enum GraphType {
    Bar,
}


impl Graph {
    /// Creates a new graph, with a bunch of default configurations set up
    /// Once the new graph is created we can go ahead and modify whichever relevant options 
    /// we need to fine tune to create the final graph
    pub fn new(graph_type: GraphType) -> Graph {

        match graph_type {
            GraphType::Bar => Graph {
                schema: "https://vega.github.io/schema/vega/v3.0.json",
                description: "This is the description".to_string(),
                width: 500,
                height: 200,
                padding: 5,
                autosize: AutoSize::Pad,
                signals: vec![],
                data: vec![],
                scales: vec![
                    Scale::new(XSCALE, WIDTH, XCOORD, ScaleType::Band),
                    Scale::new(YSCALE, HEIGHT, YCOORD, ScaleType::None),
                ],
                axes: vec![
                    Axis::new(Orientation::Bottom, XSCALE),
                    Axis::new(Orientation::Left, YSCALE),
                ],
                marks: vec![Mark::new()],
            },
        }

    }
}


/*
 * -------------------------------------------------------------
 * This section begins Trait implementations for the Graph Struct
 * -------------------------------------------------------------
 */
impl Serialize for Graph {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Graph", 11)?;
        let _ = s.serialize_field("$schema", &self.schema);
        let _ = s.serialize_field("description", &self.description);
        let _ = s.serialize_field("width", &self.width);
        let _ = s.serialize_field("height", &self.height);
        let _ = s.serialize_field("padding", &self.padding);
        let _ = s.serialize_field("autosize", &self.autosize);
        let _ = s.serialize_field("data", &self.data);
        let _ = s.serialize_field("signals", &self.signals);
        let _ = s.serialize_field("scales", &self.scales);
        let _ = s.serialize_field("marks", &self.marks);
        let _ = s.serialize_field("axes", &self.axes);
        s.end()
    }
}
