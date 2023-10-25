// Authors: Sogol Malek, Luke Schoen
// Original Source: https://github.com/sogolmalek/EIP-x/tree/main/my_discv5_app

// start a discovery v5 service and listens to events that the server emits.
// bootstrapped to a DHT by providing an ENR to add to its DHT.
//
// To run this example simply run:
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
// cargo run --example find_node
// ```

use std::error::Error;
use std::net::{SocketAddr, Ipv4Addr};
use async_std::task;
use anyhow::anyhow;
use anyhow::{Result};
use discv5::{enr, enr::{CombinedKey, Enr, EnrBuilder}, ConfigBuilder, Discv5, Event, ListenConfig, RequestError};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // reference: https://docs.rs/discv5/latest/discv5/#usage

    let address = "127.0.0.1".parse::<Ipv4Addr>().unwrap();
    let port = 9000; // or 8001

    // generate a new enr key
    let enr_key = CombinedKey::generate_secp256k1();

    // construct a local ENR
    let enr = EnrBuilder::new("v4")
        .ip4(address)
        .udp4(port)
        .build(&enr_key)
        .unwrap();

    // listening address and port
    let listen_config = ListenConfig::Ipv4 {
        ip: Ipv4Addr::UNSPECIFIED,
        port,
    };

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

    // default configuration
    let config = ConfigBuilder::new(listen_config).build();

    // construct the discv5 server
    let mut discv5: Discv5 = Discv5::new(enr, enr_key, config).unwrap();

    // in order to bootstrap the routing table an external ENR should be added
    // this can be done via `add_enr`

    // Existing remote peer running on port 8001
    // Obtain ths after running the node with the command shown above
    let existing_remote_peer = "enr:-Jy4QDAjCCWVxtgqqd5c_ObTqHVpovx56HJD4GYJYGGxW_pcRvmQbxpn4lENvMCl4ZAmz5vfpLoXO3FSBCkzAD3JrNoBY5Z0IDAuMS4xLWFscGhhLjEtZjNlYTFkgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQLiJVQ_hAjtXTK37nvdWjJZ5YwTLZxK0ChU5HHZNOpALoN1ZHCCH0E".parse::<Enr<CombinedKey>>().unwrap();

    let remote_peers: Vec<discv5::Enr> = vec![
        existing_remote_peer.clone(),
    ];
    // if we know of another peer's ENR, add it to known peers
    // using `add_enr` so the nodes are linked together
    for enr in remote_peers.iter() {
        println!(
            "Remote ENR read. udp4 socket: {:?}, udp6 socket: {:?}, tcp4_port {:?}, tcp6_port: {:?}, node id: {:?}",
            enr.udp4_socket(),
            enr.udp6_socket(),
            enr.tcp4(),
            enr.tcp6(),
            enr.node_id(),
        );
        if let Err(e) = discv5.add_enr(enr.clone()) {
            println!("Failed to add remote ENR {}", e);
            // It's unlikely we want to continue in this example after this
            return Err(anyhow!("Failed to add remote ENR {e:?}"));
        };
    }

    // start the discv5 service
    discv5.start().await.unwrap();
    println!("Server started on port {:#?}", port);

    // run a find_node query
    let found_nodes = discv5.find_node(existing_remote_peer.node_id()).await.unwrap();
    println!("Found nodes: {:?}", found_nodes);

    Ok(())
}
