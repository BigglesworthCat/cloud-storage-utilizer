use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    ParseCommand(#[from] clap::error::Error),

    #[error("Access token is absent")]
    AbsentAccessToken,

    #[error("Failed to prepare cloud client: {0}")]
    PrepareClient(String),

    #[error("Failed to prepare request parameters")]
    PrepareRequestParameters,

    #[error("Failed to prepare request")]
    PrepareRequest,

    #[error("Failed to send request: {0}")]
    SendRequest(String),

    #[error("Request error: {0}")]
    Request(String),

    #[error("Response error: {0}")]
    Response(String),

    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
}

pub static  PREPARE_AUTHORIZATION_HEADER_ERROR: &str = "unable to prepare authorization header value";
pub static BUILD_REQUEST_CLIENT_ERROR: &str = "unable to build request client";
pub static RESPONSE_BODY_ERROR: &str = "unable to get response content";
pub static BAD_REQUEST_ERROR: &str = "check your input paths";
pub static UNAUTHORIZED_ERROR: &str = "check your input paths";
pub static OTHER_ERROR: &str = "something went wrong";
