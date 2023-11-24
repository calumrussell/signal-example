use std::collections::HashSet;

use insolvent_detect_signal::api::{ EthJsonRpc, InfuraAPIHttp};
use insolvent_detect_signal::types::{Event, TransferFromFixedFloatEvent, Signal, AnonymouslyFundedSmartContractTriggeredSignal, BlockJson};

//Not working, needs proper DB setup
#[tokio::main]
pub async fn main() {
    // Load caches
    let suspicious_addresses = HashSet::new();
    let suspicious_contracts= HashSet::new();
    let api = InfuraAPIHttp;

    let events: Vec<Event> = vec![
        Event::TransferFromFixedFloat(TransferFromFixedFloatEvent)
    ];

    let signals: Vec<Signal> = vec![
        Signal::AnonymouslyFundedSmartContractTriggered(AnonymouslyFundedSmartContractTriggeredSignal)
    ];

    // Fetch blocks
    loop {
        if let Ok(block_raw) = api.get_block_by_number_latest().await {
            let block_json = BlockJson::new(block_raw);
            for event in &events {
                let _res = event.event(&api, &block_json, &suspicious_addresses).await;
            }
            for signal in &signals {
                let _res = signal.signal(&block_json, &suspicious_contracts).await;
            }
        }
    }
}





