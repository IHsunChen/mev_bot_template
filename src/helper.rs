use ethers::prelude::{k256::ecdsa::SigningKey, *};

pub async fn setup_signer(
  provider: Provider<Http>,
) -> SignerMiddleware<Provider<Http>, Wallet<SigningKey>> {
  let chain_id = provider
      .get_chainid()
      .await
      .expect("Failed to get chain id.");
  println!("{}", chain_id);
  let priv_key = std::env::var("PRIVATE_KEY").expect("missing PRIVATE_KEY");
  println!("{}", priv_key);
  let wallet = priv_key
      .parse::<LocalWallet>()
      .expect("Failed to parse wallet")
      .with_chain_id(chain_id.as_u64());
  SignerMiddleware::new(provider, wallet)
}