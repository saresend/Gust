
extern crate serde;
extern crate serde_json;


#[macro_use]
extern crate serde_derive;



pub mod backend;
pub mod frontend;



#[cfg(test)]
mod tests {

    use super::backend::bar_chart::BarChart;
    use super::backend::traits::*;
    #[test]
    fn test_bar_chart() {
        let b = BarChart::new();
        b.save_to_file().unwrap();
    }
}
