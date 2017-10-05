
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod bar_chart;
mod axes;
mod bar;


#[cfg(test)]
mod tests {
    use std::fs::remove_file;
    use axes::{YAxis, XAxis};
    #[test]
    fn test_XAxis_Serialization() {
        let x1 = XAxis::new("Hi there".to_string(), "Feet".to_string(), false, 200);

        println!("{}", x1.to_json());
    }
    
}
