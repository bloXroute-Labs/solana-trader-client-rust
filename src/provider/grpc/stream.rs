use anyhow::Result;
use solana_trader_proto::api;
use tonic::Request;
use tonic::Streaming;

use super::GrpcClient;

impl GrpcClient {
    pub async fn get_prices_stream(
        &mut self,
        projects: Vec<api::Project>,
        tokens: Vec<String>,
    ) -> Result<Streaming<api::GetPricesStreamResponse>> {
        let request = Request::new(api::GetPricesStreamRequest {
            projects: projects.iter().map(|&p| p as i32).collect(),
            tokens,
        });

        let response = self
            .client
            .get_prices_stream(request)
            .await
            .map_err(|e| anyhow::anyhow!("GetPricesStream error: {}", e))?;

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

    pub async fn get_orderbook_stream(
        &mut self,
        markets: Vec<String>,
        limit: u32,
        project: api::Project,
    ) -> Result<Streaming<api::GetOrderbooksStreamResponse>> {
        let request = Request::new(api::GetOrderbooksRequest {
            markets,
            limit,
            project: project as i32,
        });

        let response = self
            .client
            .get_orderbooks_stream(request)
            .await
            .map_err(|e| anyhow::anyhow!("GetOrderbooksStream error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_market_depths_stream(
        &mut self,
        markets: Vec<String>,
        limit: u32,
        project: api::Project,
    ) -> Result<Streaming<api::GetMarketDepthsStreamResponse>> {
        let request = Request::new(api::GetMarketDepthsRequest {
            markets,
            limit,
            project: project as i32,
        });

        let response = self
            .client
            .get_market_depths_stream(request)
            .await
            .map_err(|e| anyhow::anyhow!("GetMarketDepthsStream error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_ticker_stream(
        &mut self,
        markets: Vec<String>,
        project: api::Project,
    ) -> Result<Streaming<api::GetTickersStreamResponse>> {
        let request = Request::new(api::GetTickersStreamRequest {
            markets,
            project: project as i32,
        });

        let response = self
            .client
            .get_tickers_stream(request)
            .await
            .map_err(|e| anyhow::anyhow!("GetTickersStream error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_trades_stream(
        &mut self,
        market: String,
        limit: u32,
        project: api::Project,
    ) -> Result<Streaming<api::GetTradesStreamResponse>> {
        let request = Request::new(api::GetTradesRequest {
            market,
            limit,
            project: project as i32,
        });

        let response = self
            .client
            .get_trades_stream(request)
            .await
            .map_err(|e| anyhow::anyhow!("GetTradesStream error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_swaps_stream(
        &mut self,
        projects: Vec<api::Project>,
        pools: Vec<String>,
        include_failed: bool,
    ) -> Result<Streaming<api::GetSwapsStreamResponse>> {
        let request = Request::new(api::GetSwapsStreamRequest {
            projects: projects.iter().map(|&p| p as i32).collect(),
            pools,
            include_failed,
        });

        let response = self
            .client
            .get_swaps_stream(request)
            .await
            .map_err(|e| anyhow::anyhow!("GetSwapsStream error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_new_raydium_pools_stream(
        &mut self,
        include_cpmm: bool,
    ) -> Result<Streaming<api::GetNewRaydiumPoolsResponse>> {
        let request = Request::new(api::GetNewRaydiumPoolsRequest {
            include_cpmm: Some(include_cpmm),
        });

        let response = self
            .client
            .get_new_raydium_pools_stream(request)
            .await
            .map_err(|e| anyhow::anyhow!("GetNewRaydiumPoolsStream error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_new_raydium_pools_by_transaction_stream(
        &mut self,
    ) -> Result<Streaming<api::GetNewRaydiumPoolsByTransactionResponse>> {
        let request = Request::new(api::GetNewRaydiumPoolsByTransactionRequest {});

        let response = self
            .client
            .get_new_raydium_pools_by_transaction_stream(request)
            .await
            .map_err(|e| anyhow::anyhow!("GetNewRaydiumPoolsByTransactionStream error: {}", e))?;

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

    pub async fn get_pool_reserves_stream(
        &mut self,
        projects: Vec<api::Project>,
        pools: Vec<String>,
    ) -> Result<Streaming<api::GetPoolReservesStreamResponse>> {
        let request = Request::new(api::GetPoolReservesStreamRequest {
            projects: projects.iter().map(|&p| p as i32).collect(),
            pools,
        });

        let response = self
            .client
            .get_pool_reserves_stream(request)
            .await
            .map_err(|e| anyhow::anyhow!("GetPoolReservesStream error: {}", e))?;

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
