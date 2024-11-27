use solana_sdk::pubkey::Pubkey;
use reqwest::Error;
use serde_json::Value;
use crate::layout::{LiquidityStateV4, MarketStateV3};
use borsh::BorshDeserialize;
use solana_client::rpc_client::RpcClient;
use crate::PoolKeys;
use std::str::FromStr;
use anyhow::Result;

const OPEN_BOOK_PROGRAM: &str = "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX";

pub async fn get_pair_address(mint: &str) -> Result<Option<String>, Error> {
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

pub async fn fetch_pool_keys(rpc_client: &RpcClient, pair_address: &str) -> Result<Option<PoolKeys>> {
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