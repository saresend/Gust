
use std::collections::HashMap;

#[derive(Serialize)]
pub enum Orientation {
    Top,
    Left,
    Bottom,
    Right,
}

pub struct KeyVal {
    key: String,
    val: String,
}
impl KeyVal {
    pub fn new(key: &str, val: &str) -> KeyVal {
        KeyVal {
            key: key.to_string(),
            val: val.to_string(),
        }
    }
}

pub struct JSONDict {
    str_vals: HashMap<&'static str, String>,
    i32_vals: HashMap<&'static str, i32>,
}

impl JSONDict {
    pub fn create(xKey: &'static str, xVal: &str, yKey: &'static str, yVal: &str) -> JSONDict {
        let d = JSONDict {
            str_vals: HashMap::new(),
            i32_vals: HashMap::new(),
        };
        d.str_vals.insert(xKey, xVal.to_string());
        d.str_vals.insert(yKey, yVal.to_string());
        d
    }
}
