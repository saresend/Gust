/* Author: Samuel Resendez
 * The Top level vega object, but as a rust struct!
 */

use serde::ser::Serialize;

pub struct Graph {
    schema: String,
    description: String,
}

impl Serialize for Graph {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Graph", 2)?;
        s.serialize_field("$schema", &self.schema);
        s.serialize_field("description", &self.description);
        s.end()
    }
}
