use actix_web::{post, web, App, HttpServer, Responder};
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
    // TODO: エラー時にStatusCodeを付与する
    web::Json(result)
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
