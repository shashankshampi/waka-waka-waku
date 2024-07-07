use serde::{Deserialize, Serialize};
use std::error::Error;
use reqwest::{Client, header};

#[derive(Debug, Serialize, Clone)]
pub struct PublishMessage {
    payload: String,
    #[serde(rename = "contentTopic")]
    content_topic: String,
    timestamp: u64,
}

// Other structs and functions...

pub async fn verify_node_info() -> Result<(), Box<dyn Error>> {
    // Implementation of verify_node_info function
}

// Other HTTP client functions...
