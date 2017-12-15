
/*
 * Author: Samuel Resendez
 */

use serde::ser::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;

use backend::elements::graph::{AutoSize, GraphType};
use backend::elements::data::data::Data;
use backend::elements::data::data_entry::DataEntry;
use backend::elements::signal::Signal;
use backend::elements::axis::{Axis, Orientation};
use backend::elements::mark::mark::Mark;
use backend::elements::scale::{Scale, ScaleType};
use backend::elements::data::transform::{Transform, TransformType};
use backend::elements::constants::*;
use backend::elements::traits::identifier::Identifiable;

pub struct StackedBarChart {

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
    pub transforms: Vec<Transform>,
}


impl StackedBarChart {

    pub fn new(identifier: &str) -> StackedBarChart {
        StackedBarChart {
            identifier: identifier.to_string(),
            description: "New Stacked Bar Chart".to_string(),
            width: 500,
            height: 300,
            padding: 5,
            autosize: AutoSize::Pad,
            data: vec![
                Data::new(SERIESNAME.to_string()),
            ],
            signals: vec![],
            scales: vec![
                Scale::new(XSCALE, WIDTH, XCOORD, ScaleType::Band),
                Scale::new(YSCALE, HEIGHT, YCOORD, ScaleType::None),
                Scale::new(ZSCALE, "category", ZCOORD, ScaleType::Ordinal),
            ],
            axes: vec![
                Axis::new(Orientation::Bottom, XSCALE),
                Axis::new(Orientation::Left, YSCALE),
            ],
            marks: vec![
                Mark::new(GraphType::StackedBar),
            ],
            transforms: vec![
                Transform::new(TransformType::Stack),
            ],
        }
    }

    pub fn add_data_point(& mut self, x: i64, y: i64, z: i64) {
        let mut data_point = DataEntry::new();
        data_point.insert_data(XCOORD, x);
        data_point.insert_data(YCOORD, y);
        data_point.insert_data(ZCOORD, z);
        self.data[0].values.push(data_point);
    }



}

impl Serialize for StackedBarChart {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("stacked_bar_chart", 10)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("width", &self.width)?;
        s.serialize_field("height", &self.height)?;
        s.serialize_field("padding", &self.padding)?;
        s.serialize_field("autosize", &self.autosize)?;
        s.serialize_field("data", &self.data)?;
        s.serialize_field("signals", &self.signals)?;
        s.serialize_field("axes", &self.axes)?;
        s.serialize_field("scales", &self.scales)?;
        s.serialize_field("marks", &self.marks)?;
        s.serialize_field("transform", &self.transforms)?;
        s.end()
    }
}

impl Identifiable for StackedBarChart {

    fn get_identifier(&self) -> String {
        self.identifier.to_string()
    }
}