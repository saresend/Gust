
extern crate serde;
extern crate serde_json;


#[macro_use]
extern crate serde_derive;



pub mod backend;
pub mod frontend;


#[cfg(test)]
mod tests {
    
    
    use super::backend::bar_chart::BarChart;
    use super::backend::elements::io::save_graph;
   
    #[test]
    fn test_bar_chart() {
        let b = BarChart::new("test1");
        save_graph(&b).unwrap();
    }
    
}
