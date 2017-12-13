
use serde::ser::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;

use backend::constants::{XAXIS, YAXIS};

pub struct DataEntry {
    x_value: String, 
    y_value: i32, 
}


impl DataEntry {

    pub fn new(label: String, y_val: i32) -> DataEntry {
        DataEntry {
            x_value:label,
            y_value: y_val
        }
    }
}
impl Serialize for DataEntry {

     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("DataEntry", 2)?;
        let _ = s.serialize_field(XAXIS,&self.x_value);
        let _ = s.serialize_field(YAXIS,&self.y_value);
        s.end()
    }

}