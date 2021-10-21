use crate::config::Config;
use crate::de::De;
use crate::info::Info;
use crate::mode::Mode;
use clap::{App, Arg, ArgMatches};
use log::SetLoggerError;
use std::process;

mod client;
mod client_benchmark;
mod client_config;
mod client_requests;
mod client_statistics;
mod common_log;
mod common_request;
mod config;
mod db;
mod db_record;
mod db_statistics;
mod de;
mod info;
mod mode;
mod request_post_statistics;
mod response_get_proof_of_computation;
mod response_post_statistics;
mod server;
mod server_config;
mod server_handle_requests;

pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _log: Result<_, SetLoggerError> = common_log::init();
    let info: Info = info::Info::new();
    let cli_parameters: ArgMatches = get_cli_parameters(&info);
    let config: Config = config::parse(&cli_parameters, &info);
    let de: De = de::get(&config);

    if 1 == cli_parameters.occurrences_of("print_configuration") {
        config.print();
        process::exit(0);
    }

    match config.mode() {
        Mode::Client => client::run(config, de, info).await?,
        Mode::ClientLoadSimulator => client::run_load_simulator().await?,
        Mode::Server => server::run(config, de, info).await?,
    }

    Ok(())
}

fn get_cli_parameters(info: &Info) -> ArgMatches {
    App::new(info.name())
        .version(info.version().as_str())
        .author("Michal Piotrowski <michal@eventhorizonlabs.eu>")
        .about(format!("{} module", info.name()).as_str())
        .arg(
            Arg::with_name("client_mode")
                .short("C")
                .long("client-mode")
                .help("Run in client mode"),
        )
        .arg(
            Arg::with_name("client_load_simulator_mode")
                .short("L")
                .long("client-load-simulator-mode")
                .help("Run in client load simulator mode"),
        )
        .arg(
            Arg::with_name("data_dir")
                .short("d")
                .long("data-dir")
                .value_name("DIR")
                .help("Sets a custom data dir")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("etc_dir")
                .short("e")
                .long("etc-dir")
                .value_name("DIR")
                .help("Sets a custom etc dir")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("print_configuration")
                .short("P")
                .long("print-configuration")
                .help("Prints configuration information"),
        )
        .arg(
            Arg::with_name("server_mode")
                .short("S")
                .long("server-mode")
                .help("Run in server mode"),
        )
        .get_matches()
}
