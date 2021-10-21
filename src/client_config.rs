use serde_derive::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct ClientConfig {
    endpoint: String,
}

impl ClientConfig {
    pub fn endpoint(&self) -> &String {
        &self.endpoint
    }
}

pub fn parse(etc_dir: &str) -> ClientConfig {
    let config_file_name: String = format!(
        "{etc_dir}/{config_file}",
        etc_dir = etc_dir,
        config_file = "auditor_client.toml"
    );
    let config_file_contents: String =
        fs::read_to_string(&config_file_name).expect("Can not open configuration file.");
    let config_file_parameters: ClientConfig = toml::from_str(&config_file_contents).unwrap();

    config_file_parameters
}
