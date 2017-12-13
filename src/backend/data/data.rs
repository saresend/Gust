


use backend::data::data_entry::DataEntry;

/*
 * The primary structure that holds series Data in it
 */
#[derive(Serialize)]
pub struct Data {
    name: String,
    values: Vec<DataEntry>,
}

impl Data {

    pub fn new(name: String) -> Data {
        Data {
            name: name,
            values: vec![],
        }
    }

    pub fn create_fake_data(& mut self, _ : i32) {
        let v = vec!["A", "B", "C", "D", "E", "F", "G","H", "I", "J", "K"];
        for i in 0..10 {
            let e: DataEntry = DataEntry::new();
            e.insert(v[i].to_string(), (i * i) as i64 );
            self.values.push(e);
        }
    }
}