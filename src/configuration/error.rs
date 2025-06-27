use config::ConfigError;
use thiserror::Error;

pub type Result<T, E = ConfigurationError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum ConfigurationError {
    #[error(transparent)]
    Load(#[from] ConfigError),
}