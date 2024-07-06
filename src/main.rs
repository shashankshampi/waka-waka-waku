use serde::{Deserialize, Serialize};
use std::error::Error;
use reqwest::{Client, header};

#[derive(Debug, Serialize, Clone)] // Derive Clone trait here
struct PublishMessage {
    payload: String,
    #[serde(rename = "contentTopic")] // Ensures correct field name in JSON
    content_topic: String,
    timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct PublishedMessage {
    payload: String,
    #[serde(rename = "contentTopic")]
    content_topic: String,
    version: u64,
    timestamp: u64,
}

async fn verify_node_info() -> Result<(), Box<dyn Error>> {
    let url = "http://localhost:21161/debug/v1/info";
    let client = Client::new();
    let response = client.get(url).send().await?;

    println!("\nEquivalent curl command:\n");
    println!("curl --location '{}'", url);
    println!();

    if response.status().is_success() {
        let info: serde_json::Value = response.json().await?;
        println!("Node info: {:?}", info);
    } else {
        println!("Failed to get node info: {}", response.status());
        println!("Response body: {:?}", response.text().await?);
    }

    Ok(())
}


async fn subscribe_to_topic(topic: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://localhost:21161/relay/v1/auto/subscriptions";
    let client = Client::new();
    let payload = vec![topic];

    println!("\nEquivalent curl command:\n");
    println!("curl --location '{}'", url);
    println!(" --header 'accept: text/plain'");
    println!(" --header 'content-type: application/json'");
    println!(" -d '{}'", serde_json::to_string(&payload)?);
    println!();

    let payload_string = serde_json::to_string(&payload)?;

    let response = client
        .post(url)
        .header(header::ACCEPT, "text/plain")
        .header(header::CONTENT_TYPE, "application/json")
        .body(payload_string)
        .send()
        .await?;

    println!("Response status: {:?}", response.status());

    // Capture status and body before moving `response` into `text().await?`
    let status = response.status();
    let body = response.text().await?;

    println!("Response body: {:?}", body);

    if status.is_success() {
        println!("Subscribed to topic: {}", topic);
    } else {
        println!("Failed to subscribe to topic {}: {}", topic, status);
    }

    Ok(())
}

async fn publish_message(message: PublishMessage) -> Result<(), Box<dyn Error>> {
    let url = "http://127.0.0.1:21161/relay/v1/auto/messages";
    let client = Client::new();

    println!("\nEquivalent curl command:\n");
    println!("curl --location '{}'", url);
    println!("--header 'content-type: application/json'");
    println!("-d '{}'", serde_json::to_string(&message)?);
    println!();

    let payload_string = serde_json::to_string(&message)?;

    let response = client.post(url)
        .body(payload_string)
        .header("content-type", "application/json")
        .send()
        .await?;

    println!("Response status: {:?}", response.status());

    // Capture status and body before moving `response` into `text().await?`
    let status = response.status();
    let body = response.text().await?;
    println!("Response body: {:?}", body);

    if status.is_success() {
        println!("Published message: {:?}", message);
    } else {
        println!("Failed to publish message: {}", status);
        println!("Response body: {:?}", status);
    }

    Ok(())
}

async fn confirm_message_publication(topic: &str, expected_message: &PublishMessage) -> Result<(), Box<dyn Error>> {
    let encoded_topic = urlencoding::encode(topic);
    let url = format!("http://127.0.0.1:21161/relay/v1/auto/messages/{}", encoded_topic);
    let client = Client::new();

    println!("\nEquivalent curl command:\n");
    println!("curl --location '{}'", url);
    println!();

    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let body = response.text().await?;
        println!("confirm_message_publication body: {}", body);

        // Deserialize the JSON response into a vector of PublishedMessage
        let published_messages: Vec<PublishedMessage> = serde_json::from_str(&body)?;

        // Check if expected_message matches any published_message in the response
        let mut found = false;
        for msg in published_messages {
            if msg.payload == expected_message.payload
                && msg.content_topic == expected_message.content_topic {
                println!("Confirmed message publication for topic: {}", topic);
                println!("Published message details: {:?}", msg);
                found = true;
                break;
            }
        }

        if !found {
            println!("Published message for topic {} not found in response.", topic);
        }
    } else {
        println!("Failed to confirm message publication for topic {}: {}", topic, response.status());
        println!("Response body: {:?}", response.text().await?);
    }

    Ok(())
}


// Test 2

async fn verify_node2_autoconnection() -> Result<(), Box<dyn Error>> {
    let url = "http://localhost:21261/admin/v1/peers";
    let client = Client::new();
    let response = client.get(url).send().await?;

    println!("\nEquivalent curl command:\n");
    println!("curl --location '{}'", url);
    println!();

    if response.status().is_success() {
        let peers_info: serde_json::Value = response.json().await?;
        println!("Peers info: {:?}", peers_info);

        // Assuming there's only one peer for simplicity
        if let Some(peer) = peers_info.as_array().and_then(|arr| arr.get(0)) {
            if let Some(protocols) = peer["protocols"].as_array() {
                for protocol in protocols {
                    if protocol["protocol"] == "/vac/waku/relay/2.0.0" && protocol["connected"] == true {
                        println!("Node 2 is connected.");
                        return Ok(());
                    }
                }
            }
        }
    }

    println!("Node 2 is not connected or information not found.");
    Ok(())
}

async fn subscribe_node2_to_topic(topic: &str) -> Result<(), Box<dyn Error>> {
    let url = "http://localhost:21261/relay/v1/auto/subscriptions";
    let client = Client::new();
    let payload = vec![topic];

    println!("\nEquivalent curl command:\n");
    println!("curl --location '{}'", url);
    println!(" --header 'accept: text/plain'");
    println!(" --header 'content-type: application/json'");
    println!(" -d '{}'", serde_json::to_string(&payload)?);
    println!();

    let payload_string = serde_json::to_string(&payload)?;

    let response = client
        .post(url)
        .header(header::ACCEPT, "text/plain")
        .header(header::CONTENT_TYPE, "application/json")
        .body(payload_string)
        .send()
        .await?;

    println!("Response status: {:?}", response.status());

    let status = response.status();
    let body = response.text().await?;
    println!("Response body: {:?}", body);

    if status.is_success() {
        println!("Subscribed Node 2 to topic: {}", topic);
    } else {
        println!("Failed to subscribe Node 2 to topic {}: {}", topic, status);
    }

    Ok(())
}

//  test 2 ends

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Example usage
    if let Err(e) = verify_node_info().await {
        eprintln!("Error verifying node info: {}", e);
    }

    if let Err(e) = subscribe_to_topic("/my-app/2/chatroom-1/proto").await {
        eprintln!("Error subscribing to topic: {}", e);
    }

    let message = PublishMessage {
        payload: "UmVsYXkgd29ya3MhIQ==".to_string(),
        content_topic: "/my-app/2/chatroom-1/proto".to_string(),
        timestamp: 0,
    };

    if let Err(e) = publish_message(message.clone()).await {
        eprintln!("Error publishing message: {}", e);
    }

    if let Err(e) = confirm_message_publication("/my-app/2/chatroom-1/proto", &message).await {
        eprintln!("Error confirming message publication: {}", e);
    }

    // Test 2: Node 2 integration tests
    if let Err(e) = verify_node2_autoconnection().await {
        eprintln!("Error verifying Node 2 autoconnection: {}", e);
    }

    if let Err(e) = subscribe_node2_to_topic("/my-app/2/chatroom-1/proto").await {
        eprintln!("Error subscribing Node 2 to topic: {}", e);
    }

    let message = PublishMessage {
        payload: "UmVsYXkgd29ya3MhIQ==".to_string(),
        content_topic: "/my-app/2/chatroom-1/proto".to_string(),
        timestamp: 0,
    };

    if let Err(e) = publish_message(message.clone()).await {
        eprintln!("Error publishing message from Node 1: {}", e);
    }

    if let Err(e) = confirm_message_publication("/my-app/2/chatroom-1/proto",&message).await {
        eprintln!("Error confirming message publication for Node 1: {}", e);
    }

    if let Err(e) = publish_message(message.clone()).await {
        eprintln!("Error publishing message from Node 1: {}", e);
    }

    if let Err(e) = confirm_message_publication("/my-app/2/chatroom-1/proto",&message).await {
        eprintln!("Error confirming message publication for Node 2: {}", e);
    }

    Ok(())
}
