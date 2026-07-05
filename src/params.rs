use std::path::PathBuf;

pub struct Parameters {
    pub client_timeout: u64,
    pub input_file: PathBuf,
    pub model: String,
    pub prompt: String,
}
