
extern crate serde;
extern crate serde_json;


#[macro_use]
extern crate serde_derive;



pub mod backend;
pub mod frontend;


#[cfg(test)]
mod tests {
    
    use super::backend::graph::{Graph, GraphType};
    use super::backend::data::data::Data;
    use super::backend::constants::*;
    use super::backend::io::save_graph;

    #[test]
    fn test_save_graph() {
        let g = Graph::new(GraphType::Bar);
        let _ = save_graph(&g);
    }

    #[test] 
    fn test_hydrated_graph() {
        let mut g = Graph::new(GraphType::Bar);
        let mut data = Data::new(SERIESNAME.to_string());
        data.create_fake_data(20);
        g.data.push(data);
        let _ = save_graph(&g);
    }
    
}
