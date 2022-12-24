use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;

use crate::{
    configuration::Settings,
    moralis_client::MoralisClient,
    routes::{
        get_nft_collection_by_wallet, get_nft_transfers_by_wallet, get_nfts_by_wallet,
        get_transactions_by_wallet, get_verbose_transactions_by_wallet, health_check,
    },
};
pub struct Application {
    port: u16,
    server: Server,
}
pub struct ApplicationBaseUrl(String);

pub fn run(
    listner: TcpListener,
    moralis_client: MoralisClient,
    base_url: String,
) -> Result<Server, std::io::Error> {
    let moralis_client = web::Data::new(moralis_client);
    let base_url = web::Data::new(ApplicationBaseUrl(base_url));
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .service(
                web::scope("/nft/{address}")
                    .route("", web::get().to(get_nfts_by_wallet))
                    .route("/collections", web::get().to(get_nft_collection_by_wallet))
                    .route("/transactions", web::get().to(get_nft_transfers_by_wallet)),
            )
            .service(
                web::scope("/transaction/{address}")
                    .route("", web::get().to(get_transactions_by_wallet))
                    .route(
                        "/verbose",
                        web::get().to(get_verbose_transactions_by_wallet),
                    ),
            )
            // .route("/nft", web::get().to(get_nfts_by_wallet))
            .app_data(moralis_client.clone())
            .app_data(base_url.clone())
    })
    .listen(listner)?
    .run();
    Ok(server)
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let timeout = configuration.moralis_client.timeout();
        let moralis_client = MoralisClient::new(
            configuration.moralis_client.url,
            configuration.moralis_client.key,
            configuration.moralis_client.chain,
            timeout,
        );
        let address = configuration.application.url();
        let listner = TcpListener::bind(address)?;
        let port = listner.local_addr().unwrap().port();
        let server = run(listner, moralis_client, configuration.application.base_url)?;
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
