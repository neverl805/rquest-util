//! Test Chrome vs Safari

use rquest_util::Emulation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test Chrome 136
    println!("=== Testing Chrome 136 ===\n");
    let client = rquest::Client::builder()
        .emulation(Emulation::Chrome136)
        .build()?;

    match client.get("https://httpbin.org/get").send().await {
        Ok(resp) => {
            println!("✅ Chrome: Success! Status: {}", resp.status());
            let json: serde_json::Value = resp.json().await?;
            println!("User-Agent: {}\n", json["headers"]["User-Agent"]);
        }
        Err(e) => println!("❌ Chrome: Error: {:?}\n", e),
    }

    // Test Safari 17
    println!("=== Testing Safari 17_0 ===\n");
    let client = rquest::Client::builder()
        .emulation(Emulation::Safari17_0)
        .build()?;

    match client.get("https://httpbin.org/get").send().await {
        Ok(resp) => {
            println!("✅ Safari: Success! Status: {}", resp.status());
            let json: serde_json::Value = resp.json().await?;
            println!("User-Agent: {}\n", json["headers"]["User-Agent"]);
        }
        Err(e) => println!("❌ Safari: Error: {:?}\n", e),
    }

    Ok(())
}
