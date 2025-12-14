use super::*;

pub fn header_initializer(user_agent: &'static str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    header_firefox_ua!(headers, user_agent);
    header_firefox_accept!(headers);
    header_firefox_sec_fetch!(headers);
    headers
}

pub fn header_initializer_with_zstd(user_agent: &'static str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    header_firefox_ua!(headers, user_agent);
    header_firefox_accept!(zstd, headers);
    header_firefox_sec_fetch!(headers);
    headers.insert(
        HeaderName::from_static("priority"),
        HeaderValue::from_static("u=0, i"),
    );
    headers
}

