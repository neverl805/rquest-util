use rquest_util::Emulation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://tls.browserleaks.com/";

    // Test data from wreq-util/tests/emulation_firefox.rs: (name, emulation, expected_ja4, expected_akamai_hash)
    let tests = vec![
        ("Firefox 109", Emulation::Firefox109,
            "t13d1715h2_5b57614c22b0_3d5424432f57", "73d042072ceabaedacfd45e84dff1020"),
        ("Firefox 117", Emulation::Firefox117,
            "t13d1715h2_5b57614c22b0_3d5424432f57", "73d042072ceabaedacfd45e84dff1020"),
        ("Firefox 128", Emulation::Firefox128,
            "t13d1513h2_8daaf6152771_748f4c70de1c", "1d8a6f51fd7253d04674593073fc18b0"),
        ("Firefox 133", Emulation::Firefox133,
            "t13d1716h2_5b57614c22b0_eeeea6562960", "6ea73faa8fc5aac76bded7bd238f6433"),
        ("Firefox 135", Emulation::Firefox135,
            "t13d1717h2_5b57614c22b0_3cbfd9057e0d", "6ea73faa8fc5aac76bded7bd238f6433"),
        ("Firefox 136", Emulation::Firefox136,
            "t13d1717h2_5b57614c22b0_3cbfd9057e0d", "6ea73faa8fc5aac76bded7bd238f6433"),
    ];

    println!("🔍 Firefox JA4/Akamai Fingerprint Verification\n");

    let mut passed = 0;
    let mut failed = 0;

    for (name, emulation, expected_ja4, expected_akamai) in tests {
        print!("Testing {:<20} ... ", name);

        let client = rquest::Client::builder()
            .emulation(emulation)
            .build()?;

        match client.get(url).send().await {
            Ok(resp) => {
                let json: serde_json::Value = resp.json().await?;
                let actual_ja4 = json["ja4"].as_str().unwrap_or("N/A");
                let actual_akamai = json["akamai_hash"].as_str().unwrap_or("N/A");

                let ja4_match = actual_ja4 == expected_ja4;
                let akamai_match = actual_akamai == expected_akamai;

                if ja4_match && akamai_match {
                    println!("✅ PASS");
                    passed += 1;
                } else {
                    println!("❌ FAIL");
                    if !ja4_match {
                        println!("  JA4:    Expected: {}", expected_ja4);
                        println!("          Actual:   {}", actual_ja4);
                    }
                    if !akamai_match {
                        println!("  Akamai: Expected: {}", expected_akamai);
                        println!("          Actual:   {}", actual_akamai);
                    }
                    failed += 1;
                }
            }
            Err(e) => {
                println!("❌ Request error: {}", e);
                failed += 1;
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }

    println!("\n📊 Results: {} passed, {} failed", passed, failed);

    if failed > 0 {
        std::process::exit(1);
    }

    Ok(())
}
