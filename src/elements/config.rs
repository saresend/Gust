use super::config_elements::event_properties::EventProperties;

#[derive(Serialize)]
pub struct Config {
    defaults: EventProperties,
}
