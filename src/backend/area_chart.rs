/* Author: Samuel Resendez
 *
 */




use backend::elements::area_chart::*;
use backend::traits::Graphable;

/// AreaChart is the primary struct which describes a Area Chart, as by the vega specification
/// # Example
///
/// ```
///     use gust::backend::area_chart::AreaChart;
///
///     let mut a = AreaChart::new();
///     for i in 0..10 {
///         a.add_data(i, i*i);
///     }
///```
#[derive(Serialize)]
pub struct AreaChart {
    identifier: String,
    description: String,
    width: u32,
    height: u32,
    padding: u32,

    signals: Vec<AreaChartSignal>,

    data: Vec<AreaChartData>,
    scales: Vec<AreaChartScale>,
    axes: Vec<AreaChartAxis>,
    marks: Vec<AreaChartMark>,
}

impl AreaChart {
    pub fn new() -> AreaChart {
        AreaChart {
            identifier: String::from("area_chart"),
            description: String::from("Area Chart"),
            width: 400,
            height: 200,

            padding: 5,

            signals: vec![AreaChartSignal::default()],
            scales: vec![AreaChartScale::default_x(), AreaChartScale::default_y()],
            data: vec![AreaChartData::default()],
            axes: vec![AreaChartAxis::x_axis(), AreaChartAxis::y_axis()],
            marks: vec![AreaChartMark::default()],
        }
    }

    pub fn add_data(&mut self, u: i32, v: i32) {
        self.data[0].add_data(u, v);
    }
}

impl Graphable for AreaChart {
    fn get_description(&self) -> &str {

        &self.description
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
}
