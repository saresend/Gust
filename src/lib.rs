
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
    use super::frontend::write::render_graph;
    use std::path::Path;

    #[test]
    fn test_save_graph() {
        let g = Graph::new("test0", GraphType::Bar);
        let _ = save_graph(&g);
    }

    #[test] 
    fn test_hydrated_graph() {
        let mut g = Graph::new("test1", GraphType::Bar);
        let mut data = Data::new(SERIESNAME.to_string());
        data.create_fake_data(20);
        g.data.push(data);
        let _ = save_graph(&g);
    }
    #[test]
    fn test_html_gen() {
        let mut g = Graph::new("test2", GraphType::Bar);
        let mut data = Data::new(SERIESNAME.to_string());
        data.create_fake_data(20);
        g.data.push(data);
        let _ = save_graph(&g);
        render_graph("test", Path::new("test2.json"));
    }
    
}
