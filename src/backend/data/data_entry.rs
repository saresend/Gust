
use serde::ser::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;

use std::collections::HashMap;

pub struct DataEntry {
    pub data: HashMap<String, i64>,
}

impl DataEntry {

    pub fn new() -> DataEntry {
        DataEntry {
            data: HashMap::new(),
        }
    }

    pub fn insert(& mut self, key: String, value: i64) {
        &self.data.insert(key, value);
    }

}
impl Serialize for DataEntry {

     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("DataEntry", self.data.len())?;
        self.data.into_iter().map(|x| { s.serialize_field(&x.0, &x.1)}).collect();
        s.end()
    }

}