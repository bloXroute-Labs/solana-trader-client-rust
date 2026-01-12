use anyhow::Result;
use futures_util::StreamExt;
use solana_trader_client_rust::{
    common::constants::{MAINNET_PUMP_NY, WRAPPED_SOL},
    provider::ws::WebSocketClient,
};
use solana_trader_proto::api;
use test_case::test_case;

#[tokio::test]
#[ignore]
async fn test_pump_fun_new_amm_pool_streams() -> Result<()> {
    let ws = WebSocketClient::new(Some(MAINNET_PUMP_NY.to_string())).await?;
    let mut stream = ws.get_pump_fun_new_amm_pool_stream().await?;

    let response = stream
        .next()
        .await
        .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))??;

    println!("Response received: {:#?}", response);

    ws.close().await?;
    Ok(())
}

#[test_case(1 ; "single block")]
#[tokio::test]
#[ignore]
async fn test_block_stream_ws(expected_blocks: usize) -> Result<()> {
    let ws = WebSocketClient::new(None).await?;
    let mut stream = ws.get_block_stream().await?;

    for block_num in 1..=expected_blocks {
        let response = stream
            .next()
            .await
            .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))??;
        println!("Block {} received: {:#?}", block_num, response);
    }

    ws.close().await?;
    Ok(())
}

#[test_case(1 ; "single block hash")]
#[tokio::test]
#[ignore]
async fn test_recent_block_hash_stream_ws(expected_hashes: usize) -> Result<()> {
    let ws = WebSocketClient::new(None).await?;
    let mut stream = ws.get_recent_block_hash_stream().await?;

    for hash_num in 1..=expected_hashes {
        let response = stream
            .next()
            .await
            .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))??;

        println!("Block hash {} received: {:#?}", hash_num, response);
        assert!(
            !response.block_hash.is_empty(),
            "Block hash should not be empty"
        );
    }

    ws.close().await?;
    Ok(())
}

#[test_case(
    api::Project::PRaydium,
    None ;
    "Raydium priority fee stream"
)]
#[tokio::test]
#[ignore]
async fn test_priority_fee_stream_ws(project: api::Project, percentile: Option<f64>) -> Result<()> {
    let ws = WebSocketClient::new(None).await?;
    let mut stream = ws.get_priority_fee_stream(project, percentile).await?;

    let response = stream
        .next()
        .await
        .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))??;

    println!("Response received: {:#?}", response);

    ws.close().await?;
    Ok(())
}

#[test_case(
    vec![String::from("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"),
    String::from("CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK"),
    String::from("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C")];
    "Jupiter priority fee stream"
)]
#[tokio::test]
#[ignore]
async fn test_priority_fee_by_program_stream_grpc(programs: Vec<String>) -> Result<()> {
    let ws = WebSocketClient::new(None).await?;
    let mut stream = ws.get_priority_fee_by_program_stream(programs).await?;

    println!("starting priority fee by program stream");

    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => println!("Response received: {:#?}", response),
            Err(e) => println!("Stream error: {}", e),
        }
    }

    println!("Stream ended");
    Ok(())
}

#[test_case(1 ; "single bundle tip")]
#[tokio::test]
#[ignore]
async fn test_bundle_tip_stream_ws(expected_responses: usize) -> Result<()> {
    let ws = WebSocketClient::new(None).await?;
    let mut stream = ws.get_bundle_tip_stream().await?;

    for response_num in 1..=expected_responses {
        let response = stream
            .next()
            .await
            .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))??;

        println!("Bundle tip {} received: {:#?}", response_num, response);
    }

    ws.close().await?;
    Ok(())
}

#[test_case(1 ; "single new token")]
#[tokio::test]
#[ignore]
async fn test_pump_fun_new_tokens_stream_ws(expected_responses: usize) -> Result<()> {
    let ws = WebSocketClient::new(Some(MAINNET_PUMP_NY.to_string())).await?;
    let mut stream = ws.get_pump_fun_new_tokens_stream().await?;

    let mut last_mint = String::new();
    for response_num in 1..=expected_responses {
        let response = stream
            .next()
            .await
            .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))??;

        println!("New token {} received: {:#?}", response_num, response);
        last_mint = response.mint.clone();
        assert!(
            !response.mint.is_empty(),
            "Mint address should not be empty"
        );
    }

    println!("Last mint received: {}", last_mint);
    ws.close().await?;
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_pump_fun_tokens_and_swaps_integration_ws() -> Result<()> {
    let ws = WebSocketClient::new(Some(MAINNET_PUMP_NY.to_string())).await?;

    let mut tokens_stream = ws.get_pump_fun_new_tokens_stream().await?;
    let new_token = tokens_stream
        .next()
        .await
        .ok_or_else(|| anyhow::anyhow!("Tokens stream ended without data"))??;

    println!("New token received: {}", new_token.mint);

    let mut swaps_stream = ws.get_pump_fun_swaps_stream(vec![new_token.mint]).await?;
    let swap = swaps_stream
        .next()
        .await
        .ok_or_else(|| anyhow::anyhow!("Swaps stream ended without data"))??;

    println!("Swap received: {:#?}", swap);

    ws.close().await?;
    Ok(())
}

#[test_case(
    vec![String::from("Gj5t6KjTw3gWW7SrMHEi1ojCkaYHyvLwb17gktf96HNH")];
    "pump swap amm swaps"
)]
#[tokio::test]
#[ignore]
async fn test_pump_fun_amm_swap_stream_grpc(pools: Vec<String>) -> Result<()> {
    let ws = WebSocketClient::new(Some(MAINNET_PUMP_NY.to_string())).await?;
    let mut stream = ws.get_pump_fun_amm_swap_stream(pools).await?;
    
    println!("starting pump fun amm swap stream");
    
    let response = stream
        .next()
        .await
        .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))??;
    println!("Response received: {:#?}", response);

    ws.close().await?;
    Ok(())
}