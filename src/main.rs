use moneyflow::server::run_server;

#[tokio::main]
async fn main() {
    let server = run_server().unwrap();
    server.await.unwrap();
    
}