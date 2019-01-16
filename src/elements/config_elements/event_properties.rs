use serde::ser::{Serialize, SerializeStruct, Serializer};

pub enum EventProperties {
    AllowBool(bool),
    AllowList(Vec<String>),
    PreventBool(bool),
    PreventList(Vec<String>),
}

impl Serialize for EventProperties {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut event_properties = serializer.serialize_struct("EventProperties", 1)?;
        match &self {
            EventProperties::AllowBool(ref val) => {
                event_properties.serialize_field("allow", val)?
            }
            EventProperties::AllowList(ref val) => {
                event_properties.serialize_field("allow", val)?
            }
            EventProperties::PreventBool(ref val) => {
                event_properties.serialize_field("prevent", val)?
            }
            EventProperties::PreventList(ref val) => {
                event_properties.serialize_field("prevent", val)?
            }
        };
        event_properties.end()
    }
}
