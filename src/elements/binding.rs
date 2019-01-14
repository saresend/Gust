#[derive(Serialize, Deserialize)]
pub struct Binding {
    input: Element,
    element: Option<String>,
    name: String,
    debounce: i32,
}

#[derive(Serialize, Deserialize)]
pub enum Element {
    checkbox,
    radio,
    range,
    select,
}
