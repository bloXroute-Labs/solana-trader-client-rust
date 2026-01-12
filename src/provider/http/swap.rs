use crate::{
    common::signing::SubmitParams,
    provider::utils::{
        convert_address_lookup_table, convert_jupiter_instructions,
        create_transaction_message,
    },
};

use super::HTTPClient;
use anyhow::Result;
use serde_json::json;
use base64::{engine::general_purpose, Engine};
use solana_sdk::{
    message::{v0, VersionedMessage},
    transaction::VersionedTransaction,
};
use solana_trader_proto::api;

impl HTTPClient {


    pub async fn post_jupiter_swap(
        &self,
        request: &api::PostJupiterSwapRequest,
    ) -> Result<api::PostJupiterSwapResponse> {
        let response = self
            .client
            .post(format!("{}/api/v2/jupiter/swap", self.base_url))
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn post_jupiter_route_swap(
        &self,
        request: &api::PostJupiterRouteSwapRequest,
    ) -> Result<api::PostJupiterRouteSwapResponse> {
        let response = self
            .client
            .post(format!("{}/api/v2/jupiter/route-swap", self.base_url))
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn post_jupiter_swap_instructions(
        &self,
        request: &api::PostJupiterSwapInstructionsRequest,
    ) -> Result<api::PostJupiterSwapInstructionsResponse> {
        let response = self
            .client
            .post(format!(
                "{}/api/v2/jupiter/swap-instructions",
                self.base_url
            ))
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
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

        let response = self
            .client
            .get(format!(
                "{}/api/v2/system/blockhash?offset=0",
                self.base_url
            ))
            .send()
            .await?;

        let blockhash_response: api::GetRecentBlockHashResponseV2 =
            self.handle_response(response).await?;

        let message = VersionedMessage::V0(v0::Message::try_compile(
            &self.public_key.unwrap(),
            &instructions,
            &address_lookup_tables,
            blockhash_response.block_hash.parse()?,
        )?);

        let tx: VersionedTransaction = VersionedTransaction::try_new(message, &[keypair])?;

        let tx_message = api::TransactionMessage {
            content: general_purpose::STANDARD.encode(bincode::serialize(&tx)?),
            is_cleanup: false,
        };

        self.sign_and_submit(vec![tx_message], submit_opts, use_bundle)
            .await
    }


    pub async fn post_pump_fun_swap(
        &self,
        user_address: String,
        bonding_curve_address: String,
        token_address: String,
        token_amount: f64,
        creator: String,
        sol_threshold: f64,
        is_buy: bool,
        slippage: f64,
        compute_limit: u32,
        compute_price: u64,
        tip: Option<u64>
    ) -> anyhow::Result<api::PostPumpFunSwapResponse> {
        let url = format!("{}/api/v2/pumpfun/swap", self.base_url);
        println!("{}", url);
        
        let request_json = json!({
            "userAddress": user_address,
            "bondingCurveAddress": bonding_curve_address,
            "tokenAddress": token_address,
            "creator": creator,
            "tokenAmount": token_amount,
            "solThreshold": sol_threshold,
            "isBuy": is_buy,
            "slippage": slippage,
            "computeLimit": compute_limit,
            "computePrice": compute_price,
            "tip": tip
        });
        
        let response = self
            .client
            .post(&url)
            .json(&request_json)
            .send()
            .await?;
            
        let result: api::PostPumpFunSwapResponse = self.handle_response(response).await?;
        
        Ok(result)
    }

    pub async fn post_pump_fun_swap_sol(
        &self,
        user_address: String,
        bonding_curve_address: String,
        token_address: String,
        creator: String,
        sol_amount: f64,
        slippage: f64,
        compute_limit: u32,
        compute_price: u64,
        tip: Option<u64>
    ) -> anyhow::Result<api::PostPumpFunSwapResponse> {
        let url = format!("{}/api/v2/pumpfun/swap-sol", self.base_url);
        println!("{}", url);
        
        let request_json = json!({
            "userAddress": user_address,
            "bondingCurveAddress": bonding_curve_address,
            "tokenAddress": token_address,
            "creator": creator,
            "solAmount": sol_amount,
            "slippage": slippage,
            "computeLimit": compute_limit,
            "computePrice": compute_price,
            "tip": tip
        });
        
        let response = self
            .client
            .post(&url)
            .json(&request_json)
            .send()
            .await?;
            
        let result: api::PostPumpFunSwapResponse = self.handle_response(response).await?;
        
        Ok(result)
    }

    pub async fn post_pump_fun_amm_swap(
        &self,
        request: &api::PostPumpFunAmmSwapRequest,
    ) -> Result<api::PostPumpFunAmmSwapResponse> {
        let response = self
            .client
            .post(format!("{}/api/v2/pumpfun/amm/swap", self.base_url))
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
    }
}
