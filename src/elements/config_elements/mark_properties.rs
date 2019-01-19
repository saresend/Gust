#[derive(Serialize)]
pub struct MarkConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<MarkProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<MarkProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rect: Option<MarkProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark: Option<MarkProperties>,
}

#[derive(Serialize)]
pub struct MarkProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
}
