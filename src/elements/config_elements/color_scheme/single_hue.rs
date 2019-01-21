use super::vega_scheme::VegaColorScheme;
use serde::{Serialize, Serializer};

pub struct SingleHue {
    number: i32,
    name: SingleHueBase,
}

#[derive(Serialize)]
pub enum SingleHueBase {
    #[serde(rename = "blues")]
    Blues,
    #[serde(rename = "greens")]
    Greens,
    #[serde(rename = "greys")]
    Greys,
    #[serde(rename = "purples")]
    Purples,
    #[serde(rename = "reds")]
    Reds,
    #[serde(rename = "oranges")]
    Oranges,
}

impl Serialize for SingleHue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let base = serde_json::to_string(&self.name).unwrap();
        base = base + "-" + &self.number.to_string();
        serializer.serialize_str(&base)
    }
}

impl VegaColorScheme for SingleHue {}
