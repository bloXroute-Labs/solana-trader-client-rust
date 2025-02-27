use anyhow::Result;
use solana_trader_client_rust::{
    common::constants::{MAINNET_PUMP_NY, USDC, WRAPPED_SOL},
    provider::grpc::GrpcClient,
};
use solana_trader_proto::api;
use test_case::test_case;

#[test_case(
    WRAPPED_SOL,
    USDC,
    0.1,
    0.2;
    "BTC to USDC with higher slippage"
)]
#[tokio::test]
#[ignore]
async fn test_raydium_quotes_grpc(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

    let request = api::GetRaydiumQuotesRequest {
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
    };

    let response = client.get_raydium_quotes(&request).await?;
    println!(
        "Raydium Quote: {}",
        serde_json::to_string_pretty(&response)?
    );
    assert!(
        !response.routes.is_empty(),
        "Expected at least one route in response"
    );

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
async fn test_raydium_cpmm_quotes_grpc(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

    let request = api::GetRaydiumCpmmQuotesRequest {
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
    };

    let response = client.get_raydium_cpmm_quotes(&request).await?;
    println!(
        "Raydium CPMM Quote: {}",
        serde_json::to_string_pretty(&response)?
    );
    assert!(
        !response.routes.is_empty(),
        "Expected at least one route in response"
    );

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
async fn test_raydium_clmm_quotes_grpc(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

    let request = api::GetRaydiumClmmQuotesRequest {
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
    };

    let response = client.get_raydium_clmm_quotes(&request).await?;
    println!(
        "Raydium CLMM Quote: {}",
        serde_json::to_string_pretty(&response)?
    );
    assert!(
        !response.routes.is_empty(),
        "Expected at least one route in response"
    );

    Ok(())
}

#[test_case(
    "BAHY8ocERNc5j6LqkYav1Prr8GBGsHvBV5X3dWPhsgXw",  // Token address
    "7BcRpqUC7AF5Xsc3QEpCb8xmoi2X1LpwjUBNThbjWvyo",  // Bonding curve address
    "Sell",                                            // Quote type
    10.0 ;                                            // Amount
    "PumpFun Sell quote"
)]
#[tokio::test]
#[ignore]
async fn test_pump_fun_quotes_grpc(
    mint_address: &str,
    bonding_curve_address: &str,
    quote_type: &str,
    amount: f64,
) -> Result<()> {
    let mut client = GrpcClient::new(Some(MAINNET_PUMP_NY.to_string())).await?;

    // note slippage is still needed as part of the proto
    let request = api::GetPumpFunQuotesRequest {
        mint_address: mint_address.to_string(),
        bonding_curve_address: bonding_curve_address.to_string(),
        quote_type: quote_type.to_string(),
        amount,
    };

    let response = client.get_pump_fun_quotes(&request).await?;
    println!(
        "PumpFun Quote: {}",
        serde_json::to_string_pretty(&response)?
    );
    assert!(
        response.out_amount > 0.0,
        "Expected non-zero out amount in response"
    );

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
async fn test_jupiter_quotes_grpc(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

    let request = api::GetJupiterQuotesRequest {
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
    };

    let response = client.get_jupiter_quotes(&request).await?;
    println!(
        "Jupiter Quote: {}",
        serde_json::to_string_pretty(&response)?
    );
    assert!(
        !response.routes.is_empty(),
        "Expected at least one route in response"
    );

    Ok(())
}

#[test_case(
    "So11111111111111111111111111111111111111112",
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    0.01,
    5.0,
    5,
    vec![api::Project::PAll];
    "SOL to USDC aggregated quotes via gRPC"
)]
#[tokio::test]
#[ignore]
async fn test_get_quotes_grpc(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
    limit: i32,
    projects: Vec<api::Project>,
) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

    let request = api::GetQuotesRequest {
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
        limit,
        projects: projects.iter().map(|p| *p as i32).collect(),
    };

    let response = client.get_quotes(&request).await?;
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

    Ok(())
}

#[test_case(
    vec![
        "So11111111111111111111111111111111111111112".to_string(),  // SOL
        "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263".to_string(), // BONK
    ];
    "SOL and BONK prices"
)]
#[tokio::test]
#[ignore]
async fn test_get_raydium_prices_grpc(tokens: Vec<String>) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

    let response = client.get_raydium_prices(tokens).await?;
    println!("Raydium prices response: {:#?}", response);

    Ok(())
}

#[test_case(
    vec![
        "So11111111111111111111111111111111111111112".to_string(),  // SOL
        "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263".to_string(), // BONK
    ];
    "SOL and BONK prices via gRPC"
)]
#[tokio::test]
#[ignore]
async fn test_get_jupiter_prices_grpc(tokens: Vec<String>) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

    let response = client.get_jupiter_prices(tokens).await?;
    println!("Jupiter prices response: {:#?}", response);

    Ok(())
}
