/*
 * Author: Samuel Resendez
 * Scale is the Vega specification Scale
 */

use serde::ser::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;

use backend::constants::*;

pub struct Scale {
    pub name: String, 
    pub scale_type: ScaleType, 
    pub domain: Domain,
    pub range: String,  //[QOL]: Create Enum for this type
    pub padding: f64, 
    pub round: bool,
}
#[derive(Serialize)]
#[serde(rename_all="lowercase")]
pub enum ScaleType {
    Band,
    Ordinal, 
    None, 
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

    pub fn new(name: &str, range: &str, field: &str, st: ScaleType) -> Scale {
        let d = Domain {
            data: SERIESNAME.to_string(),
            field: field.to_string(),
        };
        Scale {
            name: name.to_string(),
            scale_type: st,
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
       
        match self.scale_type {
            ScaleType::Band => {
                let _ = s.serialize_field("type", &self.scale_type);
                },
            ScaleType::Ordinal => {
                let _ = s.serialize_field("type", &self.scale_type);
            }
            ScaleType::None => {},
        };
        let _ = s.serialize_field("domain", &self.domain);
        let _ = s.serialize_field("range", &self.range);
        let _ = s.serialize_field("padding", &self.padding);
        let _ = s.serialize_field("round", &self.round);
        s.end()

    }
}