use rquest_util::Emulation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = rquest::Client::builder()
        .emulation(Emulation::Firefox128)
        .build()?;

    let resp = client.get("https://tls.browserleaks.com/").send().await?;
    let json: serde_json::Value = resp.json().await?;

    println!("Firefox 128:");
    println!("  JA4: {}", json["ja4"].as_str().unwrap_or("N/A"));
    println!("  Akamai hash: {}", json["akamai_hash"].as_str().unwrap_or("N/A"));
    println!("  Akamai text: {}", json["akamai_text"].as_str().unwrap_or("N/A"));

    Ok(())
}
