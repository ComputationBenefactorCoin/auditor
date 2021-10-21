use crate::common_request;
use crate::config::Config;
use crate::db::Db;
use crate::db_record::DbRecord;
use crate::db_statistics::DbStatistics;
use crate::de;
use crate::de::De;
use crate::info::Info;
use crate::request_post_statistics::RequestPostStatistics;
use crate::response_get_proof_of_computation::{
    ProofOfComputationData, ResponseGetProofOfComputation,
};
use crate::response_post_statistics::ResponsePostStatistics;
use hyper::{body::HttpBody, Body, Request, Response, StatusCode};
use log::warn;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use uuid::Uuid;

pub async fn handle_get_proof_of_computation(
    config: Config,
    database: Arc<Mutex<Db>>,
    de: De,
    info: Info,
    request: Request<Body>,
) -> Result<Response<Body>, hyper::Error> {
    let path: &str = request.uri().path();
    let host_id: &str = path.strip_prefix("/proof-of-computation/").unwrap();
    let mut database = database.lock().unwrap();
    let db_record: Option<&DbRecord> = database.get(host_id);

    if let Some(db_record) = db_record {
        let mut new_db_record: DbRecord = db_record.clone();
        let mut data: Vec<ProofOfComputationData> = vec![];
        let mut partial_proof_of_computation_array: Vec<f64> = vec![];
        let proof_of_computation: f64;

        for statistic in new_db_record.statistics() {
            if !statistic.used_for_proof() {
                let mut cpu_usage: f64 = *statistic.cpu_usage() as f64 / 100.0;

                if cpu_usage == 0.0 {
                    cpu_usage = 0.01;
                }

                let mut mem_usage: f64 = *statistic.mem_usage() as f64 / 100.0;

                if mem_usage == 0.0 {
                    mem_usage = 0.01;
                }

                let mut mt_2_result_multiplier: f64 = 1.0 - statistic.mt_2_result();

                if mt_2_result_multiplier <= 0.0 {
                    mt_2_result_multiplier = 0.01;
                }

                let mut mt_4_result_multiplier: f64 = 1.0 - statistic.mt_4_result();

                if mt_4_result_multiplier <= 0.0 {
                    mt_4_result_multiplier = 0.01;
                }

                let mut mt_8_result_multiplier: f64 = 1.0 - statistic.mt_8_result();

                if mt_8_result_multiplier <= 0.0 {
                    mt_8_result_multiplier = 0.01;
                }

                let mut st_result_multiplier: f64 = 1.0 - statistic.st_result();

                if st_result_multiplier <= 0.0 {
                    st_result_multiplier = 0.01;
                }

                let mut sys_load_average_fifteen_multiplier: f64 =
                    *statistic.sys_load_average_fifteen() as f64;

                if sys_load_average_fifteen_multiplier <= 0.0 {
                    sys_load_average_fifteen_multiplier = 0.01;
                }

                let mut sys_load_average_five_multiplier: f64 =
                    *statistic.sys_load_average_five() as f64;

                if sys_load_average_five_multiplier <= 0.0 {
                    sys_load_average_five_multiplier = 0.01;
                }

                let mut sys_load_average_one_multiplier: f64 =
                    *statistic.sys_load_average_one() as f64;

                if sys_load_average_one_multiplier <= 0.0 {
                    sys_load_average_one_multiplier = 0.01;
                }

                let partial_proof_of_computation: f64 = cpu_usage
                    * mem_usage
                    * mt_2_result_multiplier
                    * mt_4_result_multiplier
                    * mt_8_result_multiplier
                    * st_result_multiplier
                    * sys_load_average_fifteen_multiplier
                    * sys_load_average_five_multiplier
                    * sys_load_average_one_multiplier
                    * statistic.sys_uptime();
                let proof_of_computation_data: ProofOfComputationData = ProofOfComputationData::new(
                    statistic.id().to_string(),
                    partial_proof_of_computation,
                );

                data.push(proof_of_computation_data);
            }
        }

        for d in &data {
            partial_proof_of_computation_array.push(*d.partial_proof_of_computation());
        }

        proof_of_computation = partial_proof_of_computation_array.iter().sum::<f64>();

        let response_get_proof_of_computation: ResponseGetProofOfComputation =
            ResponseGetProofOfComputation::new(
                data,
                config.host_id().to_string(),
                proof_of_computation,
                de.public_key_string(),
            );
        let body: String = serde_json::to_string(&response_get_proof_of_computation).unwrap();
        let signature: String = de.sign(&body);
        let respose: Response<Body> = Response::builder()
            .status(StatusCode::CREATED)
            .header("content-type", "application/json")
            .header("signature", signature)
            .header("user-agent", format!("{}/{}", info.name(), info.version()))
            .body(Body::from(body))
            .expect("request builder");

        return Ok(respose);
    }

    let body: String = String::from("{}");
    let signature: String = de.sign(&body);
    let not_found: Response<Body> = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("content-type", "application/json")
        .header("signature", signature)
        .header("user-agent", format!("{}/{}", info.name(), info.version()))
        .body(Body::from(body))
        .expect("request builder");

    Ok(not_found)
}

pub async fn handle_post_statistics(
    config: Config,
    database: Arc<Mutex<Db>>,
    de: De,
    info: Info,
    request: Request<Body>,
) -> Result<Response<Body>, hyper::Error> {
    let signature: String = common_request::get_signature_from_request(&request);
    let body_vector: Vec<u8> = request.into_body().data().await.unwrap().unwrap().to_vec();
    let body: String = String::from_utf8(body_vector).unwrap();
    let request_post_statistics: RequestPostStatistics = serde_json::from_str(&body).unwrap();
    let signature_correct: bool =
        de::check_sign(&body, request_post_statistics.public_key(), &signature);

    if !signature_correct {
        let body: String = String::from("{}");
        let signature: String = de.sign(&body);
        let unauthenticated = Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("content-type", "application/json")
            .header("signature", signature)
            .header("user-agent", format!("{}/{}", info.name(), info.version()))
            .body(Body::from(body))
            .expect("request builder");

        warn!(
            "Incorrect signature for host = {}",
            request_post_statistics.host_id()
        );

        return Ok(unauthenticated);
    }

    let host_id: String = request_post_statistics.host_id().to_string();
    let mut database = database.lock().unwrap();
    let db_statistics: DbStatistics = DbStatistics::new(
        "".to_string(),
        *request_post_statistics.cpu_count(),
        *request_post_statistics.cpu_idle(),
        *request_post_statistics.cpu_interrupt(),
        *request_post_statistics.cpu_nice(),
        *request_post_statistics.cpu_system(),
        100.0 - *request_post_statistics.cpu_idle(),
        *request_post_statistics.cpu_user(),
        Uuid::new_v4().to_string(),
        *request_post_statistics.mem_free(),
        *request_post_statistics.mem_usage(),
        *request_post_statistics.mem_total(),
        *request_post_statistics.mt_2_result(),
        *request_post_statistics.mt_4_result(),
        *request_post_statistics.mt_8_result(),
        *request_post_statistics.st_result(),
        *request_post_statistics.sys_load_average_fifteen(),
        *request_post_statistics.sys_load_average_five(),
        *request_post_statistics.sys_load_average_one(),
        *request_post_statistics.sys_uptime(),
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        false,
    );
    let db_statistics_for_response: DbStatistics = db_statistics.clone();
    let db_record: Option<&DbRecord> = database.get(&host_id);

    match db_record {
        Some(db_record) => {
            let mut new_db_record: DbRecord = db_record.clone();

            if db_record.public_key() != request_post_statistics.public_key() {
                let body: String = String::from("{}");
                let signature: String = de.sign(&body);
                let unauthenticated = Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header("content-type", "application/json")
                    .header("signature", signature)
                    .header("user-agent", format!("{}/{}", info.name(), info.version()))
                    .body(Body::from(body))
                    .expect("request builder");

                warn!(
                    "Incorrect signature for host = {}",
                    request_post_statistics.host_id()
                );

                return Ok(unauthenticated);
            }

            new_db_record.statistics().push(db_statistics);
            database.remove(&host_id);
            database.insert(host_id, new_db_record);
            database.save();
        }
        None => {
            let statistics: Vec<DbStatistics> = vec![db_statistics];
            let new_db_record: DbRecord =
                DbRecord::new(request_post_statistics.public_key().to_string(), statistics);
            database.insert(host_id, new_db_record);
            database.save();
        }
    }

    let response_post_statistics: ResponsePostStatistics = ResponsePostStatistics::new(
        db_statistics_for_response,
        config.host_id().to_string(),
        de.public_key_string(),
    );
    let body: String = serde_json::to_string(&response_post_statistics).unwrap();
    let signature: String = de.sign(&body);
    let created: Response<Body> = Response::builder()
        .status(StatusCode::CREATED)
        .header("content-type", "application/json")
        .header("signature", signature)
        .header("user-agent", format!("{}/{}", info.name(), info.version()))
        .body(Body::from(body))
        .expect("request builder");

    Ok(created)
}
