use std::error::Error;
use ethportal_api::jsonrpsee::http_client::HttpClientBuilder;
use ethportal_api::{
    EthApiClient, HistoryContentValue, HistoryContentKey, HistoryNetworkApiClient,
    PossibleHistoryContentValue, Web3ApiClient,
};
use ethereum_types::{U256};
use trin_validation::constants::CHAIN_ID;

// Run
// ```bash
//   cargo build --workspace && \
//   RUST_LOG=DEBUG && \
//   TRIN_DATA_PATH=$HOME/.local/share/trin && \
//   cargo run -- \
//   --web3-http-address http://127.0.0.1:8547 \
//   --web3-transport http \
//   --discovery-port 8001 \
//   --external-address 127.0.0.1:8001 \
//   --bootnodes default \
//   --mb 200
// ```
//
// Run
// ```bash
// cargo run --example chain_id
// ```
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a local node JSON-RPC
    let client = HttpClientBuilder::default()
        .build("http://localhost:8547")
        .unwrap();
    println!("client {:#?}", client);

    // Call web3_clientVersion endpoint
    let client_version = client.client_version().await.unwrap();
    println!("Current client version is {client_version}");

    // Call eth_chainId endpoint from `EthApiClient` 
    let chain_id = client.chain_id().await.unwrap();
    println!("Current chain id is {:#?}", chain_id);
    // For now, the chain ID is always 1 -- portal only supports mainnet Ethereum
    assert_eq!(chain_id, U256::from(CHAIN_ID));

    Ok(())
}
