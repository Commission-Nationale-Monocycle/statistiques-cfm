use std::fs::write;
use std::path::PathBuf;
use crate::statistics::events_registrants_dependency::generate_csv_file;

pub mod registration;
mod error;
mod statistics;
mod configuration;
#[cfg(test)]
pub mod test_data;

fn main() {
    let csv_content = generate_csv_file(&PathBuf::from(format!(
        "{}/test/assets/{}",
        env!("CARGO_MANIFEST_DIR"),
        "2024.xls"
    )));
    write("result.csv", &csv_content).unwrap();
}
