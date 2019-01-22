#[derive(Serialize)]
pub struct TitleProperties {
    align: String, 
    anchor: String, 
    angle: i32, 
    baseline: String, 
    color: String, 
    font: String, 
    fontSize: i32, 
    fontWeight: i32, 
    frame: FrameOptions, 
    limit: i32, 
    offset: i32, 
    orient: OrientationOptions, 
}


#[derive(Serialize)]
#[serde(rename = "lowercase")]
pub enum OrientationOptions {
    Top,
    Bottom, 
    Left, 
    Right, 
}

#[derive(Serialize)]
#[serde(rename = "lowercase")]
pub enum FrameOptions {
    Bounds, 
    Group, 
}