
/*
 *  Author: Samuel Resendez
 */
use backend::elements::bar_chart::*;

pub struct BarChart {
    pub identifier: String,
    pub width: i32,
    pub height: i32,
    pub padding: i32,

    pub data: Vec<BarChartData>,
    pub scales: Vec<BarChartScale>,
    pub axes: Vec<BarChartAxis>,

    pub marks: Vec<BarChartMark>,
}
