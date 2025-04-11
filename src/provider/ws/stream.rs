use super::WebSocketClient;
use anyhow::Result;
use solana_trader_proto::api;
use tokio_stream::Stream;

impl WebSocketClient {
    pub async fn get_pump_fun_new_amm_pool_stream(
        &self,
    ) -> Result<impl Stream<Item = Result<api::GetPumpFunNewAmmPoolStreamResponse>>> {
        let request = api::GetPumpFunNewAmmPoolStreamRequest {};

        self.conn.stream_proto("GetPumpFunNewAmmPoolStream", &request).await
    }
    
    pub async fn get_prices_stream(
        &self,
        projects: Vec<api::Project>,
        tokens: Vec<String>,
    ) -> Result<impl Stream<Item = Result<api::GetPricesStreamResponse>>> {
        let request = api::GetPricesStreamRequest {
            projects: projects.iter().map(|&p| p as i32).collect(),
            tokens,
        };

        self.conn.stream_proto("GetPricesStream", &request).await
    }

    pub async fn get_block_stream(
        &self,
    ) -> Result<impl Stream<Item = Result<api::GetBlockStreamResponse>>> {
        let request = api::GetBlockStreamRequest {};

        self.conn.stream_proto("GetBlockStream", &request).await
    }

    pub async fn get_swaps_stream(
        &self,
        projects: Vec<api::Project>,
        pools: Vec<String>,
        include_failed: bool,
    ) -> Result<impl Stream<Item = Result<api::GetSwapsStreamResponse>>> {
        let request = api::GetSwapsStreamRequest {
            projects: projects.iter().map(|&p| p as i32).collect(),
            pools,
            include_failed,
        };

        self.conn.stream_proto("GetSwapsStream", &request).await
    }

    pub async fn get_new_raydium_pools_stream(
        &self,
        include_cpmm: bool,
    ) -> Result<impl Stream<Item = Result<api::GetNewRaydiumPoolsResponse>>> {
        let request = api::GetNewRaydiumPoolsRequest {
            include_cpmm: Some(include_cpmm),
        };

        self.conn
            .stream_proto("GetNewRaydiumPoolsStream", &request)
            .await
    }

    pub async fn get_new_raydium_pools_by_transaction_stream(
        &self,
    ) -> Result<impl Stream<Item = Result<api::GetNewRaydiumPoolsByTransactionResponse>>> {
        let request = api::GetNewRaydiumPoolsByTransactionRequest {};

        self.conn
            .stream_proto("GetNewRaydiumPoolsByTransactionStream", &request)
            .await
    }

    pub async fn get_recent_block_hash_stream(
        &self,
    ) -> Result<impl Stream<Item = Result<api::GetRecentBlockHashResponse>>> {
        let request = api::GetRecentBlockHashRequest {};

        self.conn
            .stream_proto("GetRecentBlockHashStream", &request)
            .await
    }

    pub async fn get_pool_reserves_stream(
        &self,
        projects: Vec<api::Project>,
        pools: Vec<String>,
    ) -> Result<impl Stream<Item = Result<api::GetPoolReservesStreamResponse>>> {
        let request = api::GetPoolReservesStreamRequest {
            projects: projects.iter().map(|&p| p as i32).collect(),
            pools,
        };

        self.conn
            .stream_proto("GetPoolReservesStream", &request)
            .await
    }

    pub async fn get_priority_fee_stream(
        &self,
        project: api::Project,
        percentile: Option<f64>,
    ) -> Result<impl Stream<Item = Result<api::GetPriorityFeeResponse>>> {
        let request = api::GetPriorityFeeRequest {
            project: project as i32,
            percentile,
        };

        self.conn
            .stream_proto("GetPriorityFeeStream", &request)
            .await
    }

    pub async fn get_priority_fee_by_program_stream(
        &self,
        programs: Vec<String>,
    ) -> Result<impl Stream<Item = Result<api::GetPriorityFeeByProgramResponse>>> {
        let request = api::GetPriorityFeeByProgramRequest { programs };

        self.conn
            .stream_proto("GetPriorityFeeByProgramStream", &request)
            .await
    }

    pub async fn get_bundle_tip_stream(
        &self,
    ) -> Result<impl Stream<Item = Result<api::GetBundleTipResponse>>> {
        let request = api::GetBundleTipRequest {};

        self.conn.stream_proto("GetBundleTipStream", &request).await
    }

    pub async fn get_pump_fun_new_tokens_stream(
        &self,
    ) -> Result<impl Stream<Item = Result<api::GetPumpFunNewTokensStreamResponse>>> {
        let request = api::GetPumpFunNewTokensStreamRequest {};

        self.conn
            .stream_proto("GetPumpFunNewTokensStream", &request)
            .await
    }

    pub async fn get_pump_fun_swaps_stream(
        &self,
        tokens: Vec<String>,
    ) -> Result<impl Stream<Item = Result<api::GetPumpFunSwapsStreamResponse>>> {
        let request = api::GetPumpFunSwapsStreamRequest { tokens };

        self.conn
            .stream_proto("GetPumpFunSwapsStream", &request)
            .await
    }
}
