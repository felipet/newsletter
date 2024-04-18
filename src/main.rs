use newsletter::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create a TcpListener to bind the address in which the service aims to listen for requests.
    let listener = TcpListener::bind("127.0.0.1:9090").expect("Failed to bind address");

    run(listener)?.await
}