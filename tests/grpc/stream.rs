use anyhow::Result;
use solana_trader_client_rust::{
    common::constants::{MAINNET_PUMP_NY, WRAPPED_SOL},
    provider::grpc::GrpcClient,
};
use solana_trader_proto::api;
use test_case::test_case;
use tokio_stream::StreamExt;

#[test_case(
    vec![api::Project::PRaydium],
    vec![WRAPPED_SOL.to_string()] ;
    "raydium SOL price stream"
)]
#[tokio::test]
#[ignore]
async fn test_price_stream_grpc(projects: Vec<api::Project>, tokens: Vec<String>) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;
    let mut stream = client.get_prices_stream(projects, tokens).await?;

    println!("starting price stream");

    let response = stream
        .next()
        .await
        .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))?
        .map_err(|e| anyhow::anyhow!("Stream error: {}", e))?;

    println!("Response received: {:#?}", response);

    let price = response
        .price
        .ok_or_else(|| anyhow::anyhow!("Missing price"))?;

    assert!(price.buy > 0.0, "Price should be positive");
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

// TODO:
// rust SDK error
// Error: GetOrderbooksStream error: status: Cancelled, message: "Timeout expired", details: [], metadata: MetadataMap { headers: {} }
// go SDK error:
// ERRO[2024-11-07 13:15:35] subscription error: rpc error: code = Unavailable desc = error reading from server: EOF
#[test_case(
    vec!["SOL/USDC".to_string(), "SOL-USDT".to_string()], 
    false ;
    "valid markets"
)]
#[tokio::test]
#[ignore]
async fn test_orderbook_stream_grpc(markets: Vec<String>, expect_error: bool) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;
    println!("starting orderbook stream");

    let mut stream = client
        .get_orderbook_stream(markets, 3, api::Project::POpenbook)
        .await?;

    match stream.next().await {
        Some(Ok(response)) => {
            println!("Response received: {:#?}", response);
            Ok(())
        }
        Some(Err(e)) => {
            if expect_error {
                println!("Expected error received: {}", e);
                Ok(())
            } else {
                Err(anyhow::anyhow!("Stream error: {}", e))
            }
        }
        None => Err(anyhow::anyhow!("Stream ended without data")),
    }
}

// TODO: error (go sdk):
// ERRO[2024-11-07 16:19:11] subscription error: rpc error: code = Unavailable desc = error reading from server: EOF
#[test_case(
    vec!["SOL/USDC".to_string(), "xxx".to_string()], 
    vec!["SOL/USDC".to_string(), "SOL-USDT".to_string()] ;
    "valid markets"
)]
#[tokio::test]
#[ignore]
async fn test_market_depth_stream_grpc(
    invalid_market: Vec<String>,
    valid_market: Vec<String>,
) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;
    println!("starting market depth stream");

    let mut stream = client
        .get_market_depths_stream(invalid_market, 3, api::Project::POpenbook)
        .await?;

    match stream.next().await {
        Some(Err(e)) => println!("Expected subscription error received: {}", e),
        Some(Ok(_)) => return Err(anyhow::anyhow!("Expected error for invalid market")),
        None => {
            return Err(anyhow::anyhow!(
                "Stream ended without error for invalid market"
            ))
        }
    }

    let mut stream = client
        .get_market_depths_stream(valid_market, 3, api::Project::POpenbook)
        .await?;

    match stream.next().await {
        Some(Ok(response)) => {
            println!("Response received: {:#?}", response);
            Ok(())
        }
        Some(Err(e)) => Err(anyhow::anyhow!("Stream error: {}", e)),
        None => Err(anyhow::anyhow!("Stream ended without data")),
    }
}

// TODO:
// Go SDK error: ERRO[2024-11-07 14:22:52] example 'getTickersStream' failed
#[test_case(
    vec![
        "BONK/SOL".to_string(),
        "wSOL/RAY".to_string(),
        "BONK/RAY".to_string(),
        "RAY/USDC".to_string(),
        "SOL/USDC".to_string(),
        "USDT/USDC".to_string()
    ],
    false ;
    "valid markets"
)]
#[tokio::test]
#[ignore]
async fn test_ticker_stream_grpc(markets: Vec<String>, expect_error: bool) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;
    println!("starting ticker stream");

    let mut stream = client
        .get_ticker_stream(markets, api::Project::POpenbook)
        .await?;

    match stream.next().await {
        Some(Ok(response)) => {
            println!("Response received: {:#?}", response);
            if expect_error {
                Err(anyhow::anyhow!("Expected an error but got success"))
            } else {
                Ok(())
            }
        }
        Some(Err(e)) => {
            if expect_error {
                println!("Expected error received: {}", e);
                Ok(())
            } else {
                Err(anyhow::anyhow!("Stream error: {}", e))
            }
        }
        None => Err(anyhow::anyhow!("Stream ended without data")),
    }
}

// TODO:
// Go SDK error: ERRO[2024-11-07 14:22:52] example 'getTradesStream' failed
#[test_case(
    "SOL/USDC".to_string(),
    3,
    false ;
    "valid market"
)]
#[test_case(
    "INVALID/MARKET".to_string(),
    3,
    true ;
    "invalid market"
)]
#[tokio::test]
#[ignore]
async fn test_trades_stream_grpc(market: String, limit: u32, expect_error: bool) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;
    println!("starting trades stream");

    let mut stream = client
        .get_trades_stream(market, limit, api::Project::POpenbook)
        .await?;

    match stream.next().await {
        Some(Ok(response)) => {
            println!("Response received: {:#?}", response);
            if expect_error {
                Err(anyhow::anyhow!("Expected an error but got success"))
            } else {
                Ok(())
            }
        }
        Some(Err(e)) => {
            if expect_error {
                println!("Expected error received: {}", e);
                Ok(())
            } else {
                Err(anyhow::anyhow!("Stream error: {}", e))
            }
        }
        None => Err(anyhow::anyhow!("Stream ended without data")),
    }
}

// TODO:
// this stream is still somewhat active, so sometimes it does not fail
// Go SDK error: ERRO[2024-11-07 14:58:00] example 'getSwapsStream' failed
#[test_case(
    vec![api::Project::PRaydium],
    vec!["58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2".to_string()],
    true ;
    "SOL-USDC Raydium pool"
)]
#[tokio::test]
#[ignore]
async fn test_swaps_stream_grpc(
    projects: Vec<api::Project>,
    pools: Vec<String>,
    only_fills: bool,
) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;
    println!("starting swaps stream");

    let mut stream = client.get_swaps_stream(projects, pools, only_fills).await?;

    match stream.next().await {
        Some(Ok(response)) => {
            println!("Response received: {:#?}", response);
            Ok(())
        }
        Some(Err(e)) => Err(anyhow::anyhow!("Stream error: {}", e)),
        None => Err(anyhow::anyhow!("Stream ended without data")),
    }
}

#[test_case(false ; "without cpmm")]
// TODO: #[test_case(true ; "with cpmm")]
// fails with the following: Error: Failed to install crypto provider: CryptoProvider
// can't do multiple test cases like this at the moment.
#[tokio::test]
#[ignore]
async fn test_new_raydium_pools_stream_grpc(include_cpmm: bool) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;
    println!("starting new raydium pools stream");

    let mut stream = client.get_new_raydium_pools_stream(include_cpmm).await?;

    match stream.next().await {
        Some(Ok(response)) => {
            println!("Response received: {:#?}", response);
            Ok(())
        }
        Some(Err(e)) => Err(anyhow::anyhow!("Stream error: {}", e)),
        None => Err(anyhow::anyhow!("Stream ended without data")),
    }
}

#[test_case(3 ; "without cpmm, three responses")]
#[tokio::test]
#[ignore]
async fn test_new_raydium_pools_by_transaction_stream_grpc(
    expected_responses: usize,
) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;
    println!("starting new raydium pools by transaction stream");

    let mut stream = client.get_new_raydium_pools_by_transaction_stream().await?;

    let mut responses_received = 0;
    loop {
        match stream.next().await {
            Some(Ok(response)) => {
                println!(
                    "Response {} received: {:#?}",
                    responses_received + 1,
                    response
                );
                responses_received += 1;

                if responses_received >= expected_responses {
                    return Ok(());
                }
            }
            Some(Err(e)) => {
                return Err(anyhow::anyhow!("Stream error: {}", e));
            }
            None => {
                return Err(anyhow::anyhow!("Stream ended without data"));
            }
        }
    }
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

// Test implementations
#[test_case(
    vec![api::Project::PRaydium],
    vec![
        "HZ1znC9XBasm9AMDhGocd9EHSyH8Pyj1EUdiPb4WnZjo".to_string(),
        "D8wAxwpH2aKaEGBKfeGdnQbCc2s54NrRvTDXCK98VAeT".to_string(),
        "DdpuaJgjB2RptGMnfnCZVmC4vkKsMV6ytRa2gggQtCWt".to_string(),
        "AVs9TA4nWDzfPJE9gGVNJMVhcQy3V9PGazuz33BfG2RA".to_string(),
        "58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2".to_string(),
        "7XawhbbxtsRcQA8KTkHT9f9nc6d69UwqCDh6U5EEbEmX".to_string(),
    ];
    "Raydium pool reserves stream"
)]
#[tokio::test]
#[ignore]
async fn test_pool_reserves_stream_grpc(
    projects: Vec<api::Project>,
    pools: Vec<String>,
) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;
    let mut stream = client.get_pool_reserves_stream(projects, pools).await?;

    println!("starting pool reserves stream");

    let response = stream
        .next()
        .await
        .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))?
        .map_err(|e| anyhow::anyhow!("Stream error: {}", e))?;

    println!("Response received: {:#?}", response);

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
