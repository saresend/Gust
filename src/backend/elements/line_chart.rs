


use backend::elements::general::*;

#[derive(Serialize)]
pub struct LineChartSignal {
    pub name: String,
    pub value: String,
    pub bind: LineChartInterpolation,
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

#[derive(Serialize)]
pub struct LineChartInterpolation {
    pub input: String,
    pub options: Vec<String>,
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
    pub name: String,
    pub values: Vec<LineChartDataValue>,
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
    pub name: String,
    pub scale_type: String,
    pub range: String,
    pub domain: JSONDict,
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
    pub mark_type: String,
    pub from: LineChartFacet,
    pub marks: Vec<LineChartMarkDescription>,
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
pub struct LineChartMarkDescription {
    pub mark_type: String,
    pub from: KeyVal,
    pub encode: LineChartEncoding,
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
pub struct LineChartEncoding {
    pub enter: LineChartEnter,
    pub update: LineChartUpdate,
    pub hover: LineChartHover,
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

#[allow(non_snake_case)]
pub struct LineChartEnter {
    pub x: JSONDict,
    pub y: JSONDict,
    pub stroke: JSONDict,
    pub strokeWidth: QualKeyVal,
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
#[allow(non_snake_case)]
pub struct LineChartUpdate {
    pub interpolate: KeyVal,
    pub fillOpacity: QualKeyVal,
}
impl LineChartUpdate {
    pub fn new() -> LineChartUpdate {
        LineChartUpdate {
            interpolate: KeyVal::new("signal", "interpolate"),
            fillOpacity: QualKeyVal::new("value", 1.0),
        }
    }
}
#[allow(non_snake_case)]
pub struct LineChartHover {
    pub fillOpacity: QualKeyVal,
}
impl LineChartHover {
    pub fn new() -> LineChartHover {
        LineChartHover {
            fillOpacity: QualKeyVal::new("value", 0.5),
        }
    }
}

pub struct LineChartFacet {
    pub facet: LineChartDescriptor,
}
impl LineChartFacet {
    pub fn new() -> LineChartFacet {
        LineChartFacet {
            facet: LineChartDescriptor::new(),
        }
    }
}
pub struct LineChartDescriptor {
    pub name: String,
    pub data: String,
    pub groupby: String,
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
