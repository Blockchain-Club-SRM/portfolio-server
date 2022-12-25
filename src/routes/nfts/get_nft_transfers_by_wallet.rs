use actix_web::{web, HttpResponse};
use anyhow::Context;

use crate::{moralis_client::{MoralisClient}, domains::{EthereumAddress, Params, Chain}};

use super::NftFetchError;

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
pub struct TransferData {
    pub cursor:Option<String>,
    pub page:Option<u32>,
    pub page_size:Option<u32>,
    pub total:Option<u32>,
    pub result:Vec<Option<TransferInnerData>>
}

#[derive(serde::Deserialize,serde::Serialize)]
pub struct TransferInnerData {
    // amount:Option<String>,
    // block_number:Option<String>,
    pub block_timestamp:Option<String>,
    // block_hash:Option<String>,
    pub transaction_hash:Option<String>,
    // transaction_index:u32,
    // log_index:u32,
    pub value:Option<String>,
    pub transaction_type:Option<String>,
    pub contract_type:Option<String>,
    pub token_address:Option<String>,
    pub token_id:Option<String>,
    pub from_address:Option<String>,
    pub to_address:Option<String>,
    // verified:u32,
}


pub async fn get_nft_transfers_by_wallet(
    parameters: web::Path<PathData>,
    moralis_client: web::Data<MoralisClient>,
) -> Result<HttpResponse, NftFetchError> {
    let parameters = Params::try_from(parameters.into_inner()).map_err(NftFetchError::ValidationError)?;
    let result = nft_transfers_by_wallet(&moralis_client, &parameters.address, &parameters.chain).await?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn nft_transfers_by_wallet(client:&MoralisClient, address: &EthereumAddress, chain: &Chain) -> Result<TransferData, NftFetchError>{
    let result = client
        .get_request(&format!("{}/nft/transfers", address), chain)
        .await?
        .json::<TransferData>().await.context("Failed to Parse Nfts")?;
        Ok(result)
}