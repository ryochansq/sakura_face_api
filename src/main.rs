use actix_web::{http::StatusCode, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv;

mod errors;
mod http_client;
mod id_to_member;
mod types;

use http_client::{detect_and_findsimilars, findsimilars};
use types::*;

#[post("/detect_and_findsimilars")]
async fn detect_and_findsimilars_api(
    request_body: web::Json<DetectAndFindSimilarsRequest>,
) -> impl Responder {
    let result = detect_and_findsimilars(&request_body.url).await;
    match result {
        Ok(ok) => HttpResponse::Ok().json(ok),
        Err(err) => HttpResponse::build(StatusCode::from_u16(err.status_code).unwrap()).json(err),
    }
}

#[post("/findsimilars")]
async fn findsimilars_api(request_body: web::Json<FindSimilarsRequest>) -> impl Responder {
    let result = findsimilars(&request_body.faceId).await;
    match result {
        Ok(ok) => HttpResponse::Ok().json(ok),
        Err(err) => HttpResponse::build(StatusCode::from_u16(err.status_code).unwrap()).json(err),
    }
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    dotenv::dotenv().ok();
    HttpServer::new(move || {
        App::new()
            .service(detect_and_findsimilars_api)
            .service(findsimilars_api)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(())
}
