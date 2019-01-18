use super::super::event_properties::EventProperties;
use crate::test_serialize_fmt;

test_serialize_fmt!(&EventProperties::AllowBool(true), r#"{"allow":true}"#);
