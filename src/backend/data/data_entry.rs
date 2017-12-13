
use serde::ser::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;

use std::collections::HashMap;

pub struct DataEntry<T: Serialize> {
    pub data: HashMap<String, T>,
}

impl <T: Serialize> DataEntry<T> {

    pub fn new() -> DataEntry<T> {
        DataEntry {
            data: HashMap::new(),
        }
    }

    pub fn insert(& mut self, key: String, value: T) {
        &self.data.insert(key, value);
    }

    pub fn insert(&mut self, key: &str, value: T) {
        &self.data.insert(key.to_string(), value);
    }
}
impl <T: Serialize> Serialize for DataEntry<T> {

     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("DataEntry", self.data.len())?;
        self.data.into_iter().map(|x| { s.serialize_field(&x.0, &x.1)}).collect();
        s.end()
    }

}