
use backend::elements::line_chart::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use backend::traits::Graphable;

pub struct LineChart {
    identifier: String,
    description: String,
    width: u32,
    height: u32,
    padding: u32,
    signals: Vec<LineChartSignal>,
    data: Vec<LineChartData>,
    scales: Vec<LineChartScale>,
    axes: Vec<LineChartAxis>,
    marks: Vec<LineChartMark>,
}

impl LineChart {
    pub fn new() -> LineChart {
        LineChart {
            identifier: String::from("line_chart"),
            description: String::from("Line Chart"),
            width: 500,
            height: 300,
            padding: 5,
            signals: vec![LineChartSignal::new()],
            data: vec![LineChartData::new()],
            scales: vec![
                LineChartScale::new_xscale(),
                LineChartScale::new_yscale(),
                LineChartScale::new_ordinal_scale(),
            ],
            axes: vec![LineChartAxis::new_xaxis(), LineChartAxis::new_yaxis()],
            marks: vec![LineChartMark::new()],
        }

    }


    /// Sets the identifier for that graph. The identifier is used to form the
    /// output file which the graph renders to. It will have the following format:
    /// <identifier>.<extension>
    pub fn set_identifier(&mut self, id: &str) {
        self.identifier = String::from(id);
    }

    /// Sets the description for the graph. The description is used to title
    /// the graph when rendering
    pub fn set_description(&mut self, description: &str) {
        self.description = String::from(description);
    }

    /// To add data to a line chart, the data must be formatted in the following fashion:
    /// {Integer, Integer, Integer }.
    ///
    /// The first two entries represent the x and y coordinates of the point
    /// which you're adding to the graph, and the third coordinate is the series identifier.
    /// For example, if you want to add 2 different lines on a single set of axes, then you can
    /// set the z of the first series to 0, and set the z of the second series to 1.
    pub fn add_data(&mut self, x: i64, y: i64, z: i64) {
        self.data[0].add_data(x, y, z);
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
}
impl Serialize for LineChart {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("line_chart", 10)?;
        s.serialize_field(
            "$schema",
            "https://vega.github.io/schema/vega/v3.0.json",
        )?;
        s.serialize_field("width", &self.width)?;
        s.serialize_field("height", &self.height)?;
        s.serialize_field("padding", &self.padding)?;
        s.serialize_field("signals", &self.signals)?;
        s.serialize_field("data", &self.data)?;
        s.serialize_field("scales", &self.scales)?;
        s.serialize_field("axes", &self.axes)?;
        s.serialize_field("marks", &self.marks)?;

        s.end()
    }
}
impl Graphable for LineChart {
    fn get_description(&self) -> &str {
        &self.description
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
}
