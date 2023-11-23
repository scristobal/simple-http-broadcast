pub mod client;
pub mod server;

use actix_web::web;
use std::sync::RwLock;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let shared = web::Data::new(RwLock::new(None));

    tokio::spawn({
        let shared = web::Data::clone(&shared);
        async move {
            let mut client = client::RestClient::new(shared);
            client.start_periodic_requests().await;
        }
    });

    server::start_server(shared).await
}
