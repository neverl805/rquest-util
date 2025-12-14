//! Safari JA3/JA4 Fingerprint Verification Test
//!
//! This test compares actual JA3/JA4 fingerprints with expected values from wreq-util

use rquest_util::Emulation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Safari JA3/JA4 Fingerprint Verification\n");

    // Expected values from wreq-util/tests/emulation_safari.rs
    let tests = vec![
        ("Safari 15.3", Emulation::Safari15_3,
            "t13d2614h2_2802a3db6c62_14788d8d241b", "dda308d35f4e5db7b52a61720ca1b122"),
        ("Safari 15.6.1", Emulation::Safari15_6_1,
            "t13d2014h2_a09f3c656075_14788d8d241b", "dda308d35f4e5db7b52a61720ca1b122"),
        ("Safari 16", Emulation::Safari16,
            "t13d2014h2_a09f3c656075_14788d8d241b", "dda308d35f4e5db7b52a61720ca1b122"),
        ("Safari iOS 16.5", Emulation::SafariIos16_5,
            "t13d2014h2_a09f3c656075_14788d8d241b", "d5fcbdc393757341115a861bf8d23265"),
        ("Safari 17.0", Emulation::Safari17_0,
            "t13d2014h2_a09f3c656075_14788d8d241b", "959a7e813b79b909a1a0b00a38e8bba3"),
        ("Safari 17.4.1", Emulation::Safari17_4_1,
            "t13d2014h2_a09f3c656075_14788d8d241b", "dda308d35f4e5db7b52a61720ca1b122"),
        ("Safari iOS 17.2", Emulation::SafariIos17_2,
            "t13d2014h2_a09f3c656075_14788d8d241b", "ad8424af1cc590e09f7b0c499bf7fcdb"),
        ("Safari 18", Emulation::Safari18,
            "t13d2014h2_a09f3c656075_14788d8d241b", "d4a2dcbfde511b5040ed5a5190a8d78b"),
        ("Safari 18.2", Emulation::Safari18_2,
            "t13d2014h2_a09f3c656075_e42f34c56612", "d4a2dcbfde511b5040ed5a5190a8d78b"),
        ("Safari 18.5", Emulation::Safari18_5,
            "t13d2014h2_a09f3c656075_e42f34c56612", "c52879e43202aeb92740be6e8c86ea96"),
        ("Safari 26", Emulation::Safari26,
            "t13d2013h2_a09f3c656075_7f0f34a4126d", "c52879e43202aeb92740be6e8c86ea96"),
        ("Safari 26.1", Emulation::Safari26_1,
            "t13d2014h2_a09f3c656075_e42f34c56612", "773906b0efdefa24a7f2b8eb6985bf37"),
    ];

    let url = "https://tls.browserleaks.com/";

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

        // Add rate limiting
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }

    println!("\n📊 Results: {} passed, {} failed", passed, failed);

    if failed > 0 {
        std::process::exit(1);
    }

    Ok(())
}
