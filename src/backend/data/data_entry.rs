
use serde::ser::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;

use std::collections::HashMap;

pub struct DataEntry {
    pub data: HashMap<&'static str, i64>,
    pub quals: HashMap<&'static str, String>,
}

impl DataEntry {

    pub fn new() -> DataEntry {
        DataEntry {
            data: HashMap::new(),
            quals: HashMap::new(),
        }
    }

    pub fn insert_data(& mut self, key: &'static str, value: i64) {
        &self.data.insert(key, value);
    }
    pub fn insert_qual(& mut self, key: &'static str, value: String) {
        &self.quals.insert(key, value);
    }

}
impl Serialize for DataEntry {

     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("DataEntry", self.data.len())?;
        for (key, value) in self.data.iter() {
           let _ =  s.serialize_field(&key, &value);
        };
        for (key, value) in self.quals.iter() {
            let _ = s.serialize_field(&key, &value);
        }
        s.end()
    }

}