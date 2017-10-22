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

    pub fn new(color: &str, title: String, has_line: bool, max_y_value: i64, max_x_value: i64) -> BarChart {

        let unit = Unit::None;
        let unit2 = Unit::None;
        let vec : Vec<Bar> = vec![];
        
        BarChart{
            color: utils::get_rgb_representation(color).unwrap(),
            x_axis: XAxis::new(title.clone(), unit, has_line, max_x_value),
            y_axis: YAxis::new(title, unit2, has_line, max_y_value),
            show_grid: true,
            bars: vec,
        }
    }



    pub fn to_json(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(p) => p,
            Err(_) => "".to_string(),
        }
    }
}
