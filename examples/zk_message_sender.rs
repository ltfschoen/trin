// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
// use ethportal_api::jsonrpsee::http_client::HttpClientBuilder;
// use ethportal_api::{HistoryContentValue, HistoryContentKey, HistoryNetworkApiClient, Web3ApiClient};

// #[derive(Debug)]
// enum ZkpError {
//     SerializationError(serde_json::Error),
//     ApiError(String),
// }

// // Define  ZeroKnowledgeProof struct
// #[derive(Debug, Serialize, Deserialize)]
// struct ZeroKnowledgeProof {
//     key_values: HashMap<String, String>,
//     block_number: u64,
//     data: Vec<u8>,
// }

// // Function to serialize ZeroKnowledgeProof to a JSON string
// fn serialize_zkp_message(zkp_message: &ZeroKnowledgeProof) -> Result<String, ZkpError> {
//     serde_json::to_string(zkp_message).map_err(ZkpError::SerializationError)
// }

#[tokio::main]
async fn main() {
//     // Configuration
//     let api_endpoint = "http://localhost:8545"; 

//     // Connect to a local node JSON-RPC
//     let client = HttpClientBuilder::default()
//         .build(api_endpoint)
//         .map_err(|e| ZkpError::ApiError(format!("Failed to connect to API endpoint: {}", e)))
//         .unwrap();

//     // Call web3_clientVersion endpoint
//     let client_version = client.client_version().await.unwrap();
//     println!("Connected to Ethereum node with client version: {}", client_version);

//     // Replace this with pre-generated ZeroKnowledgeProof
//     let pre_generated_zkp = ZeroKnowledgeProof {
//         key_values: HashMap::new(),
//         block_number: 12345,
//         data: vec![1, 2, 3, 4],
//     };

//     // Serialize the ZKP message to JSON
//     let zkp_message_json = match serialize_zkp_message(&pre_generated_zkp) {
//         Ok(json) => json,
//         Err(e) => {
//             eprintln!("Error serializing ZKP message: {:?}", e);
//             return;
//         }
//     };

//     // Deserialise to a portal history content key type from a hex string
//     let content_key_json =
//         r#""0x00cb5cab7266694daa0d28cbf40496c08dd30bf732c41e0455e7ad389c10d79f4f""#;
//     let content_key: HistoryContentKey = serde_json::from_str(content_key_json).unwrap();

//     // Store content to the remote node, call portal_historyStore endpoint with the pre-generated ZKP
//     match client.store_with_proof(content_key.clone(), pre_generated_zkp, zkp_message_json).await {
//         Ok(result) => {
//             if result {
//                 println!("ZKP successfully stored on the Ethereum node.");
//             } else {
//                 eprintln!("Failed to store ZKP on the Ethereum node.");
//             }
//         }
//         Err(e) => {
//             eprintln!("Error storing ZKP on the Ethereum node: {:?}", e);
//         }
//     }

//     // Call portal_historyLocalContent endpoint and deserialize to `HistoryContentValue::BlockHeaderWithProof` type
//     match client.local_content(content_key).await {
//         Ok(result) => {
//             println!("Retrieved ZKP from the Ethereum node: {:?}", result);
//         }
//         Err(e) => {
//             eprintln!("Error retrieving ZKP from the Ethereum node: {:?}", e);
//         }
//     }
}
