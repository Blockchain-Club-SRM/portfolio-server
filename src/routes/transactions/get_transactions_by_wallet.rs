use actix_web::{web, HttpResponse};
use anyhow::Context;

use crate::{moralis_client::{MoralisClient}, domains::EthereumAddress};

use super::TransactionFetchError;

#[derive(serde::Deserialize)]
pub struct Parameters {
    address: String,
}

pub async fn get_transactions_by_wallet(
    parameters: web::Path<Parameters>,
    moralis_client: web::Data<MoralisClient>,
) -> Result<HttpResponse, TransactionFetchError> {
    let address = EthereumAddress::parse(parameters.address.clone()).map_err(TransactionFetchError::ValidationError)?;
    let result = moralis_client
        .get_request(&format!("{}", address))
        .await?;
    let result = result
        .parse::<serde_json::Value>()
        .context("Failed to parse Transaction Data")?;
        // .map_err(|_| TransactionFetchError::UnexpectedError(anyhow::anyhow!("Failed to parse NFTs")))?;
    Ok(HttpResponse::Ok().json(result))
}

