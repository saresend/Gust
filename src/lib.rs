/*! Gust is a library that makes it easy to build simple data visualizations with Rust. 
 It's built focused on easy of use and simplicity, and uses d3 and Vega to create an interactive
chart

# Usage 

Gust is on crates.io and can be added by adding 'gust' to your dependencies in your `Cargo.toml`.
 
```toml
[dependencies]
gust = "0.1.4"
```

 and (if using edition 2015) then add this to your crate root:

```rust 
 extern crate gust;
```
 
# Example: create a bar chart 
 
 Use of gust involves defining a Graphable object like a bar graph, and adding 
 whatever data you would like to visualize, and then simply calling render on it.
 Gust will build your graph and add it to the `gust_build` folder, under whatever file format you 
 have specified.
 
```rust 
    use gust::backend::bar_chart::BarChart;
    use gust::frontend::write::render_graph;
    use gust::backend::general::FileType;
 
    let mut b = BarChart::new();
        let v = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L"];
        for i in 0..10 {
            b.add_data(v[i].to_string(), (i * i * i) as i32);
        }
       render_graph(&b, FileType::HTML).unwrap();
```
 
 More ergonomic configuration options are in the process of being implemented, but not quite done. For the 
 time being, most member variables have been left public, and may be modified. 
 
 
# Notes about building 
 
 Gust will by default write all of the output html files into the top level gust_build directory 
 that is then stratified by the type of output file specified. 
 
 For Example: 
 
```rust
use gust::backend::bar_chart::BarChart;
use gust::frontend::write::render_graph;
use gust::backend::general::FileType;

let mut b = BarChart::new();
        let v = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L"];
        for i in 0..10 {
            b.add_data(v[i].to_string(), (i * i * i) as i32);
        }
       render_graph(&b, FileType::HTML).unwrap();
```

 The resulting file will be saved as gust_build/html/barchart.html
 If you want to update the name, just change the identifier (`b.identifier`)
*/

pub mod backend;
pub mod frontend;

extern crate liquid;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;



#[cfg(test)]
mod tests {

    use super::backend::bar_chart::BarChart;
    use super::backend::stacked_bar_chart::StackedBarChart;
    use super::backend::line_chart::LineChart;
    use super::backend::area_chart::AreaChart;
    use super::frontend::write::render_graph;
    use super::backend::general::FileType;
    #[test]
    fn test_bar_chart() {
        let mut b = BarChart::new();
        let v = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L"];
        for i in 0..10 {
            b.add_data(v[i].to_string(), (i * i * i) as i32);
        }
        render_graph(&b, FileType::HTML).unwrap();
        render_graph(&b, FileType::JSON).unwrap();
    }
    #[test]
    fn test_stacked_bar_chart() {
        let mut b = StackedBarChart::new();
        for i in 0..10 {
            b.add_data(i, i * i, 1);
            b.add_data(i, i + i, 0);
        }
        render_graph(&b, FileType::HTML).unwrap();
    }
    #[test]
    fn test_line_chart() {
        let mut c = LineChart::new();
        for i in 0..20 {
            c.add_data(i, i * i, 0);
            c.add_data(i, 2 * i, 1);
        }
        render_graph(&c, FileType::HTML).unwrap();
    }
    #[test]
    fn test_configuring_chart() {
        let mut b = BarChart::new();
        for i in 1..35 {
            b.add_data(i.to_string(), i + 1);
        }
        b.set_identifier("configured_bar_chart");
        b.set_description("Gusty Chart");
        b.set_color("black");
        render_graph(&b, FileType::HTML).unwrap();
    }
    #[test]
    fn test_area_chart() {
        let mut area_chart = AreaChart::new();


        for i in 0..20 {
            area_chart.add_data(i, 2 * i);
        }

        render_graph(&area_chart, FileType::HTML).unwrap();

    }

}
