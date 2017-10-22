

extern crate grust;

use std::fs::File;
use grust::backend::bar::Bar;
use grust::frontend::write;

fn main() {

    let b = Bar {
        height: 32,
        width: 32,
        label: "Helloo".to_string(),
        style: "uh".to_string(),
    };

    let mut file = File::create("out.html").unwrap();
    write::begin_file(&mut file);
   
    write::write_bar_svg(b,&mut file);

    write::end_file(&mut file);

    


}
