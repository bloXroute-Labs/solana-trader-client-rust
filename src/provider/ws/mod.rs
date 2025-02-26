pub mod quote;
pub mod stream;
pub mod swap;

use anyhow::{anyhow, Result};
use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_trader_proto::api::{self, GetRecentBlockHashResponseV2};

use crate::common::signing::{sign_transaction, SubmitParams};
use crate::common::{get_base_url_from_env, is_submit_only_endpoint, ws_endpoint, BaseConfig};
use crate::connections::ws::WS;

use super::utils::IntoTransactionMessage;

pub struct WebSocketConfig {
    pub endpoint: String,
    pub private_key: Option<Keypair>,
    pub auth_header: String,
    pub use_tls: bool,
    pub disable_auth: bool,
}

pub struct WebSocketClient {
    conn: WS,
    keypair: Option<Keypair>,
    pub public_key: Option<Pubkey>,
}

impl WebSocketClient {
    pub fn get_keypair(&self) -> Result<&Keypair> {
        Ok(self.keypair.as_ref().unwrap())
    }

    pub async fn new(endpoint: Option<String>) -> Result<Self> {
        let base = BaseConfig::try_from_env()?;
        let (default_base_url, secure) = get_base_url_from_env();
        let final_base_url = endpoint.unwrap_or(default_base_url);
        let endpoint = ws_endpoint(&final_base_url, secure);

        is_submit_only_endpoint(&final_base_url);

        if base.auth_header.is_empty() {
            return Err(anyhow::anyhow!("AUTH_HEADER is empty"));
        }

        let conn = WS::new(Some(endpoint))
            .await
            .map_err(|e| anyhow::anyhow!("Connection timeout: {}", e))?;

        Ok(Self {
            conn,
            keypair: base.keypair,
            public_key: base.public_key,
        })
    }

    pub async fn close(self) -> Result<()> {
        self.conn.close().await
    }

    pub async fn sign_and_submit<T: IntoTransactionMessage + Clone>(
        &self,
        txs: Vec<T>,
        submit_opts: SubmitParams,
        use_bundle: bool,
    ) -> Result<Vec<String>> {
        let keypair = self.get_keypair()?;

        let hash_res: GetRecentBlockHashResponseV2 =
            self.conn.request("GetRecentBlockHashV2", json!({})).await?;

        if txs.len() == 1 {
            let signed_tx = sign_transaction(&txs[0], keypair, hash_res.block_hash).await?;

            let request = json!({
                "transaction": {
                    "content": signed_tx.content,
                    "isCleanup": signed_tx.is_cleanup
                },
                "skipPreFlight": submit_opts.skip_pre_flight,
                "frontRunningProtection": submit_opts.front_running_protection,
                "useStakedRPCs": submit_opts.use_staked_rpcs,
                "fastBestEffort": submit_opts.fast_best_effort
            });

            let response: serde_json::Value = self.conn.request("PostSubmitV2", request).await?;

            return Ok(vec![response
                .get("signature")
                .and_then(|s| s.as_str())
                .map(String::from)
                .ok_or_else(|| anyhow!("Missing signature in response"))?]);
        }

        let mut entries = Vec::with_capacity(txs.len());
        for tx in txs {
            let signed_tx = sign_transaction(&tx, keypair, hash_res.block_hash.clone()).await?;
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

        let request = json!({
            "entries": entries,
            "useBundle": use_bundle,
            "submitStrategy": submit_opts.submit_strategy
        });

        let response: serde_json::Value = self.conn.request("PostSubmitBatchV2", request).await?;

        let signatures = response["transactions"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid response format"))?
            .iter()
            .filter(|entry| entry["submitted"].as_bool().unwrap_or(false))
            .filter_map(|entry| entry["signature"].as_str().map(String::from))
            .collect();

        Ok(signatures)
    }

    pub async fn sign_and_submit_snipe<T: IntoTransactionMessage + Clone>(
        &self,
        txs: Vec<T>,
        use_staked_rpcs: bool,
    ) -> Result<Vec<String>> {
        let keypair = self.get_keypair()?;

        let hash_res: GetRecentBlockHashResponseV2 =
            self.conn.request("GetRecentBlockHashV2", json!({})).await?;

        // Build entries for each transaction
        let mut entries = Vec::with_capacity(txs.len());
        for tx in txs {
            let signed_tx = sign_transaction(&tx, keypair, hash_res.block_hash.clone()).await?;
            entries.push(json!({
                "transaction": {
                    "content": signed_tx.content,
                    "isCleanup": signed_tx.is_cleanup
                },
                "skipPreFlight": false
            }));
        }

        let request = json!({
            "entries": entries,
            "useStakedRPCs": use_staked_rpcs
        });

        let response: serde_json::Value = self.conn.request("PostSubmitSnipeV2", request).await?;

        let signatures = response["transactions"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid response format"))?
            .iter()
            .filter(|entry| entry["submitted"].as_bool().unwrap_or(false))
            .filter_map(|entry| entry["signature"].as_str().map(String::from))
            .collect();

        Ok(signatures)
    }

    pub async fn sign_and_submit_paladin<T: IntoTransactionMessage + Clone>(
        &self,
        tx: T,
    ) -> Result<String> {
        let hash_res: GetRecentBlockHashResponseV2 =
            self.conn.request("GetRecentBlockHashV2", json!({})).await?;

        let keypair = self.get_keypair()?;
        let signed_tx = sign_transaction(&tx, keypair, hash_res.block_hash).await?;

        let request = json!({
            "transaction": {
                "content": signed_tx.content,
                "isCleanup": signed_tx.is_cleanup
            }
        });

        let response: serde_json::Value = self.conn.request("PostSubmitPaladinV2", request).await?;

        let signature = response
            .get("signature")
            .and_then(|s| s.as_str())
            .map(String::from)
            .ok_or_else(|| anyhow!("Missing signature in response"))?;

        Ok(signature)
    }

    pub async fn get_transaction(
        &self,
        request: api::GetTransactionRequest,
    ) -> anyhow::Result<api::GetTransactionResponse> {
        let params = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("Failed to serialize request: {}", e))?;

        self.conn.request("GetTransaction", params).await
    }

    pub async fn get_recent_block_hash(
        &self,
        request: api::GetRecentBlockHashRequest,
    ) -> anyhow::Result<api::GetRecentBlockHashResponse> {
        let params = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("Failed to serialize request: {}", e))?;

        self.conn.request("GetRecentBlockHash", params).await
    }

    pub async fn get_recent_block_hash_v2(
        &self,
        request: &api::GetRecentBlockHashRequestV2,
    ) -> anyhow::Result<api::GetRecentBlockHashResponseV2> {
        let params = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("Failed to serialize request: {}", e))?;

        self.conn.request("GetRecentBlockHashV2", params).await
    }
    pub async fn get_rate_limit(
        &self,
        request: api::GetRateLimitRequest,
    ) -> anyhow::Result<api::GetRateLimitResponse> {
        let params = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("Failed to serialize request: {}", e))?;

        self.conn.request("GetRateLimit", params).await
    }

    pub async fn get_account_balance_v2(
        &self,
        request: api::GetAccountBalanceRequest,
    ) -> anyhow::Result<api::GetAccountBalanceResponse> {
        let params = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("Failed to serialize request: {}", e))?;

        self.conn.request("GetAccountBalanceV2", params).await
    }

    pub async fn get_priority_fee(
        &self,
        project: api::Project,
        percentile: Option<f64>,
    ) -> Result<api::GetPriorityFeeResponse> {
        let request = api::GetPriorityFeeRequest {
            project: project as i32,
            percentile,
        };

        let params = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("Failed to serialize request: {}", e))?;

        self.conn.request("GetPriorityFee", params).await
    }

    pub async fn get_priority_fee_by_program(
        &self,
        programs: Vec<String>,
    ) -> Result<api::GetPriorityFeeByProgramResponse> {
        let request = api::GetPriorityFeeByProgramRequest { programs };

        let params = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("Failed to serialize request: {}", e))?;

        self.conn.request("GetPriorityFeeByProgram", params).await
    }

    pub async fn get_token_accounts(
        &self,
        owner_address: String,
    ) -> Result<api::GetTokenAccountsResponse> {
        let request = api::GetTokenAccountsRequest { owner_address };

        let params = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("Failed to serialize request: {}", e))?;

        self.conn.request("GetTokenAccounts", params).await
    }

    pub async fn get_account_balance(
        &self,
        owner_address: String,
    ) -> Result<api::GetAccountBalanceResponse> {
        let request = api::GetAccountBalanceRequest { owner_address };

        let params = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("Failed to serialize request: {}", e))?;

        self.conn.request("GetAccountBalance", params).await
    }

    pub async fn get_leader_schedule(
        &self,
        max_slots: u64,
    ) -> Result<api::GetLeaderScheduleResponse> {
        let request = api::GetLeaderScheduleRequest { max_slots };

        let params = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("Failed to serialize request: {}", e))?;

        self.conn.request("GetLeaderSchedule", params).await
    }
}
