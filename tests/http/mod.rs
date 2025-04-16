// Local modules
pub mod memo;
pub mod quote;
pub mod swap;

// Standard library
use std::str::FromStr;

// External crates
use anyhow::Result;
use test_case::test_case;
use base64::{engine::general_purpose, Engine};
use solana_hash::Hash;
use solana_sdk::{
    transaction::Transaction,
    pubkey::Pubkey, system_instruction,
    instruction::Instruction,
    compute_budget::ComputeBudgetInstruction,
};
use solana_trader_proto::api::{self, PostSubmitPaladinRequest, GetRecentBlockHashRequestV2, TransactionMessage, TransactionMessageV2};
use solana_trader_client_rust::{
    common::{
        constants::{SAMPLE_OWNER_ADDR, SAMPLE_TX_SIGNATURE},
        signing::create_signed_transaction,
    },
    provider::{http::HTTPClient, utils::timestamp},
};

// Constants for tests
const BLXROUTE_MIN_TIP: u64 = 1_000_000;
const PALADIN_MIN_TIP: u64 = 10_000_001;
const BLOXROUTE_TIP_WALLET: &str = "HWEoBxYs7ssKuudEjzjmpfJVX7Dvi7wescFsVx2L5yoY";
const PALADIN_MIN_PRIORITY_FEE_MICROLAMPORTS: u64 = 40_000_001;
const PALADIN_MIN_COMPUTE_BUDGET_UNITS: u32 = 1_000_000;
const SEND_AMOUNT_LAMPORTS: u64 = 1;

// Helper function to create standard transfer instructions
fn create_transfer_instructions(from: &Pubkey, tip_amount: u64) -> anyhow::Result<Vec<Instruction>> {
    let tip_wallet = Pubkey::from_str(BLOXROUTE_TIP_WALLET)?;
    
    Ok(vec![
        system_instruction::transfer(from, &tip_wallet, tip_amount),
        system_instruction::transfer(from, from, SEND_AMOUNT_LAMPORTS),
    ])
}

// Helper function to create paladin instructions with compute budget
fn create_paladin_instructions(from: &Pubkey, tip_amount: u64) -> anyhow::Result<Vec<Instruction>> {
    let tip_wallet = Pubkey::from_str(BLOXROUTE_TIP_WALLET)?;
    
    Ok(vec![
        ComputeBudgetInstruction::set_compute_unit_limit(PALADIN_MIN_COMPUTE_BUDGET_UNITS),
        ComputeBudgetInstruction::set_compute_unit_price(PALADIN_MIN_PRIORITY_FEE_MICROLAMPORTS),
        system_instruction::transfer(from, &tip_wallet, tip_amount),
        system_instruction::transfer(from, from, SEND_AMOUNT_LAMPORTS),
    ])
}

// Helper function to prepare transaction message
fn prepare_transaction_message(tx: Transaction) -> anyhow::Result<TransactionMessage> {
    let serialized_tx = bincode::serialize(&tx)?;
    
    Ok(TransactionMessage {
        content: general_purpose::STANDARD.encode(serialized_tx),
        is_cleanup: false,
    })
}

// Helper function to prepare paladin transaction message
fn prepare_paladin_transaction_message(tx: Transaction) -> anyhow::Result<TransactionMessageV2> {
    let serialized_tx = bincode::serialize(&tx)?;
    
    Ok(TransactionMessageV2 {
        content: general_purpose::STANDARD.encode(serialized_tx),
    })
}

// Helper function to create paladin submit request
fn create_paladin_submit_request(transaction_message: TransactionMessageV2) -> PostSubmitPaladinRequest {
    PostSubmitPaladinRequest {
        transaction: Some(transaction_message),
        revert_protection: Some(false),
        timestamp: timestamp()
    }
}

#[tokio::test]
#[ignore]
async fn test_post_submit() -> anyhow::Result<()> {
    // Initialize client
    let client = HTTPClient::new(None)?;

    // Get recent block hash
    let block_hash = client
        .get_recent_block_hash()
        .await?
        .block_hash
        .parse::<Hash>()?;
    
    // Get client's public key and keypair
    let pubkey = client.public_key.ok_or_else(|| anyhow::anyhow!("Missing public key"))?;
    let keypair = client.get_keypair()?;
    
    // Create transaction instructions
    let instructions = create_transfer_instructions(&pubkey, BLXROUTE_MIN_TIP)?;
    
    // Create and sign transaction
    let tx = create_signed_transaction(
        instructions,
        &pubkey,
        keypair,
        block_hash,
    )?;
    
    // Prepare transaction message
    let transaction_message = prepare_transaction_message(tx)?;
    
    // Submit transaction and handle response
    let response = client.post_submit(
        transaction_message,                // transaction
        false,                              // skip preflight
        Some(false),                        // frp
        Some(BLXROUTE_MIN_TIP),             // tip
        Some(true),                         // allow back run
        Some(false),                        // staked
        Some(false),                        // fast best effort
        None,                               // revenue address
        Some(false),                        // sniping
    ).await?;
    
    // Log response for debugging
    println!("HTTP PostSubmit Response: {}", serde_json::to_string_pretty(&response)?);
    
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_post_submit_v2() -> anyhow::Result<()> {
    // Initialize client
    let client = HTTPClient::new(None)?;

    // Get recent block hash
    let block_hash = client
        .get_recent_block_hash()
        .await?
        .block_hash
        .parse::<Hash>()?;
    
    // Get client's public key and keypair
    let pubkey = client.public_key.ok_or_else(|| anyhow::anyhow!("Missing public key"))?;
    let keypair = client.get_keypair()?;
    
    // Create transaction instructions
    let instructions = create_transfer_instructions(&pubkey, BLXROUTE_MIN_TIP)?;
    
    // Create and sign transaction
    let tx = create_signed_transaction(
        instructions,
        &pubkey,
        keypair,
        block_hash,
    )?;
    
    // Prepare transaction message
    let transaction_message = prepare_transaction_message(tx)?;
    
    // Submit transaction and handle response
    let response = client.post_submit_v2(
        transaction_message,                // transaction
        false,                              // skip preflight
        Some(false),                        // frp
        Some(BLXROUTE_MIN_TIP),             // tip
        Some(true),                         // allow back run
        Some(false),                        // staked
        Some(false),                        // fast best effort
        None,                               // revenue address
        Some(false),                        // sniping
    ).await?;
    
    // Log response for debugging
    println!("HTTP PostSubmitV2 Response: {}", serde_json::to_string_pretty(&response)?);
    
    Ok(())
}

// ************** READ *****************
// Running this test will cost ~0.05 SOL
// ************** READ *****************
#[tokio::test]
#[ignore]
async fn test_post_submit_paladin_v2() -> anyhow::Result<()> {
    // Initialize client
    let client = HTTPClient::new(None)?;

    // Get recent block hash
    let block_hash = client
        .get_recent_block_hash()
        .await?
        .block_hash
        .parse::<Hash>()?;
    
    // Get client's public key and keypair
    let pubkey = client.public_key.ok_or_else(|| anyhow::anyhow!("Missing public key"))?;
    let keypair = client.get_keypair()?;
    
    // Create transaction instructions with compute budget settings
    let instructions = create_paladin_instructions(&pubkey, PALADIN_MIN_TIP)?;
    
    // Create and sign transaction
    let tx = create_signed_transaction(
        instructions,
        &pubkey,
        keypair,
        block_hash,
    )?;
    
    // Prepare paladin transaction message
    let transaction_message = prepare_paladin_transaction_message(tx)?;
    
    // Create paladin submit request
    let request = create_paladin_submit_request(transaction_message);
    
    // Submit transaction and handle response
    let response = client.post_submit_paladin_v2(&request).await?;
    
    // Log response for debugging
    println!("HTTP PostSubmitPaladinV2 Response: {}", serde_json::to_string_pretty(&response)?);
    
    Ok(())
}

#[test_case(SAMPLE_TX_SIGNATURE)]
#[tokio::test]
#[ignore]
async fn test_get_transaction_http(signature: &str) -> Result<()> {
    let client = HTTPClient::new(None)?;

    let request = api::GetTransactionRequest {
        signature: signature.to_string(),
    };

    let response = client.get_transaction(&request).await?;
    println!(
        "Get Transaction Response: {}",
        serde_json::to_string_pretty(&response)?
    );

    // let num: u64 = response.slot.parse().expect("Failed to parse string to u64");

    assert!(
        !response.status.is_empty(),
        "Expected a lot in the tx response"
    );

    Ok(())
}
#[tokio::test]
#[ignore]
async fn test_get_recent_block_hash_http() -> Result<()> {
    let client = HTTPClient::new(None)?;

    let response = client.get_recent_block_hash().await?;
    println!(
        "Get Recent Blockhash Response: {}",
        serde_json::to_string_pretty(&response)?
    );

    assert_ne!(response.block_hash, "", "Expected a valid recent blockhash");

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_get_recent_block_hash_v2_http() -> Result<()> {
    let client = HTTPClient::new(None)?;

    // Test different offset values
    for offset in 0..5 {
        let request = api::GetRecentBlockHashRequestV2 { offset };

        let response = client.get_recent_block_hash_v2(&request).await?;
        println!(
            "GetRecentBlockHashV2 Response for offset {}: {}",
            offset,
            serde_json::to_string_pretty(&response)?
        );

        // assert_ne!(response.offset, offset, "Expected a recent blockhash");
    }
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_get_rate_limit_http() -> Result<()> {
    let client = HTTPClient::new(None)?;

    let response = client.get_rate_limit().await?;
    println!(
        "Get Rate Limit Response: {}",
        serde_json::to_string_pretty(&response)?
    );

    assert_ne!(response.tier, "", "Expected a valid account tier");

    Ok(())
}

#[test_case(SAMPLE_OWNER_ADDR)]
#[tokio::test]
#[ignore]
async fn test_get_account_balance_v2_http(owner_addr: &str) -> Result<()> {
    let client = HTTPClient::new(None)?;

    let request = api::GetAccountBalanceRequest {
        owner_address: owner_addr.to_string(),
    };

    let response = client.get_account_balance_v2(request).await?;
    println!(
        "GetAccountBalanceV2 Response: {}",
        serde_json::to_string_pretty(&response)?
    );

    assert!(
        !response.tokens.is_empty(),
        "Expected at least one token account"
    );

    Ok(())
}

#[test_case(api::Project::PJupiter, None; "Jupiter get priority fee - via http")]
#[test_case(api::Project::PRaydium, None; "Raydium get priority fee - via http")]
#[tokio::test]
#[ignore]
async fn test_get_priority_fee_http(project: api::Project, percentile: Option<f64>) -> Result<()> {
    let client = HTTPClient::new(None)?;

    let response = client.get_priority_fee(project, percentile).await?;
    println!("priority fee: {}", serde_json::to_string_pretty(&response)?);

    Ok(())
}

#[test_case(vec!["CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK".to_string(), "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C".to_string()])]
#[tokio::test]
#[ignore]
async fn test_get_priority_fee_by_program_http(programs: Vec<String>) -> Result<()> {
    let client = HTTPClient::new(None)?;

    let response = client.get_priority_fee_by_program(programs).await?;
    println!(
        "priority fee by program: {}",
        serde_json::to_string_pretty(&response)?
    );

    Ok(())
}

#[test_case(SAMPLE_OWNER_ADDR; "get token accounts - via http")]
#[tokio::test]
#[ignore]
async fn test_get_token_accounts_http(owner_address: &str) -> Result<()> {
    let client = HTTPClient::new(None)?;

    let response = client.get_token_accounts(owner_address.to_string()).await?;
    println!(
        "token accounts: {}",
        serde_json::to_string_pretty(&response)?
    );

    Ok(())
}

#[test_case(SAMPLE_OWNER_ADDR; "get account balance - via http")]
#[tokio::test]
#[ignore]
async fn test_get_account_balance_http(owner_address: &str) -> Result<()> {
    let client = HTTPClient::new(None)?;

    let response = client
        .get_account_balance(owner_address.to_string())
        .await?;
    println!(
        "account balance: {}",
        serde_json::to_string_pretty(&response)?
    );

    Ok(())
}

#[test_case(100; "max slots")]
#[tokio::test]
#[ignore]
async fn test_get_leader_schedule_grpc(max_slots: u64) -> Result<()> {
    let client = HTTPClient::new(None)?;

    let response = client.get_leader_schedule(max_slots).await?;
    println!(
        "Get Leader Schedule Response: {}",
        serde_json::to_string_pretty(&response)?
    );

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_submit_snipe_http() -> Result<()> {
    let client = HTTPClient::new(None)?;
    let block_hash = client
        .get_recent_block_hash_v2(&GetRecentBlockHashRequestV2 { offset: 0 })
        .await?
        .block_hash
        .parse::<Hash>()?;

    let small_tip = 100_000;
    let staked_tip_threshold = 1_000_000;
    let pubkey = client.public_key.unwrap();
    let keypair = client.get_keypair()?;
    let tip_wallet = Pubkey::from_str("HWEoBxYs7ssKuudEjzjmpfJVX7Dvi7wescFsVx2L5yoY")?;
    let jito_tip_wallet = Pubkey::from_str("96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5")?;

    let mut transactions = Vec::with_capacity(2);

    // First transaction: transfer to both jito and bloxroute
    let tx1 = create_signed_transaction(
        vec![
            system_instruction::transfer(&pubkey, &jito_tip_wallet, small_tip),
            system_instruction::transfer(&pubkey, &tip_wallet, small_tip),
        ],
        &pubkey,
        keypair,
        block_hash,
    )?;
    let serialized_tx1 = bincode::serialize(&tx1)?;
    transactions.push(TransactionMessage {
        content: general_purpose::STANDARD.encode(serialized_tx1),
        is_cleanup: false,
    });

    // Second transaction: staked transfer to bloxroute
    let tx2 = create_signed_transaction(
        vec![system_instruction::transfer(
            &pubkey,
            &tip_wallet,
            staked_tip_threshold,
        )],
        &pubkey,
        keypair,
        block_hash,
    )?;
    let serialized_tx2 = bincode::serialize(&tx2)?;
    transactions.push(TransactionMessage {
        content: general_purpose::STANDARD.encode(serialized_tx2),
        is_cleanup: false,
    });

    let signatures = client.sign_and_submit_snipe(transactions, true).await?;
    println!(
        "Snipe Submit Response Signatures: {}",
        serde_json::to_string_pretty(&signatures)?
    );

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_sign_and_submit_paladin_http() -> anyhow::Result<()> {
    // Create a new HTTP client
    let client = HTTPClient::new(None)?;
    
    // Get a recent block hash
    let block_hash = client
        .get_recent_block_hash_v2(&GetRecentBlockHashRequestV2 { offset: 0 })
        .await?
        .block_hash
        .parse::<Hash>()?;

    // Get public key and keypair
    let pubkey = client.public_key.unwrap();
    let keypair = client.get_keypair()?;
    
    // Create compute budget instruction to set compute unit price
    let compute_unit_price = 200_000_000;
    let compute_budget_ix = solana_sdk::compute_budget::ComputeBudgetInstruction::set_compute_unit_price(
        compute_unit_price,
    );
    
    // Create a transfer instruction
    let transfer_amount = 10_000_000;
    let recipient = Pubkey::from_str("HWEoBxYs7ssKuudEjzjmpfJVX7Dvi7wescFsVx2L5yoY")?;
    let transfer_ix = system_instruction::transfer(&pubkey, &recipient, transfer_amount);
    
    // Create a transaction with both instructions
    let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[compute_budget_ix, transfer_ix],
        Some(&pubkey),
        &[&keypair],
        block_hash,
    );

    // Serialize the transaction 
    let serialized_tx = bincode::serialize(&transaction)?;
    let encoded_tx = general_purpose::STANDARD.encode(serialized_tx);
    
    // Create a transaction message using TransactionMessageV2
    let transaction_message = api::TransactionMessageV2 {
        content: encoded_tx,
    };
    
    // Call sign_and_submit_paladin with the transaction message
    let signature = client.sign_and_submit_paladin(transaction_message, true).await?;
    
    println!("Paladin HTTP Transaction Signature: {}", signature);
    
    // Add assertion to verify the signature is not empty
    assert!(!signature.is_empty(), "Expected a valid transaction signature");
    
    Ok(())
}