
use backend::elements::line_chart::*;


pub struct LineChart {
    pub identifier: String,
    pub width: u32,
    pub height: u32,
    pub padding: u32,
    pub signals: Vec<LineChartSignal>,
    pub data: Vec<LineChartData>,
    pub scales: Vec<LineChartScale>,
    pub axes: Vec<LineChartAxis>,
    pub marks: Vec<LineChartMark>,
}

impl LineChart {
    pub fn new() -> LineChart {
        LineChart {
            identifier: String::from("line_chart"),
            width: 500,
            height: 300,
            padding: 5,
            signals: vec![],
            data: vec![],
            scales: vec![
                LineChartScale::new_xscale(),
                LineChartScale::new_yscale(),
                LineChartScale::new_ordinal_scale(),
            ],
            axes: vec![],
            marks: vec![],
        }
    }
}
