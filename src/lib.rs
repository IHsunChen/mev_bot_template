pub mod helper;
pub mod block_scanner;
pub mod mempool;
pub mod address_book;
pub mod dex;

use std::sync::Arc;

use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use tokio::task::JoinHandle;

use crate::helper::setup_signer;

pub struct Config {
  #[allow(dead_code)]
  pub http: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
  #[allow(dead_code)]
  pub wss: Arc<Provider<Ws>>,
}

impl Config {
  pub async fn new() -> Self {
      let network = std::env::var("NETWORK_RPC").expect("missing NETWORK_RPC");
      let provider: Provider<Http> = Provider::<Http>::try_from(network).unwrap();
      let middleware = Arc::new(setup_signer(provider.clone()).await);

      let ws_network = std::env::var("NETWORK_WSS").expect("missing NETWORK_WSS");
      let ws_provider: Provider<Ws> = Provider::<Ws>::connect(ws_network).await.unwrap();
      Self {
          http: middleware,
          wss: Arc::new(ws_provider),
      }
  }

  // pub async fn create_dex(&self, factory: Address, router: Address) -> Dex {
  //     Dex::new(self.http.clone(), factory, router)
  // }
}

pub async fn run() {
  let config = Config::new().await;
  let handle = tokio::spawn(async move {
    block_scanner::loop_blocks(config.http.clone()).await;
  });
  mempool::loop_mempool(Arc::clone(&config.wss)).await;
}