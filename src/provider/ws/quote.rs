use super::WebSocketClient;
use anyhow::Result;
use serde_json::json;
use solana_trader_proto::api;

impl WebSocketClient {
    pub async fn get_raydium_quotes(
        &self,
        request: &api::GetRaydiumQuotesRequest,
    ) -> Result<api::GetRaydiumQuotesResponse> {
        let params = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("Failed to serialize request: {}", e))?;

        self.conn.request("GetRaydiumQuotes", params).await
    }

    pub async fn get_raydium_cpmm_quotes(
        &self,
        request: &api::GetRaydiumCpmmQuotesRequest,
    ) -> Result<api::GetRaydiumCpmmQuotesResponse> {
        let params = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("Failed to serialize request: {}", e))?;

        self.conn.request("GetRaydiumCPMMQuotes", params).await
    }

    pub async fn get_raydium_clmm_quotes(
        &self,
        request: &api::GetRaydiumClmmQuotesRequest,
    ) -> Result<api::GetRaydiumClmmQuotesResponse> {
        let params = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("Failed to serialize request: {}", e))?;

        self.conn.request("GetRaydiumCLMMQuotes", params).await
    }

    pub async fn get_pump_fun_quotes(
        &self,
        request: &api::GetPumpFunQuotesRequest,
    ) -> Result<api::GetPumpFunQuotesResponse> {
        let params = serde_json::json!({
            "quoteType": request.quote_type,
            "mintAddress": request.mint_address,
            "bondingCurveAddress": request.bonding_curve_address,
            "amount": request.amount,
        });

        self.conn.request("GetPumpFunQuotes", params).await
    }

    // NOTE: Fast mode is not used as of 11/1, breaks the endpoint.
    pub async fn get_jupiter_quotes(
        &self,
        request: &api::GetJupiterQuotesRequest,
    ) -> Result<api::GetJupiterQuotesResponse> {
        let params = serde_json::json!({
            "inToken": request.in_token,
            "outToken": request.out_token,
            "inAmount": request.in_amount,
            "slippage": request.slippage,
        });

        self.conn.request("GetJupiterQuotes", params).await
    }

    pub async fn get_raydium_prices(
        &self,
        tokens: Vec<String>,
    ) -> Result<api::GetRaydiumPricesResponse> {
        let request = api::GetRaydiumPricesRequest { tokens };

        self.conn.request("GetRaydiumPrices", json!(request)).await
    }

    pub async fn get_jupiter_prices(
        &self,
        tokens: Vec<String>,
    ) -> Result<api::GetJupiterPricesResponse> {
        let request = api::GetJupiterPricesRequest { tokens };

        self.conn.request("GetJupiterPrices", json!(request)).await
    }
}
