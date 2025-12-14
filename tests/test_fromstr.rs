#[cfg(test)]
mod tests {
    use crate::{Emulation, EmulationOS};
    use std::str::FromStr;

    #[test]
    fn test_emulation_from_str_chrome() {
        assert!(matches!(
            Emulation::from_str("chrome_131"),
            Ok(Emulation::Chrome131)
        ));
        assert!(matches!(
            Emulation::from_str("chrome_143"),
            Ok(Emulation::Chrome143)
        ));
        assert!(matches!(
            Emulation::from_str("chrome_100"),
            Ok(Emulation::Chrome100)
        ));
    }

    #[test]
    fn test_emulation_from_str_safari() {
        assert!(matches!(
            Emulation::from_str("safari_18"),
            Ok(Emulation::Safari18)
        ));
        assert!(matches!(
            Emulation::from_str("safari_26.1"),
            Ok(Emulation::Safari26_1)
        ));
        assert!(matches!(
            Emulation::from_str("safari_ios_26"),
            Ok(Emulation::SafariIos26)
        ));
    }

    #[test]
    fn test_emulation_from_str_firefox() {
        assert!(matches!(
            Emulation::from_str("firefox_145"),
            Ok(Emulation::Firefox145)
        ));
        assert!(matches!(
            Emulation::from_str("firefox_private_136"),
            Ok(Emulation::FirefoxPrivate136)
        ));
        assert!(matches!(
            Emulation::from_str("firefox_android_135"),
            Ok(Emulation::FirefoxAndroid135)
        ));
    }

    #[test]
    fn test_emulation_from_str_edge() {
        assert!(matches!(
            Emulation::from_str("edge_142"),
            Ok(Emulation::Edge142)
        ));
        assert!(matches!(
            Emulation::from_str("edge_101"),
            Ok(Emulation::Edge101)
        ));
    }

    #[test]
    fn test_emulation_from_str_opera() {
        assert!(matches!(
            Emulation::from_str("opera_119"),
            Ok(Emulation::Opera119)
        ));
        assert!(matches!(
            Emulation::from_str("opera_116"),
            Ok(Emulation::Opera116)
        ));
    }

    #[test]
    fn test_emulation_from_str_okhttp() {
        assert!(matches!(
            Emulation::from_str("okhttp_4.12"),
            Ok(Emulation::OkHttp4_12)
        ));
        assert!(matches!(
            Emulation::from_str("okhttp_5"),
            Ok(Emulation::OkHttp5)
        ));
    }

    #[test]
    fn test_emulation_from_str_invalid() {
        assert!(Emulation::from_str("invalid_browser").is_err());
        assert!(Emulation::from_str("chrome_999").is_err());
    }

    #[test]
    fn test_emulation_os_from_str() {
        assert!(matches!(
            EmulationOS::from_str("windows"),
            Ok(EmulationOS::Windows)
        ));
        assert!(matches!(
            EmulationOS::from_str("macos"),
            Ok(EmulationOS::MacOS)
        ));
        assert!(matches!(
            EmulationOS::from_str("linux"),
            Ok(EmulationOS::Linux)
        ));
        assert!(matches!(
            EmulationOS::from_str("android"),
            Ok(EmulationOS::Android)
        ));
        assert!(matches!(
            EmulationOS::from_str("ios"),
            Ok(EmulationOS::IOS)
        ));
    }

    #[test]
    fn test_emulation_os_from_str_invalid() {
        assert!(EmulationOS::from_str("invalid_os").is_err());
        assert!(EmulationOS::from_str("bsd").is_err());
    }

    #[test]
    fn test_parse_method() {
        // Test using .parse() method
        let emulation: Emulation = "chrome_131".parse().unwrap();
        assert!(matches!(emulation, Emulation::Chrome131));

        let os: EmulationOS = "windows".parse().unwrap();
        assert!(matches!(os, EmulationOS::Windows));
    }
}
