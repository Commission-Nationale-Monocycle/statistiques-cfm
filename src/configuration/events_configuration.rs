use crate::configuration::error::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct EventsConfiguration {
    categories: HashMap<String, EventsCategory>,
}

#[allow(dead_code)]
impl EventsConfiguration {
    pub fn new(categories: HashMap<String, EventsCategory>) -> Self {
        Self { categories }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct EventsCategory {
    name: String,
    events: HashMap<String, String>,
}

#[allow(dead_code)]
impl EventsCategory {
    pub fn new(name: String, events: HashMap<String, String>) -> Self {
        Self { name, events }
    }
}

#[allow(dead_code)]
pub fn load_configuration(path: &Path) -> Result<EventsConfiguration> {
    let settings = config::Config::builder()
        .add_source(config::File::from(path))
        .build()?;

    Ok(settings.try_deserialize::<EventsConfiguration>()?)
}

#[cfg(test)]
mod test {
    mod load_events_configuration {
        use crate::configuration::error::ConfigurationError;
        use crate::configuration::events_configuration::{load_configuration, EventsCategory, EventsConfiguration};
        use crate::test_data::get_test_asset;

        #[test]
        fn success() {
            let expected_result = EventsConfiguration::new([
                ("athletisme".to_string(), EventsCategory::new("Athlétisme".to_string(), [("100m".to_string(), "100m".to_string())].into_iter().collect())),
                ("artistique".to_string(), EventsCategory::new("Artistique".to_string(), [("individuel".to_string(), "Individuel".to_string()), ("paire".to_string(), "Paire".to_string())].into_iter().collect())),
            ].into_iter().collect());

            let file = get_test_asset("configuration/events.yml");
            let configuration = load_configuration(&file).unwrap();

            assert_eq!(expected_result, configuration);
        }

        #[test]
        fn fail_wrong_format() {
            let file = get_test_asset("configuration/events-wrong-format.yml");
            let error = load_configuration(&file).unwrap_err();

            assert!(matches!(error, ConfigurationError::Load(_)));
        }
    }
}
