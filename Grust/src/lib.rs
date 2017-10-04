

mod bar_chart;
mod axes;
mod bar;
mod render_d3;


#[cfg(test)]
mod tests {
    use std::fs::remove_file;
    use render_d3::start_file;
    #[test]
    fn test_start_file() {
        match start_file("test.html".to_string()) {
            Ok(_) => println!("{:?}", remove_file("text.html")),
            Err(_) => panic!("Failed to create file"),
        }
        
    }
}
