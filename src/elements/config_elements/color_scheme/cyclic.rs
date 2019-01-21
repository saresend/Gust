use super::vega_scheme::VegaColorScheme;

#[derive(Serialize)]
pub enum CyclicScheme {
    #[serde(rename = "rainbow")]
    Rainbow,
    #[serde(rename = "sinebow")]
    Sinebow,
}

// Just to indicate that this type is eligible to be a color scheme
impl VegaColorScheme for CyclicScheme {}
