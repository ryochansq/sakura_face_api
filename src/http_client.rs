use actix_web::{
    client::{Client, ClientRequest},
    http::header::CONTENT_TYPE,
    web::Bytes,
};
use serde_json::from_slice;
use std::env;

use crate::errors::*;
use crate::id_to_member::get_similar_member_list;
use crate::types::*;

fn get_key() -> String {
    match env::var("OCP_APIM_SUBSCRIPTION_KEY") {
        Ok(key) => key,
        Err(_) => panic!("KEY is not found"),
    }
}

fn get_endpoint() -> String {
    match env::var("ENDPOINT") {
        Ok(endpoint) => endpoint,
        Err(_) => panic!("ENDPOINT is not found"),
    }
}

fn make_request(method: &str) -> ClientRequest {
    let key = get_key();
    let endpoint = get_endpoint();
    let url = format!("{}/{}", endpoint, method);
    Client::new()
        .post(url)
        .header("Ocp-Apim-Subscription-Key", key)
}

async fn detect(image_binary: Bytes) -> Result<Vec<DetectedFace>, MyError> {
    let request = make_request("detect?recognitionModel=recognition_03");

    let mut response = request
        .header(CONTENT_TYPE, "application/octet-stream")
        .send_body(image_binary)
        .await?;
    let status = response.status();
    let response_body = response.body().await?;

    if status.is_success() {
        Ok(from_slice::<Vec<DetectedFace>>(&response_body)?)
    } else {
        let azure_error = serde_json::from_slice::<AzureError>(&response_body)?;
        Err(MyError {
            status_code: status.as_u16(),
            code: azure_error.error.code,
            message: azure_error.error.message,
        })
    }
}

async fn find_similar_member(face_id: &str) -> Result<Vec<SimilarMember>, MyError> {
    let request = make_request("findsimilars");
    let request_json = FindSimilarRequest {
        faceId: String::from(face_id),
        faceListId: String::from("sakura_gakuin_all"),
        maxNumOfCandidatesReturned: 3,
        mode: String::from("matchFace"),
    };

    let mut response = request
        .header(CONTENT_TYPE, "application/json")
        .send_json(&request_json)
        .await?;
    let status = response.status();
    let response_body = response.body().await?;

    if status.is_success() {
        let similar_face_list = from_slice::<Vec<SimilarFace>>(&response_body)?;
        Ok(get_similar_member_list(similar_face_list))
    } else {
        let azure_error = serde_json::from_slice::<AzureError>(&response_body)?;
        Err(MyError {
            status_code: status.as_u16(),
            code: azure_error.error.code,
            message: azure_error.error.message,
        })
    }
}

pub async fn detect_and_findsimilars(
    image_binary: Bytes,
) -> Result<DetectAndFindSimilarsResponse, MyError> {
    let face_list = detect(image_binary).await?;
    let similar_list = if face_list.len() == 1 {
        find_similar_member(&face_list[0].faceId).await?
    } else {
        vec![]
    };
    Ok(DetectAndFindSimilarsResponse {
        face_list,
        similar_list,
    })
}

pub async fn findsimilars(face_id: &str) -> Result<FindSimilarsResponse, MyError> {
    let similar_list = find_similar_member(face_id).await?;
    Ok(FindSimilarsResponse { similar_list })
}
