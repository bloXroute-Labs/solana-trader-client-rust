use crate::provider::ws::WebSocketClient;
use anyhow::Result;
use solana_trader_proto::api;
use solana_trader_proto::api::{
    GetAccountBalanceRequest, GetRateLimitRequest, GetRecentBlockHashRequest,
    GetRecentBlockHashRequestV2, GetTransactionRequest,
};

impl WebSocketClient {
    pub async fn get_transaction(
        &self,
        request: GetTransactionRequest,
    ) -> anyhow::Result<api::GetTransactionResponse> {
        let params = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("Failed to serialize request: {}", e))?;

        self.conn.request("GetTransaction", params).await
    }

    pub async fn get_recent_block_hash(
        &self,
        request: GetRecentBlockHashRequest,
    ) -> anyhow::Result<api::GetRecentBlockHashResponse> {
        let params = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("Failed to serialize request: {}", e))?;

        self.conn.request("GetRecentBlockHash", params).await
    }

    pub async fn get_recent_block_hash_v2(
        &self,
        request: &GetRecentBlockHashRequestV2,
    ) -> anyhow::Result<api::GetRecentBlockHashResponseV2> {
        let params = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("Failed to serialize request: {}", e))?;

        self.conn.request("GetRecentBlockHashV2", params).await
    }
    pub async fn get_rate_limit(
        &self,
        request: GetRateLimitRequest,
    ) -> anyhow::Result<api::GetRateLimitResponse> {
        let params = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("Failed to serialize request: {}", e))?;

        self.conn.request("GetRateLimit", params).await
    }

    pub async fn get_account_balance_v2(
        &self,
        request: GetAccountBalanceRequest,
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
        let request = api::GetPriorityFeeByProgramRequest {
            programs: programs
        };

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
}
