use actix_web::{web, HttpResponse};

use crate::{moralis_client::{MoralisClient}, domains::EthereumAddress};

use super::NftFetchError;

#[derive(serde::Deserialize)]
pub struct Parameters {
    address: String,
}

pub async fn get_nft_collection_by_wallet(
    parameters: web::Path<Parameters>,
    moralis_client: web::Data<MoralisClient>,
) -> Result<HttpResponse, NftFetchError> {
    let address = EthereumAddress::parse(parameters.address.clone()).map_err(NftFetchError::ValidationError)?;
    let result = moralis_client
        .get_request(&format!("{}/nft/collections", address))
        .await?;
    let result = result
        .parse::<serde_json::Value>()
        .map_err(|_| NftFetchError::UnexpectedError(anyhow::anyhow!("Failed to parse NFTs")))?;
    Ok(HttpResponse::Ok().json(result))
}

