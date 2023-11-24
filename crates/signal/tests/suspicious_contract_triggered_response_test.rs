use std::{collections::HashSet, fs::File, io::BufReader};

use insolvent_detect_signal::types::{BlockJson, Signal, AnonymouslyFundedSmartContractTriggeredSignal};

#[tokio::test]
async fn suspcious_contract_triggered_response_test() {
    // Load block from file that has a contract triggered event when also passed a HashSet
    // containing a list of contract addresses that have been marked as suspicious.
    //
    // The HashSet should be cached locally in prod.
    let mut signal_id = u32::MAX;

    let mut suspicious_contracts = HashSet::new();
    suspicious_contracts.insert("0x03e7b13bcd9b8383f403696c1494845560607eca".to_string());

    let file =
        File::open("tests/__data__/suspicious_contract_triggered_signal_response.json").unwrap();
    let reader = BufReader::new(file);
    let value: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let json_block = BlockJson::new(value);
    let suspicious_contract_triggered = Signal::AnonymouslyFundedSmartContractTriggered(AnonymouslyFundedSmartContractTriggeredSignal);
    if let Some(signal) = suspicious_contract_triggered.signal(&json_block, &suspicious_contracts).await {
        //Anonymous funded smart contract triggered signal has an id of 0
        signal_id = signal.0;
    }
    assert_eq!(signal_id, 0);
}
