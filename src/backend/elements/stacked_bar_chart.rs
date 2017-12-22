

use backend::elements::general::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Serialize)]
pub struct StackedBarData {
    name: String,
    transform: Vec<StackedBarTransform>,
    values: Vec<StackedBarDataValue>,
}

#[derive(Serialize)]
pub struct StackedBarDataValue {
    x: i64,
    y: i64,
    z: i64,
}

impl StackedBarData {
    pub fn new() -> StackedBarData {
        StackedBarData {
            name: String::from("table"),
            transform: vec![StackedBarTransform::new()],
            values: vec![],
        }
    }
    pub fn add_data(&mut self, x: i64, y: i64, z: i64) {
        self.values.push(StackedBarDataValue { x, y, z });
    }
}

pub struct StackedBarTransform {
    transform_type: String,
    group_by: Vec<String>,
    sort: KeyVal,
    field: String,
}
impl StackedBarTransform {
    pub fn new() -> StackedBarTransform {
        StackedBarTransform {
            transform_type: String::from("stack"),
            group_by: vec![String::from("x")],
            sort: KeyVal::new("field", "z"),
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
    pub fn new_ordinal_scale() -> StackedBarScale {
        StackedBarScale {
            name: String::from("color"),
            scale_type: String::from("ordinal"),
            range: String::from("category"),
            domain: JSONDict::create("data", "table", "field", "z"),
        }
    }
}
impl Serialize for StackedBarScale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("scale", 10)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("type", &self.scale_type)?;
        s.serialize_field("range", &self.range)?;
        s.serialize_field("domain", &self.domain)?;
        s.end()
    }
}
#[derive(Serialize)]
pub struct StackedBarAxis {
    orient: Orientation,
    scale: String,
}

impl StackedBarAxis {
    pub fn new_xaxis() -> StackedBarAxis {
        StackedBarAxis {
            orient: Orientation::Bottom,
            scale: String::from("x"),
        }
    }
    pub fn new_yaxis() -> StackedBarAxis {
        StackedBarAxis {
            orient: Orientation::Left,
            scale: String::from("y"),
        }
    }
}

pub struct StackedBarMark {
    mark_type: String,
    from: KeyVal,
    encode: StackedBarEncoding,
}
impl StackedBarMark {
    pub fn new() -> StackedBarMark {
        StackedBarMark {
            mark_type: String::from("rect"),
            from: KeyVal::new("data", "table"),
            encode: StackedBarEncoding::new(),
        }
    }
}

impl Serialize for StackedBarMark {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("mark", 4)?;
        s.serialize_field("type", &self.mark_type)?;
        s.serialize_field("from", &self.from)?;
        s.serialize_field("encode", &self.encode)?;

        s.end()
    }
}

#[derive(Serialize)]
struct StackedBarEncoding {
    enter: StackedBarEnter,
    update: StackedBarFill,
    hover: StackedBarFill,
}
impl StackedBarEncoding {
    pub fn new() -> StackedBarEncoding {
        StackedBarEncoding {
            enter: StackedBarEnter::new(),
            update: StackedBarFill::new(1.0),
            hover: StackedBarFill::new(0.5),
        }
    }
}

#[derive(Serialize)]
struct StackedBarEnter {
    x: JSONDict,
    width: JSONDict,
    y: JSONDict,
    y2: JSONDict,
    fill: JSONDict,
}

impl StackedBarEnter {
    pub fn new() -> StackedBarEnter {
        StackedBarEnter {
            x: JSONDict::create("scale", "x", "field", "x"),
            width: JSONDict::tri_create("scale", "x", "band", 1, "offset", -1),
            y: JSONDict::create("scale", "y", "field", "y0"),
            y2: JSONDict::create("scale", "y", "field", "y1"),
            fill: JSONDict::create("scale", "color", "field", "z"),
        }
    }
}



#[derive(Serialize)]
#[allow(non_snake_case)]
struct StackedBarFill {
    fillOpacity: QualKeyVal,
}
impl StackedBarFill {
    pub fn new(val: f32) -> StackedBarFill {
        StackedBarFill {
            fillOpacity: QualKeyVal::new("value", val),
        }
    }
}
