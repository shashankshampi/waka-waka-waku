use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Execute test suite 1
    println!("Test Suite 1: Basic Node Operation (easy)");
    if let Err(e) = waka_waka_waku::test_suite_1().await {
        eprintln!("Error running test suite 1: {}", e);
    }

    // Execute test suite 2
    println!("Test Suite 2: Inter-Node Communication (advanced)");
    if let Err(e) = waka_waka_waku::test_suite_2().await {
        eprintln!("Error running test suite 2: {}", e);
    }

    Ok(())
}

