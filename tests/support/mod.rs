#![allow(dead_code)]

use std::{sync::LazyLock, time::Duration};

use tokio::sync::Semaphore;
use rquest::Client;

pub static TEST_SEMAPHORE: LazyLock<Semaphore> = LazyLock::new(|| Semaphore::new(1));

pub static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .connect_timeout(Duration::from_secs(60))
        .build()
        .unwrap()
});

#[allow(unused_macros)]
macro_rules! test_emulation {
    ($test_name:ident, $emulation:expr, $ja4:expr, $akamai_hash:expr) => {
        #[tokio::test]
        async fn $test_name() {
            let _permit = crate::support::TEST_SEMAPHORE.acquire().await.unwrap();

            let resp = crate::support::CLIENT
                .get("https://tls.browserleaks.com/json")
                .emulation($emulation)
                .send()
                .await
                .unwrap();

            assert_eq!(resp.status(), rquest::StatusCode::OK);
            let json: serde_json::Value = resp.json().await.unwrap();

            // Check JA4 fingerprint
            if let Some(ja4_fingerprint) = json.get("ja4").and_then(|v| v.as_str()) {
                let conditional = $ja4.iter().any(|&expected| ja4_fingerprint == expected);
                if !conditional {
                    println!("Expected JA4: {:?}", $ja4);
                    println!("Got JA4: {}", ja4_fingerprint);
                    println!("Full response: {}", serde_json::to_string_pretty(&json).unwrap());
                }
                assert!(conditional, "JA4 fingerprint mismatch");
            }

            // Check Akamai hash (JA3 hash)
            if let Some(ja3_hash) = json.get("ja3_hash").and_then(|v| v.as_str()) {
                let conditional = ja3_hash == $akamai_hash;
                if !conditional {
                    println!("Expected JA3 hash: {}", $akamai_hash);
                    println!("Got JA3 hash: {}", ja3_hash);
                    println!("Full response: {}", serde_json::to_string_pretty(&json).unwrap());
                }
                assert!(conditional, "JA3 hash mismatch");
            }

            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        }
    };
}
