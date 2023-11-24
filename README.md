# Dev notes:

* Events create condition for signals. In order to find a suspicious contract call we first need to identify suspicious addresses then identify the contracts that these addresses create. The tests use the example in docs/suspicious_contract_call_signal.md.
* Tests require network connection and Infura token, not ideal but wanted to get things up and be sure they were working
* Events can call the API so they aren't strictly pure. When a smart contract is created we need to call eth_getTransactionReceipt to get contract address, I wasn't sure whether it was a good idea to let events make further API calls but added this so we could get it working. Because we need async, there are relatively few perf implications...all events/signals can be called async, I think the only footgun here is when we update cache.

## Testing

Tests currently require Infura token because of the dependency of ID:3 event on eth_getTransactionReceipt.

`TOKEN=<token> cargo test`

## DB

Currently uses sqlx. Because of an issue with Supabase the db can't be reset.

Setup: `cargo install sqlx-cli`

Add migration: `sqlx migrate add <name>`

Run migration: `sqlx migrate run`

Create reversible migrations: `sqlx migrated add -r <name>`

Revert migration: `sqlx migrate revert`

## Binaries

write_block_by_number_json - gets a block from API and writes `block.json` file to CWD, used to get the right blocks for testing

`TOKEN=<token> cargo run --bin write_block_by_number_json <block_number>`

write_transaction_receipt_by_hash_json - gets a transaction receipt from API and writes `transaction.json` file to CWD, used to get the right receipts for testing

`TOKEN=<token> cargo run --bin write_transaction_receipt_by_hash_json <transaction_hash>`

runner - this is an example of the event loop showing how everything fits together, not functional

# roadmap to PoC (getting up to what defimon has)

# Configure  Reth Node: 
 - sync node ( takes time), so likely need to  use provider like infura then switch over.
 - configure node to specs / manage peer quality
 - archive node or full node ? 
# Use eth_getCode JSON-RPC Method:
- event listener : new contract created 
- event listener: large transaction 
- sends an eth_getCode JSON-RPC request to Reth node (provider in interim)
# Stolen from defimon's docs  but we will definitely need to implement most of these methods
 the idea here is to extract information that we can use to determine if a contract is malicious or not based on 
  a function's name, the solidity code, the opcodes, and other factors.

Chain access

get_balance(<hexstring>) -> u256 - return the balance of a given address

is_contract(<hexstring>) -> bool - check if the provided address contains a contract

resolve_name(<string lit>) -> hexstring - resolve an ENS address into an address hexstring

read_slot(addr: <hexstring>, slot: <u256 lit>) -> u256 - read the value from a slot location of the given address

static_call(addr: <hexstring>, function_name: <string lit>, param_types: <string lit>, param_values: <string lit>) -> ethabi - perform a constant call (on functions marked as view or pure) with param_types and param_values specified with ethabi syntax, and function_name a method of the contract (e.g. `static_call(<addr>, "name", "bytes", "dead..beef")), return an ethabi format token value

static_call_hash(addr: <hexstring>, function_hash: <string lit>, param_types: <string lit>, param_values: <string lit>) -> ethabi - perform a constant call (on functions marked as view or pure) with param_types and param_values specified with ethabi syntax, and function_hash the 4-byte selector signature of the function in hexadecimal (e.g. `static_call(<addr>, "3b3b57de", "bytes", "dead..beef")), return an ethabi format token value

static_call_repr(addr: <hexstring>, function_name: <string lit>, param_types: <string lit>, param_values: <string lit>) -> bytes - perform a constant call (on functions marked as view or pure) with param_types and param_values specified with ethabi syntax, and function_name a method of the contract (e.g. `static_call(<addr>, "name", "bytes", "dead..beef")), return an ethabi-formatted value string representation

static_call_hash_repr(addr: <hexstring>, function_hash: <string lit>, param_types: <string lit>, param_values: <string lit>) -> bytes - perform a constant call (on functions marked as view or pure) with param_types and param_values specified with ethabi syntax, and function_hash the 4-byte selector signature of the function in hexadecimal (e.g. static_call(<addr>, "3b3b57de", "bytes", "dead..beef")), return an ethabi-formatted value string representation

get_contract_events(addr: <hexstring>, event_def: <string lit>, topics: <string lit>) -> array<map<ethabi>> - get the event logs of a contract, filtered by the provided topics, with event_def holding a solidity event definition (e.g. event Transfer(address indexed from, address indexed to, uint value)), and the topics filter represented as an ethabi-formatted literal of a tuple of arrays containing the values, with an empty array representing any value. Returns an array of maps of ethabi values, representing the list of matching events


# Implement a severity scoring system to rate the suspiciousness of a contract based on our criteria.
-
- Analyze the decompiled opcode to determine if the contract is funded by Tornado Cash. look for specific function names, code patterns, or other indicators (Natspec, logs, function types, events emitted).
- outside of the tornado cash bit, Defimon also gives High severity to 
  • msg.value multicalls ( "allocate with constant msg.value can be called multiple times")
  • no slippage check ( "no slippage check on uniswap")
  • solidity.encode-packed-collision	(abi.encodePacked hash collision with variable length arguments in deploy())
  • solidity.unrestricted-transferownership	(Unrestricted transferOwnership) 
  • solidity.incorrect-use-of-blockhash	blockhash(block.number) and blockhash(block.number + N) always returns 0.
- defimon applies a medium risk severity to these: 
   • solidity.uniswap-callback-not-protected	Uniswap callback is not protected


- impl function that scans a user-provided transaction hash for the contract creation event and then calls the eth_getCode JSON-RPC method to extract the contract bytecode and checks for signals.

# Data Persistence and caching
- store the results of the analysis in a database (likely postgres) so that we can query the results later.
- Decision needs to be made about what is actually persisted. Likely only information about contracts that are deemed malicious will be persisted.

# Backtesting 
- run the analysis on a set of known malicious contracts to see if the system would have flagged them as malicious.
- run the analysis on a set of known benign contracts to see if the system would have flagged them as malicious.

# FE 