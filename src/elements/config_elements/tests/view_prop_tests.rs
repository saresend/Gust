use super::super::view_properties::{GroupOptions, ViewProperties};
use crate::own;
use crate::test_serialize_fmt;

test_serialize_fmt!(
    test_sample,
    &ViewProperties {
        autosize: None,
        background: Some(own!("white")),
        group: Some(GroupOptions::Fill(own!("#dedede"))),
    },
    r#"
    {
        "background": "white",
        "group": {
            "fill": "\#dedede"
        }
    }
    "#
);
