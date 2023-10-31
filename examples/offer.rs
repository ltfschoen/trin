// Run with IPC
// Note: Only works if provide `--bootnodes default` as shown below
// ```
//   cargo build --workspace && \
//   RUST_LOG=DEBUG && \
//   TRIN_DATA_PATH=$HOME/.local/share/trin && \
//   cargo run -- \
//   --external-address 127.0.0.1:8995 \
//   --web3-ipc-path /tmp/trin-jsonrpc.ipc \
//   --ephemeral \
//   --discovery-port 8995 \
//   --bootnodes default
// ```
//
// Note: Bootnodes listed here: https://github.com/ethereum/portal-network-specs/blob/master/testnet.md
//
// Run
// ```bash
// cargo run --example offer
// ``` 

use std::error::Error;
use std::str::FromStr;
use anyhow::anyhow;
use anyhow::{Result};
use ethportal_peertest::constants::fixture_header_with_proof;
use ethportal_peertest::utils::wait_for_history_content;
use ethportal_api::{
    utils::bytes::hex_encode,
    Enr, HistoryContentValue, HistoryContentKey, HistoryNetworkApiClient, PossibleHistoryContentValue,
};
use ethportal_peertest as peertest;
use ethportal_peertest::Peertest;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Connect to a local node JSON-RPC with IPC
    let DEFAULT_WEB3_IPC_PATH = "/tmp/trin-jsonrpc.ipc";
    let client = reth_ipc::client::IpcClientBuilder::default()
        .build(DEFAULT_WEB3_IPC_PATH)
        .await
        .unwrap();
    println!("client {:#?}", client);

    let (content_key, content_value) = fixture_header_with_proof();

    // Store content to remote node, call portal_historyStore endpoint
    let result: bool = client
        .store(content_key.clone(), content_value.clone())
        .await
        .unwrap();
    assert!(result);

    // Call portal_historyLocalContent endpoint and deserialize to `HistoryContentValue::BlockHeaderWithProof` type
    let result: PossibleHistoryContentValue = client.local_content(content_key.clone()).await.unwrap();
    assert_eq!(result, PossibleHistoryContentValue::ContentPresent(content_value.clone()));

    // Run a client, as a buddy peer for ping tests, etc.
    let peertest = peertest::launch_peertest_nodes(1).await;
    println!("peertest node created with ENR {:?}", &peertest.bootnode.enr.to_base64());

    let enr = &peertest.bootnode.enr;
    // if the ENR is useful print it
    println!("Node Id: {}", enr.node_id());
    if enr.udp4_socket().is_some() {
        println!("Base64 ENR: {}", enr.to_base64());
        println!(
            "IP: {}, UDP_PORT:{}",
            enr.ip4().unwrap(),
            enr.udp4().unwrap()
        );
    } else {
        println!("ENR is not printed as no IP:PORT was specified");
    }

    let result = client
        .offer(
            Enr::from_str(&enr.to_base64()).unwrap(),
            content_key.clone(),
            Some(content_value.clone()),
        )
        .await
        .unwrap();

    // Check that ACCEPT response sent by bootnode accepted the offered content
    assert_eq!(hex_encode(result.content_keys.into_bytes()), "0x03");

    // Check if the stored content value in bootnode's DB matches the offered
    let response = wait_for_history_content(&peertest.bootnode.ipc_client, content_key).await;
    let received_content_value = match response {
        PossibleHistoryContentValue::ContentPresent(c) => c,
        PossibleHistoryContentValue::ContentAbsent => panic!("Expected content to be found"),
    };
    assert_eq!(
        content_value, received_content_value,
        "The received content {received_content_value:?}, must match the expected {content_value:?}",
    );

    Ok(())
}
