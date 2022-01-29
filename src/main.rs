#[tokio::main]
async fn main() {
    let server = auditor::run();

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
