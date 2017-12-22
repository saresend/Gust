
/*
 * Author: Samuel Resendez
 */

use backend::elements::stacked_bar_chart::*;
use backend::traits::Graphable;

use serde::ser::{Serialize, SerializeStruct, Serializer};


pub struct StackedBarChart {
    identifier: String,
    description: String,
    width: u32,
    height: u32,
    padding: u32,

    data: Vec<StackedBarData>,

    scales: Vec<StackedBarScale>,
    axes: Vec<StackedBarAxis>,
    marks: Vec<StackedBarMark>,
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

    pub fn set_identifier(&mut self, id: &str) {
        self.identifier = String::from(id);
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = String::from(description);
    }

    /// Sets the dimensions of the graph:
    /// the dimensions are set as (height, width)
    pub fn set_dimensions(&mut self, t: (u32, u32)) {
        self.height = t.0;
        self.width = t.1;
    }
    /// Sets the padding pixel count around the border of the graph
    pub fn set_padding(&mut self, padding: u32) {
        self.padding = padding;
    }

    /// adds data to the stacked_bar chart
    /// x value denotes which bar the value is supposed to be on.
    /// y values denotes the height of the bar
    /// z is the stratification variable, meaning you can separate multiple stacked bars based on
    /// z.
    /// # Example:
    /// ```rust
    ///    use gust::backend::stacked_bar_chart::StackedBarChart;
    ///
    ///    let mut b = StackedBarChart::new();
    ///    for i in 0..10 {
    ///        b.add_data(i, i * i, 1);
    ///        b.add_data(i, i + i, 0);
    ///    }
    /// ```
    /// Here, we see there are two values entered for each i, one with a 1 value for z, and one with
    /// a zero value. This is how gust splits the stacked bars into two.
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

impl Graphable for StackedBarChart {
    fn get_description(&self) -> &str {
        &self.description
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
}
