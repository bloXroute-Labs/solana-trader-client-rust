use base64::engine::general_purpose;
use base64::Engine;
use solana_hash::Hash;
use solana_sdk::instruction::{AccountMeta, CompiledInstruction, Instruction};
use solana_sdk::message::VersionedMessage;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Signature, Signer};
use solana_sdk::transaction::{Transaction, VersionedTransaction};
use solana_trader_client_rust::common::signing::SubmitParams;
use solana_trader_client_rust::provider::grpc::GrpcClient;
use solana_trader_proto::api::{GetRecentBlockHashRequestV2, TransactionMessage};
use std::str::FromStr;

const TRADER_API_MEMO_PROGRAM: &str = "HQ2UUt18uJqKaQFJhgV9zaTdQxUZjNrsKFgoEDquBkcx";
const JITO_TIP_WALLET: &str = "95cfoy472fcQHaw4tPGBTKpn6ZQnfEPfBgDQx6gcRmRg";
const MEMO_MESSAGE: &str = "Powered by bloXroute Trader Api";

#[tokio::test]
#[ignore]
async fn test_add_memo_to_tx() -> anyhow::Result<()> {
    let mut client = GrpcClient::new(None).await?;

    let block_hash = client
        .get_recent_block_hash_v2(GetRecentBlockHashRequestV2 { offset: 0 })
        .await?
        .block_hash
        .parse::<Hash>()?;

    let lamports_to_transfer = 1_000_000;
    let pubkey = client.public_key.unwrap();
    let keypair = client.get_keypair()?;
    let jito_tip_wallet = Pubkey::from_str(JITO_TIP_WALLET)?;

    let transfer_instruction = solana_system_interface::instruction::transfer(&pubkey, &pubkey, lamports_to_transfer);
    let jito_tip_instruction =
        solana_system_interface::instruction::transfer(&pubkey, &jito_tip_wallet, lamports_to_transfer);

    let mut transaction = Transaction::new_signed_with_payer(
        &[
            transfer_instruction,
            jito_tip_instruction,
            build_memo_instruction(),
        ],
        Some(&pubkey),
        &[&keypair],
        block_hash,
    );

    let message_data = transaction.message.serialize();
    transaction.signatures = vec![Signature::default()];
    transaction.signatures[0] = keypair.sign_message(&message_data);

    let serialized_tx = bincode::serialize(&transaction)?;
    let messages = vec![TransactionMessage {
        content: general_purpose::STANDARD.encode(serialized_tx),
        is_cleanup: false,
    }];

    let submit_opts = SubmitParams::default();
    let signature = client.sign_and_submit(messages, submit_opts, false).await?;
    println!("Signature: {signature:?}");
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_add_memo_to_serialized_tx() -> anyhow::Result<()> {
    let mut client = GrpcClient::new(None).await?;

    let lamports_to_transfer = 2000;
    let pubkey = client.public_key.unwrap();
    let keypair = client.get_keypair();

    let transfer_instruction = solana_system_interface::instruction::transfer(&pubkey, &pubkey, lamports_to_transfer);

    let mut transaction = Transaction::new_with_payer(&[transfer_instruction], Some(&pubkey));

    let message_data = transaction.message.serialize();
    transaction.signatures = vec![Signature::default()];
    transaction.signatures[0] = keypair.unwrap().sign_message(&message_data);

    let serialized_tx = bincode::serialize(&transaction)?;
    let mut deserialized_tx: VersionedTransaction = bincode::deserialize(&serialized_tx)?;

    if let VersionedMessage::Legacy(message) = &mut deserialized_tx.message {
        let memo_program = Pubkey::from_str(TRADER_API_MEMO_PROGRAM)?;
        message.account_keys.push(memo_program);

        message
            .instructions
            .push(CompiledInstruction::new_from_raw_parts(
                (message.account_keys.len() - 1) as u8,
                MEMO_MESSAGE.as_bytes().to_vec(),
                vec![],
            ));
    }

    let content = bincode::serialize(&deserialized_tx)?;
    let messages = vec![TransactionMessage {
        content: general_purpose::STANDARD.encode(content),
        is_cleanup: false,
    }];

    let submit_opts = SubmitParams {
        use_staked_rpcs: false,
        ..Default::default()
    };

    let signature = client.sign_and_submit(messages, submit_opts, false).await?;
    println!("Signature: {signature:?}");
    Ok(())
}

fn build_memo_instruction() -> Instruction {
    let program_id = Pubkey::from_str(TRADER_API_MEMO_PROGRAM).unwrap();

    Instruction {
        program_id,
        accounts: vec![AccountMeta::new(program_id, false)],
        data: MEMO_MESSAGE.as_bytes().to_vec(),
    }
}
