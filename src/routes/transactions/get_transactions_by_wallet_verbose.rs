use actix_web::{web, HttpResponse};

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
pub struct TransactionVerboseData {
    pub cursor:Option<String>,
    pub page:Option<u32>,
    pub page_size:Option<u32>,
    pub total:Option<u32>,
    pub result:Vec<Option<TransactionVerboseInnerData>>
}

#[derive(serde::Deserialize,serde::Serialize)]
pub struct TransactionVerboseInnerData {
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
    pub logs: Option<Vec<Option<TokenTransactionLog>>>,
}

#[derive(serde::Deserialize,serde::Serialize)]
pub struct TokenTransactionLog {
    pub log_index: Option<String>,
    pub transaction_hash: Option<String>,
    pub transaction_index: Option<String>,
    pub address: Option<String>,
    pub data: Option<String>,
    pub topic0: Option<String>,
    pub topic1: Option<String>,
    pub topic2: Option<String>,
    pub topic3: Option<String>,
    pub block_timestamp: Option<String>,
    pub block_number: Option<String>,
    pub block_hash: Option<String>,
    pub transfer_index: Option<Vec<Option<u32>>>,
    pub transaction_value: Option<String>,
}


pub async fn get_verbose_transactions_by_wallet(
    parameters: web::Path<PathData>,
    moralis_client: web::Data<MoralisClient>,
) -> Result<HttpResponse, TransactionFetchError> {
    let parameters = Params::try_from(parameters.into_inner()).map_err(TransactionFetchError::ValidationError)?;
    let result = verbose_transactions_by_wallet(&moralis_client, &parameters.address, &parameters.chain).await?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn verbose_transactions_by_wallet(client:&MoralisClient, address: &EthereumAddress, chain:&Chain) -> Result<TransactionVerboseData, TransactionFetchError>{
    let result = client
    .get_request(&format!("{}/verbose", address),chain)
        .await?
        .json::<TransactionVerboseData>().await
        .map_err(|e| TransactionFetchError::UnexpectedError(anyhow::anyhow!(e.to_string())))?;
        Ok(result)
}