
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
    use super::backend::elements::io::save_graph;
   
    #[test]
    fn test_bar_chart() {
        let mut b = BarChart::new("test1");
        let v = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "J"];
        for i in 0..10 {
            b.add_data_point(v[i], (i * i) as i64);
        }
        save_graph(&b).unwrap();
    }
    #[test]
    fn test_stacked_bar_chart() {
        let mut b = StackedBarChart::new("test_stacked_b_chart");
        for i in 0..10 {
            
            let mut z = 0;
            if i % 2 == 0 {
                z = 1;
            }
             b.add_data_point(i, i*i, z);
        }
        for i in 0..10 {
            
            let mut z = 0;
            if i % 2 != 0 {
                z = 1;
            }
             b.add_data_point(i, i*i, z);
        }
        save_graph(&b).unwrap();
       
    }
    
}
