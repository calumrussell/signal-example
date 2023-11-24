use std::{fs::File, io::BufReader, collections::HashSet};

use insolvent_detect_signal::{types::{BlockJson, Event, TornadoCashWithdrawEvent}, api::InfuraAPIHttp};

#[tokio::test]
async fn tornado_cash_block_response_test() {
    // Load block from file that has a tornado cash withdrawal transaction, this function
    // should detect that
    let mut event_id = u32::MAX;

    let api = InfuraAPIHttp;
    let file = File::open("tests/__data__/tornado_cash_block_response.json").unwrap();
    let reader = BufReader::new(file);
    let value: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let json_block = BlockJson::new(value);
    let tornado_cash_withdraw = Event::TornadoCashWithdraw(TornadoCashWithdrawEvent);
    if let Some(event) = tornado_cash_withdraw.event(&api, &json_block, &HashSet::new()).await {
        // Tornado cash withdraw has id of 1
        event_id = event.0;
    }
    assert_eq!(event_id, 1);
}
