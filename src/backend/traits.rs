

use serde::ser::Serialize;
use serde_json;

pub trait Graphable: Sized + Serialize {
    fn get_json_representation(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn get_description(&self) -> &str;
    fn get_identifier(&self) -> &str;

    // Define the API for what we can adjust through a Graph
    // set_color will certainly be part of the API, likely accepting a &str
    fn set_color(&mut self, &str);
    // set_dimensions
    fn set_dimensions(&mut self, (i32, i32));

    fn get_dimensions(&self) -> (i32, i32);

    fn set_description(&mut self, &str);

    fn set_identifier(&mut self, &str);
}
