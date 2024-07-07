use serde::{Deserialize, Serialize};
use std::error::Error;
use reqwest::{Client, header};

/// Represents a message to be published.
#[derive(Debug, Serialize, Clone)]
pub struct PublishMessage {
    pub payload: String,                     // The payload of the message, base64 encoded.
    #[serde(rename = "contentTopic")]
    pub content_topic: String,               // The topic to which the message will be published.
    pub timestamp: u64,                      // The timestamp of the message in milliseconds since epoch.
}

/// Represents a message that has been published.
#[derive(Debug, Serialize, Deserialize)]
pub struct PublishedMessage {
    pub payload: String,                     // The payload of the published message, base64 encoded.
    #[serde(rename = "contentTopic")]
    pub content_topic: String,               // The topic under which the message is published.
    pub version: u64,                        // The version of the published message.
    pub timestamp: u64,                      // The timestamp of the message in milliseconds since epoch.
}

/// Verifies node information by making an HTTP GET request to the node's info endpoint.
///
/// # Returns
///
/// - `Ok(())` if the node information is retrieved successfully.
/// - `Err(Box<dyn Error>)` if there is an error in making the request or processing the response.
pub async fn verify_node_info() -> Result<(), Box<dyn Error>> {
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

/// Subscribes to a specified topic by making an HTTP POST request to the node's subscriptions endpoint.
///
/// # Parameters
///
/// - `topic`: A string slice that holds the name of the topic to subscribe to.
///
/// # Returns
///
/// - `Ok(())` if the subscription is successful.
/// - `Err(Box<dyn Error>)` if there is an error in making the request or processing the response.
pub async fn subscribe_to_topic(topic: &str) -> Result<(), Box<dyn Error>> {
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

/// Publishes a message to a specified topic by making an HTTP POST request to the node's messages endpoint.
///
/// # Parameters
///
/// - `message`: The `PublishMessage` struct containing the message details to be published.
///
/// # Returns
///
/// - `Ok(())` if the message is published successfully.
/// - `Err(Box<dyn Error>)` if there is an error in making the request or processing the response.
pub async fn publish_message(message: PublishMessage) -> Result<(), Box<dyn Error>> {
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

    let status = response.status();
    let body = response.text().await?;
    println!("Response body: {:?}", body);

    if status.is_success() {
        println!("Published message: {:?}", message);
    } else {
        println!("Failed to publish message: {}", status);
        println!("Response body: {:?}", body);
    }

    Ok(())
}

/// Confirms that a message has been published to a specified topic by making an HTTP GET request to the node's messages endpoint.
///
/// # Parameters
///
/// - `topic`: A string slice representing the topic to check for published messages.
/// - `expected_message`: A reference to a `PublishMessage` struct representing the expected message to confirm.
///
/// # Returns
///
/// - `Ok(())` if the expected message is found in the list of published messages.
/// - `Err(Box<dyn Error>)` if there is an error in making the request or processing the response, or if the message is not found.
pub async fn confirm_message_publication(topic: &str, expected_message: &PublishMessage) -> Result<(), Box<dyn Error>> {
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

        let published_messages: Vec<PublishedMessage> = serde_json::from_str(&body)?;

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

/// Verifies if Node 2 is auto-connected by making an HTTP GET request to the node's peers endpoint.
///
/// # Returns
///
/// - `Ok(())` if Node 2 is connected and the protocol details are available.
/// - `Err(Box<dyn Error>)` if there is an error in making the request or processing the response, or if Node 2 is not connected.
pub async fn verify_node2_autoconnection() -> Result<(), Box<dyn Error>> {
    let url = "http://localhost:21261/admin/v1/peers";
    let client = Client::new();
    let response = client.get(url).send().await?;

    println!("\nEquivalent curl command:\n");
    println!("curl --location '{}'", url);
    println!();

    if response.status().is_success() {
        let peers_info: serde_json::Value = response.json().await?;
        println!("Peers info: {:?}", peers_info);

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

/// Subscribes Node 2 to a specified topic by making an HTTP POST request to Node 2's subscriptions endpoint.
///
/// # Parameters
///
/// - `topic`: A string slice representing the topic to which Node 2 should subscribe.
///
/// # Returns
///
/// - `Ok(())` if the subscription is successful.
/// - `Err(Box<dyn Error>)` if there is an error in making the request or processing the response.
pub async fn subscribe_node2_to_topic(topic: &str) -> Result<(), Box<dyn Error>> {
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

/// Test suite 1 that performs a series of actions such as verifying node info, subscribing to topics, publishing messages, and confirming message publication.
///
/// # Returns
///
/// - `Ok(())` if all actions in the test suite are completed successfully.
/// - `Err(Box<dyn Error>)` if there is an error in any of the actions.
pub async fn test_suite_1() -> Result<(), Box<dyn Error>> {
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

    if let Err(e) = verify_node2_autoconnection().await {
        eprintln!("Error verifying Node 2 autoconnection: {}", e);
    }

    if let Err(e) = subscribe_node2_to_topic("/my-app/2/chatroom-1/proto").await {
        eprintln!("Error subscribing Node 2 to topic: {}", e);
    }

    if let Err(e) = publish_message(message.clone()).await {
        eprintln!("Error publishing message from Node 1: {}", e);
    }

    if let Err(e) = confirm_message_publication("/my-app/2/chatroom-1/proto", &message).await {
        eprintln!("Error confirming message publication for Node 2: {}", e);
    }

    Ok(())
}

/// Test suite 2 that performs actions to verify Node 2's autoconnection, subscribes to topics, publishes messages, and confirms message publication.
///
/// # Returns
///
/// - `Ok(())` if all actions in the test suite are completed successfully.
/// - `Err(Box<dyn Error>)` if there is an error in any of the actions.
pub async fn test_suite_2() -> Result<(), Box<dyn Error>> {

    let message = PublishMessage {
        payload: "UmVsYXkgd29ya3MhIQ==".to_string(),
        content_topic: "/my-app/2/chatroom-1/proto".to_string(),
        timestamp: 0,
    };

    if let Err(e) = verify_node2_autoconnection().await {
        eprintln!("Error verifying Node 2 autoconnection: {}", e);
    }

    if let Err(e) = subscribe_node2_to_topic("/my-app/2/chatroom-1/proto").await {
        eprintln!("Error subscribing Node 2 to topic: {}", e);
    }

    if let Err(e) = publish_message(message.clone()).await {
        eprintln!("Error publishing message from Node 1: {}", e);
    }

    if let Err(e) = confirm_message_publication("/my-app/2/chatroom-1/proto", &message).await {
        eprintln!("Error confirming message publication for Node 2: {}", e);
    }

    Ok(())
}
