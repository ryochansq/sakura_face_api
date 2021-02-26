use actix_web::client::{PayloadError, SendRequestError};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct AzureErrorBody {
    pub code: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AzureError {
    pub error: AzureErrorBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyError {
    pub status_code: u16,
    pub code: Option<String>,
    pub message: Option<String>,
}

impl From<SendRequestError> for MyError {
    fn from(_: SendRequestError) -> Self {
        Self {
            status_code: 500,
            code: Some(String::from("ServerError")),
            message: Some(String::from("SendRequestError")),
        }
    }
}

impl From<PayloadError> for MyError {
    fn from(_: PayloadError) -> Self {
        Self {
            status_code: 500,
            code: Some(String::from("ServerError")),
            message: Some(String::from("PayloadError")),
        }
    }
}

impl From<serde_json::Error> for MyError {
    fn from(_: serde_json::Error) -> Self {
        Self {
            status_code: 500,
            code: Some(String::from("ParseError")),
            message: Some(String::from("serde_json parse Error")),
        }
    }
}
