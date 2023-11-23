use actix_web::web;
use reqwest::Client;
use std::env;
use std::sync::RwLock;
use std::time::Duration;
use tokio::time::sleep;

pub struct RestClient {
    client: Client,
    standby_millis: u64,
    server_url: String,
    shared_data: web::Data<RwLock<Option<String>>>,
}

const DEFAULT_STANDBY_MILLIS: u64 = 1000;

impl RestClient {
    pub fn new(shared_data: web::Data<RwLock<Option<String>>>) -> Self {
        let url = env::var("SERVER_URL").expect("SERVER_URL must be set");

        let standby_millis = env::var("STANDBY_MILLIS")
            .unwrap_or_else(|_| {
                println!(
                    "STANDBY_MILLIS not set, using default {}",
                    DEFAULT_STANDBY_MILLIS
                );
                DEFAULT_STANDBY_MILLIS.to_string()
            })
            .parse::<u64>()
            .expect("STANDBY_MILLIS must be a number");

        RestClient {
            client: Client::new(),
            standby_millis,
            server_url: url,
            shared_data,
        }
    }

    pub async fn make_request(&mut self) {
        let response = self.client.get(&self.server_url).send().await;

        if let Ok(response) = response {
            let result = response.text().await;

            if let Ok(result) = result {
                let mut shared = self.shared_data.write().unwrap();
                *shared = Some(result);
            }
        }
    }

    pub async fn start_periodic_requests(&mut self) {
        loop {
            self.make_request().await;
            sleep(Duration::from_millis(self.standby_millis)).await;
        }
    }
}
