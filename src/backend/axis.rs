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
#[serde(rename_all="lowercase")]
pub enum Orientation {
    Left,
    Right, 
    Top,
    Bottom,
}


impl Axis {
    pub fn new(orient: Orientation, scale: &str) -> Axis {
        Axis {
            orient: orient,
            scale: scale.to_string(),
        }
    }
}