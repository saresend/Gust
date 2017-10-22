extern crate serde_json;
use backend::axes::{XAxis, YAxis};
use backend::bar::Bar;
use backend::utils;
use backend::unit::Unit;

#[derive(Serialize)]
pub struct BarChart {
    color: (u8, u8, u8), //Denotes the color as RGB
    x_axis: XAxis,
    y_axis: YAxis,
    show_grid: bool,
    bars: Vec<Bar>,
}



impl BarChart {
    pub fn get_dimensions(&self) -> (i64, i64) {
        (self.x_axis.get_dimension(), self.y_axis.get_dimension())
    }

    pub fn set_color(&mut self, col: &str) {
        match utils::get_rgb_representation(col) {
            Ok(val) => self.color = val,
            Err(_) => (),
        }
    }

    pub fn should_show_grid(&mut self, i: bool) {
        self.show_grid = i;
    }

    pub fn add_bar(&mut self, b: Bar) {
        self.bars.push(b);
    }

    pub fn new() -> BarChart {

        let x = XAxis::new("X axis".to_string(), Unit::None, true, 100);
        let y = YAxis::new("Y axis".to_string(), Unit::None, true, 100);

        BarChart {
            color: (0, 0, 0),
            x_axis: x,
            y_axis: y,
            show_grid: true,
            bars: vec![],
        }
    }

    pub fn to_json(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(p) => p,
            Err(_) => "".to_string(),
        }
    }
}
