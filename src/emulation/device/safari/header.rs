use super::*;

pub fn header_initializer_for_15(user_agent: &'static str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"));
    #[cfg(all(feature = "emulation-gzip", feature = "emulation-deflate", feature = "emulation-brotli"))]
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert(USER_AGENT, HeaderValue::from_static(user_agent));
    headers
}

pub fn header_initializer_for_16_17(user_agent: &'static str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("sec-fetch-dest"), HeaderValue::from_static("document"));
    headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"));
    headers.insert(HeaderName::from_static("sec-fetch-site"), HeaderValue::from_static("none"));
    #[cfg(all(feature = "emulation-gzip", feature = "emulation-deflate", feature = "emulation-brotli"))]
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert(HeaderName::from_static("sec-fetch-mode"), HeaderValue::from_static("navigate"));
    headers.insert(USER_AGENT, HeaderValue::from_static(user_agent));
    headers
}

pub fn header_initializer_for_18(user_agent: &'static str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"));
    headers.insert(HeaderName::from_static("sec-fetch-site"), HeaderValue::from_static("none"));
    #[cfg(all(feature = "emulation-gzip", feature = "emulation-deflate", feature = "emulation-brotli"))]
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br"));
    headers.insert(HeaderName::from_static("sec-fetch-mode"), HeaderValue::from_static("navigate"));
    headers.insert(USER_AGENT, HeaderValue::from_static(user_agent));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert(HeaderName::from_static("priority"), HeaderValue::from_static("u=0, i"));
    headers.insert(HeaderName::from_static("sec-fetch-dest"), HeaderValue::from_static("document"));
    headers
}

