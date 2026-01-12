use anyhow::Result;
use solana_trader_client_rust::{
    common::constants::{MAINNET_PUMP_NY, USDC, WRAPPED_SOL},
    provider::ws::WebSocketClient,
};
use solana_trader_proto::api;
use std::time::Duration;
use test_case::test_case;
use tokio::time::timeout;

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
    api::GetPumpFunAmmQuotesRequest {
        in_token: WRAPPED_SOL.to_string(),
        in_amount: 10.0,
        out_token: USDC.to_string(),
        pool: "Gf7sXMoP8iRw4iiXmJ1nq4vxcRycbGXy5RL8a8LnTd3v".to_string(),
        slippage: 0.9,
    };
    "PumpFun AMM quote via WS"
)]
#[tokio::test]
#[ignore]
async fn test_get_pump_fun_amm_quotes_ws(request: api::GetPumpFunAmmQuotesRequest) -> Result<()> {
    let client = WebSocketClient::new(Some(MAINNET_PUMP_NY.to_string())).await?;

    let response = timeout(
        Duration::from_secs(10),
        client.get_pump_fun_amm_quotes(&request),
    )
    .await
    .map_err(|e| anyhow::anyhow!("Timeout: {}", e))??;

    println!(
        "PumpFun AMM Quote: {}",
        serde_json::to_string_pretty(&response)?
    );
    assert!(response.out_amount > 0.0, "Expected non-zero out amount");

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
