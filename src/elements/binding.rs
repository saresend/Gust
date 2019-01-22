#[derive(Serialize, Deserialize)]
pub struct Binding {
    input: Element,
    element: Option<String>,
    name: String,
    debounce: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Element {
    Checkbox,
    Radio,
    Range,
    Select,
}
