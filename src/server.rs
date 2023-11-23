use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::{env, sync::RwLock};

#[get("/")]
async fn get_last_result(shared_data: web::Data<RwLock<Option<String>>>) -> impl Responder {
    let shared_response = shared_data.read().unwrap();

    if let Some(result) = shared_response.clone() {
        HttpResponse::Ok().body(result.clone())
    } else {
        HttpResponse::NotFound().body("No result available")
    }
}

const DEFAULT_HOST: &str = "0.0.0.0";
const DEFAULT_PORT: u32 = 8080;

pub async fn start_server(
    shared_data: web::Data<RwLock<Option<String>>>,
) -> Result<(), std::io::Error> {
    let host = env::var("HOST").unwrap_or_else(|_| {
        println!("HOST not set, using default {}", DEFAULT_HOST);
        DEFAULT_HOST.to_string()
    });

    let port = env::var("PORT")
        .unwrap_or_else(|_| {
            println!("PORT not set, using default {}", DEFAULT_PORT);
            DEFAULT_PORT.to_string()
        })
        .parse::<u32>()
        .expect("STANDBY_MILLIS must be a number");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::clone(&shared_data))
            .service(get_last_result)
    })
    .bind(format!("{}:{}", host, port))
    .unwrap()
    .run()
    .await
}
