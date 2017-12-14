


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
    transform: Vec<Transform>,
}

impl Data {

    pub fn new(name: String) -> Data {
        Data {
            name: name,
            values: vec![],
            transform: vec![],
        }
    }

    pub fn add_transform(& mut self, transform: Transform) {
        self.transform.push(transform);
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
    pub fn create_fake_stack_bchart_data(&mut self) {
        for i in 0..10 {
            let mut e = DataEntry::new();
            e.insert_data(XCOORD, i);
            e.insert_data(YCOORD, 2 * i);
            if i % 2 == 0 {
                e.insert_data(ZCOORD,1);
            }
            else {
                e.insert_data(ZCOORD, 0);
            }
            self.values.push(e);
        }
        for i in 0..10 {
            let mut e = DataEntry::new();
            e.insert_data(XCOORD, i);
            e.insert_data(YCOORD, 2 * i);
            if i % 2 == 0 {
                e.insert_data(ZCOORD,1);
            }
            else {
                e.insert_data(ZCOORD, 0);
            }
            self.values.push(e);
        }
    }
}