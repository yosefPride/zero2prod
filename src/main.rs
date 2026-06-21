use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 1. Bind to desired production address and port
    let address = "127.0.0.1:8080";
    let listener = TcpListener::bind(address).expect("Failed to bind to port 8080");

    println!("Listening on http://{}", address);

    // 2. Pass the listener to architectural run function
    // and .await it so the main function doesn't exit immediately.
    zero2prod::run(listener)?.await
}
