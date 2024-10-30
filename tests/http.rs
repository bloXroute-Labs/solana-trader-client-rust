#[cfg(test)]
mod tests {
    use solana_trader_client_rust::provider::http::HTTPClient;
    use solana_trader_proto::api;
    use std::error::Error;
    use test_case::test_case;

    const ENDPOINT: &str = "https://ny.solana.dex.blxrbdn.com/api/v2/raydium";

    #[test_case(
        "SOL",
        "USDC",
        1.0,
        0.1 ;
        "SOL to USDC quote via HTTP"
    )]
    #[tokio::test]
    #[ignore]
    async fn test_raydium_quotes_http(
        in_token: &str,
        out_token: &str,
        in_amount: f64,
        slippage: f64,
    ) -> Result<(), Box<dyn Error>> {
        let client = HTTPClient::new(ENDPOINT.to_string())?;

        let request = api::GetRaydiumQuotesRequest {
            in_token: in_token.to_string(),
            out_token: out_token.to_string(),
            in_amount,
            slippage,
        };

        let response = client.get_raydium_quotes(&request).await?;
        assert!(
            !response.routes.is_empty(),
            "Expected at least one route in response"
        );

        Ok(())
    }
}