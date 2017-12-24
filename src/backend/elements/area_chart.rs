/* Author: Samuel Resendez
 * Contains most of the needed elements for
 * area_charts
 */

pub struct AreaChartSignal {
    name: String,
    value: String,
    bind: SignalBinding,
}
#[derive(Serialize)]
pub struct SignalBinding {
    input: String,
    options: Vec<String>,
}
pub struct AreaChartData {
    name: String,
    values: Vec<AreaChartDataEntry>,
}
#[derive(Serialize)]
pub struct AreaChartDataEntry {
    u: i32,
    v: i32,

}
pub struct AreaChartAxis {}

pub struct AreaChartMark {}

pub struct AreaChartScale {}
