
/* 
 * Author: Samuel Resendez
 * 
 */

use serde::ser::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;

use backend::constants::*;

pub struct Mark {
    pub mark_type: String, 
    pub from: Source, 
    pub encode: Encoding,
}

#[derive(Serialize)]
pub struct Encoding {
    pub enter: Visualization,
    pub update: Fill,
    pub hover: Fill,
}

#[derive(Serialize)]
pub struct Fill {
    pub fill: Val,
}
#[derive(Serialize)]
pub struct Val {
    pub value: String,
}

#[derive(Serialize)]
pub struct Visualization {
    pub x: Scaler,
    pub width: Scaler,
    pub y: Scaler,
    pub y2: Scaler,
}

pub struct Scaler {
    pub scale: String,
    pub y_name: &'static str,
    pub field: String,
}


#[derive(Serialize)]
pub struct Source {
    pub data: String,
}

impl  Scaler {
    pub fn new(scale: &str,name: &'static str, field: &str) -> Scaler {
        Scaler {
            scale: scale.to_string(),
            y_name: name,
            field: field.to_string(),
        }
    }
}

impl Val {
    pub fn new(color: &str) -> Val {
        Val {
            value: color.to_string(),
        }
    }
}

impl Fill {
    pub fn new(color: &str) -> Fill {
        Fill {
            fill: Val::new(color),
        }
    }
}

impl Visualization {
    pub fn new() -> Visualization {
        Visualization {
            x: Scaler::new(XSCALE,"field", XCOORD),
            width: Scaler::new(XSCALE, "band", "1"),
            y: Scaler::new(YSCALE, "field", YCOORD),
            y2: Scaler::new(YSCALE, "value", "0"),
        }
    }
}


impl Encoding {

    pub fn new() -> Encoding {
        Encoding {
            enter: Visualization::new(),
            update: Fill::new("steelblue"),
            hover: Fill::new("red"),
        }
    }
}

impl Mark {

    pub fn new() -> Mark {
        Mark {
            mark_type: "rect".to_string(),
            from: Source { data: SERIESNAME.to_string() },
            encode: Encoding::new(),
        }
    }
}


impl Serialize for Mark {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Mark", 3)?;
        let _ = s.serialize_field("type", &self.mark_type);
        let _ = s.serialize_field("from", &self.from);
        let _ = s.serialize_field("encode", &self.encode);
        s.end()
    }

}


impl Serialize for Scaler {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Scaler", 2)?;
        let _ = s.serialize_field(&self.y_name, &self.field);
        let _ = s.serialize_field("scale", &self.scale);
        s.end()
    }
}