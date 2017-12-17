
/*
 *  Author: Samuel Resendez
 */
use backend::elements::bar_chart::*;

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

            data: vec![],
            scales: vec![
                BarChartScale::create_xscale(),
                BarChartScale::create_yscale(),
            ],
            axes: vec![BarChartAxis::create_xaxis(), BarChartAxis::create_yaxis()],
            marks: vec![],
        }
    }
}
