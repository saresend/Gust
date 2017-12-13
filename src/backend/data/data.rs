


use backend::data::data_entry::DataEntry;
use backend::data::transform::Transform;

use backend::constants::*;
/*
 * The primary structure that holds series Data in it
 */
#[derive(Serialize)]
pub struct Data {
    name: String,
    values: Vec<DataEntry>,
    transforms: Vec<Transform>,
}

impl Data {

    pub fn new(name: String) -> Data {
        Data {
            name: name,
            values: vec![],
            transforms: vec![],
        }
    }

    pub fn create_fake_data(& mut self, _ : i32) {
        let v = vec!["A", "B", "C", "D", "E", "F", "G","H", "I", "J", "K"];
        for i in 0..10 {
            let mut e = DataEntry::new();
            e.insert_data(YCOORD, (i * i) as i64 );
            e.insert_qual(XCOORD, v[i].to_string());
            self.values.push(e);
        }
    }
}