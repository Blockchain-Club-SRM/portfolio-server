use actix_web::{web, HttpResponse};
use anyhow::Context;

use crate::{moralis_client::{MoralisClient}, domains::{EthereumAddress, Params, Chain}};

use super::BalanceFetchError;

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
pub struct BalanceData {
    pub balance:Option<String>,
}


pub async fn get_native_balance_by_wallet(
    parameters: web::Path<PathData>,
    moralis_client: web::Data<MoralisClient>,
) -> Result<HttpResponse, BalanceFetchError> {
    let parameters = Params::try_from(parameters.into_inner()).map_err(BalanceFetchError::ValidationError)?;
    let result = native_balance_by_wallet(&moralis_client, &parameters.address, &parameters.chain).await?;
    Ok(HttpResponse::Ok().json(result))}

pub async fn native_balance_by_wallet(client:&MoralisClient, address: &EthereumAddress, chain:&Chain) -> Result<BalanceData, BalanceFetchError>{
    let result = client
        .get_request(&format!("{}/balance", address),chain)
        .await?
        .json::<BalanceData>().await.context("Failed to Parse Balance")?;
        Ok(result)
}