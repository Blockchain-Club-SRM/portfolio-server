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
pub struct CollectionData {
    pub cursor:Option<String>,
    pub page:Option<u32>,
    pub page_size:Option<u32>,
    pub total:Option<u32>,
    pub result:Vec<Option<CollectionInnerData>>
}

#[derive(serde::Deserialize,serde::Serialize)]
pub struct CollectionInnerData {
    pub token_address: Option<String>,
    pub contract_type:Option<String>,
    pub name :Option<String>,
    pub symbol :Option<String>,
}

pub async fn get_nft_collection_by_wallet(
    parameters: web::Path<PathData>,
    moralis_client: web::Data<MoralisClient>,
) -> Result<HttpResponse, NftFetchError> {
    let parameters = Params::try_from(parameters.into_inner()).map_err(NftFetchError::ValidationError)?;
    let result = nft_collection_by_wallet(&moralis_client, &parameters.address, &parameters.chain).await?;
    Ok(HttpResponse::Ok().json(result))}

pub async fn nft_collection_by_wallet(client:&MoralisClient, address: &EthereumAddress, chain:&Chain) -> Result<CollectionData, NftFetchError>{
    let result = client
        .get_request(&format!("{}/nft/collections", address),chain)
        .await?
        .json::<CollectionData>().await.context("Failed to Parse Nfts")?;
        Ok(result)
}