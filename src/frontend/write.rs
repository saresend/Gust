/* File Stub for the Future */
/* Author: Samuel Resendez */

use backend::traits::Graphable;
use backend::general::FileType;
use frontend::html_render::create_html;
use std;
use std::io::Write;
use std::fs::DirBuilder;



pub fn render_graph<T: Graphable>(graph: &T, ft: FileType) -> Result<(), std::io::Error> {
    create_folder_structure(DirBuilder::new().recursive(true));

    let extension = match ft {
        FileType::HTML => "html",
        FileType::JSON => "json",
    };

    let s = graph.get_json_representation();
    let mut f = std::fs::File::create(format!(
        "gust_build/{}/{}.{}",
        extension,
        graph.get_identifier(),
        extension,
    ))?;


    match ft {
        FileType::HTML => {
            let file_path = format!("gust_build/html/raw/{}.json", graph.get_identifier());
            let mut raw_file = std::fs::File::create(&file_path)?;
            raw_file.write_all(s.as_bytes())?;
            let html_str = create_html(
                &format!("raw/{}.json", graph.get_identifier()),
                graph.get_identifier(),
            ).unwrap();
            f.write_all(html_str.as_bytes())?;
        }

        FileType::JSON => {
            f.write_all(s.as_bytes())?;
        }
    }

    Ok(())
}



fn create_folder_structure(builder: &DirBuilder) {
    let path1 = "gust_build/html/raw";
    let path2 = "gust_build/json";
    builder.create(path1).unwrap();
    builder.create(path2).unwrap();
}
