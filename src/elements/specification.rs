use super::autosize::Autosize;
use super::padding::Padding;
use super::signal::Signal;

#[derive(Serialize, Deserialize)]
pub struct Specification {
    #[serde(rename = "$schema")]
    schema: String,
    description: String,
    width: i32,
    height: i32,
    padding: Padding,
    autosize: Autosize, // Change to enum (probably)
    signals: Vec<Signal>,
    data: Vec<Data>,
    scales: Vec<Scale>,
    projections: Vec<Projection>,
    axes: Vec<Axis>,
    legends: Vec<Legend>,
    title: Title,
    marks: Vec<Mark>,
    encode: Encoding,
}
