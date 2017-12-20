
use std::io;
use backend::general::FileType;

pub trait Graphable {
    fn save_to_file(&self, FileType) -> Result<(), io::Error>;
}
