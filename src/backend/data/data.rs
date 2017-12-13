


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
            self.values.push(DataEntry::new(v[i].to_string(), (i * i) as i32 ));
        }
    }
}
/* 
 * DataEntry represent a single point or bar on a graph
 */
