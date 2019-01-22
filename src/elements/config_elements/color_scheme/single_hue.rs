use super::colors::Colors;
use super::vega_scheme::VegaColorScheme;
use serde::{Serialize, Serializer};

pub struct SingleHue {
    number: i32,
    base: Colors,
}

impl Serialize for SingleHue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let base = serde_json::to_string(&self.base).unwrap();
        let base = base + "s-" + &self.number.to_string();
        serializer.serialize_str(&base)
    }
}

impl VegaColorScheme for SingleHue {}
