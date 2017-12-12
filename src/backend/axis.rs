/* 
 * Author: Samuel Resendez
 * Primary implementation for Vega axis
 */


#[derive(Serialize)]
pub struct Axis {
    orient: Orientation, 
    scale: String,
}

#[derive(Serialize)]
pub enum Orientation {
    left,
    right, 
    top,
    bottom,
}


impl Axis {
    pub fn new(orient: Orientation, scale: &str) -> Axis {
        Axis {
            orient: orient,
            scale: scale.to_string(),
        }
    }
}