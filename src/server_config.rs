use serde_derive::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    address: String,
    port: u16,
}

impl ServerConfig {
    pub fn address(&self) -> &String {
        &self.address
    }

    pub fn port(&self) -> &u16 {
        &self.port
    }
}

pub fn parse(etc_dir: &str) -> ServerConfig {
    let config_file_name: String = format!(
        "{etc_dir}/{config_file}",
        etc_dir = etc_dir,
        config_file = "auditor_server.toml"
    );
    let config_file_contents: String =
        fs::read_to_string(&config_file_name).expect("Can not open configuration file.");
    let config_file_parameters: ServerConfig = toml::from_str(&config_file_contents).unwrap();

    config_file_parameters
}
