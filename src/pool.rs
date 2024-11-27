use serde::{Deserialize, Serialize};
use reqwest;
use anyhow::Result;

#[derive(Debug, Deserialize)]
struct RaydiumPoolResponse {
    id: String,
    #[serde(rename = "mintProgramIdA")]
    mint_program_id_a: String,
    #[serde(rename = "mintProgramIdB")]
    mint_program_id_b: String,
    #[serde(rename = "mintA")]
    mint_a: String,
    #[serde(rename = "mintB")]
    mint_b: String,
    #[serde(rename = "vaultA")]
    vault_a: String,
    #[serde(rename = "vaultB")]
    vault_b: String,
    #[serde(rename = "mintDecimalsA")]
    mint_decimals_a: u8,
    #[serde(rename = "mintDecimalsB")]
    mint_decimals_b: u8,
    #[serde(rename = "lookupTableAccount")]
    lookup_table_account: String,
    #[serde(rename = "openTime")]
    open_time: u64,
    price: f64,
    // Add other fields as needed from the JSON response
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiquidityPool {
    pub id: String,
    pub mint_program_id_a: String,
    pub mint_program_id_b: String,
    pub mint_a: String,
    pub mint_b: String,
    pub vault_a: String,
    pub vault_b: String,
    pub mint_decimals_a: u8,
    pub mint_decimals_b: u8,
    pub lookup_table_account: String,
    pub open_time: u64,
    pub price: f64,
    // Add other fields as needed
}

impl LiquidityPool {
    fn new(
        id: String,
        mint_program_id_a: String,
        mint_program_id_b: String,
        mint_a: String,
        mint_b: String,
        vault_a: String,
        vault_b: String,
        mint_decimals_a: u8,
        mint_decimals_b: u8,
        lookup_table_account: String,
        open_time: u64,
        price: f64,
    ) -> Self {
        Self {
            id,
            mint_program_id_a,
            mint_program_id_b,
            mint_a,
            mint_b,
            vault_a,
            vault_b,
            mint_decimals_a,
            mint_decimals_b,
            lookup_table_account,
            open_time,
            price,
        }
    }
}

pub async fn fetch_pool_info(token_mint: &str) -> Result<LiquidityPool> {
    let url = "https://api.raydium.io/v2/ammV3/ammPools";
    
    let response = reqwest::get(url).await?;
    let text = response.text().await?;
    
    // Convert input token_mint to lowercase for comparison
    let token_mint_lower = token_mint.to_lowercase();
    
    let json_value: serde_json::Value = serde_json::from_str(&text)?;
    
    if let Some(pools) = json_value["data"].as_array() {
        for pool in pools {
            let mint_a = pool["mintA"].as_str().map(|s| s.to_lowercase());
            let mint_b = pool["mintB"].as_str().map(|s| s.to_lowercase());
            
            // Print first few pools to debug
            println!("Checking pool: mintA: {:?}, mintB: {:?}", mint_a, mint_b);
            
            if mint_a.as_deref() == Some(&token_mint_lower) || mint_b.as_deref() == Some(&token_mint_lower) {
                let response: RaydiumPoolResponse = serde_json::from_value(pool.clone())?;
                
                println!("Found pool!");
                println!("Pool ID: {}", response.id);
                println!("MintA: {}", response.mint_a);
                println!("MintB: {}", response.mint_b);
                
                return Ok(LiquidityPool::new(
                    response.id,
                    response.mint_program_id_a,
                    response.mint_program_id_b,
                    response.mint_a,
                    response.mint_b,
                    response.vault_a,
                    response.vault_b,
                    response.mint_decimals_a,
                    response.mint_decimals_b,
                    response.lookup_table_account,
                    response.open_time,
                    response.price,
                ));
            }
        }
    }
    
    // If we get here, let's print some sample pools to see what we're dealing with
    println!("Token not found. First few pools in response:");
    if let Some(pools) = json_value["data"].as_array() {
        for pool in pools.iter().take(3) {
            println!("Pool: mintA: {}, mintB: {}", 
                pool["mintA"].as_str().unwrap_or("none"),
                pool["mintB"].as_str().unwrap_or("none")
            );
        }
    }

    Err(anyhow::anyhow!(
        "No Raydium pool found for token mint: {}. Please verify the token address and ensure there is a liquidity pool for this token. Looking for lowercase: {}",
        token_mint,
        token_mint_lower
    ))
}