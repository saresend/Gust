use super::super::mark_properties::{MarkConfig, MarkProperties};
use crate::test_serialize_fmt;

test_serialize_fmt!(
    test_config_sample,
    &MarkConfig {
        area: None,
        rect: None,
        line: None,
        mark: Some(MarkProperties {
            fill: Some("#123456".to_string()),
            stroke: None,
            size: Some(10),
            font: None,
        })
    },
    r#"
    {
      "mark": {
        "fill": "\#123456",
        "size": 10 }
    }"#
);
