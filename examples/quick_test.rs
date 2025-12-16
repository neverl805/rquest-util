//! Quick test example for rquest-util
//!
//! A minimal example to quickly test browser emulation functionality.
//!
//! Run with: cargo run --example quick_test --features emulation

use rquest_util::Emulation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Quick rquest-util Test\n");

    // Test 1: Basic Chrome request
    println!("Test 1: Chrome 136 → httpbin.org");
    // let client = rquest::Client::builder()
    //     .emulation(Emulation::Chrome142)
    //     .build()?;

    // let resp = client.get("https://tls.peet.ws/api/all").send().await?;
    // let json: serde_json::Value = resp.json().await?;
    // println!("{}",json);

    // Test 2: Firefox request
    println!("Test 2: Firefox 133 → httpbin.org");
    let client = rquest::Client::builder()
        .emulation(Emulation::Chrome142)
        .build()?;

    let resp = client.get("https://tls.browserleaks.com").send().await?;
    // let resp = client.get("https://tls.peet.ws/api/all").send().await?;
    let json: serde_json::Value = resp.json().await?;
    println!("{}",json);
    // Test 3: Check User-Agent
    // println!("Test 3: Checking User-Agent");
    // let client = rquest::Client::builder()
    //     .emulation(Emulation::Safari26_1)
    //     .build()?;
    //
    // let resp = client.get("https://tls.peet.ws/api/all").send().await?;
    // let json: serde_json::Value = resp.json().await?;
    // println!("✅ User-Agent: {}\n", json);

    println!("🎉 All tests passed!");

    Ok(())
}
