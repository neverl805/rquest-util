use rquest_util::Emulation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tests = vec![
        ("Firefox 109", Emulation::Firefox109),
        ("Firefox 117", Emulation::Firefox117),
        ("Firefox 128", Emulation::Firefox128),
    ];

    for (name, emulation) in tests {
        let client = rquest::Client::builder()
            .emulation(emulation)
            .build()?;

        let resp = client.get("https://tls.browserleaks.com/").send().await?;
        let json: serde_json::Value = resp.json().await?;

        println!("\n{}:", name);
        println!("  JA4: {}", json["ja4"].as_str().unwrap_or("N/A"));
        println!("  Akamai hash: {}", json["akamai_hash"].as_str().unwrap_or("N/A"));
        println!("  Akamai text: {}", json["akamai_text"].as_str().unwrap_or("N/A"));

        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }

    Ok(())
}
