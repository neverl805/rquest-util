fn main() {
    // 真实Firefox 146浏览器的JA3 text
    let real_ja3 = "771,4865-4867-4866-49195-49199-52393-52392-49196-49200-49162-49161-49171-49172-156-157-47-53,0-23-65281-10-11-35-16-5-34-18-51-45-43-13-28-27-21,4588-29-23-24-25-256-257,0";

    // 当前Firefox 145的JA3 text
    let current_ja3 = "771,4865-4867-4866-49195-49199-52393-52392-49196-49200-49162-49161-49171-49172-156-157-47-53,0-65037-23-65281-10-11-35-16-5-13-18-51-45-43-27-34-28,4588-29-23-24-25-256-257,0";

    let real_ext: Vec<&str> = real_ja3.split(',').nth(2).unwrap().split('-').collect();
    let current_ext: Vec<&str> = current_ja3.split(',').nth(2).unwrap().split('-').collect();

    println!("Extension ID mapping:");
    println!("0 = SERVER_NAME");
    println!("5 = STATUS_REQUEST (OCSP)");
    println!("10 = SUPPORTED_GROUPS");
    println!("11 = EC_POINT_FORMATS");
    println!("13 = SIGNATURE_ALGORITHMS");
    println!("16 = ALPN");
    println!("18 = CERTIFICATE_TIMESTAMP (SCT)");
    println!("21 = PADDING");
    println!("23 = EXTENDED_MASTER_SECRET");
    println!("27 = CERT_COMPRESSION");
    println!("28 = RECORD_SIZE_LIMIT");
    println!("34 = DELEGATED_CREDENTIAL");
    println!("35 = SESSION_TICKET");
    println!("43 = SUPPORTED_VERSIONS");
    println!("45 = PSK_KEY_EXCHANGE_MODES");
    println!("51 = KEY_SHARE");
    println!("65037 = ENCRYPTED_CLIENT_HELLO (ECH GREASE)");
    println!("65281 = RENEGOTIATION_INFO");
    println!();

    println!("Real Firefox 146 extensions ({} total):", real_ext.len());
    println!("  {}", real_ext.join(", "));
    println!();

    println!("Current Firefox 145 extensions ({} total):", current_ext.len());
    println!("  {}", current_ext.join(", "));
    println!();

    println!("Differences:");
    println!("  ❌ Current has 65037 (ECH GREASE), Real doesn't");
    println!("  ❌ Real has 21 (PADDING), Current doesn't");
    println!("  ❌ Extension order is different");
}
