use ethers::prelude::*;

pub(crate) const SPOOKY_SWAP_ROUTER: &str = "0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45";
pub(crate) const SPOOKY_SWAP_FACTORY: &str = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f";

abigen!(UniV2Router, "src/abi/UniV2Router.json");
abigen!(UniV2Factory, "src/abi/UniV2Factory.json");
