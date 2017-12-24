

use std;
use liquid;


const TEMPLATE: &'static str = include_str!("template.html");

pub fn create_html(data: &str, title: &str) -> Result<(String), std::io::Error> {
    let mut globals = liquid::Object::new();
    globals.insert("data".to_owned(), liquid::Value::str(data));
    globals.insert("title".to_owned(), liquid::Value::str(title));

    let template = liquid::ParserBuilder::with_liquid()
        .build()
        .parse(TEMPLATE) 
        .unwrap();
    let output = template.render(&globals).unwrap();
    Ok((output))
}
