#[derive(Serialize, Deserialize)]
pub struct Binding {
    input: Element,
    element: Option<String>,
    name: String,
    debounce: i32,
}

#[derive(Serialize, Deserialize)]
pub enum Element {
    #[serde(rename = "checkbox")]
    Checkbox,
    #[serde(rename = "radio")]
    Radio,
    #[serde(rename = "range")]
    Range,
    #[serde(rename = "select")]
    Select,
}
