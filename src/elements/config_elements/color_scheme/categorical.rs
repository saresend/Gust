use super::vega_scheme::VegaColorScheme;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CategoricalScheme {
    Accent,
    Category10,
    Category20,
    Category20b,
    Category20c,
    Dark2,
    Paired,
    Pastel1,
    Pastel2,
    Set1,
    Set2,
    Set3,
    Tableu10,
    Tableu20,
}

impl VegaColorScheme for CategoricalScheme {}
