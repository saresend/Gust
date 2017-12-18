
extern crate serde;
extern crate serde_json;


#[macro_use]
extern crate serde_derive;



pub mod backend;
pub mod frontend;



#[cfg(test)]
mod tests {

    use super::backend::bar_chart::BarChart;
    use super::backend::stacked_bar_chart::StackedBarChart;
    use super::backend::traits::*;
    #[test]
    fn test_bar_chart() {
        let mut b = BarChart::new();
        let v = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L"];
        for i in 0..10 {
            b.add_data(v[i].to_string(), (i * i * i) as i32);
        }
        b.save_to_file().unwrap();
    }
    #[test]
    fn test_stacked_bar_chart() {
        let mut b = StackedBarChart::new();
        for i in 0..50 {
            b.add_data(i, i * i, 1);
            b.add_data(i, i + i, 0);
        }
        b.save_to_file().unwrap();
    }
}
