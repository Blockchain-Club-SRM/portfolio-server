use eth_address::address;
#[derive(Debug)]

pub struct EthereumAddress(String);


impl EthereumAddress {
    pub fn parse(s: String) -> Result<EthereumAddress, String> {
        if address::is_address(s.clone()) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid Ethereum address.", s))
        }
    }
}

impl AsRef<str> for EthereumAddress {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for EthereumAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}