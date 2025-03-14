use anyhow::Result;
use base64::{engine::general_purpose, Engine};
use solana_sdk::{
    message::{v0, VersionedMessage},
    transaction::VersionedTransaction,
};
use solana_trader_proto::api;
use tonic::Request;

use crate::{
    common::signing::SubmitParams,
    provider::utils::{
        convert_address_lookup_table, convert_jupiter_instructions, convert_raydium_instructions,
        create_transaction_message,
    },
};

use super::GrpcClient;

impl GrpcClient {
    pub async fn post_raydium_swap(
        &mut self,
        request: &api::PostRaydiumSwapRequest,
    ) -> Result<api::PostRaydiumSwapResponse> {
        let response = self
            .client
            .post_raydium_swap(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("PostRaydiumSwap error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn post_raydium_route_swap(
        &mut self,
        request: &api::PostRaydiumRouteSwapRequest,
    ) -> Result<api::PostRaydiumRouteSwapResponse> {
        let response = self
            .client
            .post_raydium_route_swap(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("PostRaydiumRouteSwap error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn post_raydium_swap_instructions(
        &mut self,
        request: &api::PostRaydiumSwapInstructionsRequest,
    ) -> Result<api::PostRaydiumSwapInstructionsResponse> {
        Ok(self
            .client
            .post_raydium_swap_instructions(request.clone())
            .await?
            .into_inner())
    }

    pub async fn submit_raydium_swap_instructions(
        &mut self,
        request: api::PostRaydiumSwapInstructionsRequest,
        submit_opts: SubmitParams,
        use_bundle: bool,
    ) -> Result<Vec<String>> {
        let swap_instructions = self.post_raydium_swap_instructions(&request).await?;

        let instructions = convert_raydium_instructions(&swap_instructions.instructions)?;

        let block_hash = self
            .client
            .get_recent_block_hash_v2(api::GetRecentBlockHashRequestV2 { offset: 0 })
            .await?
            .into_inner()
            .block_hash;

        let tx_message = create_transaction_message(instructions, &block_hash)?;

        self.sign_and_submit(vec![tx_message], submit_opts, use_bundle)
            .await
    }

    pub async fn post_raydium_cpmm_swap(
        &mut self,
        request: &api::PostRaydiumCpmmSwapRequest,
    ) -> Result<api::PostRaydiumCpmmSwapResponse> {
        let response = self
            .client
            .post_raydium_cpmm_swap(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("PostRaydiumCPMMSwap error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn post_raydium_clmm_swap(
        &mut self,
        request: &api::PostRaydiumSwapRequest,
    ) -> Result<api::PostRaydiumSwapResponse> {
        let response = self
            .client
            .post_raydium_clmm_swap(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("PostRaydiumCLMMSwap error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn post_raydium_clmm_route_swap(
        &mut self,
        request: &api::PostRaydiumRouteSwapRequest,
    ) -> Result<api::PostRaydiumRouteSwapResponse> {
        let response = self
            .client
            .post_raydium_clmm_route_swap(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("PostRaydiumCLMMRouteSwap error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn post_jupiter_swap(
        &mut self,
        request: &api::PostJupiterSwapRequest,
    ) -> Result<api::PostJupiterSwapResponse> {
        let response = self
            .client
            .post_jupiter_swap(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("PostJupiterSwap error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn post_jupiter_route_swap(
        &mut self,
        request: &api::PostJupiterRouteSwapRequest,
    ) -> Result<api::PostJupiterRouteSwapResponse> {
        let response = self
            .client
            .post_jupiter_route_swap(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("PostJupiterRouteSwap error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn post_jupiter_swap_instructions(
        &mut self,
        request: &api::PostJupiterSwapInstructionsRequest,
    ) -> Result<api::PostJupiterSwapInstructionsResponse> {
        Ok(self
            .client
            .post_jupiter_swap_instructions(request.clone())
            .await?
            .into_inner())
    }

    pub async fn submit_jupiter_swap_instructions(
        &mut self,
        request: api::PostJupiterSwapInstructionsRequest,
        submit_opts: SubmitParams,
        use_bundle: bool,
    ) -> Result<Vec<String>> {
        let swap_instructions = self.post_jupiter_swap_instructions(&request).await?;

        let address_lookup_table =
            convert_address_lookup_table(&swap_instructions.address_lookup_table_addresses)?;

        let instructions = convert_jupiter_instructions(&swap_instructions.instructions)?;

        let blockhash = self
            .client
            .get_recent_block_hash_v2(api::GetRecentBlockHashRequestV2 { offset: 0 })
            .await?
            .into_inner()
            .block_hash;

        let message = VersionedMessage::V0(v0::Message::try_compile(
            &self.public_key.unwrap(),
            &instructions,
            &address_lookup_table,
            blockhash.parse()?,
        )?);

        let tx: VersionedTransaction =
            VersionedTransaction::try_new(message, &[self.keypair.as_mut().unwrap()])?;

        let tx_message = api::TransactionMessage {
            content: general_purpose::STANDARD.encode(bincode::serialize(&tx)?),
            is_cleanup: false,
        };

        self.sign_and_submit(vec![tx_message], submit_opts, use_bundle)
            .await
    }

    pub async fn post_pump_swap(
        &mut self,
        request: &api::PostPumpFunSwapRequest,
    ) -> Result<api::PostPumpFunSwapResponse> {
        let response = self
            .client
            .post_pump_fun_swap(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("PostPumpFunSwap error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn post_trade_swap(
        &mut self,
        request: &api::TradeSwapRequest,
    ) -> Result<api::TradeSwapResponse> {
        let response = self
            .client
            .post_trade_swap(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("PostTradeSwap error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn post_route_trade_swap(
        &mut self,
        request: &api::RouteTradeSwapRequest,
    ) -> Result<api::TradeSwapResponse> {
        let response = self
            .client
            .post_route_trade_swap(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("PostRouteTradeSwap error: {}", e))?;

        Ok(response.into_inner())
    }
}
