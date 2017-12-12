
/* 
 * Author: Samuel Resendez
 * 
 */

use serde::ser::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;

pub struct Mark {
    pub mark_type: String, 
    pub from: Source, 
    pub encode: Encoding,
}


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

#[derive(Serialize)]
pub struct Scaler {
    pub scale: String,
    pub field: String,
}


#[derive(Serialize)]
pub struct Source {
    pub data: String,
}

impl Mark {


    pub fn new() -> Mark {
        Mark {
            mark_type: "rect".to_string(),
            from: Source { data: }
        }
    }
}


impl Serialize for Mark {



}