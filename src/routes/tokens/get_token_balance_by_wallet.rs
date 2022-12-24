use actix_web::{web, HttpResponse};
use anyhow::Context;

use crate::{moralis_client::{MoralisClient}, domains::EthereumAddress};

use super::TokenFetchError;

#[derive(serde::Deserialize)]
pub struct Parameters {
    address: String,
}

pub async fn get_token_balance_by_wallet(
    parameters: web::Path<Parameters>,
    moralis_client: web::Data<MoralisClient>,
) -> Result<HttpResponse, TokenFetchError> {
    let address = EthereumAddress::parse(parameters.address.clone()).map_err(TokenFetchError::ValidationError)?;
    let result = moralis_client
        .get_request(&format!("{}/erc20", address))
        .await?;
    let result = result
        .parse::<serde_json::Value>()
        .context("Failed to parse Token Data")?;
    Ok(HttpResponse::Ok().json(result))
}

