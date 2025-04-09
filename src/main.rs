use moneyflow::server::run_server;

#[tokio::main]
async fn main() {
    let server = run_server().await.unwrap();
    server.await.unwrap();
}
