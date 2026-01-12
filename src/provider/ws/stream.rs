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


    pub async fn get_block_stream(
        &self,
    ) -> Result<impl Stream<Item = Result<api::GetBlockStreamResponse>>> {
        let request = api::GetBlockStreamRequest {};

        self.conn.stream_proto("GetBlockStream", &request).await
    }
    pub async fn get_recent_block_hash_stream(
        &self,
    ) -> Result<impl Stream<Item = Result<api::GetRecentBlockHashResponse>>> {
        let request = api::GetRecentBlockHashRequest {};

        self.conn
            .stream_proto("GetRecentBlockHashStream", &request)
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

    pub async fn get_pump_fun_amm_swap_stream(
        &self,
        pools: Vec<String>,
    ) -> Result<impl Stream<Item = Result<api::GetPumpFunAmmSwapStreamResponse>>> {
        let request = api::GetPumpFunAmmSwapStreamRequest { pools };

        self.conn
            .stream_proto("GetPumpFunAMMSwapStream", &request)
            .await
    }
}
