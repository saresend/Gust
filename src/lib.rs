
extern crate serde;
extern crate serde_json;


#[macro_use]
extern crate serde_derive;



pub mod backend;
pub mod frontend;


#[cfg(test)]
mod tests {
    
    use super::backend::graph::Graph;
    use super::backend::io::save_graph;

    #[test]
    fn test_save_graph() {
        let g = Graph::new("Test Description".to_string());
        let _ = save_graph(&g);
    }
    
}
