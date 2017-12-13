
/* 
 * Author: Samuel Resendez
 * 
 */

use serde::ser::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;

use backend::constants::*;

use backend::mark::encoding::Encoding;
use backend::mark::fill::{Fill, Val};
use backend::mark::scaler::Scaler;

pub struct Mark {
    pub mark_type: String, 
    pub from: Source, 
    pub encode: Encoding,
}




#[derive(Serialize)]
pub struct Visualization {
    pub x: Scaler,
    pub width: Scaler,
    pub y: Scaler,
    pub y2: Scaler,
}




#[derive(Serialize)]
pub struct Source {
    pub data: String,
}





impl Visualization {
    pub fn new() -> Visualization {
        Visualization {
            x: Scaler::new(XSCALE,"field", XAXIS),
            width: Scaler::new(XSCALE, "band", "1"),
            y: Scaler::new(YSCALE, "field", YAXIS),
            y2: Scaler::new(YSCALE, "value", "0"),
        }
    }
}



impl Mark {

    pub fn new() -> Mark {
        Mark {
            mark_type: "rect".to_string(),
            from: Source { data: SERIESNAME.to_string() },
            encode: Encoding::new(),
        }
    }
}


impl Serialize for Mark {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Mark", 3)?;
        let _ = s.serialize_field("type", &self.mark_type);
        let _ = s.serialize_field("from", &self.from);
        let _ = s.serialize_field("encode", &self.encode);
        s.end()
    }

}


impl Serialize for Scaler {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Scaler", 2)?;
        let _ = s.serialize_field(&self.y_name, &self.field);
        let _ = s.serialize_field("scale", &self.scale);
        s.end()
    }
}