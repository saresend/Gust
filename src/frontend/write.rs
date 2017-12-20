/* File Stub for the Future */
/* Author: Samuel Resendez */

use backend::traits::Graphable;
use backend::general::FileType;
use std;
use std::io::Write;



pub fn render_graph<T: Graphable>(graph: &T, ft: FileType) -> Result<(), std::io::Error> {
  let extension = match ft {
    FileType::HTML => ".html",
    FileType::JSON => ".json",
  };

  let s = graph.get_json_representation();
  let mut f = std::fs::File::create(format!("{}{}", graph.get_identifier(), extension))?;

  match ft {
    FileType::HTML => {}
    FileType::JSON => {
      f.write_all(s.as_bytes())?;
    }
  }

  Ok(())
}
