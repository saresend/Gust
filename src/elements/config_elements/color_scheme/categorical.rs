use super::vega_scheme::VegaColorScheme;

#[derive(Serialize)]
pub enum CategoricalScheme {
    #[serde(rename = "accent")]
    Accent,
    #[serde(rename = "category10")]
    Category10,
    #[serde(rename = "category20")]
    Category20,
    #[serde(rename = "category20b")]
    Category20b, 
    #[serde(rename = "category20c")]
    Category20c, 
    #[serde(rename = "dark2")]
    Dark2, 
    #[serde(rename = "paired")]
    Paired, 
    #[serde(rename = "pastel1")]
    Pastel1, 
    #[serde(rename = "pastel2")]
    Pastel2, 
    #[serde(rename = "set1")]
    Set1, 
    #[serde(rename = "set2")]
    Set2, 
    #[serde(rename = "set3")]
    Set3, 
    #[serde(rename = "tableu10")]
    Tableu10, 
    #[serde(rename = "tableu20")]
    Tableu20, 
}

impl VegaColorScheme for CategoricalScheme {}

