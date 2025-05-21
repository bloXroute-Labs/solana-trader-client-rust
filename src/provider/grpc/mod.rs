pub mod quote;
pub mod stream;
pub mod swap;

use anyhow::Result;
use rustls::crypto::ring::default_provider;
use rustls::crypto::CryptoProvider;
use solana_sdk::pubkey::Pubkey;
use solana_trader_proto::api::{self, GetServerTimeRequest, PostSubmitBatchRequest, PostSubmitPaladinRequest, TransactionMessageV2};
use std::collections::HashMap;
use tonic::service::Interceptor;
use tonic::transport::ClientTlsConfig;
use tonic::{
    metadata::MetadataValue, service::interceptor::InterceptedService, transport::Channel, Request,
};
use crate::provider::utils::timestamp;

use crate::common::signing::{sign_transaction, SubmitParams};
use crate::common::{get_base_url_from_env, grpc_endpoint, is_submit_only_endpoint, BaseConfig};
use solana_sdk::signature::Keypair;
use solana_trader_proto::api::{
    GetRecentBlockHashRequestV2, PostSubmitRequest, TransactionMessage, PostSubmitSnipeRequest
};

use super::utils::IntoTransactionMessage;

#[derive(Clone)]
struct AuthInterceptor {
    headers: HashMap<&'static str, String>,
    enabled: bool,
}

impl AuthInterceptor {
    fn new(auth_header: String, enabled: bool) -> Self {
        let mut headers = HashMap::new();
        headers.insert("authorization", auth_header);
        headers.insert("x-sdk", "rust-client".to_string());
        headers.insert("x-sdk-version", env!("CARGO_PKG_VERSION").to_string());

        Self { headers, enabled }
    }
}

impl Interceptor for AuthInterceptor {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Request<()>, tonic::Status> {
        if self.enabled {
            for (key, value) in &self.headers {
                request.metadata_mut().insert(
                    *key,
                    MetadataValue::try_from(value)
                        .map_err(|e| tonic::Status::internal(e.to_string()))?,
                );
            }
        }
        Ok(request)
    }
}

#[derive(Debug)]
pub struct GrpcClient {
    client: api::api_client::ApiClient<InterceptedService<Channel, AuthInterceptor>>,
    keypair: Option<Keypair>,
    pub public_key: Option<Pubkey>,
}

impl GrpcClient {
    pub fn get_keypair(&self) -> Result<&Keypair> {
        Ok(self.keypair.as_ref().unwrap())
    }

    pub async fn new(endpoint: Option<String>) -> Result<Self> {
        let base = BaseConfig::try_from_env()?;
        let (default_base_url, secure) = get_base_url_from_env();
        let final_base_url = endpoint.unwrap_or(default_base_url);
        let endpoint = grpc_endpoint(&final_base_url, secure);

        is_submit_only_endpoint(&final_base_url);

        if CryptoProvider::get_default().is_none() {
            default_provider()
                .install_default()
                .map_err(|e| anyhow::anyhow!("Failed to install crypto provider: {:?}", e))?;
        }

        let channel = Channel::from_shared(endpoint.clone())
            .map_err(|e| anyhow::anyhow!("Invalid URI: {}", e))?
            .tls_config(ClientTlsConfig::new().with_webpki_roots())
            .map_err(|e| anyhow::anyhow!("TLS config error: {}", e))?
            .connect()
            .await
            .map_err(|e| anyhow::anyhow!("Connection error: {}", e))?;

        let interceptor = AuthInterceptor::new(base.auth_header, true);
        let client = api::api_client::ApiClient::with_interceptor(channel, interceptor);

        Ok(Self {
            client,
            public_key: base.public_key,
            keypair: base.keypair,
        })
    }

    pub async fn sign_and_submit<T: IntoTransactionMessage + Clone>(
        &mut self,
        txs: Vec<T>,
        submit_opts: SubmitParams,
        use_bundle: bool,
    ) -> Result<Vec<String>> {
        let block_hash = self
            .client
            .get_recent_block_hash_v2(GetRecentBlockHashRequestV2 { offset: 0 })
            .await?
            .into_inner()
            .block_hash;

        let keypair = self.get_keypair()?;

        if txs.len() == 1 {
            let signed_tx = sign_transaction(&txs[0], keypair, block_hash).await?;

            let req = PostSubmitRequest {
                transaction: Some(TransactionMessage {
                    content: signed_tx.content,
                    is_cleanup: signed_tx.is_cleanup,
                }),
                skip_pre_flight: submit_opts.skip_pre_flight,
                front_running_protection: Some(submit_opts.front_running_protection),
                use_staked_rp_cs: Some(submit_opts.use_staked_rpcs),
                fast_best_effort: Some(submit_opts.fast_best_effort),
                tip: None,
                allow_back_run: submit_opts.allow_back_run,
                revenue_address: submit_opts.revenue_address,
                sniping: Some(false),
                timestamp: timestamp(),
                submit_protection: None
            };

            let signature = self
                .client
                .post_submit_v2(req)
                .await?
                .into_inner()
                .signature;

            return Ok(vec![signature]);
        }

        let mut entries = Vec::with_capacity(txs.len());
        for tx in txs {
            let signed_tx = sign_transaction(&tx, keypair, block_hash.clone()).await?;

            let entry = api::PostSubmitRequestEntry {
                transaction: Some(TransactionMessage {
                    content: signed_tx.content,
                    is_cleanup: signed_tx.is_cleanup,
                }),
                skip_pre_flight: submit_opts.skip_pre_flight,
            };
            entries.push(entry);
        }

        let batch_request = api::PostSubmitBatchRequest {
            entries,
            use_bundle: Some(use_bundle),
            submit_strategy: submit_opts.submit_strategy.into(),
            front_running_protection: Some(submit_opts.front_running_protection),
            timestamp: timestamp(),
            submit_protection: None
        };

        let response = self
            .client
            .post_submit_batch_v2(batch_request)
            .await?
            .into_inner();

        let signatures = response
            .transactions
            .into_iter()
            .filter(|entry| entry.submitted)
            .map(|entry| entry.signature)
            .collect();

        Ok(signatures)
    }

    pub async fn sign_and_submit_snipe<T: IntoTransactionMessage + Clone>(
        &mut self,
        txs: Vec<T>,
        use_staked_rpcs: bool,
    ) -> Result<Vec<String>> {
        let block_hash = self
            .client
            .get_recent_block_hash_v2(GetRecentBlockHashRequestV2 { offset: 0 })
            .await?
            .into_inner()
            .block_hash;

        let keypair = self.get_keypair()?;

        let mut entries = Vec::with_capacity(txs.len());
        for tx in txs {
            let signed_tx = sign_transaction(&tx, keypair, block_hash.clone()).await?;

            let entry = api::PostSubmitRequestEntry {
                transaction: Some(TransactionMessage {
                    content: signed_tx.content,
                    is_cleanup: signed_tx.is_cleanup,
                }),
                skip_pre_flight: false,
            };
            entries.push(entry);
        }

        let snipe_request = api::PostSubmitSnipeRequest {
            entries,
            use_staked_rp_cs: Some(use_staked_rpcs),
            timestamp: timestamp()
        };

        let response = self
            .client
            .post_submit_snipe_v2(snipe_request)
            .await?
            .into_inner();

        let signatures = response
            .transactions
            .into_iter()
            .filter(|entry| entry.submitted)
            .map(|entry| entry.signature)
            .collect();

        Ok(signatures)
    }

    pub async fn sign_and_submit_paladin<T: IntoTransactionMessage + Clone>(
        &mut self,
        tx: T,
        revert_protection: bool,
    ) -> Result<String> {
        let block_hash = self
            .client
            .get_recent_block_hash_v2(GetRecentBlockHashRequestV2 { offset: 0 })
            .await?
            .into_inner()
            .block_hash;

        let keypair = self.get_keypair()?;
        let signed_tx = sign_transaction(&tx, keypair, block_hash).await?;

        let paladin_request = api::PostSubmitPaladinRequest {
            transaction: Some(TransactionMessageV2 {
                content: signed_tx.content,
            }),
            revert_protection: Some(revert_protection),
            timestamp: timestamp()
        };

        let signature = self
            .client
            .post_submit_paladin_v2(paladin_request)
            .await?
            .into_inner()
            .signature;

        Ok(signature)
    }

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
        let response: tonic::Response<api::GetRecentBlockHashResponse> = self
            .client
            .get_recent_block_hash(Request::new(*request))
            .await
            .map_err(|e| anyhow::anyhow!("GetRecentBlockHash error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_server_time(
        &mut self,
        request: &GetServerTimeRequest,
    ) -> Result<api::GetServerTimeResponse> {
        let response: tonic::Response<api::GetServerTimeResponse> = self
            .client
            .get_server_time(Request::new(*request))
            .await
            .map_err(|e| anyhow::anyhow!("GetServerTime error: {}", e))?;

        return Ok(response.into_inner())
    }

    pub async fn post_submit(
        &mut self,
        request: &PostSubmitRequest,
    ) -> Result<api::PostSubmitResponse> {
        let response: tonic::Response<api::PostSubmitResponse> = self
            .client
            .post_submit(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("PostSubmit error: {}", e))?;

        return Ok(response.into_inner())
    }

    pub async fn post_submit_snipe_v2(
        &mut self,
        request: &PostSubmitSnipeRequest,
    ) -> Result<api::PostSubmitSnipeResponse> {
        let response: tonic::Response<api::PostSubmitSnipeResponse> = self
            .client
            .post_submit_snipe_v2(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("PostSubmitSnipeV2 error: {}", e))?;

        return Ok(response.into_inner())
    }

    pub async fn post_submit_paladin_v2(
        &mut self,
        request: &PostSubmitPaladinRequest,
    ) -> Result<api::PostSubmitResponse> {
        let response: tonic::Response<api::PostSubmitResponse> = self
            .client
            .post_submit_paladin_v2(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("PostSubmitPaladinV2 error: {}", e))?;

        return Ok(response.into_inner())
    }

    pub async fn post_submit_v2(
        &mut self,
        request: &PostSubmitRequest,
    ) -> Result<api::PostSubmitResponse> {
        let response: tonic::Response<api::PostSubmitResponse> = self
            .client
            .post_submit_v2(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("PostSubmitV2 error: {}", e))?;

        return Ok(response.into_inner())
    }

    pub async fn post_submit_batch(
        &mut self,
        request: &PostSubmitBatchRequest,
    ) -> Result<api::PostSubmitBatchResponse> {
        let response: tonic::Response<api::PostSubmitBatchResponse> = self
            .client
            .post_submit_batch(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("PostSubmitBatch error: {}", e))?;

        return Ok(response.into_inner())
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
        let request = Request::new(api::GetPriorityFeeByProgramRequest { programs });

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
