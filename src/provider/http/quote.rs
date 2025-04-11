use crate::provider::utils::convert_string_enums;

use super::HTTPClient;
use anyhow::{anyhow, Result};
use solana_trader_proto::api;

impl HTTPClient {
    pub async fn get_raydium_quotes(
        &self,
        request: &api::GetRaydiumQuotesRequest,
    ) -> Result<api::GetRaydiumQuotesResponse> {
        let url = format!(
            "{}/api/v2/raydium/quotes?inToken={}&outToken={}&inAmount={}&slippage={}",
            self.base_url, request.in_token, request.out_token, request.in_amount, request.slippage
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("HTTP GET request failed: {}", e))?;

        self.handle_response(response).await
    }

    pub async fn get_raydium_cpmm_quotes(
        &self,
        request: &api::GetRaydiumCpmmQuotesRequest,
    ) -> Result<api::GetRaydiumCpmmQuotesResponse> {
        let url = format!(
            "{}/api/v2/raydium/cpmm-quotes?inToken={}&outToken={}&inAmount={}&slippage={}",
            self.base_url, request.in_token, request.out_token, request.in_amount, request.slippage
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("HTTP GET request failed: {}", e))?;

        self.handle_response(response).await
    }

    pub async fn get_raydium_clmm_quotes(
        &self,
        request: &api::GetRaydiumClmmQuotesRequest,
    ) -> Result<api::GetRaydiumClmmQuotesResponse> {
        let url = format!(
            "{}/api/v2/raydium/clmm-quotes?inToken={}&outToken={}&inAmount={}&slippage={}",
            self.base_url, request.in_token, request.out_token, request.in_amount, request.slippage
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("HTTP GET request failed: {}", e))?;

        self.handle_response(response).await
    }

    pub async fn get_pump_fun_quotes(
        &self,
        request: &api::GetPumpFunQuotesRequest,
    ) -> Result<api::GetPumpFunQuotesResponse> {
        let url = format!(
            "{}/api/v2/pumpfun/quotes?mintAddress={}&quoteType={}&amount={}&bondingCurveAddress={}",
            self.base_url,
            request.mint_address,
            request.quote_type,
            request.amount,
            request.bonding_curve_address,
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("HTTP GET request failed: {}", e))?;

        self.handle_response(response).await
    }

    // NOTE: Fast mode is not used as of 11/1, breaks the endpoint.
    pub async fn get_jupiter_quotes(
        &self,
        request: &api::GetJupiterQuotesRequest,
    ) -> Result<api::GetJupiterQuotesResponse> {
        let url = format!(
            "{}/api/v2/jupiter/quotes?inToken={}&outToken={}&inAmount={}&slippage={}",
            self.base_url, request.in_token, request.out_token, request.in_amount, request.slippage,
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("HTTP GET request failed: {}", e))?;

        self.handle_response(response).await
    }

    pub async fn get_raydium_prices(
        &self,
        tokens: Vec<String>,
    ) -> Result<api::GetRaydiumPricesResponse> {
        let mut url = format!("{}/api/v2/raydium/prices?", self.base_url);
        for (i, token) in tokens.iter().enumerate() {
            if i > 0 {
                url.push('&');
            }
            url.push_str(&format!("tokens={}", token));
        }

        let response = self.client.get(&url).send().await?;
        self.handle_response(response).await
    }

    pub async fn get_jupiter_prices(
        &self,
        tokens: Vec<String>,
    ) -> Result<api::GetJupiterPricesResponse> {
        let mut url = format!("{}/api/v2/jupiter/prices?", self.base_url);
        for (i, token) in tokens.iter().enumerate() {
            if i > 0 {
                url.push('&');
            }
            url.push_str(&format!("tokens={}", token));
        }

        let response = self.client.get(&url).send().await?;
        self.handle_response(response).await
    }
}
