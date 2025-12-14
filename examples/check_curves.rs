use rquest::SslCurve;

fn main() {
    // List all available SslCurve variants
    println!("Available SslCurve variants:");
    println!("{:?}", SslCurve::X25519);
    println!("{:?}", SslCurve::SECP256R1);
    println!("{:?}", SslCurve::SECP384R1);
    println!("{:?}", SslCurve::SECP521R1);
    println!("{:?}", SslCurve::FFDHE2048);
    println!("{:?}", SslCurve::FFDHE3072);

    // Try X25519MLKEM768 if it exists
    #[cfg(feature = "pq-experimental")]
    println!("{:?}", SslCurve::X25519MLKEM768);
}
