use anyhow::Result;
use base64::{engine::general_purpose, Engine};
use serde_json::json;
use solana_sdk::{
    message::{v0, VersionedMessage},
    transaction::VersionedTransaction,
};
use solana_trader_proto::api;

use crate::{
    common::signing::SubmitParams,
    provider::utils::{
        convert_address_lookup_table, convert_jupiter_instructions, convert_raydium_instructions,
        create_transaction_message,
    },
};

use super::WebSocketClient;

impl WebSocketClient {
    pub async fn post_raydium_swap(
        &self,
        request: &api::PostRaydiumSwapRequest,
    ) -> Result<api::PostRaydiumSwapResponse> {
        let params = json!({
            "ownerAddress": request.owner_address,
            "inToken": request.in_token,
            "outToken": request.out_token,
            "inAmount": request.in_amount,
            "slippage": request.slippage,
            "computeLimit": request.compute_limit,
            "computePrice": request.compute_price,
            "tip": request.tip,
        });

        self.conn.request("PostRaydiumSwap", params).await
    }

    pub async fn post_raydium_route_swap(
        &self,
        request: &api::PostRaydiumRouteSwapRequest,
    ) -> Result<api::PostRaydiumRouteSwapResponse> {
        let params = json!({
            "ownerAddress": request.owner_address,
            "slippage": request.slippage,
            "steps": request.steps,
            "computeLimit": request.compute_limit,
            "computePrice": request.compute_price,
            "tip": request.tip,
        });

        self.conn.request("PostRaydiumRouteSwap", params).await
    }

    pub async fn post_raydium_swap_instructions(
        &self,
        request: &api::PostRaydiumSwapInstructionsRequest,
    ) -> Result<api::PostRaydiumSwapInstructionsResponse> {
        let params = json!({
            "ownerAddress": request.owner_address,
            "inToken": request.in_token,
            "outToken": request.out_token,
            "inAmount": request.in_amount,
            "slippage": request.slippage,
            "computeLimit": request.compute_limit,
            "computePrice": request.compute_price,
            "tip": request.tip,
        });

        self.conn
            .request("PostRaydiumSwapInstructions", params)
            .await
    }

    pub async fn submit_raydium_swap_instructions(
        &self,
        request: api::PostRaydiumSwapInstructionsRequest,
        submit_opts: SubmitParams,
        use_bundle: bool,
    ) -> Result<Vec<String>> {
        let swap_instructions = self.post_raydium_swap_instructions(&request).await?;

        let instructions = convert_raydium_instructions(&swap_instructions.instructions)?;

        let hash_res: api::GetRecentBlockHashResponseV2 =
            self.conn.request("GetRecentBlockHashV2", json!({})).await?;

        let tx_message = create_transaction_message(instructions, &hash_res.block_hash)?;

        self.sign_and_submit(vec![tx_message], submit_opts, use_bundle)
            .await
    }

    pub async fn post_raydium_cpmm_swap(
        &self,
        request: &api::PostRaydiumCpmmSwapRequest,
    ) -> Result<api::PostRaydiumCpmmSwapResponse> {
        let params = json!({
            "ownerAddress": request.owner_address,
            "poolAddress": request.pool_address,
            "inToken": request.in_token,
            "outToken": request.out_token,
            "inAmount": request.in_amount,
            "slippage": request.slippage,
            "computeLimit": request.compute_limit,
            "computePrice": request.compute_price,
            "tip": request.tip,
        });

        self.conn.request("PostRaydiumCPMMSwap", params).await
    }

    pub async fn post_raydium_clmm_swap(
        &self,
        request: &api::PostRaydiumSwapRequest,
    ) -> Result<api::PostRaydiumSwapResponse> {
        let params = json!({
            "ownerAddress": request.owner_address,
            "inToken": request.in_token,
            "outToken": request.out_token,
            "inAmount": request.in_amount,
            "slippage": request.slippage,
            "computeLimit": request.compute_limit,
            "computePrice": request.compute_price,
            "tip": request.tip,
        });

        self.conn.request("PostRaydiumCLMMSwap", params).await
    }

    pub async fn post_raydium_clmm_route_swap(
        &self,
        request: &api::PostRaydiumRouteSwapRequest,
    ) -> Result<api::PostRaydiumRouteSwapResponse> {
        let params = json!({
            "ownerAddress": request.owner_address,
            "slippage": request.slippage,
            "steps": request.steps,
        });

        self.conn.request("PostRaydiumCLMMRouteSwap", params).await
    }

    // NOTE: Fast mode is not used as of 11/1, breaks the endpoint.
    pub async fn post_jupiter_swap(
        &self,
        request: &api::PostJupiterSwapRequest,
    ) -> Result<api::PostJupiterSwapResponse> {
        let modified_request = json!({
            "ownerAddress": request.owner_address,
            "inToken": request.in_token,
            "outToken": request.out_token,
            "inAmount": request.in_amount,
            "slippage": request.slippage,
            "computeLimit": request.compute_limit,
            "computePrice": request.compute_price,
            "tip": request.tip,
        });

        self.conn.request("PostJupiterSwap", modified_request).await
    }

    pub async fn post_jupiter_route_swap(
        &self,
        request: &api::PostJupiterRouteSwapRequest,
    ) -> Result<api::PostJupiterRouteSwapResponse> {
        let params = json!({
            "ownerAddress": request.owner_address,
            "slippage": request.slippage,
            "steps": request.steps,
            "computeLimit": request.compute_limit,
            "computePrice": request.compute_price,
            "tip": request.tip,
        });

        self.conn.request("PostJupiterRouteSwap", params).await
    }

    pub async fn post_jupiter_swap_instructions(
        &self,
        request: &api::PostJupiterSwapInstructionsRequest,
    ) -> Result<api::PostJupiterSwapInstructionsResponse> {
        let params = json!({
            "ownerAddress": request.owner_address,
            "inToken": request.in_token,
            "outToken": request.out_token,
            "inAmount": request.in_amount,
            "slippage": request.slippage,
            "computePrice": request.compute_price,
            "tip": request.tip,
        });

        self.conn
            .request("PostJupiterSwapInstructions", params)
            .await
    }

    pub async fn submit_jupiter_swap_instructions(
        &self,
        request: api::PostJupiterSwapInstructionsRequest,
        submit_opts: SubmitParams,
        use_bundle: bool,
    ) -> Result<Vec<String>> {
        let keypair = self.get_keypair()?;

        let swap_instructions = self.post_jupiter_swap_instructions(&request).await?;

        let address_lookup_tables =
            convert_address_lookup_table(&swap_instructions.address_lookup_table_addresses)?;

        let instructions = convert_jupiter_instructions(&swap_instructions.instructions)?;

        let hash_res: api::GetRecentBlockHashResponseV2 =
            self.conn.request("GetRecentBlockHashV2", json!({})).await?;

        let message = VersionedMessage::V0(v0::Message::try_compile(
            &self.public_key.unwrap(),
            &instructions,
            &address_lookup_tables,
            hash_res.block_hash.parse()?,
        )?);

        let tx = VersionedTransaction::try_new(message, &[keypair])?;

        let tx_message = api::TransactionMessage {
            content: general_purpose::STANDARD.encode(bincode::serialize(&tx)?),
            is_cleanup: false,
        };

        self.sign_and_submit(vec![tx_message], submit_opts, use_bundle)
            .await
    }

    pub async fn post_trade_swap(
        &self,
        request: &api::TradeSwapRequest,
    ) -> Result<api::TradeSwapResponse> {
        let params = json!({
            "ownerAddress": request.owner_address,
            "inToken": request.in_token,
            "outToken": request.out_token,
            "inAmount": request.in_amount,
            "slippage": request.slippage,
            "project": request.project,
            "computeLimit": request.compute_limit,
            "computePrice": request.compute_price,
            "tip": request.tip,
        });

        self.conn.request("PostTradeSwap", params).await
    }

    pub async fn post_route_trade_swap(
        &self,
        request: &api::RouteTradeSwapRequest,
    ) -> Result<api::TradeSwapResponse> {
        let params = json!({
            "ownerAddress": request.owner_address,
            "project": request.project,
            "slippage": request.slippage,
            "steps": request.steps,
        });

        self.conn.request("PostRouteTradeSwap", params).await
    }
}
