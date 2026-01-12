use anyhow::Result;
use solana_trader_client_rust::{
    common::{
        constants::{MAINNET_PUMP_NY, USDC, WRAPPED_SOL},
        signing::SubmitParams,
    },
    provider::ws::WebSocketClient,
};
use solana_trader_proto::{api, common::Fee};
use std::time::Duration;
use test_case::test_case;
use tokio::time::timeout;

#[test_case(
    WRAPPED_SOL,
    USDC,
    0.01,
    1.0;
    "Jupiter SOL to USDC swap via WebSocket"
)]
#[tokio::test]
#[ignore]
async fn test_jupiter_swap_ws(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let client = WebSocketClient::new(None).await?;

    let request = api::PostJupiterSwapRequest {
        owner_address: client
            .public_key
            .unwrap_or_else(|| panic!("Public key is required for Jupiter swap"))
            .to_string(),
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
        compute_limit: 300000,
        compute_price: 2000,
        tip: Some(2000001),
    };

    let response = timeout(Duration::from_secs(10), client.post_jupiter_swap(&request))
        .await
        .map_err(|e| anyhow::anyhow!("Timeout: {}", e))??;

    println!(
        "Jupiter Quote: {}",
        serde_json::to_string_pretty(&response)?
    );

    let txs = response.transactions.as_slice();
    let submit_opts = SubmitParams::default();
    let s = client
        .sign_and_submit(txs.to_vec(), submit_opts, false)
        .await;
    println!("Jupiter signature: {:#?}", s?);

    client.close().await?;
    Ok(())
}

// TODO: does not work
// Error: RPC error: {"code":-32603,"data":"Jupiter API error: Market 61acRgpURKTU8LKPJKs6WQa18KzD9ogavXzjxfD84KLu not found","message":"Internal error"}
#[test_case(
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // Input token (USDC)
    "So11111111111111111111111111111111111111112",   // Output token (SOL)
    0.01,                                             // Input amount
    0.000123425,                                      // Output amount
    0.000123117,                                      // Minimum output amount
    0.25;                                             // Slippage
    "Jupiter Route USDC to SOL swap via Raydium WebSocket"
)]
#[tokio::test]
#[ignore]
async fn test_jupiter_route_swap_ws(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    out_amount: f64,
    out_amount_min: f64,
    slippage: f64,
) -> Result<()> {
    let client = WebSocketClient::new(None).await?;

    let request = api::PostJupiterRouteSwapRequest {
        owner_address: client
            .public_key
            .unwrap_or_else(|| panic!("Public key is required for Jupiter route swap"))
            .to_string(),
        slippage,
        steps: vec![api::JupiterRouteStep {
            project: Some(api::StepProject {
                label: "Jupiter".to_string(),
                id: "61acRgpURKTU8LKPJKs6WQa18KzD9ogavXzjxfD84KLu".to_string(),
            }),
            in_token: in_token.to_string(),
            out_token: out_token.to_string(),
            in_amount,
            out_amount,
            out_amount_min,
            fee: Some(Fee {
                amount: 0.000025,
                mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
                percent: 0.0025062656,
            }),
        }],
        compute_limit: 300000,
        compute_price: 10000,
        tip: Some(10000),
    };

    let response = client.post_jupiter_route_swap(&request).await?;
    println!(
        "Jupiter Route Quote: {}",
        serde_json::to_string_pretty(&response)?
    );

    let txs = response.transactions.as_slice();
    let submit_opts = SubmitParams::default();
    let s = client
        .sign_and_submit(txs.to_vec(), submit_opts, false)
        .await;
    println!("Jupiter signature: {:#?}", s?);

    client.close().await?;
    Ok(())
}

#[test_case(
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // USDC
    "So11111111111111111111111111111111111111112",   // SOL
    0.001,                                            // Amount
    0.4;                                              // Slippage
    "Jupiter swap instructions USDC to SOL via WebSocket"
)]
#[tokio::test]
#[ignore]
async fn test_jupiter_swap_instructions_ws(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let client = WebSocketClient::new(None).await?;

    let request = api::PostJupiterSwapInstructionsRequest {
        owner_address: client
            .public_key
            .unwrap_or_else(|| panic!("Public key is required for Jupiter swap instructions"))
            .to_string(),
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
        compute_price: 2000,
        tip: Some(2000001),
    };

    let submit_opts = SubmitParams::default();
    let signatures = client
        .submit_jupiter_swap_instructions(request, submit_opts, false)
        .await?;

    println!("Jupiter swap instructions signatures: {:#?}", signatures);

    client.close().await?;
    Ok(())
}

#[test_case(
    api::PostPumpFunAmmSwapRequest {
        owner_address: "will be set in test fn".to_string(),
        in_token: WRAPPED_SOL.to_string(),
        in_amount: 10.0,
        out_token: USDC.to_string(),
        pool: "Gf7sXMoP8iRw4iiXmJ1nq4vxcRycbGXy5RL8a8LnTd3v".to_string(),
        slippage: 0.9,
        compute_limit: 130000,
        compute_price: 100000,
        tip: Some(100000),
    };
    "PumpFun AMM swap via WS"
)]
#[tokio::test]
#[ignore]
async fn test_post_pump_fun_amm_swap_ws(mut request: api::PostPumpFunAmmSwapRequest) -> Result<()> {
    let client = WebSocketClient::new(Some(MAINNET_PUMP_NY.to_string())).await?;

    request.owner_address = client
        .public_key
        .unwrap_or_else(|| panic!("Public key is required for PumpFun AMM swap"))
        .to_string();

    let response = timeout(
        Duration::from_secs(10),
        client.post_pump_fun_amm_swap(&request),
    )
    .await
    .map_err(|e| anyhow::anyhow!("Timeout: {}", e))??;

    println!(
        "PumpFun AMM Swap: {}",
        serde_json::to_string_pretty(&response)?
    );
    assert!(
        response.buy_base_amount_out > 0.0,
        "Expected non-zero buy_base_amount_out in response"
    );
    assert!(
        response.buy_max_quote_amount_in > 0.0,
        "Expected non-zero buy_max_quote_amount_in in response"
    );
    assert!(
        response.sell_base_amount_in == 0.0,
        "Expected zero sell_base_amount_in in response"
    );
    assert!(
        response.sell_min_quote_amount_out == 0.0,
        "Expected zero sell_min_quote_amount_out in response"
    );

    client.close().await?;
    Ok(())
}
