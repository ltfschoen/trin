// Author: Sogol Malek

use async_std::task;
use discv5::{NodeDiscovery, NodeRecord, CombinedKey, CombinedKeyExt, Config, Discv5Error};
use std::net::{SocketAddr, Ipv4Addr};
use std::time::Duration;

async fn send_findnode_request(
    node_discovery: &NodeDiscovery,
    node: &NodeRecord,
    zkp_message: &[u8],
) -> Result<(), Discv5Error> {
    let request_id: [u8; 8] = rand::random(); // Generate a random request ID
    let distances = vec![0, 1, 2]; // Logarithmic distances to query

    // Generate the FINDNODE Request message
    let request = node_discovery.findnode(request_id, distances)?;

    // Extract the node's address and port
    let address = node.udp_socket().unwrap().local_addr().unwrap();

    // Append the ZKP message to the request as payload
    let request_with_payload = [&request[..], zkp_message].concat();

    // Send the request to the node
    send_udp_request(&address, &request_with_payload).await
}

async fn send_udp_request(address: &SocketAddr, request: &[u8]) -> Result<(), Discv5Error> {
    let socket = async_std::net::UdpSocket::bind("0.0.0.0:0").await?;
    socket.send_to(request, address).await?;
    Ok(())
}

fn main() {
    task::block_on(async {
        // Generate your local ENR and configure Node Discovery
        let local_key = CombinedKey::generate_secp256k1();
        let local_enr = local_key.generate_enr().unwrap();

        let config = Config {
            local_key,
            local_peer_id: local_enr.node_id(),
            listen_address: Some("0.0.0.0:9000".parse::<SocketAddr>().unwrap()),
            bootnodes: vec![SocketAddr::new(Ipv4Addr::new(1, 2, 3, 4), 30303)], // Replace with actual bootnodes
            request_timeout: Duration::from_secs(5),
            max_request_retries: 3,
            ..Config::default()
        };

        let mut node_discovery = NodeDiscovery::new(local_enr, config).unwrap();

        // Example ZKP message
        let zkp_message = b"Sample ZKP message";

        // Replace with your actual list of connected Ethereum nodes
        let connected_nodes: Vec<NodeRecord> = vec![
            // NodeRecord 1
            NodeRecord::new(
                // Replace with the actual NodeRecord details
                // Example: Node ID, ENR, IP, Port, etc.
            ),
            // NodeRecord 2
            NodeRecord::new(
                // Replace with the actual NodeRecord details
                // Example: Node ID, ENR, IP, Port, etc.
            ),
            // Add more NodeRecords as needed
        ];

        for node in connected_nodes {
            match send_findnode_request(&node_discovery, &node, zkp_message).await {
                Ok(_) => {
                    println!(
                        "FINDNODE Request sent successfully to node: {:?}",
                        node.udp_socket().unwrap().local_addr().unwrap()
                    )
                }
                Err(err) => eprintln!(
                    "Failed to send FINDNODE Request to node {:?}: {:?}",
                    node.udp_socket().unwrap().local_addr().unwrap(),
                    err
                ),
            }
        }
    });
}
