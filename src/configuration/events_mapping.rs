use std::collections::HashMap;
use std::path::Path;

use crate::configuration::error::Result;

pub type CategoriesMapping = HashMap<String, EventsMapping>;
pub type EventsMapping = HashMap<String, Vec<String>>;

#[allow(dead_code)]
pub fn load_mappings(path: &Path) -> Result<CategoriesMapping> {
    let settings = config::Config::builder()
        .add_source(config::File::from(path))
        .build()?;

    Ok(settings.try_deserialize::<CategoriesMapping>()?)
}

#[cfg(test)]
mod test {
    mod load_events_configuration {
        use crate::configuration::error::ConfigurationError;
        use crate::configuration::events_mapping::{load_mappings, CategoriesMapping};
        use crate::test_data::get_test_asset;

        #[test]
        fn test() {
            let expected_result: CategoriesMapping = [
                ("athletisme".to_string(), [("100m".to_string(), vec!["100m - All".to_string()]), ("stillstand".to_string(), vec![])].into_iter().collect()),
                ("tout-terrain".to_string(), [("cross-country".to_string(), vec!["Cross court - All".to_string(), "Cross long - All".to_string()])].into_iter().collect()),
            ].into_iter().collect();
            let file = get_test_asset("configuration/2025.yml");
            let configuration = load_mappings(&file).unwrap();

            assert_eq!(expected_result, configuration);
        }

        #[test]
        fn fail_wrong_format() {
            let file = get_test_asset("configuration/2025-wrong-format.yml");
            let error = load_mappings(&file).unwrap_err();

            assert!(matches!(error, ConfigurationError::Load(_)));
        }
    }
}