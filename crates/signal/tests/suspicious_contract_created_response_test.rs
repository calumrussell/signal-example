use std::{collections::HashSet, fs::File, io::BufReader};

use insolvent_detect_signal::{types::{BlockJson, Event, SuspiciousContractCreatedEvent}, api::InfuraAPIHttp};

#[tokio::test]
async fn suspcious_contract_created_response_test() {
    // Load block from file that has a contract creation event, this function should detect that
    // when also passed a HashSet containing a list of accounts funded from anon sources.
    //
    // The HashSet should be cached locally in prod.
    let mut event_id = u32::MAX;

    let api = InfuraAPIHttp;
    let mut suspicious_addresses = HashSet::new();
    suspicious_addresses.insert("0x864e656c57a5a119f332c47326a35422294db5c9".to_string());

    let file = File::open("tests/__data__/suspicious_contract_created_response.json").unwrap();
    let reader = BufReader::new(file);
    let value: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let json_block = BlockJson::new(value);
    let suspicious_contract_created = Event::SuspiciousContractCreated(SuspiciousContractCreatedEvent);
    if let Some(event) = suspicious_contract_created.event(&api, &json_block, &suspicious_addresses).await {
        // Suspicious contract created has id of 2
        event_id = event.0;
    }
    assert_eq!(event_id, 3);
}
