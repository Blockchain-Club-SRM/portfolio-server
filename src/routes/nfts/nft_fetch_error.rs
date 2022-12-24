use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;

use crate::utils::error_chain_fmt;

#[derive(thiserror::Error)]
pub enum NftFetchError {
    #[error("{0}")]
    ValidationError(String),
    #[error("Failed to fetch nfts from Moralis")]
    MoralisError(#[from] reqwest::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error)
}

impl std::fmt::Debug for NftFetchError {
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self,f)
    }
}


impl ResponseError for NftFetchError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::MoralisError(_) |
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(
            ErrorResponse {
                code: self.status_code().as_u16(),
                message: self.to_string(),
            }
        )
    }
}
#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
}