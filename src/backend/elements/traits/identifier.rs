

pub trait Identifiable {

    /// This method is used when saving the graph to files, and therefore must be implemented 
    /// for all saveable graph types
    fn get_identifier(&self) -> String;
}