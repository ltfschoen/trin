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
use ethers_providers::Provider;
use ethers_providers::Middleware;
// Run with HTTP
// ```bash
//   cargo build --workspace && \
//   RUST_LOG=DEBUG && \
//   TRIN_DATA_PATH=$HOME/.local/share/trin && \
//   cargo run -- \
//   --web3-http-address http://127.0.0.1:8545 \
//   --web3-transport http \
//   --discovery-port 9008 \
//   --external-address 127.0.0.1:9008 \
//   --ephemeral \
//   --bootnodes "enr:-I24QDy_atpK3KlPjl6X5yIrK7FosdHI1cW0I0MeiaIVuYg3AEEH9tRSTyFb2k6lpUiFsqxt8uTW3jVMUzoSlQf5OXYBY4d0IDAuMS4wgmlkgnY0gmlwhKEjVaWJc2VjcDI1NmsxoQOSGugH1jSdiE_fRK1FIBe9oLxaWH8D_7xXSnaOVBe-SYN1ZHCCIyg" \
//   --mb 200
// ```
//
// OR Run with IPC
// Note: Only works if provide `--bootnodes default` as shown below
// ```
//   cargo build --workspace && \
//   RUST_LOG=DEBUG && \
//   TRIN_DATA_PATH=$HOME/.local/share/trin && \
//   cargo run -- \
//   --external-address 127.0.0.1:8999 \
//   --web3-ipc-path /tmp/trin-jsonrpc.ipc \
//   --ephemeral \
//   --discovery-port 8999 \
//   --bootnodes default
// ```
//
// Run
// ```bash
// cargo run --example block_all
// ```
//
// Troubleshooting: If you're Trin node logs aren't displaying `Session established with Node: ...` and running this script outputs
// `RestartNeeded("Networking or low-level protocol error: bytes remaining on stream")` or 
// `RequestTimeout` or `Call(ErrorObject { code: ServerError(-32099), message: "Content not found", data: None })`
// then simply try using a different port (i.e. change 8999 to 8998) for both `--external-address` and `--discovery-port`
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a local node JSON-RPC with HTTP
    // let client = HttpClientBuilder::default()
    //     .build("http://localhost:8545")
    //     .unwrap();

    // Connect to a local node JSON-RPC with IPC
    let DEFAULT_WEB3_IPC_PATH = "/tmp/trin-jsonrpc.ipc";
    let client = reth_ipc::client::IpcClientBuilder::default()
        .build(DEFAULT_WEB3_IPC_PATH)
        .await
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

    // Block 17510000 has hash 0x15044f30b840d8621beee4f5f83b0a748fc38bacf65e667a1cad577d7c26147c
    let block_hash: H256 = H256::from_slice(
        &hex_decode("0x15044f30b840d8621beee4f5f83b0a748fc38bacf65e667a1cad577d7c26147c").unwrap(),
    );
    // FIXME - update Trin to support `hydrated_transactions = true` so it replies with all transaction bodies
    // see https://github.com/ethereum/trin/pull/982
    let hydrated_transactions = false;
    let block_by_hash: RethBlock = client.get_block_by_hash(block_hash, hydrated_transactions).await.unwrap();
    println!("block_by_hash {:#?}", block_by_hash);

    Ok(())
}
