pub mod quote;
pub mod swap;

use anyhow::{anyhow, Result};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::de::DeserializeOwned;
use serde_json::json;
use crate::common::constants::WARNING_TLS_SLOWDOWN;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use solana_trader_proto::api::{self, PostSubmitPaladinRequest, GetRecentBlockHashResponseV2};
use crate::provider::utils::timestamp_rfc3339;

use crate::{
    common::{
        get_base_url_from_env, http_endpoint, is_submit_only_endpoint,
        signing::{sign_transaction, SubmitParams},
        BaseConfig,
    },
    provider::utils::convert_string_enums,
};

use super::utils::IntoTransactionMessage;

pub struct HTTPClient {
    client: Client,
    base_url: String,
    keypair: Option<Keypair>,
    pub public_key: Option<Pubkey>,
}

impl HTTPClient {
    pub fn get_keypair(&self) -> Result<&Keypair> {
        Ok(self.keypair.as_ref().unwrap())
    }

    pub fn new(endpoint: Option<String>) -> Result<Self> {
        let base = BaseConfig::try_from_env()?;
        let (default_base_url, secure) = get_base_url_from_env();
        let final_base_url = endpoint.unwrap_or(default_base_url);
        let endpoint = http_endpoint(&final_base_url, secure);
        if endpoint.starts_with("https://") {
            println!("{}", WARNING_TLS_SLOWDOWN);
        }

        is_submit_only_endpoint(&final_base_url);

        let headers = Self::build_headers(&base.auth_header)?;
        let client = Client::builder()
            .default_headers(headers)
            .pool_idle_timeout(None)
            .pool_max_idle_per_host(200)
            .tcp_keepalive(Some(std::time::Duration::from_secs(15)))
            .build()
            .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))?;

        Ok(Self {
            client,
            base_url: endpoint,
            keypair: base.keypair,
            public_key: base.public_key,
        })
    }

    fn build_headers(auth_header: &str) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(auth_header)
                .map_err(|e| anyhow!("Invalid auth header: {}", e))?,
        );
        headers.insert("x-sdk", HeaderValue::from_static("rust-client"));
        headers.insert(
            "x-sdk-version",
            HeaderValue::from_static(env!("CARGO_PKG_VERSION")),
        );
        Ok(headers)
    }

    async fn handle_response<T: DeserializeOwned>(&self, response: reqwest::Response) -> Result<T> {
        println!("Response: {:?}", response);
        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error response".into());
            return Err(anyhow::anyhow!("HTTP request failed: {}", error_text));
        }

        let res = response.text().await?;

        let mut value = serde_json::from_str(&res)
            .map_err(|e| anyhow::anyhow!("Failed to parse response as JSON: {}", e))?;

        convert_string_enums(&mut value);

        serde_json::from_value(value)
            .map_err(|e| anyhow::anyhow!("Failed to parse response into desired type: {}", e))
    }

    pub async fn sign_and_submit<T: IntoTransactionMessage + Clone>(
        &self,
        txs: Vec<T>,
        submit_opts: SubmitParams,
        use_bundle: bool,
    ) -> Result<Vec<String>> {
        let keypair = self.get_keypair()?;

        // TODO: refactor once this endpoint is defined
        let response = self
            .client
            .get(format!(
                "{}/api/v2/system/blockhash?offset={}",
                self.base_url, 0
            ))
            .send()
            .await?;

        let res: GetRecentBlockHashResponseV2 = self.handle_response(response).await?;

        if txs.len() == 1 {
            let signed_tx = sign_transaction(&txs[0], keypair, res.block_hash).await?;

            let request_json = json!({
                "transaction": { "content": signed_tx.content, "isCleanup": signed_tx.is_cleanup },
                "skipPreFlight": submit_opts.skip_pre_flight,
                "frontRunningProtection": submit_opts.front_running_protection,
                "useStakedRPCs": submit_opts.use_staked_rpcs,
                "fastBestEffort": submit_opts.fast_best_effort
            });

            let response = self
                .client
                .post(format!("{}/api/v2/submit", self.base_url))
                .json(&request_json)
                .send()
                .await?;

            let result: serde_json::Value = self.handle_response(response).await?;
            return Ok(vec![result
                .get("signature")
                .and_then(|s| s.as_str())
                .map(String::from)
                .ok_or_else(|| anyhow!("Missing signature in response"))?]);
        }

        let mut entries = Vec::with_capacity(txs.len());
        for tx in txs {
            let signed_tx = sign_transaction(&tx, keypair, res.block_hash.clone()).await?;
            entries.push(json!({
                "transaction": {
                    "content": signed_tx.content,
                    "isCleanup": signed_tx.is_cleanup
                },
                "skipPreFlight": submit_opts.skip_pre_flight,
                "frontRunningProtection": submit_opts.front_running_protection,
                "useStakedRPCs": submit_opts.use_staked_rpcs,
                "fastBestEffort": submit_opts.fast_best_effort
            }));
        }

        let request_json = json!({
            "entries": entries,
            "useBundle": use_bundle,
            "submitStrategy": submit_opts.submit_strategy
        });

        let response = self
            .client
            .post(format!("{}/api/v2/submit/batch", self.base_url))
            .json(&request_json)
            .send()
            .await?;

        let result: serde_json::Value = self.handle_response(response).await?;

        let signatures = result["transactions"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid response format"))?
            .iter()
            .filter(|entry| entry["submitted"].as_bool().unwrap_or(false))
            .filter_map(|entry| entry["signature"].as_str().map(String::from))
            .collect();

        Ok(signatures)
    }

    pub async fn post_submit_batch(
        &self,
        entries: Vec<api::PostSubmitRequestEntry>,
        submit_strategy: api::SubmitStrategy,
        use_bundle: Option<bool>,
        front_running_protection: Option<bool>
    ) -> anyhow::Result<api::PostSubmitBatchResponse> {
        let url = format!("{}/api/v1/trade/submit-batch", self.base_url);
        println!("{}", url);
        
        let request_json = json!({
            "entries": entries,
            "submitStrategy": submit_strategy,
            "useBundle": use_bundle,
            "frontRunningProtection": front_running_protection,
            "timestamp": timestamp_rfc3339()
        });
        
        let response = self
            .client
            .post(&url)
            .json(&request_json)
            .send()
            .await?;
            
        let result: api::PostSubmitBatchResponse = self.handle_response(response).await?;
        
        Ok(result)
    }

    pub async fn post_submit_batch_v2(
        &self,
        entries: Vec<api::PostSubmitRequestEntry>,
        submit_strategy: api::SubmitStrategy,
        use_bundle: Option<bool>,
        front_running_protection: Option<bool>
    ) -> anyhow::Result<api::PostSubmitBatchResponse> {
        let url = format!("{}/api/v2/submit-batch", self.base_url);
        println!("{}", url);
        
        let request_json = json!({
            "entries": entries,
            "submitStrategy": submit_strategy,
            "useBundle": use_bundle,
            "frontRunningProtection": front_running_protection,
            "timestamp": timestamp_rfc3339()
        });
        
        let response = self
            .client
            .post(&url)
            .json(&request_json)
            .send()
            .await?;
            
        let result: api::PostSubmitBatchResponse = self.handle_response(response).await?;
        
        Ok(result)
    }

    pub async fn post_submit_paladin_v2(
        &self,
        request: &PostSubmitPaladinRequest
    ) -> anyhow::Result<api::PostSubmitResponse> {
        let url = format!("{}/api/v2/submit-paladin", self.base_url);
        println!("{}", url);
        
        let request_json = json!({
            "transaction": request.transaction,
            "revertProtection": request.revert_protection,
            "timestamp": timestamp_rfc3339()
        });
        
        let response = self
            .client
            .post(&url)
            .json(&request_json)
            .send()
            .await?;
            
        let result: api::PostSubmitResponse = self.handle_response(response).await?;
        
        Ok(result)
    }

    pub async fn sign_and_submit_snipe<T: IntoTransactionMessage + Clone>(
        &self,
        txs: Vec<T>,
        use_staked_rpcs: bool,
    ) -> Result<Vec<String>> {
        let keypair = self.get_keypair()?;

        // Get recent blockhash
        let response = self
            .client
            .get(format!(
                "{}/api/v2/system/blockhash?offset={}",
                self.base_url, 0
            ))
            .send()
            .await?;

        let res: GetRecentBlockHashResponseV2 = self.handle_response(response).await?;

        // Build entries for each transaction
        let mut entries = Vec::with_capacity(txs.len());
        for tx in txs {
            let signed_tx = sign_transaction(&tx, keypair, res.block_hash.clone()).await?;
            entries.push(json!({
                "transaction": {
                    "content": signed_tx.content,
                    "isCleanup": signed_tx.is_cleanup
                },
                "skipPreFlight": false
            }));
        }

        let request_json = json!({
            "entries": entries,
            "useStakedRPCs": use_staked_rpcs,
            "timestamp": timestamp_rfc3339()
        });

        let response = self
            .client
            .post(format!("{}/api/v2/submit-snipe", self.base_url))
            .json(&request_json)
            .send()
            .await?;

        let result: serde_json::Value = self.handle_response(response).await?;

        let signatures = result["transactions"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid response format"))?
            .iter()
            .filter(|entry| entry["submitted"].as_bool().unwrap_or(false))
            .filter_map(|entry| entry["signature"].as_str().map(String::from))
            .collect();

        Ok(signatures)
    }

    pub async fn post_submit_snipe_v2(
        &self,
        entries: Vec<api::PostSubmitRequestEntry>,
        use_staked_rpcs: Option<bool>
    ) -> anyhow::Result<api::PostSubmitSnipeResponse> {
        let url = format!("{}/api/v2/submit-snipe", self.base_url);
        println!("{}", url);
        
        let request_json = json!({
            "entries": entries,
            "useStakedRPCs": use_staked_rpcs,
            "timestamp": timestamp_rfc3339()
        });
        
        let response = self
            .client
            .post(&url)
            .json(&request_json)
            .send()
            .await?;
            
        let result: api::PostSubmitSnipeResponse = self.handle_response(response).await?;
        
        Ok(result)
    }
    
    pub async fn post_submit_v2(
        &self,
        transaction: api::TransactionMessage,
        skip_pre_flight: bool,
        front_running_protection: Option<bool>,
        tip: Option<u64>,
        use_staked_rpcs: Option<bool>,
        fast_best_effort: Option<bool>,
        allow_back_run: Option<bool>,
        revenue_address: Option<String>,
        sniping: Option<bool>
    ) -> anyhow::Result<api::PostSubmitResponse> {
        let url = format!("{}/api/v2/submit", self.base_url);
        println!("{}", url);
        
        let request_json = json!({
            "transaction": transaction,
            "skipPreFlight": skip_pre_flight,
            "frontRunningProtection": front_running_protection,
            "tip": tip,
            "useStakedRPCs": use_staked_rpcs,
            "fastBestEffort": fast_best_effort,
            "allowBackRun": allow_back_run,
            "revenueAddress": revenue_address,
            "sniping": sniping,
            "timestamp": timestamp_rfc3339()
        });
        
        let response = self
            .client
            .post(&url)
            .json(&request_json)
            .send()
            .await?;
            
        let result: api::PostSubmitResponse = self.handle_response(response).await?;
        
        Ok(result)
    }

    pub async fn sign_and_submit_paladin<T: IntoTransactionMessage + Clone>(
        &self,
        tx: T,
        revert_protection: bool,
    ) -> Result<String> {
        let response = self
            .client
            .get(format!(
                "{}/api/v2/system/blockhash?offset={}",
                self.base_url, 0
            ))
            .send()
            .await?;

        let res: GetRecentBlockHashResponseV2 = self.handle_response(response).await?;
        let keypair = self.get_keypair()?;
        let signed_tx = sign_transaction(&tx, keypair, res.block_hash).await?;

        let request_json = json!({
            "transaction": {
                "content": signed_tx.content
            },
            "revertProtection": revert_protection
        });

        let response = self
            .client
            .post(format!("{}/api/v2/submit-paladin", self.base_url))
            .json(&request_json)
            .send()
            .await?;

        let result: serde_json::Value = self.handle_response(response).await?;
        let signature = result
            .get("signature")
            .and_then(|s| s.as_str())
            .map(String::from)
            .ok_or_else(|| anyhow!("Missing signature in response"))?;

        Ok(signature)
    }

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

    pub async fn get_server_time(&self) -> anyhow::Result<api::GetServerTimeResponse> {
        let url = format!("{}/api/v1/system/time", self.base_url);

        println!("{}", url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("HTTP GET request failed: {}", e))?;

        self.handle_response(response).await
    }

    pub async fn post_submit(
        &self, 
        transaction: api::TransactionMessage,
        skip_pre_flight: bool,
        front_running_protection: Option<bool>,
        tip: Option<u64>,
        use_staked_rpcs: Option<bool>,
        fast_best_effort: Option<bool>,
        allow_back_run: Option<bool>,
        revenue_address: Option<String>,
        sniping: Option<bool>
    ) -> anyhow::Result<api::PostSubmitResponse> {
        let url = format!("{}/api/v1/trade/submit", self.base_url);
        println!("{}", url);
        
        let request_json = json!({
            "transaction": transaction,
            "skipPreFlight": skip_pre_flight,
            "frontRunningProtection": front_running_protection,
            "tip": tip,
            "useStakedRPCs": use_staked_rpcs,
            "fastBestEffort": fast_best_effort,
            "allowBackRun": allow_back_run,
            "revenueAddress": revenue_address,
            "sniping": sniping,
            "timestamp": timestamp_rfc3339()
        });
        
        let response = self
            .client
            .post(format!("{}/api/v1/trade/submit", self.base_url))
            .json(&request_json)
            .send()
            .await?;
            
        let result: api::PostSubmitResponse = self.handle_response(response).await?;
        
        Ok(result)
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
        request: api::GetAccountBalanceRequest,
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
            self.base_url,
            programs.join("&programs=")
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
