use std::fs::File;
use std::io::prelude::*;

use backend::bar::Bar;



pub fn begin_file(file: &mut File) {
    file.write_all(b"<svg width=\"1000\" height=\"1000\">");
}
pub fn end_file(file: &mut File) {
    file.write_all(b"</svg>");
}


pub fn write_bar_svg(bar: Bar, file: &mut File) {
    file.write_all(bar.render(1).as_bytes());
}
