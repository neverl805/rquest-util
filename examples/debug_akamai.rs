use rquest_util::Emulation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = rquest::Client::builder()
        .emulation(Emulation::Safari18)
        .build()?;

    let resp = client
        .get("https://tls.browserleaks.com/json")
        .send()
        .await?;

    let json: serde_json::Value = resp.json().await?;
    println!("{}", serde_json::to_string_pretty(&json)?);

    Ok(())
}
