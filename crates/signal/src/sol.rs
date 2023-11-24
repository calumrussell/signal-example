use alloy_sol_types::{sol, SolCall};

sol!(
    TornadoCash,
    r#"[{
         "constant": false,
         "inputs": [
             {
                 "name": "_tornado",
                 "type": "address"
             },
             {
                 "name": "_proof",
                 "type": "bytes"
             },
             {
                 "name": "_root",
                 "type": "bytes32"
             },
             {
                 "name": "_nullifierHash",
                 "type": "bytes32"
             },
             {
                 "name": "_recipient",
                 "type": "address"
             },
             {
                 "name": "_relayer",
                 "type": "address"
             },
             {
                 "name": "_fee",
                 "type": "uint256"
             },
             {
                 "name": "_refund",
                 "type": "uint256"
             }
         ],
         "name": "withdraw",
         "outputs": [
            {
                "name": "",
                "type": "string"
            }
         ],
         "type": "function",
         "stateMutability": "view"
     }]"#
);

// Not sure how solidity code is going to integrate into this but this is a start.
pub(crate) struct TornadoCashSol;

impl TornadoCashSol {
    pub fn decode(hex: &[u8]) -> Result<TornadoCash::withdrawCall, alloy_sol_types::Error> {
        TornadoCash::withdrawCall::abi_decode(hex, false)
    }
}
