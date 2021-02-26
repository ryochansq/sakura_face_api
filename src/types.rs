use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectRequest {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct DetectedFace {
    pub faceId: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct FindSimilarRequest {
    pub faceId: String,
    pub faceListId: String,
    pub maxNumOfCandidatesReturned: i32,
    pub mode: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct FindSimilarResponse {
    pub persistedFaceId: String,
    pub confidence: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimilarMember {
    pub name: String,
    pub confidence: f32,
}
