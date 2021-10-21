use crate::config::Config;
use crate::db::Db;
use crate::de::De;
use crate::info::Info;
use crate::server_config;
use crate::server_config::ServerConfig;
use crate::server_handle_requests;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use log::info;
use std::net::{Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::sync::{Arc, Mutex};

async fn handle(
    config: Config,
    database: Arc<Mutex<Db>>,
    de: De,
    info: Info,
    request: Request<Body>,
) -> Result<Response<Body>, hyper::Error> {
    info!("{} {}", request.method(), request.uri());

    match (request.method(), request.uri().path()) {
        (&Method::POST, "/statistics") => {
            server_handle_requests::handle_post_statistics(config, database, de, info, request)
                .await
        }
        _ => {
            if request.method() == Method::GET {
                server_handle_requests::handle_get_proof_of_computation(
                    config, database, de, info, request,
                )
                .await
            } else {
                let mut not_found = Response::default();
                *not_found.status_mut() = StatusCode::NOT_FOUND;
                Ok(not_found)
            }
        }
    }
}

pub async fn run(
    config: Config,
    de: De,
    info: Info,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let server_config: ServerConfig = server_config::parse(config.etc_dir());
    let mut db: Db = Db::new(config.data_dir().to_string());
    db.restore();
    let database: Arc<Mutex<Db>> = Arc::new(Mutex::new(db));
    let address: SocketAddr = (
        Ipv4Addr::from_str(server_config.address())?,
        *server_config.port(),
    )
        .into();
    let service = make_service_fn(|_| {
        let config: Config = config.clone();
        let database: Arc<Mutex<Db>> = database.clone();
        let de: De = de.clone();
        let info: Info = info.clone();

        async {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                handle(
                    config.to_owned(),
                    database.to_owned(),
                    de.to_owned(),
                    info.to_owned(),
                    req,
                )
            }))
        }
    });
    let server = Server::bind(&address).serve(service);

    info!("Listening on {}", address);

    server.await?;

    Ok(())
}
