use std::{fs::File, io::BufReader};

use insolvent_detect_signal::types::BlockJson;

#[test]
fn load_block_response_test() {
    // Load block from file and check whether main methods can run successfully
    let mut from_res = Vec::new();

    let file = File::open("tests/__data__/random_block_response.json").unwrap();
    let reader = BufReader::new(file);
    let value: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let json_block = BlockJson::new(value);
    for transaction in json_block.get_transactions() {
        from_res.push(transaction.from().unwrap().to_string());
    }
    assert!(from_res.len() > 0);
}
