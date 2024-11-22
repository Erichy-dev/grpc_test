use anyhow::Result;
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::geyser::CommitmentLevel;
use yellowstone_grpc_proto::prelude::{ PingRequest, PongResponse,
    SubscribeRequest, SubscribeUpdate };
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    system_instruction,
};
use colored::*;

#[tokio::main]
async fn main() -> Result<()> {
    let endpoint = "http://168.119.198.25:10000";
    
    println!("{} {}", "🔌".bold(), "Connecting to endpoint:".bright_blue());
    println!("   {}", endpoint.bright_white());
    
    let mut client = GeyserGrpcClient::build_from_shared(endpoint)?
        .x_token::<String>(None)?
        .connect()
        .await?;

    // Get latest blockhash
    println!("\n{} {}", "🔍".bold(), "Fetching latest blockhash...".bright_yellow());
    let blockhash_response = client.get_latest_blockhash(Some(CommitmentLevel::Finalized)).await?;
    let blockhash = blockhash_response.blockhash;
    println!("   {}", blockhash.bright_white());

    // Create a simple transfer transaction
    println!("\n{} {}", "🔑".bold(), "Generating keypairs...".bright_green());
    let payer = Keypair::new();
    let recipient = Keypair::new();
    println!("   Payer: {}", payer.pubkey().to_string().bright_white());
    println!("   Recipient: {}", recipient.pubkey().to_string().bright_white());
    
    let transfer_instruction = system_instruction::transfer(
        &payer.pubkey(),
        &recipient.pubkey(),
        1000, // lamports
    );

    println!("\n{} {}", "📝".bold(), "Creating transaction...".bright_magenta());
    let transaction = Transaction::new_signed_with_payer(
        &[transfer_instruction],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash.parse()?
    );

    println!("\n{} {}", "✨".bold(), "Transaction Details:".bright_cyan());
    println!("   Signature: {}", transaction.signatures[0].to_string().bright_white());
    println!("   Recent Blockhash: {}", transaction.message.recent_blockhash.to_string().bright_white());
    println!("   Fee Payer: {}", transaction.message.account_keys[0].to_string().bright_white());
    
    Ok(())
}