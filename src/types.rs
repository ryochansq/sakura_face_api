use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectAndFindSimilarsRequest {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectAndFindSimilarsResponse {
    pub face_list: Vec<DetectedFace>,
    pub similar_list: Vec<SimilarMember>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct FindSimilarsRequest {
    pub faceId: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FindSimilarsResponse {
    pub similar_list: Vec<SimilarMember>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FaceRectangle {
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct DetectedFace {
    pub faceId: String,
    pub faceRectangle: FaceRectangle,
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
pub struct SimilarFace {
    pub persistedFaceId: String,
    pub confidence: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimilarMember {
    pub name: String,
    pub confidence: f32,
}
