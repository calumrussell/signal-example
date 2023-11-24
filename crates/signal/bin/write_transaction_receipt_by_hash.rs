use insolvent_detect_signal::api::{EthJsonRpc, InfuraAPIHttp};
use std::{env, fs::File, io::BufWriter};

/// Retrieves a single receipt and writes to file.
///
/// Used to create test data.
#[tokio::main]
pub async fn main() {
    let args: Vec<String> = env::args().collect();
    // Can only accept one transaction hash
    if args.len() != 2 {
        panic!("Accepts a single transaction hash");
    }

    let api = InfuraAPIHttp;
    if let Ok(transaction_hash) = args.get(1).unwrap().parse::<String>() {
        if let Ok(receipt) = api.get_transaction_receipt(&transaction_hash).await {
            let file = File::create("transaction.json").unwrap();
            let writer = BufWriter::new(file);
            // Tried to print to file here and there was an issue with how strings are escaped by
            // println!, easier although less flexible to do this
            let _ = serde_json::to_writer(writer, &receipt);
        }
    } else {
        panic!("Must pass String parseable value");
    }
}