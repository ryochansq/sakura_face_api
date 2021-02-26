use actix_web::{http::StatusCode, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv;

mod errors;
mod http_client;
mod id_to_member;
mod types;

use http_client::post_face;
use types::*;

#[post("/")]
async fn index(request_body: web::Json<DetectRequest>) -> impl Responder {
    let result = post_face(&request_body.url).await;
    match result {
        Ok(ok) => HttpResponse::Ok().json(ok),
        Err(err) => HttpResponse::build(StatusCode::from_u16(err.status_code).unwrap()).json(err),
    }
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    dotenv::dotenv().ok();
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}
