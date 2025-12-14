//! Browser emulation example using rquest-util
//!
//! This example demonstrates how to use rquest-util to emulate different browsers
//! and make HTTP requests that appear to come from real browsers.
//!
//! Run with: cargo run --example emulation --features emulation

use rquest_util::{Emulation, EmulationOption, EmulationOS};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== rquest Browser Emulation Examples ===\n");

    // Example 1: Simple Chrome emulation
    example_chrome_simple().await?;

    // Example 2: Firefox with custom OS
    example_firefox_custom_os().await?;

    // Example 3: Safari on macOS
    example_safari_macos().await?;

    // Example 4: Edge browser
    example_edge().await?;

    // Example 5: Multiple requests with different browsers
    example_multiple_browsers().await?;

    // Example 6: POST request with Chrome
    example_post_request().await?;

    // Example 7: Custom headers with emulation
    example_custom_headers().await?;

    Ok(())
}

/// Example 1: Simple Chrome emulation with default settings
async fn example_chrome_simple() -> Result<(), Box<dyn std::error::Error>> {
    println!("📌 Example 1: Simple Chrome 136 Request");
    println!("----------------------------------------");

    // Create a Chrome 136 emulated client
    let client = rquest::Client::builder()
        .emulation(Emulation::Chrome136)
        .build()?;

    // Make a request to httpbin to see our headers
    let response = client
        .get("https://httpbin.org/headers")
        .send()
        .await?;

    println!("Status: {}", response.status());
    println!("Response:\n{}\n", response.text().await?);

    Ok(())
}

/// Example 2: Firefox with custom operating system
async fn example_firefox_custom_os() -> Result<(), Box<dyn std::error::Error>> {
    println!("📌 Example 2: Firefox 133 on Linux");
    println!("------------------------------------");

    // Create emulation options with custom OS
    let emulation_option = EmulationOption::builder()
        .emulation(Emulation::Firefox133)
        .emulation_os(EmulationOS::Linux)
        .build();

    let client = rquest::Client::builder()
        .emulation(emulation_option)
        .build()?;

    let response = client
        .get("https://httpbin.org/user-agent")
        .send()
        .await?;

    println!("Status: {}", response.status());
    println!("User-Agent: {}\n", response.text().await?);

    Ok(())
}

/// Example 3: Safari on macOS
async fn example_safari_macos() -> Result<(), Box<dyn std::error::Error>> {
    println!("📌 Example 3: Safari 17.5 on macOS");
    println!("-----------------------------------");

    let emulation_option = EmulationOption::builder()
        .emulation(Emulation::Safari17_5)
        .emulation_os(EmulationOS::MacOS)
        .build();

    let client = rquest::Client::builder()
        .emulation(emulation_option)
        .build()?;

    let response = client
        .get("https://httpbin.org/get")
        .send()
        .await?;

    println!("Status: {}", response.status());
    let json: serde_json::Value = response.json().await?;
    println!("Headers sent: {:#?}\n", json["headers"]);

    Ok(())
}

/// Example 4: Microsoft Edge browser
async fn example_edge() -> Result<(), Box<dyn std::error::Error>> {
    println!("📌 Example 4: Microsoft Edge 131 on Windows");
    println!("--------------------------------------------");

    let emulation_option = EmulationOption::builder()
        .emulation(Emulation::Edge131)
        .emulation_os(EmulationOS::Windows)
        .build();

    let client = rquest::Client::builder()
        .emulation(emulation_option)
        .build()?;

    let response = client
        .get("https://httpbin.org/headers")
        .send()
        .await?;

    println!("Status: {}", response.status());
    println!("Response:\n{}\n", response.text().await?);

    Ok(())
}

/// Example 5: Making requests with different browsers
async fn example_multiple_browsers() -> Result<(), Box<dyn std::error::Error>> {
    println!("📌 Example 5: Multiple Browser Emulations");
    println!("------------------------------------------");

    let browsers = vec![
        (Emulation::Chrome136, "Chrome 136"),
        (Emulation::Firefox133, "Firefox 133"),
        (Emulation::Safari18, "Safari 18"),
        (Emulation::Edge131, "Edge 131"),
        (Emulation::Opera116, "Opera 116"),
    ];

    for (browser, name) in browsers {
        let client = rquest::Client::builder()
            .emulation(browser)
            .build()?;

        let response = client
            .get("https://httpbin.org/user-agent")
            .send()
            .await?;

        if response.status().is_success() {
            let json: serde_json::Value = response.json().await?;
            println!("{}: {}", name, json["user-agent"].as_str().unwrap_or("N/A"));
        }
    }

    println!();
    Ok(())
}

/// Example 6: POST request with Chrome emulation
async fn example_post_request() -> Result<(), Box<dyn std::error::Error>> {
    println!("📌 Example 6: POST Request with Chrome 136");
    println!("-------------------------------------------");

    let client = rquest::Client::builder()
        .emulation(Emulation::Chrome136)
        .build()?;

    // Create JSON body
    let body = serde_json::json!({
        "name": "rquest-util",
        "description": "Browser emulation library",
        "version": "3.0.0"
    });

    let response = client
        .post("https://httpbin.org/post")
        .json(&body)
        .send()
        .await?;

    println!("Status: {}", response.status());
    let json: serde_json::Value = response.json().await?;
    println!("Posted data: {:#?}\n", json["json"]);

    Ok(())
}

/// Example 7: Custom headers with emulation
async fn example_custom_headers() -> Result<(), Box<dyn std::error::Error>> {
    println!("📌 Example 7: Custom Headers with Emulation");
    println!("---------------------------------------------");

    // Skip default headers to add custom ones
    let emulation_option = EmulationOption::builder()
        .emulation(Emulation::Chrome136)
        .skip_headers(true) // Don't add default browser headers
        .build();

    let client = rquest::Client::builder()
        .emulation(emulation_option)
        .build()?;

    let response = client
        .get("https://httpbin.org/headers")
        .header("X-Custom-Header", "MyValue")
        .header("Authorization", "Bearer token123")
        .send()
        .await?;

    println!("Status: {}", response.status());
    let json: serde_json::Value = response.json().await?;
    println!("Custom headers sent: {:#?}\n", json["headers"]);

    Ok(())
}
