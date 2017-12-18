
/*
 * Author: Samuel Resendez
 */

use backend::elements::stacked_bar_chart::*;

#[derive(Serialize)]
pub struct StackedBarChart {
    pub identifier: String,
    pub width: u16,
    pub height: u16,
    pub padding: u16,

    pub data: Vec<StackedBarData>,
    pub transform: Vec<StackedBarTransform>,
    pub scales: Vec<StackedBarScale>,
    pub axes: Vec<StackedBarAxis>,
    pub marks: Vec<StackedBarMark>,
}


impl StackedBarChart {
    pub fn new() -> StackedBarChart {
        StackedBarChart {
            identifier: String::from("stacked_bar_chart"),
            width: 500,
            height: 300,
            padding: 5,
            data: vec![],

            transform: vec![StackedBarTransform::new()],

            scales: vec![
                StackedBarScale::new_xscale(),
                StackedBarScale::new_yscale(),
                StackedBarScale::new_ordinal_scale(),
            ],
            axes: vec![StackedBarAxis::new_xaxis(), StackedBarAxis::new_yaxis()],
            marks: vec![StackedBarMark::new()],
        }
    }
}
