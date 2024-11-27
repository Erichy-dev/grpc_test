mod layout;

use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcProgramAccountsConfig, RpcAccountInfoConfig};
use solana_client::rpc_filter::{RpcFilterType, Memcmp, MemcmpEncodedBytes};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use reqwest::Error;
use serde_json::Value;
use layout::{LiquidityStateV4, MarketStateV3};
use borsh::BorshDeserialize;

const OPEN_BOOK_PROGRAM: &str = "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX";

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

    match get_pair_address(token_address).await {
        Ok(Some(pair_address)) => {
            println!("Pair address: {}", pair_address);
            match fetch_pool_keys(&rpc_client, &pair_address).await {
                Ok(Some(pool_keys)) => println!("Pool keys fetched successfully: {:?}", pool_keys),
                Ok(None) => println!("Could not fetch pool keys"),
                Err(e) => println!("Error fetching pool keys: {}", e),
            }
        },
        Ok(None) => println!("Pair address not found in the response."),
        Err(e) => println!("An error occurred: {}", e),
    }

    Ok(())
}

async fn get_pair_address(mint: &str) -> Result<Option<String>, Error> {
    let url = format!("https://api-v3.raydium.io/pools/info/mint?mint1={}&poolType=all&poolSortField=default&sortType=desc&pageSize=1&page=1", mint);
    let response = reqwest::get(&url).await?;
    
    if response.status().is_success() {
        let json: Value = response.json().await?;
        if let Some(pair_address) = json["data"]["data"][0]["id"].as_str() {
            return Ok(Some(pair_address.to_string()));
        }
    }
    
    Ok(None)
}

async fn fetch_pool_keys(rpc_client: &RpcClient, pair_address: &str) -> Result<Option<PoolKeys>> {
    let amm_id = Pubkey::from_str(pair_address)?;
    
    // Fetch AMM account data
    let amm_account = rpc_client.get_account(&amm_id)?;
    let amm_data = LiquidityStateV4::try_from_slice(&amm_account.data)?;
    
    // Get market ID and fetch market data
    let market_id = Pubkey::new_from_array(*amm_data.serum_market());
    let market_account = rpc_client.get_account(&market_id)?;
    let market_data = MarketStateV3::try_from_slice(&market_account.data)?;
    
    // Calculate market authority
    let nonce = market_data.vault_signer_nonce.to_le_bytes();
    let seeds = &[
        market_id.as_ref(),
        &nonce,
        &[7u8],
    ];
    let (market_authority, _bump) = Pubkey::find_program_address(
        seeds,
        &Pubkey::from_str(OPEN_BOOK_PROGRAM)?
    );

    Ok(Some(PoolKeys {
        amm_id,
        base_mint: Pubkey::new_from_array(market_data.base_mint),
        quote_mint: Pubkey::new_from_array(market_data.quote_mint),
        base_decimals: amm_data.coin_decimals(),
        quote_decimals: amm_data.pc_decimals(),
        open_orders: Pubkey::new_from_array(*amm_data.amm_open_orders()),
        target_orders: Pubkey::new_from_array(*amm_data.amm_target_orders()),
        base_vault: Pubkey::new_from_array(*amm_data.pool_coin_token_account()),
        quote_vault: Pubkey::new_from_array(*amm_data.pool_pc_token_account()),
        withdraw_queue: Pubkey::new_from_array(*amm_data.pool_withdraw_queue()),
        market_id,
        market_authority,
        market_base_vault: Pubkey::new_from_array(market_data.base_vault),
        market_quote_vault: Pubkey::new_from_array(market_data.quote_vault),
        bids: Pubkey::new_from_array(market_data.bids),
        asks: Pubkey::new_from_array(market_data.asks),
        event_queue: Pubkey::new_from_array(market_data.event_queue),
    }))
}