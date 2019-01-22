use super::config_elements::{
    event_properties::EventProperties,
    view_properties::ViewProperties,
    mark_properties::MarkProperties,
};

#[derive(Serialize)]
pub struct Config {
    events: EventProperties,
    view: ViewProperties,
    marks: MarkProperties,
}
