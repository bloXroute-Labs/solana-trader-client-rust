use anyhow::Result;
use solana_trader_client_rust::{
    common::{
        constants::{MAINNET_PUMP_NY, USDC, WRAPPED_SOL},
        signing::SubmitParams,
    },
    provider::grpc::GrpcClient,
};
use solana_trader_proto::api;
use solana_trader_proto::common::Fee;
use test_case::test_case;

#[test_case(
    WRAPPED_SOL,
    USDC,
    0.001,
    20.0;
    "BTC to USDC with higher slippage"
)]
#[tokio::test]
#[ignore]
async fn test_raydium_swap_grpc(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

    let request = api::PostRaydiumSwapRequest {
        owner_address: client
            .public_key
            .unwrap_or_else(|| panic!("Public key is required for pump fun swap"))
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
        "raydium Quote: {}",
        serde_json::to_string_pretty(&response)?
    );

    let txs = response.transactions.as_slice();
    let submit_opts = SubmitParams::default();
    let s = client
        .sign_and_submit(txs.to_vec(), submit_opts, false)
        .await;
    println!("rayidum signature : {:#?}", s?);

    Ok(())
}

#[test_case(
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // USDC
    "So11111111111111111111111111111111111111112",   // SOL
    0.001,                                            // Amount
    0.4;                                              // Slippage
    "Raydium swap instructions USDC to SOL"
)]
#[tokio::test]
#[ignore]
async fn test_raydium_swap_instructions_grpc(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

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
    "So11111111111111111111111111111111111111112",   // Input token (SOL)
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // Output token (USDC)
    0.01,                                             // Input amount
    0.0074,                                           // Output amount
    0.007505,                                         // Minimum output amount
    0.1;                                              // Slippage
    "Raydium Route SOL to USDC swap"
)]
#[tokio::test]
#[ignore]
async fn test_raydium_route_swap_grpc(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    out_amount: f64,
    out_amount_min: f64,
    slippage: f64,
) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

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
    println!("Raydium Route signature: {:#?}", s?);

    Ok(())
}

#[test_case(
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // USDC
    "So11111111111111111111111111111111111111112",   // SOL
    0.01,
    0.5;
    "Raydium CPMM USDC to SOL swap"
)]
#[tokio::test]
#[ignore]
async fn test_raydium_cpmm_swap_grpc(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

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
    println!("Raydium CPMM signature: {:#?}", s?);

    Ok(())
}

#[test_case(
    "So11111111111111111111111111111111111111112",   // SOL
    "HDa3zJc12ahykSsBRvgiWzr6WLEByf36yzKKbVvy4gnF", // USDC
    0.0089,
    0.1;
    "Raydium CLMM SOL to USDC swap via gRPC"
)]
#[tokio::test]
#[ignore]
async fn test_raydium_clmm_swap_grpc(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

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
    println!("Raydium CLMM signature: {:#?}", s?);

    Ok(())
}

#[test_case(
    "HDa3zJc12ahykSsBRvgiWzr6WLEByf36yzKKbVvy4gnF", // Input token (USDC)
    "So11111111111111111111111111111111111111112",   // Output token (SOL)
    0.000303,                                         // Input amount
    0.00064,                                          // Output amount
    0.0006005,                                        // Minimum output amount
    0.1;                                              // Slippage
    "Raydium CLMM Route USDC to SOL swap"
)]
#[tokio::test]
#[ignore]
async fn test_raydium_clmm_route_swap_grpc(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    out_amount: f64,
    out_amount_min: f64,
    slippage: f64,
) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

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
        compute_price: 2600000,
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
    println!("Raydium CLMM Route signature: {:#?}", s?);

    Ok(())
}

#[test_case(
    WRAPPED_SOL,
    USDC,
    0.01,
    1.0;
    "Jupiter SOL to USDC swap"
)]
#[tokio::test]
#[ignore]
async fn test_jupiter_swap_grpc(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

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
    println!("Jupiter signature: {:#?}", s?);

    Ok(())
}

// TODO: does not work
// Error: RPC error: {"code":-32603,"data":"Jupiter API error: Market 61acRgpURKTU8LKPJKs6WQa18KzD9ogavXzjxfD84KLu not found","message":"Internal error"}
#[test_case(
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // Input token (USDC)
    "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB",   // Output token (SOL)
    0.01,                                             // Input amount
    0.000123425,                                      // Output amount
    0.000123117,                                      // Minimum output amount
    0.25;                                             // Slippage
    "Jupiter Route USDC to SOL swap via Raydium"
)]
#[tokio::test]
#[ignore]
async fn test_jupiter_route_swap_grpc(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    out_amount: f64,
    out_amount_min: f64,
    slippage: f64,
) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

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
    println!("Jupiter Route signature: {:#?}", s?);

    Ok(())
}

#[test_case(
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // USDC
    "So11111111111111111111111111111111111111112",   // SOL
    0.001,                                            // Amount
    0.4;                                              // Slippage
    "Jupiter swap instructions USDC to SOL"
)]
#[tokio::test]
#[ignore]
async fn test_jupiter_swap_instructions_grpc(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

    let request = api::PostJupiterSwapInstructionsRequest {
        owner_address: client
            .public_key
            .unwrap_or_else(|| panic!("Public key is required for Jupiter swap instructions"))
            .to_string(),
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
        compute_price: 10000,
        tip: Some(10000),
    };

    let submit_opts = SubmitParams::default();
    let signatures = client
        .submit_jupiter_swap_instructions(request, submit_opts, false)
        .await?;

    println!("Jupiter swap instructions signatures: {:#?}", signatures);

    Ok(())
}

#[test_case(
    0.0001,
    10.0;
    "Pumpfun swap"
)]
#[tokio::test]
#[ignore]
async fn test_pumpfun_swap_grpc(in_amount: f64, slippage: f64) -> Result<()> {
    let bonding_curve_address = "Fh8fnZUVEpPStJ2hKFNNjMAyuyvoJLMouENawg4DYCBc";
    let mint_address = "2DEsbYgW94AtZxgUfYXoL8DqJAorsLrEWZdSfriipump";
    let mut client = GrpcClient::new(Some(MAINNET_PUMP_NY.to_string())).await?;

    let request = api::GetPumpFunQuotesRequest {
        quote_type: "buy".to_string(),
        bonding_curve_address: bonding_curve_address.to_string(),
        amount: in_amount,
        mint_address: mint_address.to_string(),
    };

    let pump_quote_response = client.get_pump_fun_quotes(&request).await?;

    let request = api::PostPumpFunSwapRequest {
        user_address: client
            .public_key
            .unwrap_or_else(|| panic!("Public key is required for pump fun swap"))
            .to_string(),
        bonding_curve_address: bonding_curve_address.to_string(),
        token_address: "2DEsbYgW94AtZxgUfYXoL8DqJAorsLrEWZdSfriipump".to_string(),
        token_amount: pump_quote_response.out_amount,
        sol_threshold: pump_quote_response.in_amount,
        compute_limit: 300000,
        compute_price: 2000,
        tip: Some(2000001),
        is_buy: true,
        slippage,
    };

    let response = client.post_pump_swap(&request).await?;
    println!(
        "pumpfun Quote: {}",
        serde_json::to_string_pretty(&response)?
    );

    let txs = response.transaction.as_slice();
    let submit_opts = SubmitParams::default();
    let s = client
        .sign_and_submit(txs.to_vec(), submit_opts, false)
        .await;
    println!("signature : {:#?}", s?);
    Ok(())
}
