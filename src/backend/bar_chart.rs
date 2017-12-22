
/*
 *  Author: Samuel Resendez
 */
use backend::elements::bar_chart::*;
use backend::traits::Graphable;
use serde::ser::{Serialize, SerializeStruct, Serializer};


pub struct BarChart {
    identifier: String,
    description: String,
    width: i32,
    height: i32,
    padding: i32,

    data: Vec<BarChartData>,
    scales: Vec<BarChartScale>,
    axes: Vec<BarChartAxis>,
    marks: Vec<BarChartMark>,
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
    /// Bar Chart accept data in the following format:
    /// { String, Integer }, which represent the category (aka Bar), as well as the value of that bar
    /// (i.e. it's height).
    pub fn add_data(&mut self, category: String, amount: i32) {
        self.data[0].add_data(category, amount);
    }

    /// Sets the description that is used to title the chart when rendering
    pub fn set_description(&mut self, description: &str) {
        self.description = String::from(description);
    }

    ///Sets the identifier for the graph. This is the value used when
    /// actually naming the file, so you can expect files rendered to fit
    /// the following format: <identifier>.<extension>
    pub fn set_identifier(&mut self, id: &str) {
        self.identifier = String::from(id);
    }

    ///This sets the size of the overal graph to be rendered
    /// The tuple it takes represents the (height, width)
    pub fn set_dimension(&mut self, t: (i32, i32)) {
        self.height = t.0;
        self.width = t.1;
    }

    /// Sets the number of padding pixels around the chart
    pub fn set_padding(&mut self, pad: i32) {
        self.padding = pad;
    }

    /// clear_data empties out all of the data entries
    /// that currently exist in the given graph
    pub fn clear_data(&mut self) {
        self.data[0].clear()
    }

    /// set_color sets the default color of the bars in the bar chart
    pub fn set_color(&mut self, color: &str) {
        self.marks[0].set_color(color);
    }
}

impl Serialize for BarChart {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("graph", 10)?;
        s.serialize_field(
            "$schema",
            "https://vega.github.io/schema/vega/v3.0.json",
        )?;
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
    fn get_description(&self) -> &str {
        &self.description
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
}
