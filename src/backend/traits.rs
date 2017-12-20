

use serde::ser::Serialize;
use serde_json;

pub trait Graphable: Sized + Serialize {
    fn get_json_representation(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn get_description(&self) -> &str;
    fn get_identifier(&self) -> &str;
}
