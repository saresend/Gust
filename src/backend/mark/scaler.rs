

use serde::ser::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;


pub struct Scaler<T: Serialize > {
    pub scale: String,
    pub y_name: &'static str,
    pub field: T,
}

impl <T: Serialize> Scaler<T> {
    pub fn new(scale: &str,name: &'static str, field: T) -> Scaler<T> {
        Scaler {
            scale: scale.to_string(),
            y_name: name,
            field: field,
        }
    }
}

impl <T: Serialize> Serialize for Scaler<T> {
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