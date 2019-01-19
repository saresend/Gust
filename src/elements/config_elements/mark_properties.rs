#[derive(Serialize)]
pub struct MarkConfig {
    area: MarkProperties,
    line: MarkProperties,
    rect: MarkProperties,
    mark: MarkProperties,
}

#[derive(Serialize)]
pub struct MarkProperties {
    fill: String,
    stroke: String,
    size: i32,
    font: String,
}
