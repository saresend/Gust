
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;


pub mod backend;
pub mod frontend;


#[cfg(test)]
mod tests {
    use backend::utils::get_rgb_representation;
    use backend::bar_chart::BarChart;
    
}
