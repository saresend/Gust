

use backend::elements::general::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

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
    pub fn add_data(&mut self, x: i64, y: i64, z: i64) {
        self.values.push(StackedBarDataValue { x, y, z });
    }
}

pub struct StackedBarTransform {
    pub transform_type: String,
    pub group_by: Vec<String>,
    pub sort: KeyVal,
    pub field: String,
}
impl StackedBarTransform {
    pub fn new() -> StackedBarTransform {
        StackedBarTransform {
            transform_type: String::from("stack"),
            group_by: vec![String::from("x")],
            sort: KeyVal::new("field", "c"),
            field: String::from("y"),
        }
    }
}
impl Serialize for StackedBarTransform {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("transform", 3)?;
        s.serialize_field("type", &self.transform_type)?;
        s.serialize_field("groupby", &self.group_by)?;
        s.serialize_field("sort", &self.sort)?;
        s.serialize_field("field", &self.field)?;
        s.end()
    }
}

pub struct StackedBarScale {
    name: String,
    scale_type: String,
    range: String,
    domain: JSONDict,
}
impl StackedBarScale {
    pub fn new_xscale() -> StackedBarScale {
        StackedBarScale {
            name: String::from("x"),
            scale_type: String::from("band"),
            range: String::from("width"),
            domain: JSONDict::create("data", "table", "field", "x"),
        }
    }
    pub fn new_yscale() -> StackedBarScale {
        StackedBarScale {
            name: String::from("y"),
            scale_type: String::from("linear"),
            range: String::from("height"),
            domain: JSONDict::create("data", "table", "field", "y1"),
        }
    }
    pub fn new_ordinal_scale() -> StackedBarScale {}
}

pub struct StackedBarAxis {}

pub struct StackedBarMark {}
