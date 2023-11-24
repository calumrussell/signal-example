use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::api::EthJsonRpc;

/// All signals should implement a signal method that returns (ID, serde_json::value). Should
/// expect these values to be written somewhere.
pub enum Signal {
    AnonymouslyFundedSmartContractTriggered(AnonymouslyFundedSmartContractTriggeredSignal),
}

impl Signal {
    pub async fn signal(&self,block: &BlockJson, suspicious_contracts: &HashSet<String>) -> Option<(u32, serde_json::Value)> {
        match self {
            Signal::AnonymouslyFundedSmartContractTriggered(inner) => inner.signal(block, suspicious_contracts).await,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnonymouslyFundedSmartContractTriggeredJson {
    contract_address: String,
    transaction_hash: String,
    block: u64,
}

/// We store lists of smart contract addresses that have been created by accounts that are
/// primarily funded using anon sources i.e. Tornado Cash. When these smart contracts are triggered
/// we create a signal.
pub struct AnonymouslyFundedSmartContractTriggeredSignal;

impl AnonymouslyFundedSmartContractTriggeredSignal {
    pub const ID: u32 = 0;

    pub async fn signal(
        &self,
        block: &BlockJson,
        suspicious_contracts: &HashSet<String>,
    ) -> Option<(u32, serde_json::Value)> {
        for transaction in block.get_transactions() {
            if let Some(to) = transaction.to() {
                if suspicious_contracts.contains(to) {
                    let json_resp = AnonymouslyFundedSmartContractTriggeredJson {
                        contract_address: to.to_string(),
                        transaction_hash: transaction.hash().unwrap_or("").to_string(),
                        block: block.number(),
                    };
                    return Some((Self::ID, serde_json::to_value(json_resp).unwrap()));
                }
            }
        }
        None
    }
}

/// [Event]s are on-chain events that we wish to track for use with [crate::signal::Signal]s.
///
/// Should implement an event method that returns (i32, serde_json::Value) expecting that this
/// value will be written for use in signal code later.
///
/// One option that should be considered here is to pass a writer/sink to event method because
/// blocks will have more than one transaction that we want to check.
pub enum Event {
    TornadoCashWithdraw(TornadoCashWithdrawEvent),
    TransferFromFixedFloat(TransferFromFixedFloatEvent),
    SuspiciousContractCreated(SuspiciousContractCreatedEvent),
}

impl Event {
    pub async fn event(&self, api: &impl EthJsonRpc, block: &BlockJson, suspicious_addresses: &HashSet<String>) -> Option<(u32, serde_json::Value)> {
        match self {
            Event::TornadoCashWithdraw(inner) => inner.event(block).await,
            Event::TransferFromFixedFloat(inner) => inner.event(block).await,
            Event::SuspiciousContractCreated(inner) => inner.event(block, api, suspicious_addresses).await
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct TornadoCashWithdrawEventJson {
    tornado_address: String,
    recipient: String,
    relayer: String,
    tornado_address_name: String,
    block_timestamp: u64,
    block: u64,
    transaction_hash: String,
}

pub struct TornadoCashWithdrawEvent;

impl TornadoCashWithdrawEvent {
    const ID: u32 = 1;

    const TCETH01: &str = "0x12D66f87A04A9E220743712cE6d9bB1B5616B8Fc";
    const TCETH1: &str = "0x47CE0C6eD5B0Ce3d3A51fdb1C52DC66a7c3c2936";

    fn decode_transaction(input: &str) -> Option<(String, String, String)> {
        let hex = alloy_primitives::hex::decode(input).unwrap();
        if let Ok(tc) = crate::sol::TornadoCashSol::decode(&hex) {
            return Some((
                tc._tornado.to_string(),
                tc._recipient.to_string(),
                tc._relayer.to_string(),
            ));
        }
        None
    }

    pub async fn event(&self, block: &BlockJson) -> Option<(u32, serde_json::Value)> {
        for transaction in block.get_transactions() {
            if let Some(input) = transaction.input() {
                if let Some(withdraw) = TornadoCashWithdrawEvent::decode_transaction(input) {
                    // This will only find one signal per block, in practice we would probably write
                    // signals to a queue or pass a writer in to every signa
                    let mut tornado_address_name = "Unknown".to_string();
                    if withdraw.0.eq(Self::TCETH01) {
                        tornado_address_name = "ETH/0.1".to_string();
                    } else if withdraw.0.eq(Self::TCETH1) {
                        tornado_address_name = "ETH/1".to_string();
                    }
                    let json_resp = TornadoCashWithdrawEventJson {
                        tornado_address: withdraw.0,
                        recipient: withdraw.1,
                        relayer: withdraw.2,
                        tornado_address_name,
                        block: block.number(),
                        block_timestamp: block.timestamp(),
                        transaction_hash: transaction.hash().unwrap_or("").to_string(),
                    };

                    // Unwrap should only fail when code here is wrong, so we need to exit
                    return Some((Self::ID, serde_json::to_value(json_resp).unwrap()));
                }
            }
        }
        None
    }
}

#[derive(Deserialize, Serialize)]
pub struct TransferFromFixedFloatJson {
    pub recipient: String,
    pub value: u64,
    pub block_timestamp: u64,
    pub block: u64,
    pub transaction_hash: String,
}

pub struct TransferFromFixedFloatEvent;

impl TransferFromFixedFloatEvent {
    pub const ID: u32 = 2;
    const FIXED_FLOAT_ADDRESS: &str = "0x4e5b2e1dc63f6b91cb6cd759936495434c7e972f";

    pub async fn event(&self, block: &BlockJson) -> Option<(u32, serde_json::Value)> {
        for transaction in block.get_transactions() {
            if let Some(from_address) = transaction.from() {
                if from_address.eq(Self::FIXED_FLOAT_ADDRESS) {
                    if let Some(to_address) = transaction.to() {
                        let json_resp = TransferFromFixedFloatJson {
                            recipient: to_address.to_string(),
                            block: block.number(),
                            value: transaction.value().unwrap_or(0),
                            block_timestamp: block.timestamp(),
                            transaction_hash: transaction.hash().unwrap_or("").to_string(),
                        };
                        return Some((Self::ID, serde_json::to_value(json_resp).unwrap()));
                    }
                }
            }
        }
        None
    }
}

#[derive(Deserialize, Serialize)]
pub struct SuspiciousContractCreatedJson {
    creator: String,
    contract_code: String,
    contract_address: String,
    block_timestamp: u64,
    block: u64,
    transaction_hash: String,
}

/// This code can only identify when a suspicious contract has been created but the address of the
/// contract itself is only stored in logs so we need to get transaction receipt to get the actual
/// address.
pub struct SuspiciousContractCreatedEvent;

impl SuspiciousContractCreatedEvent {
    const ID: u32 = 3;

    pub async fn event(
        &self,
        block: &BlockJson,
        api: &impl EthJsonRpc,
        suspicious_addresses: &HashSet<String>,
    ) -> Option<(u32, serde_json::Value)> {
        for transaction in block.get_transactions() {
            if let Some(from_address) = transaction.from() {
                // Possible bug with is_contract_creation, not 100% sure whether this check is correct
                if suspicious_addresses.contains(from_address) && transaction.is_contract_creation() {
                    if let Some(hash) = transaction.hash() {
                        if let Ok(receipt) = api.get_transaction_receipt(hash).await {
                            let transaction_receipt = TransactionReceiptJson::new(receipt);
                            let json_resp = SuspiciousContractCreatedJson {
                                creator: from_address.to_string(),
                                contract_code: transaction.input().unwrap_or("x0").to_string(),
                                contract_address: transaction_receipt.contract_address().unwrap().to_string(),
                                block_timestamp: block.timestamp(),
                                block: block.number(),
                                transaction_hash: hash.to_string(),
                            };
                            return Some((Self::ID, serde_json::to_value(json_resp).unwrap()));
                        }
                    }
                }
            }
        }
        None
    }
}

fn convert_i64_from_hex(str: &str) -> i64 {
    i64::from_str_radix(str.trim_start_matches("0x"), 16).unwrap()
}

/// Thin logic around a block in the chain. Should contain little logic itself other than
/// getters. Used as input to algo.
///
/// In this case, it is just a wrapper around serde_json so we don't have to think about serializing
/// and deserializing/go through eth API in depth. In practice, the underlying representation would
/// be completely irrelevant to callers and there would be complete separation i.e. you would have
/// a completely standard [Block] struct rather than [BlockJson].
#[derive(Debug)]
pub struct BlockJson {
    value: serde_json::Value,
}

impl BlockJson {
    pub fn new(value: serde_json::Value) -> Self {
        Self { value }
    }

    pub fn timestamp(&self) -> u64 {
        // Every block should have a timestamp
        let ts = self.value["result"]["timestamp"].as_str().unwrap();
        convert_i64_from_hex(ts) as u64
    }

    pub fn number(&self) -> u64 {
        // Every block should have a number
        let number = self.value["result"]["number"].as_str().unwrap();
        convert_i64_from_hex(number) as u64
    }

    pub fn get_transactions(&self) -> Vec<TransactionJson> {
        let mut res = Vec::new();
        if let Some(transactions) = self.value["result"]["transactions"].as_array() {
            //This only makes sense because we don't have an internal repr that isn't related to
            //json. Once this is gone we just return proper values
            for transaction in transactions {
                res.push(TransactionJson {
                    value: transaction.clone(),
                });
            }
        }
        //If we don't have any transactions on result then this will fail silently, this shouldn't
        //happen without errors in client code
        res
    }
}

// Thin logic around transaction. Same as above. Would also need to be moved away from wrapper
// around serde_json.
#[derive(Debug)]
pub struct TransactionJson {
    value: serde_json::Value,
}

impl TransactionJson {
    pub fn hash(&self) -> Option<&str> {
        self.value["hash"].as_str()
    }

    pub fn input(&self) -> Option<&str> {
        self.value["input"].as_str()
    }

    pub fn from(&self) -> Option<&str> {
        self.value["from"].as_str()
    }

    pub fn to(&self) -> Option<&str> {
        self.value["to"].as_str()
    }

    pub fn value(&self) -> Option<u64> {
        self.value["value"].as_u64()
    }

    pub fn timestamp(&self) -> Option<i64> {
        self.value["timestamp"].as_i64()
    }

    pub fn is_contract_creation(&self) -> bool {
        if let Some(_val) = self.value["to"].as_null() {
            return true;
        }
        false
    }
}

#[derive(Debug)]
pub struct TransactionReceiptJson {
    value: serde_json::Value,
}

impl TransactionReceiptJson {
    pub fn new(value: serde_json::Value) -> Self {
        Self { value }
    }

    pub fn contract_address(&self) -> Option<&str> {
        self.value["result"]["contractAddress"].as_str()
    }
}

