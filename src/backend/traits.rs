
use std::io;

pub trait Graphable {
    fn save_to_file(&self) -> Result<(), io::Error>;
}
