use std::error::Error;
use waka_waka_waku::*;

#[tokio::test]
async fn test_verify_node_info() -> Result<(), Box<dyn Error>> {
    verify_node_info().await?;
    Ok(())
}

#[tokio::test]
async fn test_subscribe_to_topic() -> Result<(), Box<dyn Error>> {
    subscribe_to_topic("/my-app/2/chatroom-1/proto").await?;
    Ok(())
}

#[tokio::test]
async fn test_publish_message() -> Result<(), Box<dyn Error>> {
    let message = PublishMessage {
        payload: "UmVsYXkgd29ya3MhIQ==".to_string(),
        content_topic: "/my-app/2/chatroom-1/proto".to_string(),
        timestamp: 0,
    };
    publish_message(message).await?;
    Ok(())
}

#[tokio::test]
async fn test_confirm_message_publication() -> Result<(), Box<dyn Error>> {
    let message = PublishMessage {
        payload: "UmVsYXkgd29ya3MhIQ==".to_string(),
        content_topic: "/my-app/2/chatroom-1/proto".to_string(),
        timestamp: 0,
    };
    confirm_message_publication("/my-app/2/chatroom-1/proto", &message).await?;
    Ok(())
}

#[tokio::test]
async fn test_verify_node2_autoconnection() -> Result<(), Box<dyn Error>> {
    verify_node2_autoconnection().await?;
    Ok(())
}

#[tokio::test]
async fn test_subscribe_node2_to_topic() -> Result<(), Box<dyn Error>> {
    subscribe_node2_to_topic("/my-app/2/chatroom-1/proto").await?;
    Ok(())
}
