
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;


pub mod backend;



#[cfg(test)]
mod tests {
    use backend::utils::get_rgb_representation;
    use backend::bar_chart::BarChart;
    
    #[test]
    fn test_rgb_conversion() {
        assert_eq!(get_rgb_representation("#FF0000"), Ok((255, 0, 0)));
        assert_eq!(get_rgb_representation("#000"), Err("Malformatted"));
    }
    #[test]
    fn test_bar_chart_serialization() {
        let chart = BarChart::new("#FFFFFF", "Test".to_string(),true, 800, 800 );
        assert_eq!(chart.to_json() == "".to_string(), false);
    }
    
}
