
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
