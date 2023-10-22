use std::error::Error;
use ethereum_types::{H256, U256};
use ethers_core::types::{Block, U64};
use ethportal_api::jsonrpsee::http_client::HttpClientBuilder;
use ethportal_api::{
    EthApiClient, HistoryContentValue, HistoryContentKey, HistoryNetworkApiClient,
    PossibleHistoryContentValue, Web3ApiClient,
};
use ethportal_api::utils::bytes::hex_decode;
use reth_rpc_types::Block as RethBlock;
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
// cargo run --example block_all
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

    // Call eth_chainId endpoint using `EthApi` function `chain_id` from
    // rpc/src/eth_rpc.rs that requires dependency `EthApiClient` 
    let chain_id = client.chain_id().await.unwrap();
    println!("Current chain id is {:#?}", chain_id);
    // For now, the chain ID is always 1 -- portal only supports mainnet Ethereum
    assert_eq!(chain_id, U256::from(CHAIN_ID));

    let block_hash: H256 = H256::from_slice(
        &hex_decode("0xa6e1126374bb864d9b2f7a8483e3bc53647473a7dac4d8a3831bab63558a4acd").unwrap(),
    );
    // FIXME - update Trin to support `hydrated_transactions = true` so it replies with all transaction bodies
    let hydrated_transactions = false;
    let block_by_hash: RethBlock = client.get_block_by_hash(block_hash, hydrated_transactions).await.unwrap();
    println!("block_by_hash {:#?}", block_by_hash);

    Ok(())
}
