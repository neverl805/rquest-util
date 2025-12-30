use super::*;

pub fn header_initializer(
    sec_ch_ua: &'static str,
    user_agent: &'static str,
    emulation_os: EmulationOS,
) -> HeaderMap {
    let mut headers = HeaderMap::new();

    // Chrome header order (正确顺序):
    // 1. sec-ch-ua, sec-ch-ua-mobile, sec-ch-ua-platform
    // 2. upgrade-insecure-requests
    // 3. user-agent
    // 4. accept
    // 5. sec-fetch-site, sec-fetch-mode, sec-fetch-user, sec-fetch-dest
    // 6. accept-encoding
    // 7. accept-language

    // 1. Sec-CH-UA headers
    header_chrome_sec_ch_ua!(
        headers,
        sec_ch_ua,
        emulation_os.platform(),
        emulation_os.is_mobile()
    );

    // 2. Upgrade-Insecure-Requests
    headers.insert(
        HeaderName::from_static("upgrade-insecure-requests"),
        HeaderValue::from_static("1"),
    );

    // 3. User-Agent
    header_chrome_ua!(headers, user_agent);

    // 4. Accept
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
    );

    // 5. Sec-Fetch headers (按Chrome顺序: site, mode, user, dest)
    headers.insert(
        HeaderName::from_static("sec-fetch-site"),
        HeaderValue::from_static("none"),
    );
    headers.insert(
        HeaderName::from_static("sec-fetch-mode"),
        HeaderValue::from_static("navigate"),
    );
    headers.insert(
        HeaderName::from_static("sec-fetch-user"),
        HeaderValue::from_static("?1"),
    );
    headers.insert(
        HeaderName::from_static("sec-fetch-dest"),
        HeaderValue::from_static("document"),
    );

    // 6. Accept-Encoding
    #[cfg(all(
        feature = "emulation-gzip",
        feature = "emulation-deflate",
        feature = "emulation-brotli",
        feature = "emulation-zstd"
    ))]
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, br, zstd, deflate"),
    );

    #[cfg(all(
        feature = "emulation-gzip",
        feature = "emulation-deflate",
        feature = "emulation-brotli",
        not(feature = "emulation-zstd")
    ))]
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    );

    // 7. Accept-Language
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("en-US,en;q=0.9")
    );

    headers
}

#[allow(dead_code)]
pub fn header_initializer_with_zstd(
    sec_ch_ua: &'static str,
    user_agent: &'static str,
    emulation_os: EmulationOS,
) -> HeaderMap {
    let mut headers = HeaderMap::new();

    // 1. Sec-CH-UA
    header_chrome_sec_ch_ua!(
        headers,
        sec_ch_ua,
        emulation_os.platform(),
        emulation_os.is_mobile()
    );

    // 2. Upgrade-Insecure-Requests
    headers.insert(
        HeaderName::from_static("upgrade-insecure-requests"),
        HeaderValue::from_static("1"),
    );

    // 3. User-Agent
    header_chrome_ua!(headers, user_agent);

    // 4. Accept
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
    );

    // 5. Sec-Fetch
    headers.insert(
        HeaderName::from_static("sec-fetch-site"),
        HeaderValue::from_static("none"),
    );
    headers.insert(
        HeaderName::from_static("sec-fetch-mode"),
        HeaderValue::from_static("navigate"),
    );
    headers.insert(
        HeaderName::from_static("sec-fetch-user"),
        HeaderValue::from_static("?1"),
    );
    headers.insert(
        HeaderName::from_static("sec-fetch-dest"),
        HeaderValue::from_static("document"),
    );

    // 6. Accept-Encoding with zstd
    #[cfg(all(
        feature = "emulation-gzip",
        feature = "emulation-deflate",
        feature = "emulation-brotli",
        feature = "emulation-zstd"
    ))]
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, br, zstd, deflate"),
    );

    // 7. Accept-Language
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("en-US,en;q=0.9")
    );

    headers
}

#[allow(dead_code)]
pub fn header_initializer_with_zstd_priority(
    sec_ch_ua: &'static str,
    user_agent: &'static str,
    emulation_os: EmulationOS,
) -> HeaderMap {
    let mut headers = HeaderMap::new();

    // 1. Sec-CH-UA
    header_chrome_sec_ch_ua!(
        headers,
        sec_ch_ua,
        emulation_os.platform(),
        emulation_os.is_mobile()
    );

    // 2. Upgrade-Insecure-Requests
    headers.insert(
        HeaderName::from_static("upgrade-insecure-requests"),
        HeaderValue::from_static("1"),
    );

    // 3. User-Agent
    header_chrome_ua!(headers, user_agent);

    // 4. Accept
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
    );

    // 5. Sec-Fetch
    headers.insert(
        HeaderName::from_static("sec-fetch-site"),
        HeaderValue::from_static("none"),
    );
    headers.insert(
        HeaderName::from_static("sec-fetch-mode"),
        HeaderValue::from_static("navigate"),
    );
    headers.insert(
        HeaderName::from_static("sec-fetch-user"),
        HeaderValue::from_static("?1"),
    );
    headers.insert(
        HeaderName::from_static("sec-fetch-dest"),
        HeaderValue::from_static("document"),
    );

    // 6. Accept-Encoding
    #[cfg(all(
        feature = "emulation-gzip",
        feature = "emulation-deflate",
        feature = "emulation-brotli",
        feature = "emulation-zstd"
    ))]
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, br, zstd, deflate"),
    );

    // 7. Accept-Language
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("en-US,en;q=0.9")
    );

    // 8. Priority (Chrome 102+)
    headers.insert(
        HeaderName::from_static("priority"),
        HeaderValue::from_static("u=0, i"),
    );

    headers
}
