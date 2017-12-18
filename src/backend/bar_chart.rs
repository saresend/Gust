
/*
 *  Author: Samuel Resendez
 */
use backend::elements::bar_chart::*;
use backend::traits::Graphable;
use serde_json;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::io;
use std::io::Write;
use std;

pub struct BarChart {
    pub identifier: String,
    pub description: String,
    pub width: i32,
    pub height: i32,
    pub padding: i32,

    pub data: Vec<BarChartData>,
    pub scales: Vec<BarChartScale>,
    pub axes: Vec<BarChartAxis>,

    pub marks: Vec<BarChartMark>,
}

impl BarChart {
    pub fn new() -> BarChart {
        BarChart {
            identifier: String::from("barchart"),
            description: String::from("A barchart"),
            width: 500,
            height: 300,
            padding: 5,

            data: vec![BarChartData::new()],
            scales: vec![
                BarChartScale::create_xscale(),
                BarChartScale::create_yscale(),
            ],
            axes: vec![BarChartAxis::create_xaxis(), BarChartAxis::create_yaxis()],
            marks: vec![BarChartMark::create_mark()],
        }
    }
}

impl Serialize for BarChart {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("graph", 10)?;
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

impl Graphable for BarChart {
    fn save_to_file(&self) -> io::Result<()> {
        let mut f = std::fs::File::create(format!("{}.json", &self.identifier))?;
        f.write_all(serde_json::to_string(self)?.as_bytes())?;
        Ok(())
    }
}
