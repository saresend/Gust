
/*
 * Author: Samuel Resendez
 */

use backend::elements::stacked_bar_chart::*;
use backend::traits::Graphable;

use serde::ser::{Serialize, SerializeStruct, Serializer};


pub struct StackedBarChart {
    pub identifier: String,
    pub description: String,
    pub width: u16,
    pub height: u16,
    pub padding: u16,

    pub data: Vec<StackedBarData>,

    pub scales: Vec<StackedBarScale>,
    pub axes: Vec<StackedBarAxis>,
    pub marks: Vec<StackedBarMark>,
}


impl StackedBarChart {
    pub fn new() -> StackedBarChart {
        StackedBarChart {
            identifier: String::from("stacked_bar_chart"),
            description: String::from("Stacked Bar Chart"),
            width: 500,
            height: 300,
            padding: 5,
            data: vec![StackedBarData::new()],

            scales: vec![
                StackedBarScale::new_xscale(),
                StackedBarScale::new_yscale(),
                StackedBarScale::new_ordinal_scale(),
            ],
            axes: vec![StackedBarAxis::new_xaxis(), StackedBarAxis::new_yaxis()],
            marks: vec![StackedBarMark::new()],
        }
    }
    pub fn add_data(&mut self, x: i64, y: i64, z: i64) {
        self.data[0].add_data(x, y, z);
    }
}
impl Serialize for StackedBarChart {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("sb_graph", 9)?;
        s.serialize_field("$schema", "https://vega.github.io/schema/vega/v3.0.json")?;
        s.serialize_field("width", &self.width)?;
        s.serialize_field("height", &self.height)?;
        s.serialize_field("padding", &self.padding)?;

        s.serialize_field("data", &self.data)?;
        s.serialize_field("scales", &self.scales)?;
        s.serialize_field("axes", &self.axes)?;
        s.serialize_field("marks", &self.marks)?;
        s.end()
    }
}

impl Graphable for StackedBarChart {
    fn get_description(&self) -> &str {
        &self.description
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
}
