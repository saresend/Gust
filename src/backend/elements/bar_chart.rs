

use backend::elements::general::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Serialize)]
pub struct BarChartData {
    pub name: String,
    pub values: Vec<BarChartValue>,
}


#[derive(Serialize)]
pub struct BarChartValue {
    category: String,
    amount: i32,
}


pub struct BarChartScale {
    pub name: String,
    pub scale_type: String,
    domain: BarChartDomain,
    pub range: String,
    pub padding: f64,
}

impl BarChartScale {
    pub fn create_xscale() -> BarChartScale {
        BarChartScale {
            name: String::from("xscale"),
            scale_type: String::from("band"),
            domain: BarChartDomain {
                data: String::from("table"),
                field: String::from("category"),
            },
            range: String::from("width"),
            padding: 0.05,
        }
    }

    pub fn create_yscale() -> BarChartScale {
        BarChartScale {
            name: String::from("yscale"),
            scale_type: String::from("linear"),
            domain: BarChartDomain {
                data: String::from("table"),
                field: String::from("amount"),
            },
            range: String::from("height"),
            padding: 0.05,
        }
    }
}

impl Serialize for BarChartScale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("scale", 10)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("type", &self.scale_type)?;
        s.serialize_field("domain", &self.domain)?;
        s.serialize_field("range", &self.range)?;
        s.serialize_field("padding", &self.padding)?;
        s.end()
    }
}

#[derive(Serialize)]
struct BarChartDomain {
    data: String,
    field: String,
}

#[derive(Serialize)]
pub struct BarChartAxis {
    orient: Orientation,
    scale: String,
}

impl BarChartAxis {
    pub fn create_xaxis() -> BarChartAxis {
        BarChartAxis {
            orient: Orientation::Bottom,
            scale: String::from("xscale"),
        }
    }

    pub fn create_yaxis() -> BarChartAxis {
        BarChartAxis {
            orient: Orientation::Left,
            scale: String::from("yscale"),
        }
    }
}

pub struct BarChartMark {
    pub mark_type: String,
    pub from: KeyVal,
    encode: BarChartEncoding,
}

impl BarChartMark {
    pub fn create_mark() -> BarChartMark {
        BarChartMark {
            mark_type: String::from("rect"),
            from: KeyVal::new("data", "table"),
            encode: BarChartEncoding::create(),
        }
    }
}

impl Serialize for BarChartMark {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("mark", 10)?;
        s.serialize_field("type", &self.mark_type)?;
        s.serialize_field("from", &self.from)?;
        s.serialize_field("encode", &self.encode)?;
        s.end()
    }
}

struct BarChartEncoding {
    enter: BarChartEnter,
    update: BarChartFill,
    hover: BarChartFill,
}
impl BarChartEncoding {
    pub fn create() -> BarChartEncoding {
        BarChartEncoding {
            enter: BarChartEnter::default(),
            update: BarChartFill::new("steelblue"),
            hover: BarChartFill::new("red"),
        }
    }
}

struct BarChartEnter {
    x: JSONDict,
    width: JSONDict,
    y: JSONDict,
    y2: JSONDict,
}
impl BarChartEnter {
    pub fn default() -> BarChartEnter {
        BarChartEnter {
            x: JSONDict::create("scale", "xscale", "field", "category"),
            width: JSONDict::band_create("scale", "xscale", "band", 1),
            y: JSONDict::create("scale", "yscale", "field", "amount"),
            y2: JSONDict::band_create("scale", "yscale", "value", 0),
        }
    }
}

struct BarChartFill {
    fill: JSONDict,
}

impl BarChartFill {
    pub fn new(color: &str) -> BarChartFill {
        BarChartFill {
            fill: JSONDict::create("value", color, "fillOpacity", "0.5"),
        }
    }
}
