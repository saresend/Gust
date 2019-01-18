use super::binding::Binding;
use super::expression::Expression;
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Serialize, Deserialize)]
pub enum JSAny {
    Str(String),
    Num(i32),
    Bool(bool),
    #[serde(rename = "null")]
    Null,
}

#[derive(Serialize)]
pub enum VegaEvaluation {
    #[serde(rename = "init")]
    Init(Expression),
    #[serde(rename = "on")]
    On(Expression),
}

pub struct Signal {
    name: String,
    bind: Binding,
    description: String,
    eval: VegaEvaluation,
    update: Option<String>,
    react: bool,
    value: JSAny,
}

// Needed so that we can enforce mutual exclusion of init and on at compile time
impl Serialize for Signal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut signal = serializer.serialize_struct("Signal", 7)?;
        signal.serialize_field("name", &self.name)?;
        signal.serialize_field("bind", &self.bind)?;
        signal.serialize_field("description", &self.bind)?;
        match self.eval {
            VegaEvaluation::Init(ref exp) => signal.serialize_field("init", exp)?,
            VegaEvaluation::On(ref exp) => signal.serialize_field("on", exp)?,
        };
        signal.serialize_field("update", &self.update)?;
        signal.serialize_field("react", &self.react)?;
        signal.serialize_field("value", &self.value)?;
        signal.end()
    }
}
