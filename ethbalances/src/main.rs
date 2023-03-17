// use ethers::abi::LogFilter;
// use ethers::prelude::*;
// use ethers::{
//     core::types::{Address, BlockNumber},
//     providers::{Http, Middleware, Provider},
// };

// use eyre::Result;
// use std::convert::TryFrom;
// use std::sync::Arc;

// const HTTP_URL: &str = "https://mainnet.infura.io/v3/b17715f3b04d4ccb90389a946de9c598";
// const ACCOUNT_ADDRESS: &str = "0xdAC17F958D2ee523a2206206994597C13D831ec7";

// #[tokio::main]
// async fn main() {
//     // current_block().await;
//     get_account_balances().await;
// }

// async fn current_block() -> eyre::Result<()> {
//     let provider = Provider::<Http>::try_from(HTTP_URL);
//     let block = provider?.get_block_number().await?;
//     println!("Current block: {block}");
//     Ok(())
// }

// async fn get_account_balances() -> eyre::Result<()> {
//     let provider = Provider::<Http>::try_from(HTTP_URL)?;

//     // let mut balances = Vec::new();

//     // let token_topics = vec![ethers::utils::keccak256(
//     //     "Transfer(address,address,uint256)".as_bytes().to_vec(),
//     // )];
//     let client = Arc::new(provider);
//     let filter: Filter = Filter::new()
//         .address(ACCOUNT_ADDRESS.parse::<Address>()?)
//         .event("Transfer(address,address,uint256)")
//         .from_block(0);
//     // .to_block(BlockNumber::Latest);

//     let logs = client.get_logs(&filter).await?;
//     println!("{} pools found!", logs.iter().len());

//     let token_addresses: Vec<Address> = logs
//         .iter()
//         .map(|log| Address::from_slice(&log.address.0))
//         // .filter(|address| address != &address) // Remove the given address from the list
//         .collect();

//     println!("{:?} pools found!", token_addresses.iter().len());

//     // for token_address in token_addresses {
//     //     let token_contract = Contract::new(
//     //         token_address,
//     //         include_bytes!("erc20_abi.json"),
//     //         provider.clone(),
//     //     );

//     Ok(())
// }

use core::num::dec2flt::number::Number;
use ethers::{
    core::types::{Address, Filter, H160, H256, U256},
    providers::{Http, Middleware, Provider},
    types::BlockNumber,
};
use eyre::Result;
use std::sync::Arc;
use tokio;

const HTTP_URL: &str = "https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27";
const V3FACTORY_ADDRESS: &str = "0xdAC17F958D2ee523a2206206994597C13D831ec7";
// const DAI_ADDRESS: &str = "0x6B175474E89094C44Da98b954EedeAC495271d0F";
// const USDC_ADDRESS: &str = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";
// const USDT_ADDRESS: &str = "0xdAC17F958D2ee523a2206206994597C13D831ec7";

/// This example demonstrates filtering and parsing event logs by fetching all Uniswap V3 pools
/// where both tokens are in the set [USDC, USDT, DAI].
///
/// V3 factory reference: https://github.com/Uniswap/v3-core/blob/main/contracts/interfaces/IUniswapV3Factory.sol
#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Http>::try_from(HTTP_URL)?;
    let client = Arc::new(provider);
    let token_topics = vec![
        H256::from("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".parse::<H160>()?),
        H256::from("0xCc33Db5fEc8cb1393adD7318Ca99cb916547E1B5".parse::<H160>()?),
    ];

    let filter = Filter::new()
        .address(V3FACTORY_ADDRESS.parse::<Address>()?)
        .event("Transfer(address,address,uint256)")
        // .topic1(token_topics.to_vec())
        // .topic2(token_topics.to_vec())
    let logs = client.get_logs(&filter).await?;
    println!("{} pools found!", logs.iter().len());
    // for log in logs.iter() {
    //     let token0 = Address::from(log.topics[1]);
    //     let token1 = Address::from(log.topics[2]);
    //     let fee_tier = U256::from_big_endian(&log.topics[3].as_bytes()[29..32]);
    //     let tick_spacing = U256::from_big_endian(&log.data[29..32]);
    //     let pool = Address::from(&log.data[44..64].try_into()?);
    //     println!(
    //         "pool = {}, token0 = {}, token1 = {}, fee = {}, spacing = {}",
    //         pool, token0, token1, fee_tier, tick_spacing,
    //     );
    // }
    Ok(())
}
