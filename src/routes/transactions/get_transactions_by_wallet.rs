use actix_web::{web, HttpResponse};
// use anyhow::Context;

use crate::{moralis_client::{MoralisClient}, domains::{EthereumAddress, Params, Chain}};

use super::TransactionFetchError;

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
pub struct TransactionData {
    pub cursor:Option<String>,
    pub page:Option<u32>,
    pub page_size:Option<u32>,
    pub total:Option<u32>,
    pub result:Vec<Option<TransactionInnerData>>
}

#[derive(serde::Deserialize,serde::Serialize)]
pub struct TransactionInnerData {
    pub hash: Option<String>,
    pub nonce: Option<String>,
    pub transaction_index: Option<String>,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
    pub value: Option<String>,
    pub gas: Option<String>,
    pub gas_price: Option<String>,
    pub input: Option<String>,
    pub receipt_cumulative_gas_used: Option<String>,
    pub receipt_gas_used: Option<String>,
    pub receipt_contract_address: Option<String>,
    pub receipt_root: Option<String>,
    pub receipt_status: Option<String>,
    pub block_timestamp: Option<String>,
    pub block_number: Option<String>,
    pub block_hash: Option<String>,
    pub transfer_index: Option<Vec<Option<u32>>>,
}


pub async fn get_transactions_by_wallet(
    parameters: web::Path<PathData>,
    moralis_client: web::Data<MoralisClient>,
) -> Result<HttpResponse, TransactionFetchError> {
    let parameters = Params::try_from(parameters.into_inner()).map_err(TransactionFetchError::ValidationError)?;
    let result = transactions_by_wallet(&moralis_client, &parameters.address, &parameters.chain).await?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn transactions_by_wallet(client:&MoralisClient, address: &EthereumAddress, chain:&Chain) -> Result<TransactionData, TransactionFetchError>{
    let result = client
    .get_request(&format!("{}", address),chain)
        .await?
        .json::<TransactionData>().await.map_err(|e| TransactionFetchError::UnexpectedError(anyhow::anyhow!(e.to_string())))?;
        Ok(result)
}
