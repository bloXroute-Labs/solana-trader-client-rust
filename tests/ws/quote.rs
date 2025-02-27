use anyhow::Result;
use solana_trader_client_rust::{
    common::constants::{MAINNET_PUMP_NY, USDC, WRAPPED_SOL},
    provider::ws::WebSocketClient,
};
use solana_trader_proto::api;
use std::time::Duration;
use test_case::test_case;
use tokio::time::timeout;

#[test_case(
        WRAPPED_SOL,
        USDC,
        0.01,
        5.0 ;
        "SOL to USDC quote"
    )]
#[tokio::test]
#[ignore]
async fn test_raydium_quotes_ws(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let client = WebSocketClient::new(None).await?;

    let request = api::GetRaydiumQuotesRequest {
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
    };

    let response = timeout(Duration::from_secs(10), client.get_raydium_quotes(&request))
        .await
        .map_err(|e| anyhow::anyhow!("Timeout: {}", e))??;

    println!(
        "Raydium Quote: {}",
        serde_json::to_string_pretty(&response)?
    );
    assert!(
        !response.routes.is_empty(),
        "Expected at least one route in response"
    );

    client.close().await?;
    Ok(())
}

#[test_case(
    WRAPPED_SOL,
    USDC,
    0.01,
    5.0;
    "SOL to USDC CPMM quote"
)]
#[tokio::test]
#[ignore]
async fn test_raydium_cpmm_quotes_ws(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let client = WebSocketClient::new(None).await?;

    let request = api::GetRaydiumCpmmQuotesRequest {
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
    };

    let response = timeout(
        Duration::from_secs(10),
        client.get_raydium_cpmm_quotes(&request),
    )
    .await
    .map_err(|e| anyhow::anyhow!("Timeout: {}", e))??;

    println!(
        "Raydium CPMM Quote: {}",
        serde_json::to_string_pretty(&response)?
    );

    assert!(
        !response.routes.is_empty(),
        "Expected at least one route in response"
    );

    client.close().await?;
    Ok(())
}

#[test_case(
    WRAPPED_SOL,
    USDC,
    0.01,
    5.0;
    "SOL to USDC CLMM quote"
)]
#[tokio::test]
#[ignore]
async fn test_raydium_clmm_quotes_ws(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let client = WebSocketClient::new(None).await?;

    let request = api::GetRaydiumClmmQuotesRequest {
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
    };

    let response = timeout(
        Duration::from_secs(10),
        client.get_raydium_clmm_quotes(&request),
    )
    .await
    .map_err(|e| anyhow::anyhow!("Timeout: {}", e))??;

    println!(
        "Raydium CLMM Quote: {}",
        serde_json::to_string_pretty(&response)?
    );
    assert!(
        !response.routes.is_empty(),
        "Expected at least one route in response"
    );

    client.close().await?;
    Ok(())
}

// TODO: 10/31 remove when works: unknown field "slippage"
#[test_case(
    "BAHY8ocERNc5j6LqkYav1Prr8GBGsHvBV5X3dWPhsgXw",  // Token address
    "7BcRpqUC7AF5Xsc3QEpCb8xmoi2X1LpwjUBNThbjWvyo",  // Bonding curve address
    "Sell",                                            // Quote type
    10.0;                                             // Amount
    "PumpFun Sell quote"
)]
#[tokio::test]
#[ignore]
async fn test_pump_fun_quotes_ws(
    mint_address: &str,
    bonding_curve_address: &str,
    quote_type: &str,
    amount: f64,
) -> Result<()> {
    let client = WebSocketClient::new(Some(MAINNET_PUMP_NY.to_string())).await?;

    let request = api::GetPumpFunQuotesRequest {
        mint_address: mint_address.to_string(),
        bonding_curve_address: bonding_curve_address.to_string(),
        quote_type: quote_type.to_string(),
        amount,
    };

    let response = timeout(
        Duration::from_secs(10),
        client.get_pump_fun_quotes(&request),
    )
    .await
    .map_err(|e| anyhow::anyhow!("Timeout: {}", e))??;

    println!(
        "PumpFun Quote: {}",
        serde_json::to_string_pretty(&response)?
    );
    assert!(
        response.out_amount > 0.0,
        "Expected non-zero out amount in response"
    );

    client.close().await?;
    Ok(())
}

#[test_case(
    WRAPPED_SOL,
    USDC,
    0.01,
    5.0;
    "SOL to USDC Jupiter quote"
)]
#[tokio::test]
#[ignore]
async fn test_jupiter_quotes_ws(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let client = WebSocketClient::new(None).await?;

    let request = api::GetJupiterQuotesRequest {
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
    };

    let response = timeout(Duration::from_secs(10), client.get_jupiter_quotes(&request))
        .await
        .map_err(|e| anyhow::anyhow!("Timeout: {}", e))??;

    println!(
        "Jupiter Quote: {}",
        serde_json::to_string_pretty(&response)?
    );
    assert!(
        !response.routes.is_empty(),
        "Expected at least one route in response"
    );

    client.close().await?;
    Ok(())
}

// TODO:
// Error: Error: Failed to parse result: invalid type: string "P_JUPITER", expected i32
#[test_case(
    "So11111111111111111111111111111111111111112",
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    0.01,
    5.0,
    5,
    vec![api::Project::PAll];
    "SOL to USDC aggregated quotes via WebSocket"
)]
#[tokio::test]
#[ignore]
async fn test_get_quotes_ws(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
    limit: i32,
    projects: Vec<api::Project>,
) -> Result<()> {
    let client = WebSocketClient::new(None).await?;

    let request = api::GetQuotesRequest {
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
        limit,
        projects: projects.iter().map(|p| *p as i32).collect(),
    };

    let response = timeout(Duration::from_secs(10), client.get_quotes(&request))
        .await
        .map_err(|e| anyhow::anyhow!("Timeout: {}", e))??;

    println!(
        "Aggregated Quotes: {}",
        serde_json::to_string_pretty(&response)?
    );

    assert!(
        response.quotes.len() == 2,
        "Expected exactly 2 quotes in response, got {}",
        response.quotes.len()
    );

    for quote in &response.quotes {
        assert!(
            !quote.routes.is_empty(),
            "No routes found for project {}",
            quote.project
        );
    }

    client.close().await?;
    Ok(())
}

#[test_case(
    vec![
        "So11111111111111111111111111111111111111112".to_string(),
        "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263".to_string(),
    ];
    "SOL and BONK prices via WebSocket"
)]
#[tokio::test]
#[ignore]
async fn test_get_raydium_prices_ws(tokens: Vec<String>) -> Result<()> {
    let ws = WebSocketClient::new(None).await?;

    let response = ws.get_raydium_prices(tokens).await?;
    println!("Raydium prices response: {:#?}", response);

    ws.close().await?;
    Ok(())
}

#[test_case(
    vec![
        "So11111111111111111111111111111111111111112".to_string(),  // SOL
        "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263".to_string(), // BONK
    ];
    "SOL and BONK prices via WebSocket"
)]
#[tokio::test]
#[ignore]
async fn test_get_jupiter_prices_ws(tokens: Vec<String>) -> Result<()> {
    let ws = WebSocketClient::new(None).await?;

    let response = ws.get_jupiter_prices(tokens).await?;
    println!("Jupiter prices response: {:#?}", response);

    ws.close().await?;
    Ok(())
}
