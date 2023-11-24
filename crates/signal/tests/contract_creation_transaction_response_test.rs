use std::{fs::File, io::BufReader};

use insolvent_detect_signal::types::TransactionReceiptJson;

#[test]
fn contract_creation_transaction_response_test() {
    // Load transaction receipt from file, check that we are parsing this correctly
    let file = File::open("tests/__data__/contract_creation_transaction_response.json").unwrap();
    let reader = BufReader::new(file);
    let value: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let json_transaction_receipt = TransactionReceiptJson::new(value);
    assert_eq!(
        json_transaction_receipt.contract_address().unwrap(),
        "0x03e7b13bcd9b8383f403696c1494845560607eca"
    );
}
