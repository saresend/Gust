extern crate serde_json;
use elements::{Tick};
use unit::Unit;


#[derive(Serialize, Deserialize)]
pub struct XAxis {
    title: String,
    unit: Unit,
    has_line: bool,
    max_value: i64,
    ticks: Vec<Tick>,
}
#[derive(Serialize, Deserialize)]
pub struct YAxis {
    title: String,
    unit: Unit,
    has_line: bool,
    max_value: i64,
    ticks: Vec<Tick>,
}



impl XAxis {
    pub fn to_json(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(s) => s,
            Err(_) => "".to_string(),
        }

    }


    pub fn new(title: String, unit: Unit, has_line: bool, max_value: i64) -> XAxis {
        let ticks  = vec![];
        XAxis {
            title,
            unit,
            has_line,
            max_value,
            ticks,
        }
    }

    pub fn add_tick(&mut self, tickmark: Tick) {
        self.ticks.push(tickmark);
    }

    
}

impl YAxis {
    pub fn to_json(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(s) => s,
            Err(_) => "".to_string(),
        }
    }
    
    pub fn new(title: String, unit: Unit, has_line: bool, max_value: i64) -> YAxis {
        YAxis{
            title,
            unit,
            has_line,
            max_value,
            ticks: vec![],
        }

    }
    
    pub fn add_tick(& mut self, tick: Tick) {
        self.ticks.push(tick);
    }

    
}
