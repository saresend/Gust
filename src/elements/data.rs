#[derive(Serialize)]
pub enum InputType {
    #[serde(rename = "json")]
    JSON,
    #[serde(rename = "csv")]
    CSV,
    #[serde(rename = "tsv")]
    TSV,
    #[serde(rename = "dsv")]
    DSV,
    #[serde(rename = "topojson")]
    TopoJSON,
}

// TODO: Implement DSL for custom parsing logic, see issue #30
#[derive(Serialize)]
pub enum ParseType {
    #[serde(rename = "auto")]
    Auto,
}

#[derive(Serialize)]
pub struct Format {
    #[serde(rename = "type")]
    input_type: InputType,
    parse: ParseType,
}

#[derive(Serialize)]
pub struct Dataset {
    name: String,
    format: Format,
}
