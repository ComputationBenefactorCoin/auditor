use crate::client_benchmark;
use crate::client_benchmark::ClientBenchmark;
use crate::client_config;
use crate::client_config::ClientConfig;
use crate::client_requests;
use crate::client_statistics;
use crate::client_statistics::ClientStatistics;
use crate::config::Config;
use crate::de::De;
use crate::info::Info;
use std::thread;
use std::time::Duration;

pub async fn run(
    config: Config,
    de: De,
    info: Info,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client_benchmark: ClientBenchmark = client_benchmark::run();
    let client_config: ClientConfig = client_config::parse(config.etc_dir());
    let mut client_statistics: ClientStatistics;
    let sleep_duration: u64 = 10;
    loop {
        client_statistics = client_statistics::get();
        client_requests::post_statistics(
            &client_benchmark,
            &client_config,
            &client_statistics,
            &config,
            &de,
            &info,
        )
        .await?;
        thread::sleep(Duration::from_secs(sleep_duration));
    }
}

pub async fn run_load_simulator() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let sleep_duration: u64 = 10;
    loop {
        client_benchmark::run();
        thread::sleep(Duration::from_secs(sleep_duration));
    }
}
