use super::super::autosize::Autosize;

use serde::ser::{Serialize, SerializeStruct, Serializer};

pub enum GroupOptions {
    Fill(String),
    Stroke(String),
    StrokeWidth(i32),
}

impl Serialize for GroupOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut group_options = serializer.serialize_struct("GroupOptions", 1)?;
        match &self {
            GroupOptions::Fill(color) => group_options.serialize_field("fill", color)?,
            GroupOptions::Stroke(color) => group_options.serialize_field("stroke", color)?,
            GroupOptions::StrokeWidth(color) => {
                group_options.serialize_field("strokeWidth", color)?
            }
        }
        group_options.end()
    }
}

#[derive(Serialize)]
pub struct ViewProperties {
    autosize: Autosize,
    background: String,
    group: GroupOptions,
}
