use std::{collections::HashMap, str::FromStr};

use anyhow::Result;
use base64::{engine::general_purpose, Engine};
use serde_json::{json, Value};
use solana_sdk::{
    address_lookup_table::AddressLookupTableAccount,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    transaction::Transaction,
};
use solana_trader_proto::api::{self, Project, TransactionMessage, TransactionMessageV2};

pub trait IntoTransactionMessage {
    fn into_transaction_message(self) -> TransactionMessage;
}

impl IntoTransactionMessage for TransactionMessage {
    fn into_transaction_message(self) -> TransactionMessage {
        self
    }
}

impl IntoTransactionMessage for TransactionMessageV2 {
    fn into_transaction_message(self) -> TransactionMessage {
        TransactionMessage {
            content: self.content,
            is_cleanup: false,
        }
    }
}

pub fn convert_address_lookup_table(
    address_lookup_table: &HashMap<String, api::PublicKeys>,
) -> Result<Vec<AddressLookupTableAccount>> {
    let mut result = Vec::new();

    for (key, accounts) in address_lookup_table {
        let lookup_table_address = Pubkey::from_str(key)?;
        let addresses: Vec<Pubkey> = accounts
            .pks
            .iter()
            .map(|pk| Pubkey::from_str(pk))
            .collect::<Result<_, _>>()?;

        let lookup_table = AddressLookupTableAccount {
            key: lookup_table_address,
            addresses,
        };

        result.push(lookup_table);
    }

    Ok(result)
}

pub fn convert_jupiter_instructions(
    instructions: &[api::InstructionJupiter],
) -> Result<Vec<Instruction>> {
    let mut solana_instructions = Vec::new();

    for inst in instructions {
        let program_id = Pubkey::from_str(&inst.program_id)?;

        let accounts: Vec<AccountMeta> = inst
            .accounts
            .iter()
            .map(|acc| {
                let pubkey = Pubkey::from_str(&acc.program_id)?;
                Ok(AccountMeta {
                    pubkey,
                    is_signer: acc.is_signer,
                    is_writable: acc.is_writable,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        solana_instructions.push(Instruction {
            program_id,
            accounts,
            data: inst.data.clone(),
        });
    }

    Ok(solana_instructions)
}

pub fn convert_raydium_instructions(
    instructions: &[api::InstructionRaydium],
) -> Result<Vec<Instruction>> {
    let mut solana_instructions = Vec::new();

    for inst in instructions {
        let program_id = Pubkey::from_str(&inst.program_id)?;

        let accounts: Vec<AccountMeta> = inst
            .accounts
            .iter()
            .map(|acc| {
                let pubkey = Pubkey::from_str(&acc.program_id)?;
                Ok(AccountMeta {
                    pubkey,
                    is_signer: acc.is_signer,
                    is_writable: acc.is_writable,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        solana_instructions.push(Instruction {
            program_id,
            accounts,
            data: inst.data.clone(),
        });
    }

    Ok(solana_instructions)
}

pub fn create_transaction_message(
    instructions: Vec<Instruction>,
    recent_blockhash: &str,
) -> Result<api::TransactionMessage> {
    let mut transaction = Transaction::new_with_payer(&instructions, None);
    transaction.message.recent_blockhash = recent_blockhash.parse()?;

    let serialized = bincode::serialize(&transaction)?;
    let content = general_purpose::STANDARD.encode(serialized);

    Ok(api::TransactionMessage {
        content,
        is_cleanup: false,
    })
}

pub fn convert_string_enums(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for (key, val) in map {
                match (key.as_str(), &val) {
                    ("project", Value::String(s)) => {
                        if let Some(project_enum) = Project::from_str_name(s) {
                            *val = json!(project_enum as i32);
                        }
                    }

                    ("infinity", Value::String(s)) => {
                        let mapped = match s.as_str() {
                            "INF_NOT" => 0,
                            "INF" => 1,
                            "INF_NEG" => 2,
                            _ => return,
                        };
                        *val = json!(mapped);
                    }

                    _ => convert_string_enums(val),
                }
            }
        }
        Value::Array(arr) => arr.iter_mut().for_each(convert_string_enums),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversions() {
        let mut value = json!({
            "project": "P_JUPITER",
            "tradeFeeRate": "1000",
            "nested": {
                "project": "P_RAYDIUM",
                "priceImpactPercent": {
                    "infinity": "INF_NOT"
                }
            }
        });

        convert_string_enums(&mut value);

        assert_eq!(value["project"], 2);
        assert_eq!(value["nested"]["project"], 3);
        assert_eq!(value["nested"]["priceImpactPercent"]["infinity"], 0);
    }
}
