use anyhow::Result;
use solana_trader_proto::api;
use solana_trader_proto::api::GetRecentBlockHashRequestV2;
use tonic::Request;

use super::GrpcClient;

impl GrpcClient {
    pub async fn get_transaction(
        &mut self,
        request: &api::GetTransactionRequest,
    ) -> Result<api::GetTransactionResponse> {
        let response = self
            .client
            .get_transaction(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("GetTransactionResponse error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_recent_block_hash(
        &mut self,
        request: &api::GetRecentBlockHashRequest,
    ) -> Result<api::GetRecentBlockHashResponse> {
        let response = self
            .client
            .get_recent_block_hash(Request::new(*request))
            .await
            .map_err(|e| anyhow::anyhow!("GetRecentBlockHash error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_recent_block_hash_v2(
        &mut self,
        request: GetRecentBlockHashRequestV2,
    ) -> Result<api::GetRecentBlockHashResponseV2> {
        let response = self
            .client
            .get_recent_block_hash_v2(Request::new(request))
            .await
            .map_err(|e| anyhow::anyhow!("GetRecentBlockHashV2 error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_rate_limit(
        &mut self,
        request: &api::GetRateLimitRequest,
    ) -> Result<api::GetRateLimitResponse> {
        let response = self
            .client
            .get_rate_limit(Request::new(*request))
            .await
            .map_err(|e| anyhow::anyhow!("GetRateLimit error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_account_balance_v2(
        &mut self,
        request: &api::GetAccountBalanceRequest,
    ) -> Result<api::GetAccountBalanceResponse> {
        let response = self
            .client
            .get_account_balance_v2(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("GetAccountBalanceV2 error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_priority_fee(
        &mut self,
        project: api::Project,
        percentile: Option<f64>,
    ) -> Result<api::GetPriorityFeeResponse> {
        let request = Request::new(api::GetPriorityFeeRequest {
            project: project as i32,
            percentile,
        });

        let response = self
            .client
            .get_priority_fee(request)
            .await
            .map_err(|e| anyhow::anyhow!("GetPriorityFee error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_priority_fee_by_program(
        &mut self,
        programs: Vec<String>,
    ) -> Result<api::GetPriorityFeeByProgramResponse> {
        let request = Request::new(api::GetPriorityFeeByProgramRequest {
            programs: programs
        });

        let response = self
            .client
            .get_priority_fee_by_program(request)
            .await
            .map_err(|e| anyhow::anyhow!("GetPriorityFeeByProgram error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_token_accounts(
        &mut self,
        owner_address: String,
    ) -> Result<api::GetTokenAccountsResponse> {
        let request = Request::new(api::GetTokenAccountsRequest { owner_address });

        let response = self
            .client
            .get_token_accounts(request)
            .await
            .map_err(|e| anyhow::anyhow!("GetTokenAccounts error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_account_balance(
        &mut self,
        owner_address: String,
    ) -> Result<api::GetAccountBalanceResponse> {
        let request = Request::new(api::GetAccountBalanceRequest { owner_address });

        let response = self
            .client
            .get_account_balance(request)
            .await
            .map_err(|e| anyhow::anyhow!("GetAccountBalance error: {}", e))?;

        Ok(response.into_inner())
    }
}
