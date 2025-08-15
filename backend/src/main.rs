use std::net::SocketAddr;
use tokio::net::TcpListener;
use zero2prod_backend::run;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(&addr).await.unwrap();
    run(listener).await.unwrap();
}
