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

#[derive(serde::Deserialize,serde::Serialize)]
pub struct TokenInnerData {
    pub token_address: Option<String>,
    pub name :Option<String>,
    pub symbol :Option<String>,
    pub logo: Option<String>,
    pub decimals: Option<u32>,
    pub balance: Option<String>,
}

pub async fn get_token_balance_by_wallet(
    parameters: web::Path<PathData>,
    moralis_client: web::Data<MoralisClient>,
) -> Result<HttpResponse, TokenFetchError> {
    let parameters = Params::try_from(parameters.into_inner()).map_err(TokenFetchError::ValidationError)?;
    let result = token_balance_by_wallet(&moralis_client, &parameters.address, &parameters.chain).await?;
    Ok(HttpResponse::Ok().json(result))
}


pub async fn token_balance_by_wallet(client:&MoralisClient, address: &EthereumAddress, chain:&Chain) -> Result<Vec<Option<TokenInnerData>>, TokenFetchError>{
    let result = client
    .get_request(&format!("{}/erc20", address),chain)
    .await?
        .json::<Vec<Option<TokenInnerData>>>().await.context("Failed to Parse Tokens")?;
        Ok(result)
}