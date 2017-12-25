/* Author: Samuel Resendez
 *
 */




use backend::elements::area_chart::*;


/// AreaChart is the primary struct which describes a Area Chart, as by the vega specification
/// # Example
///
/// ```rust
///     use gust::backend::area_chart::AreaChart;
///
///     let mut a = AreaChart::new();
///     for i in 0..10 {
///         a.add_data(i, i*i);
///     }
///```
pub struct AreaChart {
    width: u32,
    height: u32,
    padding: u32,

    signals: Vec<AreaChartSignal>,

    data: Vec<AreaChartData>,
    scales: Vec<AreaChartScale>,
    axes: Vec<AreaChartAxis>,
    marks: Vec<AreaChartMark>,
}
