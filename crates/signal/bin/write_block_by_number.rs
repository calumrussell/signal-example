use insolvent_detect_signal::api::{EthJsonRpc, InfuraAPIHttp};
use std::{env, fs::File, io::BufWriter};

/// Retrieves a single block and writes to file.
///
/// Used to create test data.
#[tokio::main]
pub async fn main() {
    let args: Vec<String> = env::args().collect();
    // Can only accept one block
    if args.len() != 2 {
        panic!("Accepts a single block");
    }

    let api = InfuraAPIHttp;
    if let Ok(block_number) = args.get(1).unwrap().parse::<u128>() {
        if let Ok(block) = api.get_block_by_number_hash(block_number).await {
            let file = File::create("block.json").unwrap();
            let writer = BufWriter::new(file);
            // Tried to print to file here and there was an issue with how strings are escaped by
            // println!, easier although less flexible to do this
            let _ = serde_json::to_writer(writer, &block);
        }
    } else {
        panic!("Must pass u128 parseable value");
    }
}