use crate::domains::Chain;
use crate::domains::EthereumAddress;

pub struct Params {
    pub chain: Chain,
    pub address: EthereumAddress,
}