
extern crate serde;
extern crate serde_json;


#[macro_use]
extern crate serde_derive;



pub mod backend;
pub mod frontend;


#[cfg(test)]
mod tests {
    
    use super::backend::elements::graph::{Graph, GraphType};
    use super::backend::elements::data::data::Data;
    use super::backend::elements::data::transform::*;
    use super::backend::elements::constants::*;
    use super::backend::elements::io::save_graph;
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
    #[test]
    fn test_stacked_bar_chart() {
        let mut g = Graph::new("StackedBar", GraphType::StackedBar);
        let mut data = Data::new(SERIESNAME.to_string());
        data.add_transform(Transform::new(TransformType::Stack));
        data.create_fake_stack_bchart_data();
        g.data.push(data);
        let _ = save_graph(&g);
        
    }
    
}
