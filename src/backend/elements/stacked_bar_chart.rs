

use backend::elements::general::*;


#[derive(Serialize)]
pub struct StackedBarData {
    name: String,
    values: Vec<StackedBarDataValue>,
}

#[derive(Serialize)]
pub struct StackedBarDataValue {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl StackedBarData {
    pub fn new() -> StackedBarData {
        StackedBarData {
            name: String::from("table"),
            values: vec![],
        }
    }
}

pub struct StackedBarTransform {
    pub transform_type: String,
    pub group_by: Vec<String>,
    pub sort: KeyVal,
    pub field: String,
}

pub struct StackedBarScale {}

pub struct StackedBarAxis {}

pub struct StackedBarMark {}
