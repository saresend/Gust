use super::super::event_properties::EventProperties;
use crate::own;
use crate::test_serialize_fmt;

test_serialize_fmt!(
    allow_all,
    &EventProperties::AllowBool(true),
    r#"{"allow":true}"#
);

test_serialize_fmt!(
    allow_list,
    &EventProperties::AllowList(vec![own!("One"), own!("Two"), own!("Three")]),
    r#"{"allow":["One", "Two", "Three"]}"#
);

test_serialize_fmt!(
    prevent_all,
    &EventProperties::PreventBool(true),
    r#"{"prevent":true}"#
);

test_serialize_fmt!(
    prevent_list,
    &EventProperties::PreventList(vec![]),
    r#"{"prevent": []}"#
);
