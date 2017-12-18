
/*
 * Author: Samuel Resendez
 */

use backend::elements::stacked_bar_chart::*;

#[derive(Serialize)]
pub struct StackedBarChart {
    pub identifier: String,
    pub width: u16,
    pub height: u16,
    pub padding: f32,

    pub data: Vec<StackedBarData>,
    pub transform: Vec<StackedBarTransform>,
    pub scales: Vec<StackedBarScale>,
    pub axes: Vec<StackedBarAxis>,
    pub marks: Vec<StackedBarMark>,
}
