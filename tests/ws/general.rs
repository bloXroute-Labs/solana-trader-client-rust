use anyhow::Result;
use std::time::Duration;
use tokio::time::timeout;

use solana_trader_client_rust::{
    common::{constants::SAMPLE_OWNER_ADDR, constants::SAMPLE_TX_SIGNATURE},
    provider::ws::WebSocketClient,
};
use solana_trader_proto::api;
use test_case::test_case;

#[test_case(SAMPLE_TX_SIGNATURE)]
#[tokio::test]
#[ignore]

async fn test_get_transaction_ws(signature: &str) -> Result<()> {
    let client = WebSocketClient::new(None).await?;

    let request = api::GetTransactionRequest {
        signature: signature.to_string(),
    };

    let response = client.get_transaction(request).await?;
    println!(
        "Get Transaction Response: {}",
        serde_json::to_string_pretty(&response)?
    );
    assert!(
        !response.status.is_empty(),
        "Expected a lot in the tx response"
    );

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_get_recent_block_hash() -> Result<()> {
    let client = WebSocketClient::new(None).await?;

    let request = api::GetRecentBlockHashRequest {};

    let response = client.get_recent_block_hash(request).await?;
    println!(
        "Get Transaction Response: {}",
        serde_json::to_string_pretty(&response)?
    );
    assert_ne!(response.block_hash, "", "Expected a valid recent blockhash");

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_get_recent_block_hash_v2_ws() -> Result<()> {
    let client = WebSocketClient::new(None).await?;

    // Test different offset values
    for offset in 0..5 {
        let request = api::GetRecentBlockHashRequestV2 { offset };

        let response = client.get_recent_block_hash_v2(&request).await?;
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
async fn test_get_rate_limit_ws() -> Result<()> {
    let client = WebSocketClient::new(None).await?;

    let request = api::GetRateLimitRequest {};

    let response = client.get_rate_limit(request).await?;
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
async fn test_get_account_balance_v2_ws(owner_addr: &str) -> Result<()> {
    let client = WebSocketClient::new(None).await?;

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

#[test_case(api::Project::PJupiter, None; "Jupiter get priority fee - via ws")]
#[test_case(api::Project::PRaydium, None; "Raydium get priority fee - via ws")]
#[tokio::test]
#[ignore]
async fn test_get_priority_fee_ws(project: api::Project, percentile: Option<f64>) -> Result<()> {
    let client = WebSocketClient::new(None).await?;

    let response = timeout(
        Duration::from_secs(10),
        client.get_priority_fee(project, percentile),
    )
    .await
    .map_err(|e| anyhow::anyhow!("Timeout: {}", e))??;

    println!("priority fee: {}", serde_json::to_string_pretty(&response)?);

    client.close().await?;
    Ok(())
}

#[test_case(vec!["CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK".to_string(), "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C".to_string()])]
#[tokio::test]
#[ignore]
async fn test_get_priority_fee_by_program_ws(programs: Vec<String>) -> Result<()> {
    let client = WebSocketClient::new(None).await?;

    let response = timeout(
        Duration::from_secs(10),
        client.get_priority_fee_by_program(programs),
    )
    .await
    .map_err(|e| anyhow::anyhow!("Timeout: {}", e))??;

    println!("priority fee by program: {}", serde_json::to_string_pretty(&response)?);

    client.close().await?;
    Ok(())
}

#[test_case(SAMPLE_OWNER_ADDR; "get token accounts - via ws")]
#[tokio::test]
#[ignore]
async fn test_get_token_accounts_ws(owner_address: &str) -> Result<()> {
    let client = WebSocketClient::new(None).await?;

    let response = timeout(
        Duration::from_secs(10),
        client.get_token_accounts(owner_address.to_string()),
    )
    .await
    .map_err(|e| anyhow::anyhow!("Timeout: {}", e))??;

    println!(
        "token accounts: {}",
        serde_json::to_string_pretty(&response)?
    );

    client.close().await?;
    Ok(())
}

#[test_case(SAMPLE_OWNER_ADDR; "get account balance - via ws")]
#[tokio::test]
#[ignore]
async fn test_get_account_balance_ws(owner_address: &str) -> Result<()> {
    let client = WebSocketClient::new(None).await?;

    let response = timeout(
        Duration::from_secs(10),
        client.get_account_balance(owner_address.to_string()),
    )
    .await
    .map_err(|e| anyhow::anyhow!("Timeout: {}", e))??;

    println!(
        "account balance: {}",
        serde_json::to_string_pretty(&response)?
    );

    client.close().await?;
    Ok(())
}
