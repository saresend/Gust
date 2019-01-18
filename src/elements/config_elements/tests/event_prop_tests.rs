use super::super::event_properties::EventProperties;

#[test]
fn test_list_allow_all_serialize() {
    let all_allow = EventProperties::AllowBool(true);
    assert_eq!(
        serde_json::to_string(&all_allow).unwrap(),
        r#"{"allow":true}"#
    );
}
