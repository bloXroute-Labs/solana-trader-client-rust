use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::Serialize;
use anyhow::{anyhow};
use solana_hash::Hash;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::{Transaction, VersionedTransaction},
};
use solana_trader_proto::api;

use crate::provider::utils::IntoTransactionMessage;

#[derive(Debug, Clone, Serialize)]
pub struct SubmitParams {
    pub skip_pre_flight: bool,
    pub front_running_protection: bool,
    pub use_staked_rpcs: bool,
    pub fast_best_effort: bool,
    pub submit_strategy: api::SubmitStrategy,
    pub allow_back_run: Option<bool>,
    pub revenue_address: Option<String>,
    pub allow_revert: Option<bool>,
}

impl Default for SubmitParams {
    fn default() -> Self {
        Self {
            skip_pre_flight: true,
            front_running_protection: false,
            use_staked_rpcs: false,
            fast_best_effort: false,
            submit_strategy: api::SubmitStrategy::PSubmitAll,
            allow_back_run: None,
            revenue_address: None,
            allow_revert: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SignedTransaction {
    pub content: String,
    pub is_cleanup: bool,
}

pub async fn sign_transaction<T>(
    tx: &T,
    keypair: &Keypair,
) -> Result<SignedTransaction>
where
    T: IntoTransactionMessage + Clone,
{
    let tx_message = tx.clone().into_transaction_message();

    let signed_b64 = sign_existing_transaction(&tx_message.content, keypair)?;
    Ok(SignedTransaction {
        content: signed_b64,
        is_cleanup: tx_message.is_cleanup,
    })
}

pub fn create_signed_transaction(
    instruction: Vec<Instruction>,
    payer: &Pubkey,
    keypair: &Keypair,
    block_hash: Hash,
) -> anyhow::Result<Transaction> {
    let mut transaction =
        Transaction::new_signed_with_payer(&instruction, Some(payer), &[keypair], block_hash);

    let message_data = transaction.message.serialize();
    transaction.signatures = vec![Signature::default()];
    transaction.signatures[0] = keypair.sign_message(&message_data);

    Ok(transaction)
}

fn sign_existing_transaction(base64_tx: &str, keypair: &Keypair) -> Result<String> {
    let tx_bytes = STANDARD.decode(base64_tx)?;

    // Versioned transactions should be a super set of versioned and legacy transactions
    if let Ok(mut tx) = bincode::deserialize::<VersionedTransaction>(&tx_bytes) {
        // sign versioned tx logic here
        let sig_index = tx.signatures.iter().position(|sig| *sig == Signature::default())
            .ok_or_else(|| anyhow!("No empty signature slot found"))?;

        let msg_bytes = tx.message.serialize();
        tx.signatures[sig_index] = keypair.sign_message(&msg_bytes);

        let signed_bytes = bincode::serialize(&tx)?;
        return Ok(STANDARD.encode(signed_bytes));
    }

    // Fallback to legacy
    let mut legacy_tx: Transaction = bincode::deserialize(&tx_bytes)?;
    legacy_tx.try_partial_sign(&[keypair], legacy_tx.message.recent_blockhash)?;
    let signed_bytes = bincode::serialize(&legacy_tx)?;
    Ok(STANDARD.encode(signed_bytes))
}


