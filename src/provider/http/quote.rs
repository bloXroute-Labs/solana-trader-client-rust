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

    pub async fn get_raydium_clmm_pools(
        &self,
        pair_or_address: String
    ) -> anyhow::Result<api::GetRaydiumClmmPoolsResponse> {
        let url = format!("{}/api/v2/raydium/clmm-pools?pairOrAddress={}", self.base_url, pair_or_address);
        println!("{}", url);
        
        let response = self
            .client
            .get(&url)
            .send()
            .await?;
            
        let result: api::GetRaydiumClmmPoolsResponse = self.handle_response(response).await?;
        
        Ok(result)
    }

    pub async fn get_raydium_pools(
        &self
    ) -> anyhow::Result<api::GetRaydiumPoolsResponse> {
        let url = format!("{}/api/v2/raydium/pools", self.base_url);
        println!("{}", url);
        
        let response = self
            .client
            .get(&url)
            .send()
            .await?;
            
        let result: api::GetRaydiumPoolsResponse = self.handle_response(response).await?;
        
        Ok(result)
    }

    pub async fn get_raydium_pool_reserve(
        &self,
        pairs_or_addresses: Vec<String>
    ) -> anyhow::Result<api::GetRaydiumPoolReserveResponse> {
        let pairs_query = pairs_or_addresses.join(",");
        let url = format!("{}/api/v2/raydium/pool-reserves?pairsOrAddresses={}", self.base_url, pairs_query);
        println!("{}", url);
        
        let response = self
            .client
            .get(&url)
            .send()
            .await?;
            
        let result: api::GetRaydiumPoolReserveResponse = self.handle_response(response).await?;
        
        Ok(result)
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

    pub async fn get_quotes(
        &self,
        in_token: &str,
        out_token: &str,
        in_amount: f64,
        slippage: f64,
        limit: i32,
        projects: &[api::Project],
    ) -> Result<api::GetQuotesResponse> {
        let project_params: Vec<String> = projects
            .iter()
            .map(|p| format!("&project={}", *p as i32))
            .collect();

        let url = format!(
            "{}/api/v1/market/quote?inToken={}&outToken={}&inAmount={}&slippage={}&limit={}{}",
            self.base_url,
            in_token,
            out_token,
            in_amount,
            slippage,
            limit,
            project_params.join("")
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("HTTP GET request failed: {}", e))?;

        let response_text = response.text().await?;

        let mut value: serde_json::Value = serde_json::from_str(&response_text)
            .map_err(|e| anyhow::anyhow!("Failed to parse response as JSON: {}", e))?;

        convert_string_enums(&mut value);

        serde_json::from_value(value)
            .map_err(|e| anyhow::anyhow!("Failed to parse response into GetQuotesResponse: {}", e))
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
