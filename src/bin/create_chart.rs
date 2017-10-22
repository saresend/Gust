extern crate grust;

use grust::backend::bar_chart::BarChart;




fn main() {
    let chart = BarChart::new();
    println!("{}", chart.to_json());

}
