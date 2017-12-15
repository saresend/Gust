
/* 
 *  Author: Samuel Resendez
 */ 
use serde::ser::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;

use backend::elements::constants::*;
use backend::elements::graph::{GraphType, AutoSize};
use backend::elements::data::data::Data;
use backend::elements::data::data_entry::DataEntry;
use backend::elements::signal::Signal;
use backend::elements::scale::{Scale, ScaleType};
use backend::elements::axis::{Axis, Orientation};
use backend::elements::mark::mark::Mark;
use backend::elements::traits::identifier::Identifiable;

pub struct BarChart {

    pub identifier: String,
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

impl BarChart {

    /// Creates a new Chart with the Appropriate settings to render into a vega bar chart
    pub fn new(identifier: &str) -> BarChart {
        BarChart {
            identifier: identifier.to_string(),
            description: "New BarChart".to_string(),
            width: 400,
            height: 200,
            padding: 5,
            autosize: AutoSize::Pad,
            data: vec![Data::new(SERIESNAME.to_string())],
            signals: vec![],
            scales: vec![
                Scale::new(XSCALE, WIDTH, XCOORD, ScaleType::Band),
                Scale::new(YSCALE, HEIGHT, YCOORD, ScaleType::None),
            ],
            axes: vec![ 
                Axis::new(Orientation::Bottom, XSCALE),
                Axis::new(Orientation::Left, YSCALE),
            ],
            marks: vec![
                Mark::new(GraphType::Bar),
            ],
        }
    }


    pub fn add_data_point(& mut self, category: &'static str, value: i64) {
        self.data[0].values.push(DataEntry::new_barchart_point(category, value));
    }
}

// REMEMBER TO ADD THE SCHEMA 

impl Serialize for BarChart {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("bar_chart", 11)?;
        s.serialize_field("$schema", "https://vega.github.io/schema/vega/v3.0.json")?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("width", &self.width)?;
        s.serialize_field("height", &self.height)?;
        s.serialize_field("padding", &self.padding)?;
        s.serialize_field("data", &self.data)?;
        s.serialize_field("autosize", &self.autosize)?;
        s.serialize_field("signals", &self.signals)?;
        s.serialize_field("scales", &self.scales)?;
        s.serialize_field("axes", &self.axes)?;
        s.serialize_field("marks", &self.marks)?;
        s.end()
    }
}
impl Identifiable for BarChart {

    fn get_identifier(&self) -> String {
        self.identifier.to_string()
    }
}