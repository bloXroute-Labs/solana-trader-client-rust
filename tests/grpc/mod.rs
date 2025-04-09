// Import modules
pub mod memo;
pub mod quote;
pub mod stream;
pub mod swap;

// Standard and external dependencies
use std::str::FromStr;
use anyhow::Result;
use base64::{engine::general_purpose, Engine};
use solana_hash::Hash;
use solana_sdk::{
    pubkey::Pubkey,
    signature::Signature,
    signer::Signer as _, system_instruction,
    transaction::Transaction,
    bs58::decode
};

// Project dependencies
use solana_trader_client_rust::{
    common::{
        constants::{
            SAMPLE_OWNER_ADDR,
            SAMPLE_TX_SIGNATURE,
            SAMPLE_TX_SIGNATURE_INVALID
        },
        signing::{create_signed_transaction, SubmitParams},
    },
    provider::grpc::GrpcClient,
};
use solana_trader_proto::api::{
    self,
    GetRecentBlockHashRequestV2,
    TransactionMessage, 
    TransactionMessageV2
};
use test_case::test_case;

/// Tests retrieving a transaction via gRPC
///
/// This test verifies that:
/// 1. A gRPC connection can be established successfully
/// 2. A transaction response is recieved
/// 3. The transaction response contains no errors
/// 4. The transaction response contains a valid slot
///
/// # Parameters
///
/// * `signature` - The transaction signature to query
///
/// # Returns
///
/// * `Result<()>` - Success if the response passes assertions
#[test_case(SAMPLE_TX_SIGNATURE)]
#[tokio::test]
#[ignore]
async fn test_get_transaction_grpc(signature: &str) -> Result<()> {
    // Initialize gRPC client
    let mut client = GrpcClient::new(None).await?;
    
    // Prepare request with the given transaction signature
    let request = api::GetTransactionRequest {
        signature: signature.to_string(),
    };
    
    // Execute the transaction query
    let response = client.get_transaction(&request).await?;
    
    // Log the response for debugging purposes
    println!(
        "Get Transaction Response: {}",
        serde_json::to_string_pretty(&response)?
    );
    
    // Verify success status
    assert!(response.status == "success", "Expected a slot in the tx response");
    // Verify errors to be empty
    let metadata = response.metadata.unwrap();
    assert!(metadata.err == "", "Expected err string to be empty");
    assert!(metadata.errored == false, "Expected errored to be false");
    // Verify the transaction contains a valid slot
    assert!(response.slot > 0, "Expected a slot in the tx response");
    
    Ok(())
}

/// Tests retrieving an invalid transaction via gRPC
///
/// This test verifies that:
/// 1. A gRPC connection can be established successfully
/// 2. A transaction response is recieved
/// 3. The transaction response contains expected error information
///
/// # Parameters
///
/// * `signature` - An invalid transaction signature
///
/// # Returns
///
/// * `Result<()>` - Success if the response passes assertions
#[test_case(SAMPLE_TX_SIGNATURE_INVALID)]
#[tokio::test]
#[ignore]
async fn test_get_transaction_invalid_grpc(signature: &str) -> Result<()> {
    // Initialize gRPC client
    let mut client = GrpcClient::new(None).await?;
    
    // Prepare request with the given transaction signature
    let request = api::GetTransactionRequest {
        signature: signature.to_string(),
    };
    
    // Execute the transaction query
    let response = client.get_transaction(&request).await?;
    
    // Log the response for debugging purposes
    println!(
        "Get Transaction Response: {}",
        serde_json::to_string_pretty(&response)?
    );

    // Verify error status
    assert!(response.status == "not_found", "Expected status to be not found");
    // Verify errors are present
    let metadata = response.metadata.unwrap();
    assert!(metadata.err == "tx not found", "Expected err string to be tx not found");
    assert!(metadata.errored == true, "Expected errored to be true");
    
    Ok(())
}

/// Tests retrieving a recent block hash
///
/// This test verifies that:
/// 1. A gRPC connection can be established successfully
/// 2. A blockhash response is recieved
/// 3. The blockhash is not empty and is a valid base58 string with 32 bytes
///
/// # Parameters
///
/// * None
///
/// # Returns
///
/// * `Result<()>` - Success if the response passes assertions
#[tokio::test]
#[ignore]
async fn test_get_recent_block_hash_grpc() -> Result<()> {
    // Initalize gRPC client
    let mut client = GrpcClient::new(None).await?;

    // Prepare request
    let request = api::GetRecentBlockHashRequest {};

    // Execute recent block hash query
    let response = client.get_recent_block_hash(&request).await?;

    // Log the response for debugging purposes
    println!(
        "Get Recent BlockHash Response: {}",
        serde_json::to_string_pretty(&response)?
    );

    // Assert the block hash is not empty
    assert_ne!(response.block_hash, "", "Expected a recent blockhash");
    // Decode the Base58 string into bytes
    let decoded: Vec<u8> = decode(&response.block_hash)
        .into_vec()
        .map_err(|_| panic!("Expected block_hash to be a valid Base58 string, but it is not"))
        .unwrap();
    // Assert that the decoded block_hash has exactly 32 bytes (SHA-256 length)
    assert_eq!(decoded.len(), 32, "Expected block_hash to be 32 bytes long");

    Ok(())
}

/// Tests retrieving a recent block hash (v2)
///
/// This test verifies that:
/// 1. A gRPC connection can be established successfully
/// 2. A blockhash response is received for different offsets
/// 3. The blockhash is not empty and is 32 bytes long (SHA-256 length)
///
/// # Parameters
///
/// * None
///
/// # Returns
///
/// * `Result<()>` - Success if the response passes assertions
#[tokio::test]
#[ignore]
async fn test_get_recent_block_hash_v2_grpc() -> Result<()> {
    // Initialize gRPC client
    let mut client = GrpcClient::new(None).await?;

    // Test different offset values (0 to 4)
    for offset in 0..5 {
        let request = api::GetRecentBlockHashRequestV2 { offset };

        // Execute recent block hash query with the offset
        let response = client.get_recent_block_hash_v2(request).await?;

        // Log the response for debugging purposes
        println!(
            "GetRecentBlockHashV2 Response for offset {}: {}",
            offset,
            serde_json::to_string_pretty(&response)?
        );

        // Assert the block hash is not empty
        assert_ne!(response.block_hash, "", "Expected a recent blockhash");
        
        // Decode the Base58 string into bytes
        let decoded: Vec<u8> = decode(&response.block_hash)
            .into_vec()
            .map_err(|_| panic!("Expected block_hash to be a valid Base58 string, but it is not"))
            .unwrap();
        
        // Assert that the decoded block_hash has exactly 32 bytes (SHA-256 length)
        assert_eq!(decoded.len(), 32, "Expected block_hash to be 32 bytes long");
    }

    Ok(())
}

/// Tests retrieving the rate limit information
///
/// This test verifies that:
/// 1. A gRPC connection can be established successfully
/// 2. A rate limit response is received
/// 3. The response contains a valid account tier (non-empty)
/// 4. The response contains a valid account ID (non-empty)
///
/// # Parameters
///
/// * None
///
/// # Returns
///
/// * `Result<()>` - Success if the response passes assertions
#[tokio::test]
#[ignore]
async fn test_get_rate_limit_grpc() -> Result<()> {
    // Initialize gRPC client
    let mut client = GrpcClient::new(None).await?;

    let request = api::GetRateLimitRequest {};

    // Execute the rate limit query
    let response = client.get_rate_limit(&request).await?;

    // Log the response for debugging purposes
    println!(
        "Get Rate Limit Response: {}",
        serde_json::to_string_pretty(&response)?
    );

    // Assert that the 'tier' is not empty (should contain a valid account tier)
    assert_ne!(response.tier, "", "Expected a valid account tier");
    assert_ne!(response.account_id, "", "Expected a valid account ID");

    Ok(())
}

/// Tests retrieving the account balance information (v2)
///
/// This test verifies that:
/// 1. A gRPC connection can be established successfully
/// 2. An account balance response is received
/// 3. The response contains at least one token account
///
/// # Parameters
///
/// * `owner_addr` - The address of the account owner to check balance for
///
/// # Returns
///
/// * `Result<()>` - Success if the response passes assertions
#[test_case(SAMPLE_OWNER_ADDR)]
#[tokio::test]
#[ignore]
async fn test_get_account_balance_v2_grpc(owner_addr: &str) -> Result<()> {
    // Initialize gRPC client
    let mut client = GrpcClient::new(None).await?;

    // Prepare the request with the owner address
    let request = api::GetAccountBalanceRequest {
        owner_address: owner_addr.to_string(),
    };

    // Execute the account balance query
    let response = client.get_account_balance_v2(&request).await?;

    // Log the response for debugging purposes
    println!(
        "GetAccountBalanceV2 Response: {}",
        serde_json::to_string_pretty(&response)?
    );

    // Assert that the response contains at least one token account
    assert!(
        !response.tokens.is_empty(),
        "Expected at least one token account"
    );

    Ok(())
}

/// Tests retrieving the priority fee information via gRPC
///
/// This test verifies that:
/// 1. A gRPC connection can be established successfully.
/// 2. A priority fee response is received.
/// 3. The response contains a valid `percentile` value within the range [0.0, 100.0].
/// 4. The response contains a valid `project` value (non-negative integer).
///
/// # Parameters
///
/// * `project` - The project for which to fetch the priority fee (e.g., Jupiter, Raydium).
/// * `percentile` - An optional percentile value to filter the fee at a specific percentile (default is `None`).
///
/// # Returns
///
/// * `Result<()>` - Success if the response passes assertions
#[test_case(api::Project::PJupiter, None; "Jupiter get priority fee - via grpc")]
#[test_case(api::Project::PRaydium, None; "Raydium get priority fee - via grpc")]
#[tokio::test]
#[ignore]
async fn test_get_priority_fee_grpc(project: api::Project, percentile: Option<f64>) -> Result<()> {
    // Initialize gRPC client
    let mut client = GrpcClient::new(None).await?;

    // Execute the priority fee query for the given project and optional percentile
    let response = client.get_priority_fee(project, percentile).await?;

    // Log the response for debugging purposes
    println!(
        "PriorityFeeResponse: {}",
        serde_json::to_string_pretty(&response)?
    );

    // Assert that the percentile is within the valid range [0.0, 100.0]
    assert!(response.percentile >= 0.0 && response.percentile <= 100.0, "Expected percentile to be a valid percentile");

    // Assert that the project is a non-negative integer (valid project ID)
    assert!(response.project >= 0, "Expected project to be a non-negative integer");

    Ok(())
}

/// Tests retrieving the priority fee information by program via gRPC
///
/// This test verifies that:
/// 1. A gRPC connection can be established successfully.
/// 2. A priority fee response is received.
/// 3. The response contains only those programs which were submited.
///
/// # Parameters
///
/// * `programs` - A list of program IDs for which to fetch the priority fee information.
///
/// # Returns
///
/// * `Result<()>` - Success if the response passes assertions
#[test_case(
    vec![
        "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK".to_string(),
        "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C".to_string()
    ]
)]
#[tokio::test]
#[ignore]
async fn test_get_priority_fee_by_program_grpc(programs: Vec<String>) -> Result<()> {
    // Initialize gRPC client
    let mut client = GrpcClient::new(None).await?;

    // Execute the priority fee query for the provided programs
    let response = client.get_priority_fee_by_program(programs.clone()).await?;

    // Log the response for debugging purposes
    println!(
        "priority fee by program: {}",
        serde_json::to_string_pretty(&response)?
    );

    // Assert that the response contains data for each program
    assert!(!response.data.is_empty(), "Expected non-empty data for the provided programs");

    // Loop over each entry in the response data
    for entry in &response.data {
        // Assert that each program ID is non-empty and matches one of the provided programs
        assert!(programs.contains(&entry.program), "Program ID in response does not match the expected program");
    }

    Ok(())
}

/// Tests retrieving token accounts information via gRPC
///
/// This test verifies that:
/// 1. A gRPC connection can be established successfully
/// 2. A token accounts response is received
/// 3. The response contains at least one token account with valid data
/// 4. The token symbol, token account, token mint address, and amount are valid
///
/// # Parameters
/// * `owner_address` - The address of the account owner to check token accounts for
///
/// # Returns
/// * `Result<()>` - Success if the response passes assertions
#[test_case(SAMPLE_OWNER_ADDR; "get token accounts - via grpc")]
#[tokio::test]
#[ignore]
async fn test_get_token_accounts(owner_address: &str) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

    let response = client.get_token_accounts(owner_address.to_string()).await?;
    println!(
        "TokenAccountResponse: {}",
        serde_json::to_string_pretty(&response)?
    );
    
    // Assert that the response contains at least one account
    assert!(!response.accounts.is_empty(), "Expected at least one token account in the response");

    // Loop through each token account in the response and assert validity
    for account in &response.accounts {
        // Assert that the symbol is not empty
        assert!(!account.symbol.is_empty(), "Expected a valid symbol for the token account");

        let decoded: Vec<u8> = decode(&account.token_mint)
            .into_vec()
            .map_err(|_| panic!("Expected token mint to be a valid Base58 string, but it is not"))
            .unwrap();
        
        // Assert that the decoded token mint has exactly 32 bytes (SHA-256 length)
        assert_eq!(decoded.len(), 32, "Expected token mint to be 32 bytes long");

        // Assert that the token account address is not empty
        assert!(!account.token_account.is_empty(), "Expected a valid token account address");

        // Assert that the amount is greater than or equal to zero
        assert!(account.amount >= 0.0, "Expected the token amount to be non-negative");
    }

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

#[tokio::test]
#[ignore]
async fn test_sign_and_submit_paladin() -> anyhow::Result<()> {
    // Create a new gRPC client
    let mut client = GrpcClient::new(None).await?;
    
    // Get a recent block hash
    let block_hash = client
        .get_recent_block_hash_v2(GetRecentBlockHashRequestV2 { offset: 0 })
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
    
    // Create a transfer instruction (similar to the Go example)
    let transfer_amount = 10_000_000;
    let recipient = Pubkey::from_str("HWEoBxYs7ssKuudEjzjmpfJVX7Dvi7wescFsVx2L5yoY")?;
    let transfer_ix = system_instruction::transfer(&pubkey, &recipient, transfer_amount);
    
    // Create a transaction with both instructions
    let transaction = Transaction::new_signed_with_payer(
        &[compute_budget_ix, transfer_ix],
        Some(&pubkey),
        &[&keypair],
        block_hash,
    );

    // Serialize the transaction 
    let serialized_tx = bincode::serialize(&transaction)?;
    let encoded_tx = general_purpose::STANDARD.encode(serialized_tx);
    
    // Create a transaction message
    let transaction_message = TransactionMessageV2 {
        content: encoded_tx,
    };
    
    // Call sign_and_submit_paladin with the transaction message
    let signature = client.sign_and_submit_paladin(transaction_message, true).await?;
    
    println!("Paladin Transaction Signature: {}", signature);
    
    // Add assertion to verify the signature is not empty
    assert!(!signature.is_empty(), "Expected a valid transaction signature");
    
    Ok(())
}
