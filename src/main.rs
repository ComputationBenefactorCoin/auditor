#[tokio::main]
async fn main() {
    let _result: Result<(), Box<dyn std::error::Error + Send + Sync>> = auditor::run().await;
}
