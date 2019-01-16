use super::super::autosize::Autosize;

#[derive(Serialize)]
pub enum GroupOptions {
    #[serde(rename = "fill")]
    Fill(String),
    #[serde(rename = "stroke")]
    Stroke(String),
    #[serde(rename = "strokeWidth")]
    StrokeWidth(i32),
}

pub struct ViewProperties {
    autosize: Autosize,
    background: String,
    group: GroupOptions,
}
