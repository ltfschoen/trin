// Run Client1 with IPC
// ```
//   rm -rf /tmp/trin-jsonrpc1.ipc && \
//   cargo build --workspace && \
//   RUST_LOG=INFO && \
//   cargo run -- \
//   --web3-ipc-path /tmp/trin-jsonrpc1.ipc \
//   --discovery-port 8995
// ```
//
// Run Client2 with IPC
// ```
//   rm -rf /tmp/trin-jsonrpc2.ipc && \
//   cargo build --workspace && \
//   RUST_LOG=INFO && \
//   cargo run -- \
//   --web3-ipc-path /tmp/trin-jsonrpc2.ipc \
//   --discovery-port 8996
// ```
//
// Run
// ```bash
// cargo run --example offer_manual
// ``` 

use std::error::Error;
use std::str::FromStr;
use anyhow::{anyhow, bail, Result};
use ethportal_api::{Discv5ApiClient, Web3ApiClient};
use ethportal_peertest::constants::fixture_header_with_proof;
use ethportal_peertest::utils::wait_for_history_content;
use ethportal_api::{
    utils::bytes::hex_encode,
    Enr, HistoryContentValue, HistoryContentKey, HistoryNetworkApiClient, PossibleHistoryContentValue,
};
use discv5::{enr::{CombinedKey as Discv5CombinedKey, Enr as Discv5Enr}};
use jsonrpsee::core::Error as JRErr;


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Connect to a client1 local node JSON-RPC with IPC
    let WEB3_IPC_PATH_1: &str = "/tmp/trin-jsonrpc1.ipc";
    let client1 = reth_ipc::client::IpcClientBuilder::default()
        .build(WEB3_IPC_PATH_1)
        .await
        .unwrap();

    let client1_version = client1.client_version().await.unwrap();
    println!("Current client1 version is {client1_version}");

    let client1_node_info = client1.node_info().await.unwrap();
    println!("client1_node_info {:#?}", client1.node_info().await.unwrap());

    let client1_enr = &client1_node_info.enr;
    println!("client1 Node Id: {}", client1_enr.node_id());
    if client1_enr.udp4_socket().is_some() {
        println!("client1 base64 ENR: {}", client1_enr.to_base64());
        println!(
            "client1 IP: {}, UDP_PORT:{}",
            client1_enr.ip4().unwrap(),
            client1_enr.udp4().unwrap()
        );
    } else {
        println!("client1 ENR is not printed as no IP:PORT was specified");
    }

    // Connect to a client1 local node JSON-RPC with IPC
    let WEB3_IPC_PATH_2: &str = "/tmp/trin-jsonrpc2.ipc";
    let client2 = reth_ipc::client::IpcClientBuilder::default()
        .build(WEB3_IPC_PATH_2)
        .await
        .unwrap();

    let client2_version = client2.client_version().await.unwrap();
    println!("Current client2 version is {client2_version}\n");

    let client2_node_info = client2.node_info().await.unwrap();
    println!("client2_node_info {:#?}\n", client2_node_info);

    let client2_enr = &client2_node_info.enr;
    println!("client2 Node Id: {}\n", client2_enr.node_id());
    if client2_enr.udp4_socket().is_some() {
        println!("client2 base64 ENR: {}", client2_enr.to_base64());
        println!(
            "client2 IP: {}, UDP_PORT:{}",
            client2_enr.ip4().unwrap(),
            client2_enr.udp4().unwrap()
        );
    } else {
        println!("client2 ENR is not printed as no IP:PORT was specified");
    }

    // Show routing tables before changes
    let client1_routing_table_info = HistoryNetworkApiClient::routing_table_info(&client1).await.unwrap();
    println!("client1 routing table info {:?}\n", client1_routing_table_info);

    let client2_routing_table_info = HistoryNetworkApiClient::routing_table_info(&client2).await.unwrap();
    println!("client2 routing table info {:?}\n", client2_routing_table_info);

    let enr_to_delete = "enr:-I24QDy_atpK3KlPjl6X5yIrK7FosdHI1cW0I0MeiaIVuYg3AEEH9tRSTyFb2k6lpUiFsqxt8uTW3jVMUzoSlQf5OXYBY4d0IDAuMS4wgmlkgnY0gmlwhKEjVaWJc2VjcDI1NmsxoQOSGugH1jSdiE_fRK1FIBe9oLxaWH8D_7xXSnaOVBe-SYN1ZHCCIyg".parse::<Discv5Enr<Discv5CombinedKey>>().unwrap();
    let enr_to_delete_node_id = enr_to_delete.node_id();

    // Delete an existing ENR from the routing table of client1 otherwise we cannot add
    // client2 because it will say its bucket is already full
    if let Err(JRErr::Custom(e)) = HistoryNetworkApiClient::delete_enr(&client1, enr_to_delete_node_id).await {
        println!("Cannot delete existing ENR from client1 as it may have already been deleted {}", e);
    } else {
        println!("Deleted existing ENR from client1 routing table");
    }

    if let Err(JRErr::Custom(e)) = HistoryNetworkApiClient::add_enr(&client1, client2_enr.clone()).await {
        bail!("Failed to add client2 ENR to client1 routing table");
    } else {
        println!("Added client2 ENR to client1 routing table");
    }

    // Delete an existing ENR from the routing table of client2 otherwise we cannot add
    // client1 because it will say its bucket is already full
    if let Err(JRErr::Custom(e)) = HistoryNetworkApiClient::delete_enr(&client2, enr_to_delete_node_id).await {
        println!("Cannot delete existing ENR from client2 as it may have already been deleted {}", e);
    } else {
        println!("Deleted existing ENR from client2 routing table");
    }

    if let Err(JRErr::Custom(e)) = HistoryNetworkApiClient::add_enr(&client2, client1_enr.clone()).await {
        bail!("Failed to add client1 ENR to client2 routing table");
    } else {
        println!("Added client1 ENR to client2 routing table");
    }

    // Check that routing table has been changed
    let client1_routing_table_info = HistoryNetworkApiClient::routing_table_info(&client1).await.unwrap();
    println!("client1 routing table info {:?}\n", client1_routing_table_info);

    let client2_routing_table_info = HistoryNetworkApiClient::routing_table_info(&client2).await.unwrap();
    println!("client2 routing table info {:?}\n", client2_routing_table_info);

    let (content_key, content_value) = fixture_header_with_proof();

    // Store content on client1 node, call portal_historyStore endpoint
    let result: bool = client1
        .store(content_key.clone(), content_value.clone())
        .await
        .unwrap();
    assert!(result);

    // Check content stored on client1 node.
    // Call portal_historyLocalContent endpoint and deserialize to `HistoryContentValue::BlockHeaderWithProof` type
    let result: PossibleHistoryContentValue = client1.local_content(content_key.clone()).await.unwrap();
    assert_eq!(result, PossibleHistoryContentValue::ContentPresent(content_value.clone()));

    // Offer content stored on client1 node to client2 node
    let result = client1
        .offer(
            Enr::from_str(&client2_enr.to_base64()).unwrap(),
            content_key.clone(),
            Some(content_value.clone()),
        )
        .await
        .unwrap();
    println!("content stored on client1 node was offered to client2 node");

    // Check that ACCEPT response sent by client2 accepted the offered content
    assert_eq!(hex_encode(result.content_keys.into_bytes()), "0x03");
    println!("client2 accepted the the content offered by client1");

    // Check if the stored content value in the client2 DB matches the content value offered by client1
    let response: PossibleHistoryContentValue = wait_for_history_content(&client2, content_key).await;
    let received_content_value = match response {
        PossibleHistoryContentValue::ContentPresent(c) => c,
        PossibleHistoryContentValue::ContentAbsent => panic!("Expected content to be found"),
    };
    assert_eq!(
        content_value, received_content_value,
        "The received content {received_content_value:?}, must match the expected {content_value:?}",
    );
    println!("client2 successfully stored the content value in its DB that matches the content value stored on client1");

    Ok(())
}
