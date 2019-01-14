#[derive(Serialize, Deserialize)]
pub enum Autosize {
    Pad,
    Fit,
    FitX,
    FitY,
    None,
    Custom(String),
}
