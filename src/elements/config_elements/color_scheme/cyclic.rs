use super::vega_scheme::VegaColorScheme;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CyclicScheme {
    Rainbow,
    Sinebow,
}

// Just to indicate that this type is eligible to be a color scheme
impl VegaColorScheme for CyclicScheme {}
