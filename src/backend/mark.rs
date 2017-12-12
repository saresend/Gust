
/* 
 * Author: Samuel Resendez
 * 
 */
pub struct Mark {

    mark_type: String, 
    from: Source, 
    encode: Encoding,
}


pub struct Encoding {
    enter: Visualization,
    update: Fill,
    hover: Fill,
}

#[derive(Serialize)]
pub struct Fill {
    fill: Val,
}
#[derive(Serialize)]
pub struct Val {
    value: String,
}

#[derive(Serialize)]
pub struct Visualization {
    x: Scaler,
    width: Scaler,
    y: Scaler,
    y2: Scaler,
}

#[derive(Serialize)]
pub struct Scaler {
    scale: String,
    field: String,
}


#[derive(Serialize)]
pub struct Source {
    data: String,
}

impl Mark {


    pub fn new()
}


impl Serialize for Mark {



}