use actix_web::{web, HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;

use crate::{moralis_client::{MoralisClient}, utils::error_chain_fmt, domains::EthereumAddress};

#[derive(serde::Deserialize)]
pub struct Parameters {
    address: String,
}

pub async fn get_nfts_by_wallet(
    parameters: web::Query<Parameters>,
    moralis_client: web::Data<MoralisClient>,
) -> Result<HttpResponse, NftFetchError> {
    let address = EthereumAddress::parse(parameters.address.clone()).map_err(NftFetchError::ValidationError)?;
    let result = moralis_client
        .get_request(&format!("{}/nft", address))
        .await?;
    let result = result
        .parse::<serde_json::Value>()
        .map_err(|_| NftFetchError::UnexpectedError(anyhow::anyhow!("Failed to parse NFTs")))?;
    Ok(HttpResponse::Ok().json(result))
}


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