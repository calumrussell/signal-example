use std::{fs::File, io::BufReader, collections::HashSet};

use insolvent_detect_signal::{types::{BlockJson, Event, TransferFromFixedFloatEvent}, api::InfuraAPIHttp};

#[tokio::test]
async fn fixed_float_deposit_response_test() {
    // Load block from file that has a transaction that deposits from fixed float, this
    // function shoould detect that
    let mut event_id = u32::MAX;

    let api = InfuraAPIHttp;
    let file = File::open("tests/__data__/fixed_float_deposit_response.json").unwrap();
    let reader = BufReader::new(file);
    let value: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let json_block = BlockJson::new(value);
    let transfer_from_fixed_float = Event::TransferFromFixedFloat(TransferFromFixedFloatEvent);
    if let Some(event) = transfer_from_fixed_float.event(&api, &json_block, &HashSet::new()).await {
        // Fixed float deposit has id of 2
        event_id = event.0;
    }
    assert_eq!(event_id, 2);
}
