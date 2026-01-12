use anyhow::Result;
use solana_trader_client_rust::{
    common::constants::{MAINNET_PUMP_NY, WRAPPED_SOL},
    provider::grpc::GrpcClient,
};
use solana_trader_proto::api;
use test_case::test_case;
use tokio_stream::StreamExt;

#[test_case(1 ; "pump fun amm pool stream")]
#[tokio::test]
#[ignore]
async fn test_pump_new_amm_pool(expected_pool: usize) -> Result<()> {
    let mut client = GrpcClient::new(Some(MAINNET_PUMP_NY.to_string())).await?;
    let mut stream = client.get_pump_fun_new_amm_pool_stream().await?;

    println!("starting pump fun amm pool stream");

    for pool_num in 1..=expected_pool {
        let response = stream
            .next()
            .await
            .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))?
            .map_err(|e| anyhow::anyhow!("Stream error: {}", e))?;

        println!("New Pump Swap pool {} received: {:#?}", pool_num, response);
    }

    Ok(())
}

#[test_case(
    vec![String::from("Gj5t6KjTw3gWW7SrMHEi1ojCkaYHyvLwb17gktf96HNH")],
    1 ;
    "pump swap amm swaps"
)]
#[tokio::test]
#[ignore]
async fn test_pump_fun_amm_swap_stream_grpc(pools: Vec<String>, expected_pools: usize) -> Result<()> {
    let mut client = GrpcClient::new(Some(MAINNET_PUMP_NY.to_string())).await?;
    let mut stream = client.get_pump_fun_amm_swap_stream(pools).await?;
    
    println!("starting pump fun amm swap stream");
    
    for swap_num in 1..=expected_pools {
        let response = stream
            .next()
            .await
            .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))?
            .map_err(|e| anyhow::anyhow!("Stream error: {}", e))?;
            
        println!("New PumpSwap swap {} received: {:#?}", swap_num, response);
    }
    
    Ok(())
}

#[test_case(1 ; "single block")]
#[tokio::test]
#[ignore]
async fn test_block_stream_grpc(expected_blocks: usize) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;
    let mut stream = client.get_block_stream().await?;

    println!("starting block stream");

    for block_num in 1..=expected_blocks {
        let response = stream
            .next()
            .await
            .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))?
            .map_err(|e| anyhow::anyhow!("Stream error: {}", e))?;

        println!("Block {} received: {:#?}", block_num, response);
    }

    Ok(())
}

#[test_case(1 ; "single block hash")]
#[tokio::test]
#[ignore]
async fn test_recent_block_hash_stream_grpc(expected_hashes: usize) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;
    let mut stream = client.get_recent_block_hash_stream().await?;

    println!("starting recent block hash stream");

    for hash_num in 1..=expected_hashes {
        let response = stream
            .next()
            .await
            .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))?
            .map_err(|e| anyhow::anyhow!("Stream error: {}", e))?;

        println!("Block hash {} received: {:#?}", hash_num, response);

        // Optional: Add assertions based on the response
        assert!(
            !response.block_hash.is_empty(),
            "Block hash should not be empty"
        );
    }

    Ok(())
}

#[test_case(
    api::Project::PRaydium,
    None ;
    "Raydium priority fee stream"
)]
#[tokio::test]
#[ignore]
async fn test_priority_fee_stream_grpc(
    project: api::Project,
    percentile: Option<f64>,
) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;
    let mut stream = client.get_priority_fee_stream(project, percentile).await?;

    println!("starting priority fee stream");

    let response = stream
        .next()
        .await
        .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))?
        .map_err(|e| anyhow::anyhow!("Stream error: {}", e))?;

    println!("Response received: {:#?}", response);

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
    let mut client = GrpcClient::new(None).await?;
    let mut stream = client.get_priority_fee_by_program_stream(programs).await?;

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
async fn test_bundle_tip_stream_grpc(expected_responses: usize) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;
    let mut stream = client.get_bundle_tip_stream().await?;

    println!("starting bundle tip stream");

    for response_num in 1..=expected_responses {
        let response = stream
            .next()
            .await
            .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))?
            .map_err(|e| anyhow::anyhow!("Stream error: {}", e))?;

        println!("Bundle tip {} received: {:#?}", response_num, response);
    }

    Ok(())
}

#[test_case(1 ; "single new token")]
#[tokio::test]
#[ignore]
async fn test_pump_fun_new_tokens_stream_grpc(expected_responses: usize) -> Result<()> {
    let mut client = GrpcClient::new(Some(MAINNET_PUMP_NY.to_string())).await?;
    let mut stream = client.get_pump_fun_new_tokens_stream().await?;

    println!("starting pump fun new tokens stream");

    let mut last_mint = String::new();
    for response_num in 1..=expected_responses {
        let response = stream
            .next()
            .await
            .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))?
            .map_err(|e| anyhow::anyhow!("Stream error: {}", e))?;

        println!("New token {} received: {:#?}", response_num, response);

        // Save the mint for potential use/verification
        last_mint = response.mint.clone();

        // Basic validation
        assert!(
            !response.mint.is_empty(),
            "Mint address should not be empty"
        );
    }

    println!("Last mint received: {}", last_mint);
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_pump_fun_tokens_and_swaps_integration_grpc() -> Result<()> {
    let mut client = GrpcClient::new(Some(MAINNET_PUMP_NY.to_string())).await?;

    let mut tokens_stream = client.get_pump_fun_new_tokens_stream().await?;
    let new_token = tokens_stream
        .next()
        .await
        .ok_or_else(|| anyhow::anyhow!("Tokens stream ended without data"))?
        .map_err(|e| anyhow::anyhow!("Tokens stream error: {}", e))?;

    println!("New token received: {}", new_token.mint);

    let mut swaps_stream = client
        .get_pump_fun_swaps_stream(vec![new_token.mint])
        .await?;
    let swap = swaps_stream
        .next()
        .await
        .ok_or_else(|| anyhow::anyhow!("Swaps stream ended without data"))?
        .map_err(|e| anyhow::anyhow!("Swaps stream error: {}", e))?;

    println!("Swap received: {:#?}", swap);
    Ok(())
}
