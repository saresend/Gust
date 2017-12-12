/* Author: Samuel Resendez
 * The Top level vega object, but as a rust struct!
 */

use serde::ser::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;

use backend::signal::Signal;
use backend::scale::Scale;
use backend::axis::*;
use backend::constants::*;


pub struct Graph {
    schema:&'static str,
    pub description: String,
    pub width: u16,
    pub height: u16,
    pub padding: u16,
    pub autosize: AutoSize,

    pub signals: Vec<Signal>,
    pub scales: Vec<Scale>,
    pub axes: Vec<Axis>,
    /*
    pub marks: Vec<Mark>,
    */
}

/* 
 * AutoSize represents the different padding options that exist in Vega
 */
#[derive(Serialize)]
#[serde(rename_all="lowercase")]
pub enum AutoSize {
    Pad,
    Fit, 
    None,
}


impl Graph {

    //TODO: As Graph module gets fleshed out add more
    pub fn new(description: String) -> Graph {
        Graph {
            schema: "https://vega.github.io/schema/vega/v3.0.json",
            description: description,
            width: 500,
            height: 200,
            padding: 5,
            autosize: AutoSize::None,
            signals: vec![],
            scales: vec![Scale::new(XSCALE, WIDTH, XAXIS), Scale::new(YSCALE, HEIGHT, YAXIS)],
            axes: vec![Axis::new(Orientation::Bottom, XSCALE), Axis::new(Orientation::Left, YSCALE) ]
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
        let mut s = serializer.serialize_struct("Graph", 9)?;
        let _ = s.serialize_field("$schema", &self.schema);
        let _ = s.serialize_field("description", &self.description);
        let _ = s.serialize_field("width", &self.width);
        let _ = s.serialize_field("height", &self.height);
        let _ = s.serialize_field("padding", &self.padding);
        let _ = s.serialize_field("autosize", &self.autosize);
        let _ = s.serialize_field("signals", &self.signals);
        let _ = s.serialize_field("scales", &self.scales);
        let _ = s.serialize_field("axes", &self.axes);
        s.end()
    }
}