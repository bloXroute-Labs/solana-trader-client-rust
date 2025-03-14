use anyhow::Result;
use solana_trader_proto::api;
use tonic::Request;

use super::GrpcClient;

impl GrpcClient {
    pub async fn get_raydium_quotes(
        &mut self,
        request: &api::GetRaydiumQuotesRequest,
    ) -> Result<api::GetRaydiumQuotesResponse> {
        let response = self
            .client
            .get_raydium_quotes(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("GetRaydiumQuotes error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_raydium_cpmm_quotes(
        &mut self,
        request: &api::GetRaydiumCpmmQuotesRequest,
    ) -> Result<api::GetRaydiumCpmmQuotesResponse> {
        let response = self
            .client
            .get_raydium_cpmm_quotes(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("GetRaydiumCPMMQuotes error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_raydium_clmm_quotes(
        &mut self,
        request: &api::GetRaydiumClmmQuotesRequest,
    ) -> Result<api::GetRaydiumClmmQuotesResponse> {
        let response = self
            .client
            .get_raydium_clmm_quotes(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("GetRaydiumCLMMQuotes error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_pump_fun_quotes(
        &mut self,
        request: &api::GetPumpFunQuotesRequest,
    ) -> Result<api::GetPumpFunQuotesResponse> {
        let response = self
            .client
            .get_pump_fun_quotes(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("GetPumpFunQuotes error: {}", e))?;

        Ok(response.into_inner())
    }

    // NOTE: Fast mode is not used as of 11/1/24, breaks the endpoint.
    pub async fn get_jupiter_quotes(
        &mut self,
        request: &api::GetJupiterQuotesRequest,
    ) -> Result<api::GetJupiterQuotesResponse> {
        let req = Request::new(request.clone());

        let response = self
            .client
            .get_jupiter_quotes(req)
            .await
            .map_err(|e| anyhow::anyhow!("GetJupiterQuotes error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_quotes(
        &mut self,
        request: &api::GetQuotesRequest,
    ) -> Result<api::GetQuotesResponse> {
        let response = self
            .client
            .get_quotes(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("GetQuotes error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_raydium_prices(
        &mut self,
        tokens: Vec<String>,
    ) -> Result<api::GetRaydiumPricesResponse> {
        let request = Request::new(api::GetRaydiumPricesRequest { tokens });

        let response = self
            .client
            .get_raydium_prices(request)
            .await
            .map_err(|e| anyhow::anyhow!("GetRaydiumPrices error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_jupiter_prices(
        &mut self,
        tokens: Vec<String>,
    ) -> Result<api::GetJupiterPricesResponse> {
        let request = Request::new(api::GetJupiterPricesRequest { tokens });

        let response = self
            .client
            .get_jupiter_prices(request)
            .await
            .map_err(|e| anyhow::anyhow!("GetJupiterPrices error: {}", e))?;

        Ok(response.into_inner())
    }
}
