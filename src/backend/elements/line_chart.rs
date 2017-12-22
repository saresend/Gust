


use backend::elements::general::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Serialize)]
pub struct LineChartSignal {
    name: String,
    value: String,
    bind: LineChartInterpolation,
}
impl LineChartSignal {
    pub fn new() -> LineChartSignal {
        LineChartSignal {
            name: String::from("interpolate"),
            value: String::from("linear"),
            bind: LineChartInterpolation::new(),
        }
    }
}

///A small json object which represents the selector for different interpolation options that are
/// currently supported by Vega.
/// They are:
/// basis, cardinal, catmull-rom, linear, monotone, natural, step, step-after, and step-before
#[derive(Serialize)]
pub struct LineChartInterpolation {
    input: String,
    options: Vec<String>,
}
impl LineChartInterpolation {
    pub fn new() -> LineChartInterpolation {
        LineChartInterpolation {
            input: String::from("select"),
            options: vec![
                String::from("basis"),
                String::from("cardinal"),
                String::from("catmull-rom"),
                String::from("linear"),
                String::from("monotone"),
                String::from("natural"),
                String::from("step"),
                String::from("step-after"),
                String::from("step-before"),
            ],
        }
    }
}



#[derive(Serialize)]
pub struct LineChartData {
    name: String,
    values: Vec<LineChartDataValue>,
}

impl LineChartData {
    pub fn new() -> LineChartData {
        LineChartData {
            name: String::from("table"),
            values: vec![],
        }
    }
    pub fn add_data(&mut self, x: i64, y: i64, z: i64) {
        self.values.push(LineChartDataValue { x, y, z });
    }
}

#[derive(Serialize)]
pub struct LineChartDataValue {
    x: i64,
    y: i64,
    z: i64,
}

pub struct LineChartScale {
    name: String,
    scale_type: String,
    range: String,
    domain: JSONDict,
}
impl LineChartScale {
    pub fn new_xscale() -> LineChartScale {
        LineChartScale {
            name: String::from("x"),
            scale_type: String::from("point"),
            range: String::from("width"),
            domain: JSONDict::create("data", "table", "field", "x"),
        }
    }
    pub fn new_yscale() -> LineChartScale {
        LineChartScale {
            name: String::from("y"),
            scale_type: String::from("linear"),
            range: String::from("height"),
            domain: JSONDict::create("data", "table", "field", "y"),
        }
    }
    pub fn new_ordinal_scale() -> LineChartScale {
        LineChartScale {
            name: String::from("color"),
            scale_type: String::from("ordinal"),
            range: String::from("category"),
            domain: JSONDict::create("data", "table", "field", "z"),
        }
    }
}
impl Serialize for LineChartScale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("scale", 4)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("type", &self.scale_type)?;
        s.serialize_field("range", &self.range)?;
        s.serialize_field("domain", &self.domain)?;
        s.end()
    }
}

#[derive(Serialize)]
pub struct LineChartAxis {
    scale: String,
    orient: Orientation,
}

impl LineChartAxis {
    pub fn new_xaxis() -> LineChartAxis {
        LineChartAxis {
            scale: String::from("x"),
            orient: Orientation::Bottom,
        }
    }

    pub fn new_yaxis() -> LineChartAxis {
        LineChartAxis {
            scale: String::from("y"),
            orient: Orientation::Left,
        }
    }
}

pub struct LineChartMark {
    mark_type: String,
    from: LineChartFacet,
    marks: Vec<LineChartMarkDescription>,
}
impl LineChartMark {
    pub fn new() -> LineChartMark {
        LineChartMark {
            mark_type: String::from("group"),
            from: LineChartFacet::new(),
            marks: vec![LineChartMarkDescription::new()],
        }
    }
}
impl Serialize for LineChartMark {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("mark", 3)?;
        s.serialize_field("type", &self.mark_type)?;
        s.serialize_field("from", &self.from)?;
        s.serialize_field("marks", &self.marks)?;
        s.end()
    }
}
pub struct LineChartMarkDescription {
    mark_type: String,
    from: KeyVal,
    encode: LineChartEncoding,
}
impl LineChartMarkDescription {
    pub fn new() -> LineChartMarkDescription {
        LineChartMarkDescription {
            mark_type: String::from("line"),
            from: KeyVal::new("data", "series"),
            encode: LineChartEncoding::new(),
        }
    }
}
impl Serialize for LineChartMarkDescription {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("mark", 3)?;
        s.serialize_field("type", &self.mark_type)?;
        s.serialize_field("from", &self.from)?;
        s.serialize_field("encode", &self.encode)?;
        s.end()
    }
}

#[derive(Serialize)]
pub struct LineChartEncoding {
    enter: LineChartEnter,
    update: LineChartUpdate,
    hover: LineChartHover,
}
impl LineChartEncoding {
    pub fn new() -> LineChartEncoding {
        LineChartEncoding {
            enter: LineChartEnter::new(),
            update: LineChartUpdate::new(),
            hover: LineChartHover::new(),
        }
    }
}

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct LineChartEnter {
    x: JSONDict,
    y: JSONDict,
    stroke: JSONDict,
    strokeWidth: QualKeyVal,
}
impl LineChartEnter {
    pub fn new() -> LineChartEnter {
        LineChartEnter {
            x: JSONDict::create("scale", "x", "field", "x"),
            y: JSONDict::create("scale", "y", "field", "y"),
            stroke: JSONDict::create("scale", "color", "field", "z"),
            strokeWidth: QualKeyVal::new("value", 2.0),
        }
    }
}
#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct LineChartUpdate {
    interpolate: KeyVal,
    fillOpacity: QualKeyVal,
}
impl LineChartUpdate {
    pub fn new() -> LineChartUpdate {
        LineChartUpdate {
            interpolate: KeyVal::new("signal", "interpolate"),
            fillOpacity: QualKeyVal::new("value", 1.0),
        }
    }
}

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct LineChartHover {
    fillOpacity: QualKeyVal,
}
impl LineChartHover {
    pub fn new() -> LineChartHover {
        LineChartHover {
            fillOpacity: QualKeyVal::new("value", 0.5),
        }
    }
}

#[derive(Serialize)]
pub struct LineChartFacet {
    facet: LineChartDescriptor,
}
impl LineChartFacet {
    pub fn new() -> LineChartFacet {
        LineChartFacet {
            facet: LineChartDescriptor::new(),
        }
    }
}

#[derive(Serialize)]
pub struct LineChartDescriptor {
    name: String,
    data: String,
    groupby: String,
}
impl LineChartDescriptor {
    pub fn new() -> LineChartDescriptor {
        LineChartDescriptor {
            name: String::from("series"),
            data: String::from("table"),
            groupby: String::from("z"),
        }
    }
}
