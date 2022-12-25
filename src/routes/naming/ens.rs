use actix_web::{web, HttpResponse};
use anyhow::Context;

use crate::{moralis_client::{MoralisClient}, domains::{EthereumAddress, Chain}};

use super::EnsResolveError;

#[derive(serde::Deserialize)]
pub struct Parameters {
    address: String,
}

#[derive(serde::Deserialize,serde::Serialize)]
pub struct EnsData {
    pub name : Option<String>,
}

pub async fn get_ens_name(
    parameters: web::Path<Parameters>,
    moralis_client: web::Data<MoralisClient>,
) -> Result<HttpResponse, EnsResolveError> {
    let address = EthereumAddress::parse(parameters.address.clone()).map_err(EnsResolveError::ValidationError)?;
    let result = resolve_ens_by_address(&moralis_client, &address).await?;
    Ok(HttpResponse::Ok().json(result))}

pub async fn resolve_ens_by_address(client:&MoralisClient, address: &EthereumAddress) -> Result<EnsData, EnsResolveError>{
    let result = client
        .get_request(&format!("resolve/{}/reverse", address), &Chain::Ethereum)
        .await?
        .json::<EnsData>().await.context("Failed to Parse ENS")?;
        Ok(result)
}