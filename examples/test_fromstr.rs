use rquest_util::{Emulation, EmulationOS};
use std::str::FromStr;

fn main() {
    println!("Testing Emulation FromStr implementation:");

    // Test Chrome versions
    let chrome_131 = Emulation::from_str("chrome_131").unwrap();
    println!("✓ Parsed chrome_131: {:?}", chrome_131);

    let chrome_143 = Emulation::from_str("chrome_143").unwrap();
    println!("✓ Parsed chrome_143: {:?}", chrome_143);

    // Test Safari versions
    let safari_18 = Emulation::from_str("safari_18").unwrap();
    println!("✓ Parsed safari_18: {:?}", safari_18);

    let safari_26_1 = Emulation::from_str("safari_26.1").unwrap();
    println!("✓ Parsed safari_26.1: {:?}", safari_26_1);

    // Test Firefox versions
    let firefox_145 = Emulation::from_str("firefox_145").unwrap();
    println!("✓ Parsed firefox_145: {:?}", firefox_145);

    let firefox_private_136 = Emulation::from_str("firefox_private_136").unwrap();
    println!("✓ Parsed firefox_private_136: {:?}", firefox_private_136);

    let firefox_android_135 = Emulation::from_str("firefox_android_135").unwrap();
    println!("✓ Parsed firefox_android_135: {:?}", firefox_android_135);

    // Test Edge versions
    let edge_142 = Emulation::from_str("edge_142").unwrap();
    println!("✓ Parsed edge_142: {:?}", edge_142);

    // Test Opera versions
    let opera_119 = Emulation::from_str("opera_119").unwrap();
    println!("✓ Parsed opera_119: {:?}", opera_119);

    // Test OkHttp versions
    let okhttp_4_12 = Emulation::from_str("okhttp_4.12").unwrap();
    println!("✓ Parsed okhttp_4.12: {:?}", okhttp_4_12);

    println!("\nTesting EmulationOS FromStr implementation:");

    // Test OS versions
    let windows = EmulationOS::from_str("windows").unwrap();
    println!("✓ Parsed windows: {:?}", windows);

    let macos = EmulationOS::from_str("macos").unwrap();
    println!("✓ Parsed macos: {:?}", macos);

    let android = EmulationOS::from_str("android").unwrap();
    println!("✓ Parsed android: {:?}", android);

    let ios = EmulationOS::from_str("ios").unwrap();
    println!("✓ Parsed ios: {:?}", ios);

    let linux = EmulationOS::from_str("linux").unwrap();
    println!("✓ Parsed linux: {:?}", linux);

    // Test invalid input
    println!("\nTesting error handling:");
    match Emulation::from_str("invalid_browser") {
        Err(e) => println!("✓ Correctly rejected invalid input: {}", e),
        Ok(_) => println!("✗ Should have failed on invalid input"),
    }

    match EmulationOS::from_str("invalid_os") {
        Err(e) => println!("✓ Correctly rejected invalid OS: {}", e),
        Ok(_) => println!("✗ Should have failed on invalid OS"),
    }

    println!("\n✅ All FromStr tests passed!");
}
