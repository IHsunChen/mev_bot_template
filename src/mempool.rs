use std::sync::Arc;

use ethers::{
  abi::AbiDecode,
  providers::{Ws, Middleware, Provider, TransactionStream, StreamExt}, 
};

use crate::address_book::UniV2RouterCalls;

pub async fn loop_mempool(provider: Arc<Provider<Ws>>) {
  let tx_hash_stream = provider.subscribe_pending_txs().await.unwrap();
  let mut tx_stream = TransactionStream::new(&provider, tx_hash_stream, 256);
  println!("---------- MONITORING MEMPOOL ----------");
  while let Some(maybe_tx) = tx_stream.next().await {
    if let Ok(tx) = maybe_tx {
      if let Ok(decode_tx) = UniV2RouterCalls::decode(&tx.input) {
        println!("Transaction: {:#?}\nRouter Call: {:#?}\n", tx, decode_tx);
      }
    }
  }
}



