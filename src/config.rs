use crate::info::Info;
use crate::mode::Mode;
use clap::ArgMatches;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Config {
    data_dir: String,
    etc_dir: String,
    host_id: String,
    mode: Mode,
}

impl Config {
    pub fn new(data_dir: String, etc_dir: String, host_id: String, mode: Mode) -> Self {
        Self {
            data_dir,
            etc_dir,
            host_id,
            mode,
        }
    }

    pub fn data_dir(&self) -> &String {
        &self.data_dir
    }

    pub fn etc_dir(&self) -> &String {
        &self.etc_dir
    }

    pub fn host_id(&self) -> &String {
        &self.host_id
    }

    pub fn mode(&self) -> &Mode {
        &self.mode
    }

    pub fn print(&self) {
        let mut info: String = String::new();
        info.push_str(format!("data_dir = {} \n", self.data_dir()).as_str());
        info.push_str(format!("etc_dir = {} \n", self.etc_dir()).as_str());
        info.push_str(format!("host_id = {} \n", self.host_id()).as_str());
        info.push_str(format!("mode = {:?} \n", self.mode()).as_str());
        println!("{}", info);
    }
}

pub fn parse(cli_parameters: &ArgMatches, info: &Info) -> Config {
    let mode: Mode;

    let default_data_dir: String = format!("/usr/share/{bn}/", bn = info.bin_name());
    let data_dir: String = cli_parameters
        .value_of("data_dir")
        .unwrap_or(&default_data_dir)
        .to_string();

    if !Path::new(data_dir.as_str()).exists() {
        panic!("Data directory doesn't exists.");
    }

    let default_etc_dir: String = format!("/etc/{bn}/", bn = info.bin_name());
    let etc_dir: String = cli_parameters
        .value_of("etc_dir")
        .unwrap_or(&default_etc_dir)
        .to_string();

    if !Path::new(etc_dir.as_str()).exists() {
        panic!("Etc directory doesn't exists.");
    }

    if 1 == cli_parameters.occurrences_of("server_mode") {
        mode = Mode::Server
    } else if 1 == cli_parameters.occurrences_of("client_load_simulator_mode") {
        mode = Mode::ClientLoadSimulator
    } else {
        mode = Mode::Client
    }

    let host_id_path: String = format!("{dir}{file}", dir = data_dir, file = "host_id.dat");
    let host_id: String = get_host_id(&host_id_path);

    Config::new(data_dir, etc_dir, host_id, mode)
}

fn get_host_id(host_id_path: &str) -> String {
    if !Path::new(host_id_path).exists() {
        let host_id: String = Uuid::new_v4().to_string();
        let mut host_id_file: File = File::create(&host_id_path).expect("Failed to create a file");
        host_id_file
            .write_all(host_id.as_bytes())
            .expect("Failed to write host id");
        host_id
    } else {
        let mut host_id_file: File = File::open(&host_id_path).expect("Failed to open a file");
        let mut host_id: String = String::new();
        host_id_file
            .read_to_string(&mut host_id)
            .expect("Failed to read host id");
        host_id
    }
}
