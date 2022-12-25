use actix_web::{web, HttpResponse};
use anyhow::Context;

use crate::{moralis_client::{MoralisClient}, domains::{EthereumAddress, Params, Chain}};

use super::TokenFetchError;

#[derive(serde::Deserialize)]
pub struct PathData {
    chain: String,
    address: String,
}
impl TryFrom<PathData> for Params {
    type Error = String;

    fn try_from(value: PathData) -> Result<Self, Self::Error> {
        let address = EthereumAddress::parse(value.address)?;
        let chain = Chain::try_from(value.chain)?;
        Ok(Self { chain, address })
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
pub struct TokenTransactionData {
    pub cursor:Option<String>,
    pub page:Option<u32>,
    pub page_size:Option<u32>,
    pub total:Option<u32>,
    pub result:Vec<Option<TokenTransactionInnerData>>
}

#[derive(serde::Deserialize,serde::Serialize)]
pub struct TokenTransactionInnerData {
    pub transaction_hash:Option<String>,
    pub address:Option<String>,
    pub block_timestamp:Option<String>,
    pub block_number:Option<String>,
    pub block_hash:Option<String>,
    pub to_address:Option<String>,
    pub from_address:Option<String>,
    pub value:Option<String>,
    pub transaction_index: Option<u32>,
    pub logo_index: Option<u32>,
}
pub async fn get_token_transaction_by_wallet(
    parameters: web::Path<PathData>,
    moralis_client: web::Data<MoralisClient>,
) -> Result<HttpResponse, TokenFetchError> {
    let parameters = Params::try_from(parameters.into_inner()).map_err(TokenFetchError::ValidationError)?;
    let result = token_transaction_by_wallet(&moralis_client, &parameters.address, &parameters.chain).await?;
    Ok(HttpResponse::Ok().json(result))
}


pub async fn token_transaction_by_wallet(client:&MoralisClient, address: &EthereumAddress, chain:&Chain) -> Result<TokenTransactionData, TokenFetchError>{
    let result = client
    .get_request(&format!("{}/erc20/transfers", address),chain)
        .await?
        .json::<TokenTransactionData>().await.context("Failed to Parse Tokens")?;
        Ok(result)
}