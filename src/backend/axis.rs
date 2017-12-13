/* 
 * Author: Samuel Resendez
 * Primary implementation for Vega axis
 */


/// A struct to describe the axis, as specified by the Vega spec 3.0
/// https://vega.github.io/vega/docs/axes/
/// Still a work in progress to support everything in the specification
#[derive(Serialize)]
pub struct Axis {
    orient: Orientation, 
    scale: String,
}

/// Orientation represents the different directions to place the axis
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