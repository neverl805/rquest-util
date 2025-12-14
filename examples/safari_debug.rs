//! Debug Safari HTTP2 configuration

use rquest_util::Emulation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test Safari 17
    println!("=== Testing Safari 17_0 ===\n");
    let client = rquest::Client::builder()
        .emulation(Emulation::Safari17_0)
        .build()?;

    match client.get("https://httpbin.org/get").send().await {
        Ok(resp) => println!("✅ Safari 17: Success! Status: {}\n", resp.status()),
        Err(e) => println!("❌ Safari 17: Error: {:?}\n", e),
    }

    // Test Safari 18
    println!("=== Testing Safari 18 ===\n");
    let client = rquest::Client::builder()
        .emulation(Emulation::Safari18)
        .build()?;

    match client.get("https://httpbin.org/get").send().await {
        Ok(resp) => println!("✅ Safari 18: Success! Status: {}\n", resp.status()),
        Err(e) => println!("❌ Safari 18: Error: {:?}\n", e),
    }

    // Test Safari 15_6_1
    println!("=== Testing Safari 15_6_1 ===\n");
    let client = rquest::Client::builder()
        .emulation(Emulation::Safari15_6_1)
        .build()?;

    match client.get("https://httpbin.org/get").send().await {
        Ok(resp) => println!("✅ Safari 15.6.1: Success! Status: {}\n", resp.status()),
        Err(e) => println!("❌ Safari 15.6.1: Error: {:?}\n", e),
    }

    Ok(())
}
