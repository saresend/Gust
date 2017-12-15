
/*
 * Author: Samuel Resendez
 */

use serde::ser::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;

use backend::elements::graph::AutoSize;
use backend::elements::data::data::Data;
use backend::elements::signal::Signal;
use backend::elements::axis::Axis;
use backend::elements::mark::mark::Mark;
use backend::elements::scale::Scale;
use backend::elements::data::transform::Transform;

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