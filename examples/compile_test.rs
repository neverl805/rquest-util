// Simple test to verify FromStr implementation compiles correctly
use rquest_util::{Emulation, EmulationOS};

fn test_parse_chrome() {
    let _: Result<Emulation, _> = "chrome_131".parse();
    let _: Result<Emulation, _> = "chrome_143".parse();
}

fn test_parse_safari() {
    let _: Result<Emulation, _> = "safari_26.1".parse();
    let _: Result<Emulation, _> = "safari_ios_26".parse();
}

fn test_parse_firefox() {
    let _: Result<Emulation, _> = "firefox_145".parse();
    let _: Result<Emulation, _> = "firefox_private_136".parse();
    let _: Result<Emulation, _> = "firefox_android_135".parse();
}

fn test_parse_os() {
    let _: Result<EmulationOS, _> = "windows".parse();
    let _: Result<EmulationOS, _> = "macos".parse();
    let _: Result<EmulationOS, _> = "android".parse();
}

fn main() {
    println!("FromStr implementation compiles correctly!");
}
