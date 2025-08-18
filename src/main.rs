use crate::configuration::{events_configuration, events_mapping};
use crate::statistics::events_registrants_dependency::generate_csv_file;
use std::fs::write;
use std::path::{Path, PathBuf};

mod configuration;
mod error;
pub mod registration;
mod statistics;
#[cfg(test)]
pub mod test_data;

fn main() {
    for year in [2016, 2017, 2018, 2019, 2023, 2024, 2025] {
        compute_year_statistics(&year);
    }
}

fn compute_year_statistics(year: &i32) {
    let events_mapping =
        events_mapping::load_mappings(Path::new(&format!("configuration/{year}.yml"))).unwrap();
    let events_configuration =
        events_configuration::load_configuration(Path::new("configuration/events.yml")).unwrap();
    let csv_content = generate_csv_file(
        &PathBuf::from(format!(
            "{}/test/assets/{year}.xls",
            env!("CARGO_MANIFEST_DIR")
        )),
        &events_mapping,
        &events_configuration,
    );
    write(format!("result-{year}.csv"), &csv_content).unwrap();
}
