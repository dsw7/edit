mod config_file;
mod configs;
pub mod files;

pub use config_file::write_configs_to_file;
pub use configs::{Configs, setup_configurations};
