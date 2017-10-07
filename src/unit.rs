#[derive(Serialize, Deserialize)]
pub enum Unit {
    Meters,
    Feet,
    Inches,
    Yards,
    None,
}

pub fn parse_unit(unit: Unit) -> String {
    match unit {
        Unit::Meters => "Meters".to_string(),
        Unit::Feet => "Feet".to_string(),
        Unit::Inches => "Inches".to_string(),
        Unit::Yards => "Yards".to_string(),
        Unit::None => "".to_string(),
    }
}
