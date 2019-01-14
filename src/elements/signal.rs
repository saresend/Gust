use super::binding::Binding;

#[derive(Serialize, Deserialize)]
pub enum JSAny {
    Str(String),
    Num(i32),
    Bool(bool),
    #[serde(rename = "null")]
    Null,
}

#[derive(Serialize, Deserialize)]
pub struct Signal {
    name: String,
    bind: Binding,
    description: String,
    on: Vec<Handler>,
    init: Option<String>,
    update: Option<String>,
    react: bool,
    value: String,
}
