#[derive(serde::Serialize, serde::Deserialize)]
pub enum Chain {
    Ethereum,
    Goerli,
    Sepolia,
    Polygon,
    Mumbai,
    Bsc,
    BscTestnet,
    Avalanche,
    AvalancheTestnet,
    Fantom,
    Palm,
    Cronos,
    CrosonsTestnet,
    Arbitrum,
}
impl Chain {
    pub fn as_str(&self) -> &str {
        match self {
            Chain::Ethereum => "eth",
            Chain::Goerli => "goerli",
            Chain::Sepolia => "sepolia",
            Chain::Polygon => "polygon",
            Chain::Mumbai => "mumbai",
            Chain::Bsc => "bsc",
            Chain::BscTestnet => "bsc%20testnet",
            Chain::Avalanche => "avalanche",
            Chain::AvalancheTestnet => "avalanche%20testnet",
            Chain::Fantom => "fantom",
            Chain::Palm => "palm",
            Chain::Cronos => "cronos",
            Chain::CrosonsTestnet => "crosons%20testnet",
            Chain::Arbitrum => "arbitrum",
        }
    }
}
impl TryFrom<String> for Chain {
    type Error = String;
    fn try_from(value:String)->Result<Self,Self::Error>{
        match value.as_str() {
            "eth" => Ok(Chain::Ethereum),
            "goerli" => Ok(Chain::Goerli),
            "sepolia" => Ok(Chain::Sepolia),
            "polygon" => Ok(Chain::Polygon),
            "mumbai" => Ok(Chain::Mumbai),
            "bsc" => Ok(Chain::Bsc),
            "bsctestnet" => Ok(Chain::BscTestnet),
            "avalanche" => Ok(Chain::Avalanche),
            "avalanchetestnet" => Ok(Chain::AvalancheTestnet),
            "fantom" => Ok(Chain::Fantom),
            "palm" => Ok(Chain::Palm),
            "cronos" => Ok(Chain::Cronos),
            "crosonstestnet" => Ok(Chain::CrosonsTestnet),
            "arbitrum" => Ok(Chain::Arbitrum),
            _ => Err(format!("Unknown chain: {}", value)),
        }
    }
}
