use std::error::Error;
use waka_waka_waku::*;

/// Test Suite 1: Basic Node Operation (easy)
#[tokio::test]
async fn node1_test1_verify_node_info() -> Result<(), Box<dyn Error>> {
    verify_node_info().await?;
    Ok(())
}

#[tokio::test]
async fn node1_test2_subscribe_to_topic() -> Result<(), Box<dyn Error>> {
    subscribe_to_topic("/my-app/2/chatroom-1/proto").await?;
    Ok(())
}

#[tokio::test]
async fn node1_test3_publish_message() -> Result<(), Box<dyn Error>> {
    let message = PublishMessage {
        payload: "UmVsYXkgd29ya3MhIQ==".to_string(),
        content_topic: "/my-app/2/chatroom-1/proto".to_string(),
        timestamp: 0,
    };
    publish_message(message).await?;
    Ok(())
}

#[tokio::test]
async fn node1_test4_confirm_message_publication() -> Result<(), Box<dyn Error>> {
    let message = PublishMessage {
        payload: "UmVsYXkgd29ya3MhIQ==".to_string(),
        content_topic: "/my-app/2/chatroom-1/proto".to_string(),
        timestamp: 0,
    };
    confirm_message_publication("/my-app/2/chatroom-1/proto", &message).await?;
    Ok(())
}

/// Test Suite 2: Inter-Node Communication (advanced)
#[tokio::test]
async fn node2_test1_verify_node2_autoconnection() -> Result<(), Box<dyn Error>> {
    verify_node2_autoconnection().await?;
    Ok(())
}

#[tokio::test]
async fn node2_test2_subscribe_node2_to_topic() -> Result<(), Box<dyn Error>> {
    subscribe_node2_to_topic("/my-app/2/chatroom-1/proto").await?;
    Ok(())
}

#[tokio::test]
async fn node2_test3_publish_message_from_node1_for_node2() -> Result<(), Box<dyn Error>> {
    let message = PublishMessage {
        payload: "UmVsYXkgd29ya3MhIQ==".to_string(),
        content_topic: "/my-app/2/chatroom-1/proto".to_string(),
        timestamp: 0,
    };
    publish_message(message).await?;
    Ok(())
}

#[tokio::test]
async fn node2_test4_confirm_message_publication_from_node2() -> Result<(), Box<dyn Error>> {
    let message = PublishMessage {
        payload: "UmVsYXkgd29ya3MhIQ==".to_string(),
        content_topic: "/my-app/2/chatroom-1/proto".to_string(),
        timestamp: 0,
    };
    confirm_message_publication_from_node2("/my-app/2/chatroom-1/proto", &message).await?;
    Ok(())
}
