use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine};
use bincode::{deserialize, serialize};
use serde::Serialize;
use solana_hash::Hash;
use solana_sdk::{
    instruction::Instruction,
    message::VersionedMessage,
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
    blockhash: String,
) -> Result<SignedTransaction>
where
    T: IntoTransactionMessage + Clone,
{
    let tx_message = tx.clone().into_transaction_message();
    let rawbytes = STANDARD.decode(&tx_message.content)?;
    let parsed_hash = blockhash.parse()?;

    let signed_data = match deserialize(&rawbytes) {
        Ok(versioned_tx) => sign_versioned_transaction(versioned_tx, keypair, parsed_hash)?,
        Err(_) => sign_legacy_transaction(&rawbytes, keypair, parsed_hash)?,
    };

    Ok(SignedTransaction {
        content: STANDARD.encode(signed_data),
        is_cleanup: tx_message.is_cleanup,
    })
}

fn sign_versioned_transaction(
    mut tx: VersionedTransaction,
    keypair: &Keypair,
    blockhash: solana_sdk::hash::Hash,
) -> Result<Vec<u8>> {
    match &mut tx.message {
        VersionedMessage::Legacy(message) => {
            message.recent_blockhash = blockhash;
        }
        VersionedMessage::V0(message) => {
            message.recent_blockhash = blockhash;
        }
    }

    tx.signatures = vec![Signature::default()];
    let message_data = tx.message.serialize();
    tx.signatures[0] = keypair.sign_message(&message_data);

    Ok(serialize(&tx)?)
}

fn sign_legacy_transaction(
    rawbytes: &[u8],
    keypair: &Keypair,
    blockhash: solana_sdk::hash::Hash,
) -> Result<Vec<u8>> {
    let mut tx: Transaction = deserialize(rawbytes)?;
    tx.try_partial_sign(&[keypair], blockhash)?;
    Ok(serialize(&tx)?)
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
