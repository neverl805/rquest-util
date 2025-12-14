//! Advanced browser emulation example
//!
//! This example demonstrates advanced features like:
//! - Concurrent requests with different browsers
//! - Cookie handling
//! - Redirect handling
//! - Error handling
//! - Custom configurations
//!
//! Run with: cargo run --example advanced --features emulation

use rquest_util::{Emulation, EmulationOption, EmulationOS};
use std::time::Instant;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Advanced rquest-util Examples ===\n");

    // Example 1: Concurrent requests
    example_concurrent_requests().await?;

    // Example 2: Cookie handling
    example_cookie_handling().await?;

    // Example 3: Redirect handling
    example_redirect_handling().await?;

    // Example 4: Error handling
    example_error_handling().await?;

    // Example 5: Skip HTTP/2
    example_skip_http2().await?;

    // Example 6: Mobile browsers
    example_mobile_browsers().await?;

    Ok(())
}

/// Example 1: Making concurrent requests with different browsers
async fn example_concurrent_requests() -> Result<(), Box<dyn std::error::Error>> {
    println!("📌 Example 1: Concurrent Requests (10 browsers)");
    println!("------------------------------------------------");

    let start = Instant::now();
    let mut tasks = JoinSet::new();

    let browsers = vec![
        Emulation::Chrome136,
        Emulation::Chrome135,
        Emulation::Firefox133,
        Emulation::Firefox135,
        Emulation::Safari18,
        Emulation::Safari17_5,
        Emulation::Edge131,
        Emulation::Edge134,
        Emulation::Opera116,
        Emulation::Opera117,
    ];

    for browser in browsers {
        tasks.spawn(async move {
            let client = rquest::Client::builder()
                .emulation(browser)
                .build()
                .unwrap();

            let response = client
                .get("https://httpbin.org/get")
                .send()
                .await
                .unwrap();

            (browser, response.status().is_success())
        });
    }

    let mut success_count = 0;
    while let Some(result) = tasks.join_next().await {
        if let Ok((browser, success)) = result {
            if success {
                success_count += 1;
                println!("✅ {:?} request succeeded", browser);
            }
        }
    }

    let duration = start.elapsed();
    println!("Completed {} requests in {:?}\n", success_count, duration);

    Ok(())
}

/// Example 2: Cookie handling with browser emulation
async fn example_cookie_handling() -> Result<(), Box<dyn std::error::Error>> {
    println!("📌 Example 2: Cookie Handling");
    println!("------------------------------");

    // Create a client with cookie store
    let client = rquest::Client::builder()
        .emulation(Emulation::Chrome136)
        .cookie_store(true)
        .build()?;

    // First request sets cookies
    println!("Setting cookies...");
    let resp = client
        .get("https://httpbin.org/cookies/set?session=abc123&user=test")
        .send()
        .await?;

    println!("Status: {}", resp.status());

    // Second request should include the cookies
    println!("Retrieving cookies...");
    let resp = client
        .get("https://httpbin.org/cookies")
        .send()
        .await?;

    let json: serde_json::Value = resp.json().await?;
    println!("Cookies: {:#?}\n", json["cookies"]);

    Ok(())
}

/// Example 3: Redirect handling
async fn example_redirect_handling() -> Result<(), Box<dyn std::error::Error>> {
    println!("📌 Example 3: Redirect Handling");
    println!("--------------------------------");

    let client = rquest::Client::builder()
        .emulation(Emulation::Firefox133)
        .redirect(rquest::redirect::Policy::limited(5))
        .build()?;

    // httpbin redirects to /get
    let resp = client
        .get("https://httpbin.org/redirect/3")
        .send()
        .await?;

    println!("Final URL: {:?}", resp.url());
    println!("Status: {}", resp.status());
    println!("Followed {} redirects\n", 3);

    Ok(())
}

/// Example 4: Error handling
async fn example_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    println!("📌 Example 4: Error Handling");
    println!("----------------------------");

    let client = rquest::Client::builder()
        .emulation(Emulation::Safari18)
        .timeout(std::time::Duration::from_secs(5))
        .build()?;

    // Test with invalid domain
    match client.get("https://invalid.domain.that.does.not.exist").send().await {
        Ok(_) => println!("Unexpected success"),
        Err(e) => {
            if e.is_connect() {
                println!("✅ Connection error handled: {}", e);
            } else if e.is_timeout() {
                println!("✅ Timeout error handled: {}", e);
            } else {
                println!("✅ Other error handled: {}", e);
            }
        }
    }

    // Test 404 error
    let resp = client
        .get("https://httpbin.org/status/404")
        .send()
        .await?;

    if resp.status().is_client_error() {
        println!("✅ 404 error detected: {}\n", resp.status());
    }

    Ok(())
}

/// Example 5: Skip HTTP/2 configuration
async fn example_skip_http2() -> Result<(), Box<dyn std::error::Error>> {
    println!("📌 Example 5: Skip HTTP/2 Configuration");
    println!("----------------------------------------");

    // Create emulation without HTTP/2 settings
    let emulation_option = EmulationOption::builder()
        .emulation(Emulation::Chrome136)
        .skip_http2(true)
        .build();

    let client = rquest::Client::builder()
        .emulation(emulation_option)
        .build()?;

    let resp = client
        .get("https://httpbin.org/get")
        .send()
        .await?;

    println!("Status: {}", resp.status());
    println!("Version: {:?}\n", resp.version());

    Ok(())
}

/// Example 6: Mobile browser emulation
async fn example_mobile_browsers() -> Result<(), Box<dyn std::error::Error>> {
    println!("📌 Example 6: Mobile Browser Emulation");
    println!("---------------------------------------");

    let mobile_browsers = vec![
        (Emulation::Chrome136, EmulationOS::Android, "Chrome Android"),
        (Emulation::Chrome136, EmulationOS::IOS, "Chrome iOS"),
        (Emulation::Safari18, EmulationOS::IOS, "Safari iOS"),
        (Emulation::Firefox133, EmulationOS::Android, "Firefox Android"),
    ];

    for (browser, os, name) in mobile_browsers {
        let emulation_option = EmulationOption::builder()
            .emulation(browser)
            .emulation_os(os)
            .build();

        let client = rquest::Client::builder()
            .emulation(emulation_option)
            .build()?;

        let resp = client
            .get("https://httpbin.org/user-agent")
            .send()
            .await?;

        if resp.status().is_success() {
            let json: serde_json::Value = resp.json().await?;
            println!("{}: {}", name, json["user-agent"].as_str().unwrap_or("N/A"));
        }
    }

    println!();
    Ok(())
}
