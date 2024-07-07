use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    waka_waka_waku::execute_tests().await
}
