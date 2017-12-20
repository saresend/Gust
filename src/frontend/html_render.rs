

use std;

use liquid;




pub fn create_html(data: &str, title: &str) -> Result<(String), std::io::Error> {
    let mut globals = liquid::Object::new();
    globals.insert("data".to_owned(), liquid::Value::str(data));
    globals.insert("title".to_owned(), liquid::Value::str(title));

    let template = liquid::ParserBuilder::with_liquid()
        .build()
        .parse_file("./src/frontend/template.html")
        .unwrap();
    let output = template.render(&globals).unwrap();
    Ok((output))
}
