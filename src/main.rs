use rust_email_service::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8000";

    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}
