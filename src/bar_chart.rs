
use axes::{XAxis, YAxis};
use bar::Bar;

pub struct BarChart {
    color : (u8, u8, u8), //Denotes the color as RGB
    x_axis : XAxis,
    y_axis: YAxis,
    show_grid: bool,
    bars: Vec<Bar>,
}



impl BarChart {


    pub fn get_dimensions(&self) -> (i64, i64) {
        (self.x_axis.get_dimension(), self.y_axis.get_dimension())
    }

    
    


}
