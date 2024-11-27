mod layout;
mod utils;

use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

const SOL_MINT: &str = "So11111111111111111111111111111111111111112";

#[derive(Debug)]
pub struct PoolKeys {
    pub amm_id: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_decimals: i64,
    pub quote_decimals: i64,
    pub open_orders: Pubkey,
    pub target_orders: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub withdraw_queue: Pubkey,
    pub market_id: Pubkey,
    pub market_authority: Pubkey,
    pub market_base_vault: Pubkey,
    pub market_quote_vault: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub event_queue: Pubkey,
}

#[tokio::main]
async fn main() -> Result<()> {
    let token_address = "879CUvWpjiNXwgx4dXsaLkR779JCkUgagB2Povqepump";  // Token to search for
    
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());

    match utils::get_pair_address(token_address).await {
        Ok(Some(pair_address)) => {
            println!("Pair address: {}", pair_address);
            match utils::fetch_pool_keys(&rpc_client, &pair_address).await {
                Ok(Some(pool_keys)) => {
                    println!("Pool keys fetched successfully: {:?}", pool_keys);
                    
                    // Get SOL pubkey for comparison
                    let sol_pubkey = Pubkey::from_str(SOL_MINT).unwrap();
                    
                    // Select the appropriate mint
                    let mint = if pool_keys.base_mint == sol_pubkey {
                        pool_keys.quote_mint
                    } else {
                        pool_keys.base_mint
                    };
                    
                    println!("Selected mint: {}", mint);
                },
                Ok(None) => println!("Could not fetch pool keys"),
                Err(e) => println!("Error fetching pool keys: {}", e),
            }
        },
        Ok(None) => println!("Pair address not found in the response."),
        Err(e) => println!("An error occurred: {}", e),
    }

    Ok(())
}