pub mod memo;
pub mod quote;
pub mod stream;
pub mod swap;

use std::str::FromStr;

use anyhow::Result;
use base64::{engine::general_purpose, Engine};
use solana_hash::Hash;
use solana_sdk::{
    pubkey::Pubkey, signature::Signature, signer::Signer as _, system_instruction,
    transaction::Transaction,
};
use solana_trader_client_rust::{
    common::{
        constants::{SAMPLE_OWNER_ADDR, SAMPLE_TX_SIGNATURE},
        signing::{create_signed_transaction, SubmitParams},
    },
    provider::grpc::GrpcClient,
};
use solana_trader_proto::api::{self, GetRecentBlockHashRequestV2, TransactionMessage};
use test_case::test_case;

#[test_case(SAMPLE_TX_SIGNATURE)]
#[tokio::test]
#[ignore]
async fn test_get_transaction_grpc(signature: &str) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

    let request = api::GetTransactionRequest {
        signature: signature.to_string(),
    };

    let response = client.get_transaction(&request).await?;
    println!(
        "Get Transaction Response: {}",
        serde_json::to_string_pretty(&response)?
    );
    assert!(response.slot > 0, "Expected a lot in the tx response");

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_get_recent_block_hash_grpc() -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

    let request = api::GetRecentBlockHashRequest {};

    let response = client.get_recent_block_hash(&request).await?;
    println!(
        "Get Recent BlockHash Response: {}",
        serde_json::to_string_pretty(&response)?
    );

    assert_ne!(response.block_hash, "", "Expected a recent blockhash");

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_get_recent_block_hash_v2_grpc() -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

    // Test different offset values
    for offset in 0..5 {
        let request = api::GetRecentBlockHashRequestV2 { offset };

        let response = client.get_recent_block_hash_v2(request).await?;
        println!(
            "GetRecentBlockHashV2 Response for offset {}: {}",
            offset,
            serde_json::to_string_pretty(&response)?
        );

        assert_ne!(response.block_hash, "", "Expected a recent blockhash");
    }
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_get_rate_limit_grpc() -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

    let request = api::GetRateLimitRequest {};

    let response = client.get_rate_limit(&request).await?;
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
async fn test_get_account_balance_v2_grpc(owner_addr: &str) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

    let request = api::GetAccountBalanceRequest {
        owner_address: owner_addr.to_string(),
    };

    let response = client.get_account_balance_v2(&request).await?;
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

#[test_case(api::Project::PJupiter, None; "Jupiter get priority fee - via grpc")]
#[test_case(api::Project::PRaydium, None; "Raydium get priority fee - via grpc")]
#[tokio::test]
#[ignore]
async fn test_get_priority_fee_grpc(project: api::Project, percentile: Option<f64>) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

    let response = client.get_priority_fee(project, percentile).await?;
    println!("priority fee: {}", serde_json::to_string_pretty(&response)?);

    Ok(())
}

#[test_case(vec!["CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK".to_string(), "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C".to_string()])]
#[tokio::test]
#[ignore]
async fn test_get_priority_fee_by_program_grpc(programs: Vec<String>) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

    let response = client.get_priority_fee_by_program(programs).await?;
    println!(
        "priority fee by program: {}",
        serde_json::to_string_pretty(&response)?
    );

    Ok(())
}

#[test_case(SAMPLE_OWNER_ADDR; "get token accounts - via grpc")]
#[tokio::test]
#[ignore]
async fn test_get_token_accounts(owner_address: &str) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

    let response = client.get_token_accounts(owner_address.to_string()).await?;
    println!(
        "token accounts: {}",
        serde_json::to_string_pretty(&response)?
    );

    Ok(())
}

#[test_case(SAMPLE_OWNER_ADDR; "get account balance - via grpc")]
#[tokio::test]
#[ignore]
async fn test_get_account_balance_grpc(owner_address: &str) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

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
    let mut client = GrpcClient::new(None).await?;

    let response = client.get_leader_schedule(max_slots).await?;
    println!(
        "Get Leader Schedule Response: {}",
        serde_json::to_string_pretty(&response)?
    );

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_submit_snipe() -> anyhow::Result<()> {
    let mut client = GrpcClient::new(None).await?;
    let block_hash = client
        .get_recent_block_hash_v2(GetRecentBlockHashRequestV2 { offset: 0 })
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
    println!("Snipe Signatures: {signatures:?}");

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_paladin_race() -> anyhow::Result<()> {
    let mut handles = vec![];

    for _ in 0..100 {
        let handle = tokio::spawn(async move {
            // Create new client for each task
            let mut client = GrpcClient::new(None).await?;

            // Similar to test_add_memo_to_tx but with Paladin params
            let block_hash = client
                .get_recent_block_hash_v2(GetRecentBlockHashRequestV2 { offset: 0 })
                .await?
                .block_hash
                .parse::<Hash>()?;

            let lamports_to_transfer = 1_000_000;
            let pubkey = client.public_key.unwrap();
            let keypair = client.get_keypair()?;

            let transfer_instruction =
                system_instruction::transfer(&pubkey, &pubkey, lamports_to_transfer);

            let mut transaction = Transaction::new_signed_with_payer(
                &[transfer_instruction],
                Some(&pubkey),
                &[&keypair],
                block_hash,
            );

            let message_data = transaction.message.serialize();
            transaction.signatures = vec![Signature::default()];
            transaction.signatures[0] = keypair.sign_message(&message_data);

            let serialized_tx = bincode::serialize(&transaction)?;
            let messages = vec![TransactionMessage {
                content: general_purpose::STANDARD.encode(serialized_tx),
                is_cleanup: false,
            }];

            let submit_opts = SubmitParams {
                allow_revert: Some(true), // Trigger Paladin path
                ..Default::default()
            };

            client.sign_and_submit(messages, submit_opts, false).await
        });
        handles.push(handle);
    }

    // Wait for all transactions
    for handle in handles {
        handle.await??;
    }

    Ok(())
}
