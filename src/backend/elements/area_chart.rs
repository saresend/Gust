/* Author: Samuel Resendez
 * Contains most of the needed elements for
 * area_charts
 */

use backend::elements::general::*;


use serde::ser::{Serializer, SerializeStruct, Serialize};

#[derive(Serialize)]
pub struct AreaChartSignal {
    name: String,
    value: String,
    bind: SignalBinding,
}

impl AreaChartSignal {
    pub fn default() -> AreaChartSignal {
        AreaChartSignal {
            name: String::from("interpolate"),
            value: String::from("monotone"),
            bind: SignalBinding::default(),
        }
    }
}
#[derive(Serialize)]
pub struct SignalBinding {
    input: String,
    options: Vec<String>,
}
impl SignalBinding {
    pub fn default() -> SignalBinding {
        SignalBinding {
            input: String::from("select"),
            options: vec![
                String::from("basis"),
                String::from("cardinal"),
                String::from("linear"),
                String::from("monotone"),
            ],
        }
    }
}
#[derive(Serialize)]
pub struct AreaChartData {
    name: String,
    values: Vec<AreaChartDataEntry>,
}

impl AreaChartData {
    pub fn default() -> AreaChartData {
        AreaChartData {
            name: String::from("table"),
            values: vec![],
        }
    }
    pub fn add_data(&mut self, u: i32, v: i32) {
        self.values.push(AreaChartDataEntry { u, v });
    }
}
#[derive(Serialize)]
pub struct AreaChartDataEntry {
    u: i32,
    v: i32,
}


#[derive(Serialize)]
pub struct AreaChartAxis {
    orient: Orientation,
    scale: String,
}

impl AreaChartAxis {
    pub fn x_axis() -> AreaChartAxis {
        AreaChartAxis {
            orient: Orientation::Bottom,
            scale: String::from("xscale"),
        }
    }
    pub fn y_axis() -> AreaChartAxis {
        AreaChartAxis {
            orient: Orientation::Left,
            scale: String::from("yscale"),
        }
    }
}
pub struct AreaChartMark {
    mark_type: String,
    from: KeyVal,
    encode: AreaChartEncoding,
}

impl AreaChartMark {
    pub fn default() -> AreaChartMark {
        AreaChartMark {
            mark_type: String::from("area"),
            from: KeyVal::new("data", "table"),
            encode: AreaChartEncoding::default(),
        }
    }
}

impl Serialize for AreaChartMark {
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
pub struct AreaChartEncoding {
    enter: AreaChartEnter,
    update: AreaChartUpdate,
    hover: AreaChartHover,
}

impl AreaChartEncoding {
    pub fn default() -> AreaChartEncoding {
        AreaChartEncoding {
            enter: AreaChartEnter::default(),
            update: AreaChartUpdate::default(),
            hover: AreaChartHover::default(),
        }
    }
}
#[derive(Serialize)]
pub struct AreaChartEnter {
    x: JSONDict,
    y: JSONDict,
    y2: JSONDict,
    fill: KeyVal,
}

impl AreaChartEnter {
    pub fn default() -> AreaChartEnter {
        AreaChartEnter {
            x: JSONDict::create("scale", "xscale", "field", "u"),
            y: JSONDict::create("scale", "yscale", "field", "v"),
            y2: JSONDict::band_create("scale", "yscale", "value", 0),
            fill: KeyVal::new("value", "steelblue"),
        }
    }
}
#[derive(Serialize)]
pub struct AreaChartUpdate {
    interpolate: KeyVal,
    fillOpacity: QualKeyVal,
}

impl AreaChartUpdate {
    pub fn default() -> AreaChartUpdate {

        AreaChartUpdate {
            interpolate: KeyVal::new("signal", "interpolate"),
            fillOpacity: QualKeyVal::new("value", 1.0),
        }
    }
}

#[derive(Serialize)]
#[ignore(warn_snake_case)]
pub struct AreaChartHover {
    fillOpacity: QualKeyVal,
}
impl AreaChartHover {
    pub fn default() -> AreaChartHover {
        AreaChartHover { fillOpacity: QualKeyVal::new("value", 0.5) }
    }
}

pub struct AreaChartScale {
    name: String,
    scale_type: String,
    range: String,
    zero: bool,
    domain: JSONDict,
}

impl AreaChartScale {
    pub fn default_x() -> AreaChartScale {
        AreaChartScale {
            name: String::from("xscale"),
            scale_type: String::from("linear"),
            zero: false,
            range: String::from("width"),
            domain: JSONDict::create("data", "table", "field", "u"),
        }

    }
    pub fn default_y() -> AreaChartScale {
        AreaChartScale {
            name: String::from("yscale"),
            scale_type: String::from("linear"),
            zero: true,
            range: String::from("height"),
            domain: JSONDict::create("data", "table", "field", "v"),
        }

    }
}


impl Serialize for AreaChartScale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("area_chart", 10)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("type", &self.scale_type)?;
        s.serialize_field("range", &self.range)?;
        s.serialize_field("zero", &self.zero)?;
        s.serialize_field("domain", &self.domain)?;



        s.end()

    }
}
