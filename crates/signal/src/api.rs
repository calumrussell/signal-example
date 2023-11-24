use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

#[async_trait::async_trait]
pub trait EthJsonRpc {
    async fn get_transaction_receipt(&self, hash: &str) -> Result<Value, Box<dyn std::error::Error>>;
    async fn get_block_by_number_hash(
        &self,
        block_number: u128,
    ) -> Result<Value, Box<dyn std::error::Error>>;
    async fn get_block_by_number_latest(&self) -> Result<Value, Box<dyn std::error::Error>>;
}

/// Need to consider standardization here.
///
/// Do we want a JsonRpcRequest type for each type of request to eth source? Or are these
/// standard structs enough?

///Because ETH JSON-RPC API is fixed this should be reusable code
#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: String,
    result: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub id: String,
    pub params: Vec<MultipleTypes>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum MultipleTypes {
    Str(String),
    Bool(bool),
    Int(i64),
}

/// This should produce something like [JsonRpcRequest] because these are mostly standardized across
/// the API.
struct JsonRpcApiRequestBuilder;

impl JsonRpcApiRequestBuilder {
    fn get_transaction_receipt(hash: &str) -> JsonRpcRequest {
        JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: "eth_getTransactionReceipt".to_string(),
            params: vec![MultipleTypes::Str(hash.to_string())],
        }
    }

    fn get_block_by_number_latest() -> JsonRpcRequest {
        JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: "eth_getBlockByNumber".to_string(),
            params: vec![
                MultipleTypes::Str("latest".to_string()),
                MultipleTypes::Bool(true),
            ],
        }
    }

    fn get_block_by_number_hash(block_number: u128) -> JsonRpcRequest {
        let mut block_hex = format!("{:x}", block_number);
        block_hex.insert_str(0, "0x");

        JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: "eth_getBlockByNumber".to_string(),
            params: vec![MultipleTypes::Str(block_hex), MultipleTypes::Bool(true)],
        }
    }
}

#[derive(Debug)]
struct InfuraAPIHttpJsonError;

impl std::error::Error for InfuraAPIHttpJsonError {}

impl std::fmt::Display for InfuraAPIHttpJsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error when tryign to unpack text response")
    }
}

/// Produces transactions/blocks/smart contracts/etc. Should have little logic beyond wrapping the
/// implementation details of how these things are fetched. Because eth API is standardized across
/// nodes, this logic should also remain abstracted away.
///
/// Currently, this is tightly bound to serde_json and the block module. In practice, it would be
/// ideal to build out types that can be passed around internally that are totally separated from
/// the method of calling but this will require a lot deeper research into eth API.
///
/// Request management should also be wrapped here i.e. the re-use of reqwest::Client (or the re-use)
/// of a WSS impl, etc.
pub struct InfuraAPIHttp;

impl InfuraAPIHttp {
    fn path() -> String {
        let token = match env::var_os("TOKEN") {
            Some(v) => v.into_string().unwrap(),
            None => panic!("$TOKENis not set"),
        };
        return "https://mainnet.infura.io/v3/".to_owned() + &token;
    }
}

#[async_trait::async_trait]
impl EthJsonRpc for InfuraAPIHttp {
    async fn get_transaction_receipt(&self, hash: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let path = InfuraAPIHttp::path();
        let req = JsonRpcApiRequestBuilder::get_transaction_receipt(hash);

        //Possible perf implications of doing this, I don't know what this does
        let client = reqwest::Client::new();
        let resp = client.post(path).json(&req).send().await?;
        if let Ok(txt) = resp.text().await {
            Ok(serde_json::from_str(&txt)?)
        } else {
            Err(Box::new(InfuraAPIHttpJsonError))
        }
    }

    async fn get_block_by_number_latest(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let path = InfuraAPIHttp::path();
        let req = JsonRpcApiRequestBuilder::get_block_by_number_latest();

        //Possible perf implications of doing this, I don't know what this does
        let client = reqwest::Client::new();
        let resp = client.post(path).json(&req).send().await?;
        if let Ok(txt) = resp.text().await {
            Ok(serde_json::from_str(&txt)?)
        } else {
            Err(Box::new(InfuraAPIHttpJsonError))
        }
    }

    async fn get_block_by_number_hash(
        &self,
        block_number: u128,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let path = InfuraAPIHttp::path();
        let req = JsonRpcApiRequestBuilder::get_block_by_number_hash(block_number);

        //Possible perf implications of doing this, I don't know what this does
        let client = reqwest::Client::new();
        let resp = client.post(path).json(&req).send().await?;
        if let Ok(txt) = resp.text().await {
            Ok(serde_json::from_str(&txt)?)
        } else {
            Err(Box::new(InfuraAPIHttpJsonError))
        }
    }
}
