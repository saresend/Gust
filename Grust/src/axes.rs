extern crate serde_json;

#[derive(Serialize, Deserialize)]
pub struct XAxis {
    title: String,
    unit: String,
    has_line: bool,
    max_value: i64,
}
#[derive(Serialize, Deserialize)]
pub struct YAxis {
    title: String,
    unit: String,
    has_line: bool,
    max_value: i64,
}



impl XAxis {
    pub fn to_json(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(s) => s,
            Err(_) => "".to_string(),
        }

    }


    pub fn new(title: String, unit: String, has_line: bool, max_value: i64) -> XAxis {
        XAxis {
            title,
            unit,
            has_line,
            max_value,
        }
    }
}

impl YAxis {
    pub fn to_json(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(s) => s,
            Err(_) => "".to_string(),
        }
    }
}
