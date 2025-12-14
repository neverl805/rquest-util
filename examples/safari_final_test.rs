//! Final Safari test

use rquest_util::Emulation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing Safari configurations\n");

    // Test 1: Safari 17
    println!("Test 1: Safari 17.0");
    let client = rquest::Client::builder()
        .emulation(Emulation::Safari17_0)
        .build()?;

    match client.get("https://www.google.com/").send().await {
        Ok(resp) => println!("✅ Status: {}\n", resp.status()),
        Err(e) => println!("❌ Error: {}\n", e),
    }

    // Test 2: Safari 18
    println!("Test 2: Safari 18");
    let client = rquest::Client::builder()
        .emulation(Emulation::Safari18)
        .build()?;

    match client.get("https://www.google.com/").send().await {
        Ok(resp) => println!("✅ Status: {}\n", resp.status()),
        Err(e) => println!("❌ Error: {}\n", e),
    }

    // Test 3: Safari iOS
    println!("Test 3: Safari iOS 18.1.1");
    let client = rquest::Client::builder()
        .emulation(Emulation::SafariIos18_1_1)
        .build()?;

    match client.get("https://www.google.com/").send().await {
        Ok(resp) => println!("✅ Status: {}\n", resp.status()),
        Err(e) => println!("❌ Error: {}\n", e),
    }

    println!("🎉 All tests completed!");

    Ok(())
}
