use super::color_scheme::vega_scheme::VegaColorScheme;
use serde::Serialize;
use std::boxed::Box;

#[derive(Serialize)]
pub struct ScaleRangeProperties<T, U, V, W, X>
where
    T: VegaColorScheme,
    U: VegaColorScheme,
    V: VegaColorScheme,
    W: VegaColorScheme,
    X: VegaColorScheme,
{
    category: T,
    diverging: U,
    heatmap: V,
    ordinal: W,
    ramp: X,
    symbol: Vec<String>,
}
