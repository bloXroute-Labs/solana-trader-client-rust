use anyhow::Result;
use solana_trader_proto::api;
use tonic::Request;
use tonic::Streaming;

use super::GrpcClient;

impl GrpcClient {
    pub async fn get_pump_fun_new_amm_pool_stream(
        &mut self,
    ) -> Result<Streaming<api::GetPumpFunNewAmmPoolStreamResponse>> {
        let request = Request::new(api::GetPumpFunNewAmmPoolStreamRequest {});

        let response = self
            .client
            .get_pump_fun_new_amm_pool_stream(request)
            .await
            .map_err(|e| anyhow::anyhow!("GetPumpFunNewAmmPoolStream error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_block_stream(&mut self) -> Result<Streaming<api::GetBlockStreamResponse>> {
        let request = Request::new(api::GetBlockStreamRequest {});

        let response = self
            .client
            .get_block_stream(request)
            .await
            .map_err(|e| anyhow::anyhow!("GetBlockStream error: {}", e))?;

        Ok(response.into_inner())
    }



    pub async fn get_recent_block_hash_stream(
        &mut self,
    ) -> Result<Streaming<api::GetRecentBlockHashResponse>> {
        let request = Request::new(api::GetRecentBlockHashRequest {});

        let response = self
            .client
            .get_recent_block_hash_stream(request)
            .await
            .map_err(|e| anyhow::anyhow!("GetRecentBlockHashStream error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_priority_fee_stream(
        &mut self,
        project: api::Project,
        percentile: Option<f64>,
    ) -> Result<Streaming<api::GetPriorityFeeResponse>> {
        let request = Request::new(api::GetPriorityFeeRequest {
            project: project as i32,
            percentile,
        });

        let response = self
            .client
            .get_priority_fee_stream(request)
            .await
            .map_err(|e| anyhow::anyhow!("GetPriorityFeeStream error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_bundle_tip_stream(&mut self) -> Result<Streaming<api::GetBundleTipResponse>> {
        let request = Request::new(api::GetBundleTipRequest {});

        let response = self
            .client
            .get_bundle_tip_stream(request)
            .await
            .map_err(|e| anyhow::anyhow!("GetBundleTipStream error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_pump_fun_new_tokens_stream(
        &mut self,
    ) -> Result<Streaming<api::GetPumpFunNewTokensStreamResponse>> {
        let request = Request::new(api::GetPumpFunNewTokensStreamRequest {});

        let response = self
            .client
            .get_pump_fun_new_tokens_stream(request)
            .await
            .map_err(|e| anyhow::anyhow!("GetPumpFunNewTokensStream error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_pump_fun_swaps_stream(
        &mut self,
        tokens: Vec<String>,
    ) -> Result<Streaming<api::GetPumpFunSwapsStreamResponse>> {
        let request = Request::new(api::GetPumpFunSwapsStreamRequest { tokens });

        let response = self
            .client
            .get_pump_fun_swaps_stream(request)
            .await
            .map_err(|e| anyhow::anyhow!("GetPumpFunSwapsStream error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_pump_fun_amm_swap_stream(
        &mut self,
        pools: Vec<String>,
    ) -> Result<Streaming<api::GetPumpFunAmmSwapStreamResponse>> {
        let request = Request::new(api::GetPumpFunAmmSwapStreamRequest { pools });

        let response = self
            .client
            .get_pump_fun_amm_swap_stream(request)
            .await
            .map_err(|e| anyhow::anyhow!("GetPumpFunAmmSwapStream error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_priority_fee_by_program_stream(
        &mut self,
        projects: Vec<String>,
    ) -> Result<Streaming<api::GetPriorityFeeByProgramResponse>> {
        let request = Request::new(api::GetPriorityFeeByProgramRequest { programs: projects });

        let response = self
            .client
            .get_priority_fee_by_program_stream(request)
            .await
            .map_err(|e| anyhow::anyhow!("GetPriorityFeeByProjectStream error: {}", e))?;

        Ok(response.into_inner())
    }
}
