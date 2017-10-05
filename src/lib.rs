
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub mod bar_chart;
pub mod axes;
pub mod bar;
pub mod elements;
pub mod utils;

#[cfg(test)]
mod tests {
    use utils::get_rgb_representation;
    
    #[test]
    fn test_rgb_conversion() {
        assert_eq!(get_rgb_representation("#FF0000".to_string()), Ok((255, 0, 0)));
    }
    
}
