

mod bar_chart;
mod axes;
mod bar;
mod render_d3;


#[cfg(test)]
mod tests {

    use render_d3::start_file;
    #[test]
    fn test_start_file() {
        match start_file("test.html".to_string()) {
            Ok(_) => {},
            Err(_) => panic!("Failed to create file"),
        }
    }
}
