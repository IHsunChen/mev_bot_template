use std::sync::Arc;

use ethers::prelude::*;
use ethers::{
    core::k256::ecdsa::SigningKey, middleware::SignerMiddleware, signers::Wallet, types::Address, abi::AbiDecode
};

use crate::address_book::{UniV2Factory, UniV2Router, UniV2RouterCalls};

pub struct Dex {
    factory_address: Address,
    router_address: Address,
    factory: UniV2Factory<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    router: UniV2Router<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
}

impl Dex {
    pub fn new(
        middleware: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
        factory_address: Address,
        router_address: Address,
    ) -> Self {
        let factory = UniV2Factory::new(factory_address, Arc::clone(&middleware));
        let router = UniV2Router::new(router_address, Arc::clone(&middleware));
        Self {
            factory_address,
            router_address,
            factory,
            router,
        }
    }

    /// A quick way to decode tx hex data.
    pub async fn decode_router_tx_data(&self, tx_data: String) {
        let calldata: Bytes = tx_data.parse().unwrap();
        let decoded = UniV2RouterCalls::decode(&calldata).unwrap();
        println!("Decoded dex tx: {:?}", decoded);
    }

    pub async fn get_pairs(&self) {
      println!("Calling allPairLength from {}", self.factory_address);
      println!("{:?}", self.factory.all_pairs_length().await)
    }
}
