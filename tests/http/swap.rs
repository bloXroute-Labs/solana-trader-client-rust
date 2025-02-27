use anyhow::Result;
use solana_trader_client_rust::common::signing::SubmitParams;
use solana_trader_client_rust::common::{constants::USDC, constants::WRAPPED_SOL};
use solana_trader_client_rust::provider::http::HTTPClient;
use solana_trader_proto::api;
use solana_trader_proto::common::Fee;
use test_case::test_case;

#[test_case(
    WRAPPED_SOL,
    USDC,
    0.001,
    20.0;
    "Raydium SOL to USDC swap via HTTP"
)]
#[tokio::test]
#[ignore]
async fn test_raydium_swap_http(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let client = HTTPClient::new(None)?;

    let request = api::PostRaydiumSwapRequest {
        owner_address: client
            .public_key
            .unwrap_or_else(|| panic!("Public key is required for Raydium swap"))
            .to_string(),
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
        compute_limit: 300000,
        compute_price: 2000,
        tip: Some(2000001),
    };

    let response = client.post_raydium_swap(&request).await?;
    println!(
        "Raydium Quote: {}",
        serde_json::to_string_pretty(&response)?
    );

    let txs = response.transactions.as_slice();
    let submit_opts = SubmitParams::default();
    let s = client
        .sign_and_submit(txs.to_vec(), submit_opts, false)
        .await;
    println!("Raydium signature: {:#?}", s?);

    Ok(())
}

#[test_case(
    "So11111111111111111111111111111111111111112",   // Input token (SOL)
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // Output token (USDC)
    0.01,                                             // Input amount
    0.0074,                                           // Output amount
    0.007505,                                         // Minimum output amount
    0.1;                                              // Slippage
    "Raydium Route SOL to USDC swap via HTTP"
)]
#[tokio::test]
#[ignore]
async fn test_raydium_route_swap_http(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    out_amount: f64,
    out_amount_min: f64,
    slippage: f64,
) -> Result<()> {
    let client = HTTPClient::new(None)?;

    let request = api::PostRaydiumRouteSwapRequest {
        owner_address: client
            .public_key
            .unwrap_or_else(|| panic!("Public key is required for Raydium route swap"))
            .to_string(),
        slippage,
        steps: vec![api::RaydiumRouteStep {
            in_token: in_token.to_string(),
            out_token: out_token.to_string(),
            in_amount,
            out_amount,
            out_amount_min,
            pool_address: "".to_string(),
            project: Some(api::StepProject {
                label: "Raydium".to_string(),
                id: "58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2".to_string(),
            }),
        }],
        compute_limit: 300000,
        compute_price: 10000,
        tip: Some(10000),
    };

    let response = client.post_raydium_route_swap(&request).await?;
    println!(
        "Raydium Route Quote: {}",
        serde_json::to_string_pretty(&response)?
    );

    let txs = response.transactions.as_slice();
    let submit_opts = SubmitParams::default();
    let s = client
        .sign_and_submit(txs.to_vec(), submit_opts, false)
        .await;
    println!("Raydium signature: {:#?}", s?);

    Ok(())
}

#[test_case(
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",  // USDC
    "So11111111111111111111111111111111111111112",   // SOL
    0.001,                                           // Amount
    0.4;                                             // Slippage
    "Raydium swap instructions USDC to SOL via HTTP"
)]
#[tokio::test]
#[ignore]
async fn test_raydium_swap_instructions_http(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let client = HTTPClient::new(None)?;

    let request = api::PostRaydiumSwapInstructionsRequest {
        owner_address: client
            .public_key
            .unwrap_or_else(|| panic!("Public key is required for Raydium swap instructions"))
            .to_string(),
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
        compute_limit: 300000,
        compute_price: 2000,
        tip: Some(2000001),
    };

    let submit_opts = SubmitParams::default();
    let signatures = client
        .submit_raydium_swap_instructions(request, submit_opts, false)
        .await?;

    println!("Raydium swap instructions signatures: {:#?}", signatures);

    Ok(())
}

#[test_case(
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    "So11111111111111111111111111111111111111112",
    0.01,
    0.5;
    "Raydium CPMM USDC to SOL swap via HTTP"
)]
#[tokio::test]
#[ignore]
async fn test_raydium_cpmm_swap_http(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let client = HTTPClient::new(None)?;

    let request = api::PostRaydiumCpmmSwapRequest {
        owner_address: client
            .public_key
            .unwrap_or_else(|| panic!("Public key is required for Raydium CPMM swap"))
            .to_string(),
        pool_address: "".to_string(),
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
        compute_limit: 300000,
        compute_price: 2000,
        tip: Some(2000001),
    };

    let response = client.post_raydium_cpmm_swap(&request).await?;
    println!(
        "Raydium CPMM Quote: {}",
        serde_json::to_string_pretty(&response)?
    );

    let txs = response.transaction.as_slice();
    let submit_opts = SubmitParams::default();
    let s = client
        .sign_and_submit(txs.to_vec(), submit_opts, false)
        .await;
    println!("Raydium signature: {:#?}", s?);

    Ok(())
}

#[test_case(
    "So11111111111111111111111111111111111111112",   // SOL
    "HDa3zJc12ahykSsBRvgiWzr6WLEByf36yzKKbVvy4gnF", // USDC
    0.0089,
    0.1;
    "Raydium CLMM SOL to USDC swap via HTTP"
)]
#[tokio::test]
#[ignore]
async fn test_raydium_clmm_swap_http(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let client = HTTPClient::new(None)?;

    let request = api::PostRaydiumSwapRequest {
        owner_address: client
            .public_key
            .unwrap_or_else(|| panic!("Public key is required for Raydium CLMM swap"))
            .to_string(),
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
        compute_limit: 300000,
        compute_price: 10000,
        tip: Some(10000),
    };

    let response = client.post_raydium_clmm_swap(&request).await?;
    println!(
        "Raydium CLMM Quote: {}",
        serde_json::to_string_pretty(&response)?
    );

    let txs = response.transactions.as_slice();
    let submit_opts = SubmitParams::default();
    let s = client
        .sign_and_submit(txs.to_vec(), submit_opts, false)
        .await;
    println!("Raydium signature: {:#?}", s?);

    Ok(())
}

#[test_case(
    "HDa3zJc12ahykSsBRvgiWzr6WLEByf36yzKKbVvy4gnF", // Input token (USDC)
    "So11111111111111111111111111111111111111112",   // Output token (SOL)
    0.000303,                                         // Input amount
    0.00064,                                          // Output amount
    0.0006005,                                        // Minimum output amount
    0.1;                                              // Slippage
    "Raydium CLMM Route USDC to SOL swap via HTTP"
)]
#[tokio::test]
#[ignore]
async fn test_raydium_clmm_route_swap_http(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    out_amount: f64,
    out_amount_min: f64,
    slippage: f64,
) -> Result<()> {
    let client = HTTPClient::new(None)?;

    let request = api::PostRaydiumRouteSwapRequest {
        owner_address: client
            .public_key
            .unwrap_or_else(|| panic!("Public key is required for Raydium CLMM route swap"))
            .to_string(),
        slippage,
        steps: vec![api::RaydiumRouteStep {
            in_token: in_token.to_string(),
            out_token: out_token.to_string(),
            in_amount,
            out_amount,
            out_amount_min,
            pool_address: "".to_string(),
            project: Some(api::StepProject {
                label: "".to_string(),
                id: "".to_string(),
            }),
        }],
        compute_limit: 300000,
        compute_price: 10000,
        tip: Some(10000),
    };

    let response = client.post_raydium_clmm_route_swap(&request).await?;
    println!(
        "Raydium CLMM Route Quote: {}",
        serde_json::to_string_pretty(&response)?
    );

    let txs = response.transactions.as_slice();
    let submit_opts = SubmitParams::default();
    let s = client
        .sign_and_submit(txs.to_vec(), submit_opts, false)
        .await;
    println!("Raydium signature: {:#?}", s?);

    Ok(())
}

#[test_case(
    WRAPPED_SOL,
    USDC,
    0.01,
    1.0;
    "Jupiter SOL to USDC swap via HTTP"
)]
#[tokio::test]
#[ignore]
async fn test_jupiter_swap_http(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let client = HTTPClient::new(None)?;

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

    let response = client.post_jupiter_swap(&request).await?;
    println!(
        "Jupiter Quote: {}",
        serde_json::to_string_pretty(&response)?
    );

    let txs = response.transactions.as_slice();
    let submit_opts = SubmitParams::default();
    let s = client
        .sign_and_submit(txs.to_vec(), submit_opts, false)
        .await;
    println!("Raydium signature: {:#?}", s?);

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
    "Jupiter Route USDC to SOL swap via Raydium HTTP"
)]
#[tokio::test]
#[ignore]
async fn test_jupiter_route_swap_http(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    out_amount: f64,
    out_amount_min: f64,
    slippage: f64,
) -> Result<()> {
    let client = HTTPClient::new(None)?;

    let request = api::PostJupiterRouteSwapRequest {
        owner_address: client
            .public_key
            .unwrap_or_else(|| panic!("Public key is required for Jupiter route swap"))
            .to_string(),
        slippage,
        steps: vec![api::JupiterRouteStep {
            project: Some(api::StepProject {
                label: "Raydium".to_string(),
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
    println!("Raydium signature: {:#?}", s?);

    Ok(())
}

#[test_case(
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // USDC
    "So11111111111111111111111111111111111111112",   // SOL
    0.001,                                            // Amount
    0.4;                                              // Slippage
    "Jupiter swap instructions USDC to SOL via HTTP"
)]
#[tokio::test]
#[ignore]
async fn test_jupiter_swap_instructions_http(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let client = HTTPClient::new(None)?;

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

    Ok(())
}
