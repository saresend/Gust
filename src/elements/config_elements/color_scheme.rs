#[derive(Serialize)]
pub struct ColorScheme {
    scheme: SchemeOptions,
    count: i32,
    extent: Vec<f64>,
}

#[derive(Serialize)]
pub struct SchemeOptions {
    name: SchemeNames,
}

#[derive(Serialize)]
pub enum SchemeNames {
    Category,
    Blues,
    Greens,
    Greys,
    Purples,
    Reds,
    Oranges,
}
