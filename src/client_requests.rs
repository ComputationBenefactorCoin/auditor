use crate::client_benchmark::ClientBenchmark;
use crate::client_config::ClientConfig;
use crate::client_statistics::ClientStatistics;
use crate::common_request;
use crate::config::Config;
use crate::de;
use crate::de::De;
use crate::info::Info;
use crate::request_post_statistics::RequestPostStatistics;
use crate::response_post_statistics::ResponsePostStatistics;
use hyper::{Body, Client, Method, Request};
use log::warn;

pub async fn post_statistics(
    client_benchmark: &ClientBenchmark,
    client_config: &ClientConfig,
    client_statistics: &ClientStatistics,
    config: &Config,
    de: &De,
    info: &Info,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let request_post_statistics: RequestPostStatistics = RequestPostStatistics::new(
        *client_statistics.cpu_count(),
        *client_statistics.cpu_idle(),
        *client_statistics.cpu_interrupt(),
        *client_statistics.cpu_nice(),
        *client_statistics.cpu_system(),
        *client_statistics.cpu_user(),
        config.host_id().to_string(),
        *client_statistics.mem_free(),
        *client_statistics.mem_usage(),
        *client_statistics.mem_total(),
        *client_benchmark.mt_2_result(),
        *client_benchmark.mt_4_result(),
        *client_benchmark.mt_8_result(),
        de.public_key_string(),
        *client_benchmark.st_result(),
        *client_statistics.sys_load_average_fifteen(),
        *client_statistics.sys_load_average_five(),
        *client_statistics.sys_load_average_one(),
        *client_statistics.sys_uptime(),
    );
    let body: String = serde_json::to_string(&request_post_statistics).unwrap();
    let signature: String = de.sign(&body);
    let client = Client::new();
    let request = Request::builder()
        .method(Method::POST)
        .uri(format!("{}/statistics", client_config.endpoint()))
        .header("content-type", "application/json")
        .header("signature", signature)
        .header("user-agent", format!("{}/{}", info.name(), info.version()))
        .body(Body::from(body))
        .expect("request builder");
    let future = client.request(request).await;

    match future {
        Ok(future) => {
            if future.status() == 201 {
                let signature: String = common_request::get_signature_from_response(&future);
                let body_bytes = hyper::body::to_bytes(future.into_body()).await?;
                let body: String = String::from_utf8(body_bytes.to_vec()).unwrap();
                let response_post_statistics: ResponsePostStatistics =
                    serde_json::from_str(&body).unwrap();
                let signature_correct: bool =
                    de::check_sign(&body, response_post_statistics.public_key(), &signature);

                if !signature_correct {
                    warn!(
                        "Incorrect signature for host = {}",
                        response_post_statistics.host_id()
                    );
                }
            } else {
                warn!("Server error = {}", future.status());
            }
        }
        Err(_) => {
            warn!("Error while connecting to the server");
        }
    }

    Ok(())
}
