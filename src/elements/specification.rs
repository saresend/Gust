use super::autosize::Autosize;
use super::axis::Axis;
use super::config::Config;
use super::data::Dataset;
use super::encoding::Encoding;
use super::legend::Legend;
use super::mark::Mark;
use super::padding::Padding;
use super::projection::Projection;
use super::scale::Scale;
use super::signal::Signal;
use super::title::Title;

#[derive(Serialize)]
pub struct Specification {
    #[serde(rename = "$schema")]
    schema: String,
    description: String,
    background: String,
    width: i32,
    height: i32,
    padding: Padding,
    autosize: Autosize,
    config: Config,
    signals: Vec<Signal>,
    data: Vec<Dataset>,
    scales: Vec<Scale>,
    projections: Vec<Projection>,
    axes: Vec<Axis>,
    legends: Vec<Legend>,
    title: Title,
    marks: Vec<Mark>,
    encode: Encoding,
}
