use actix_web::client::{Client, ClientRequest};
use serde_json;
use std::env;

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
    .header("Content-Type", "application/json")
    .header("Ocp-Apim-Subscription-Key", key)
}

async fn detect(image_url: &str) -> Vec<DetectResponse> {
  let request = make_request("detect?recognitionModel=recognition_03");
  let request_json = DetectRequest {
    url: String::from(image_url),
  };
  // TODO: エラーハンドリングする
  let response = request
    .send_json(&request_json)
    .await
    .expect("DetectResponse Error")
    .body()
    .await
    .expect("Detect to body Error");
  serde_json::from_slice(&response).expect("Parse DetectResponse failed")
}

async fn find_similar(face_id: &str) -> Vec<FindSimilarResponse> {
  let request = make_request("findsimilars");
  let request_json = FindSimilarRequest {
    faceId: String::from(face_id),
    faceListId: String::from("sakura_gakuin_all"),
    maxNumOfCandidatesReturned: 3,
    mode: String::from("matchFace"),
  };
  // TODO: エラーハンドリングする
  let response = request
    .send_json(&request_json)
    .await
    .expect("FindSimilarResponse Error")
    .body()
    .await
    .expect("FindSimilar to body Error");
  serde_json::from_slice(&response).expect("Parse FindSimilarResponse failed")
}

pub async fn post_face(image_url: &str) -> Vec<SimilarMember> {
  let face_list = detect(image_url).await;
  let similar_face_id_list = find_similar(&face_list[0].faceId).await;
  get_similar_member_list(similar_face_id_list)
}
