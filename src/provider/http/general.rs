use crate::provider::http::HTTPClient;
use anyhow::{anyhow, Result};
use solana_trader_proto::api;
use solana_trader_proto::api::GetAccountBalanceRequest;

impl HTTPClient {
    pub async fn get_transaction(
        &self,
        request: &api::GetTransactionRequest,
    ) -> anyhow::Result<api::GetTransactionResponse> {
        let url = format!(
            "{}/api/v2/transaction?signature={}",
            self.base_url, request.signature
        );

        println!("{}", url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("HTTP GET request failed: {}", e))?;

        let response_text = response.text().await?;

        println!("{}", response_text);

        // let mut value: serde_json::Value = serde_json::from_str(&response_text)
        //     .map_err(|e| anyhow::anyhow!("Failed to parse response as JSON: {}", e))?;
        //
        // convert_string_enums(&mut value);
        //
        // serde_json::from_value(value)
        //     .map_err(|e| anyhow::anyhow!("Failed to parse response into GetTransactionResponse: {}", e))

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("HTTP GET request failed: {}", e))?;

        self.handle_response(response).await
    }
    pub async fn get_recent_block_hash(&self) -> anyhow::Result<api::GetRecentBlockHashResponse> {
        let url = format!("{}/api/v1/system/blockhash", self.base_url);

        println!("{}", url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("HTTP GET request failed: {}", e))?;

        self.handle_response(response).await
    }

    pub async fn get_recent_block_hash_v2(
        &self,
        request: &api::GetRecentBlockHashRequestV2,
    ) -> anyhow::Result<api::GetRecentBlockHashResponseV2> {
        let url = format!(
            "{}/api/v2/system/blockhash?offset={}",
            self.base_url, request.offset
        );

        println!("{}", url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("HTTP GET request failed: {}", e))?;

        self.handle_response(response).await
    }

    pub async fn get_rate_limit(&self) -> anyhow::Result<api::GetRateLimitResponse> {
        let url = format!("{}/api/v2/rate-limit", self.base_url);

        println!("{}", url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("HTTP GET request failed: {}", e))?;

        self.handle_response(response).await
    }

    pub async fn get_account_balance_v2(
        &self,
        request: GetAccountBalanceRequest,
    ) -> anyhow::Result<api::GetAccountBalanceResponse> {
        println!("here1");

        let url = format!(
            "{}/api/v2/balance?ownerAddress={}",
            self.base_url, request.owner_address
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("HTTP GET request failed: {}", e))?;

        self.handle_response(response).await
    }

    pub async fn get_priority_fee(
        &self,
        project: api::Project,
        percentile: Option<f64>,
    ) -> Result<api::GetPriorityFeeResponse> {
        let mut url = format!(
            "{}/api/v2/system/priority-fee?project={}",
            self.base_url, project as i32
        );
        if let Some(p) = percentile {
            url = format!(
                "{}/api/v2/system/priority-fee?project={}&percentile={}",
                self.base_url, project as i32, p
            );
        }

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("HTTP GET request failed: {}", e))?;

        self.handle_response(response).await
    }

    pub async fn get_priority_fee_by_program(
        &self,
        programs: Vec<String>,
    ) -> Result<api::GetPriorityFeeByProgramResponse> {
        let url = format!(
            "{}/api/v2/system/priority-fee-by-program?programs={}",
            self.base_url, programs.join("&programs=")
        );

        let response: reqwest::Response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("HTTP GET request failed: {}", e))?;

        self.handle_response(response).await
    }

    pub async fn get_token_accounts(
        &self,
        owner_address: String,
    ) -> Result<api::GetTokenAccountsResponse> {
        let url = format!(
            "{}/api/v1/account/token-accounts?ownerAddress={}",
            self.base_url, owner_address
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("HTTP GET request failed: {}", e))?;

        self.handle_response(response).await
    }

    pub async fn get_account_balance(
        &self,
        owner_address: String,
    ) -> Result<api::GetAccountBalanceResponse> {
        let url = format!(
            "{}/api/v2/balance?ownerAddress={}",
            self.base_url, owner_address
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("HTTP GET request failed: {}", e))?;

        self.handle_response(response).await
    }
}
