


#[derive(Serialize, Deserialize)]
pub struct Tick {
    title: String,
}



#[derive(Serialize, Deserialize)]
pub enum Unit {
    Meters,
    Feet,
    Inches,
    Yards,
    None,
}
