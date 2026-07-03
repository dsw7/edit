use std::env;
use std::env::VarError::NotPresent;

pub fn load_api_key(env_var_key: &str) -> Result<String, String> {
    match env::var(env_var_key) {
        Ok(env_var_value) => Ok(env_var_value),
        Err(NotPresent) => Err(format!("Environment variable {env_var_key} not found")),
        Err(err) => Err(format!("Failed to retrieve API key: {:?}", err)),
    }
}
