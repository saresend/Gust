
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;


pub mod backend;
pub mod frontend;


#[cfg(test)]
mod tests {
    
    use super::backend::models::*;
    use super::backend::io::save_graph;


    #[test]
    fn test_graph_saving() {

        let g = Graph::new("Title".to_string(), GraphType::Bar);
        let _ = save_graph(&g);
    }

    
}
