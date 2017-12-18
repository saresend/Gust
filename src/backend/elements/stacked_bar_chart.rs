

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
    update: StackedBarFill,
    hover: StackedBarFill,
}
impl StackedBarMark {
    pub fn new() -> StackedBarMark {
        StackedBarMark {
            mark_type: String::from("rect"),
            from: KeyVal::new("data", "table"),
            encode: StackedBarEncoding::new(),
            update: StackedBarFill::new(1.0),
            hover: StackedBarFill::new(0.5),
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
        s.serialize_field("update", &self.update)?;
        s.serialize_field("hover", &self.hover)?;
        s.end()
    }
}

#[derive(Serialize)]
struct StackedBarEncoding {
    x: JSONDict,
    width: JSONDict,
    y: JSONDict,
    y2: JSONDict,
    fill: JSONDict,
}

impl StackedBarEncoding {
    pub fn new() -> StackedBarEncoding {
        StackedBarEncoding {
            x: JSONDict::create("scale", "x", "field", "x"),
            width: JSONDict::band_create("scale", "x", "band", 1),
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
