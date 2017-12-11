/*
 * Author: Samuel Resendez
 * Scale is the Vega specification Scale
 */

use serde::ser::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;

pub struct Scale {
    pub name: String, 
    pub scale_type: String, //[QOL]: Create Enum for this type
    pub domain: Domain,
    pub range: String,  //[QOL]: Create Enum for this type
    pub padding: f64, 
    pub round: bool,
}

/* 
 * Domain is the little 
 */
#[derive(Serialize)]
pub struct Domain {
    pub data: String,
    pub field: String,
}

impl Scale {

    pub fn new(name: &str, range: &str, field: &str) -> Scale {
        let d = Domain {
            data: "Graph".to_string(),
            field: field.to_string(),
        };
        Scale {
            name: name.to_string(),
            scale_type: "band".to_string(),
            domain: d,
            range: range.to_string(),
            padding: 0.05,
            round: true,
        }
    }
    
}

impl Serialize for Scale {
     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Scale", 7)?;
        let _ = s.serialize_field("name", &self.name);
        let _ = s.serialize_field("type", &self.scale_type);
        let _ = s.serialize_field("domain", &self.domain);
        let _ = s.serialize_field("range", &self.range);
        let _ = s.serialize_field("padding", &self.padding);
        let _ = s.serialize_field("round", &self.round);
        s.end()

    }
}