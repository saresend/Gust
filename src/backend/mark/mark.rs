
/* 
 * Author: Samuel Resendez
 * 
 */

use serde::ser::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;

use backend::constants::*;

use backend::mark::encoding::Encoding;


pub struct Mark {
    pub mark_type: String, 
    pub from: Source, 
    pub encode: Encoding,
}

#[derive(Serialize)]
pub struct Source {
    pub data: String,
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


