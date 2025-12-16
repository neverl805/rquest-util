use super::*;

pub fn header_initializer(
    sec_ch_ua: &'static str,
    user_agent: &'static str,
    emulation_os: EmulationOS,
) -> HeaderMap {
    let mut headers = HeaderMap::new();

    // Sec-CH-UA headers
    header_chrome_sec_ch_ua!(
        headers,
        sec_ch_ua,
        emulation_os.platform(),
        emulation_os.is_mobile()
    );

    // User-Agent
    header_chrome_ua!(headers, user_agent);

    // Accept headers
    header_chrome_accept!(headers);

    // Sec-Fetch headers
    header_chrome_sec_fetch!(headers);

    headers
}

#[allow(dead_code)]
pub fn header_initializer_with_zstd(
    sec_ch_ua: &'static str,
    user_agent: &'static str,
    emulation_os: EmulationOS,
) -> HeaderMap {
    let mut headers = HeaderMap::new();
    header_chrome_sec_ch_ua!(
        headers,
        sec_ch_ua,
        emulation_os.platform(),
        emulation_os.is_mobile()
    );
    header_chrome_ua!(headers, user_agent);
    header_chrome_sec_fetch!(headers);
    header_chrome_accept!(zstd, headers);
    headers
}

#[allow(dead_code)]
pub fn header_initializer_with_zstd_priority(
    sec_ch_ua: &'static str,
    user_agent: &'static str,
    emulation_os: EmulationOS,
) -> HeaderMap {
    let mut headers = HeaderMap::new();
    header_chrome_sec_ch_ua!(
        headers,
        sec_ch_ua,
        emulation_os.platform(),
        emulation_os.is_mobile()
    );
    header_chrome_ua!(headers, user_agent);
    header_chrome_sec_fetch!(headers);
    header_chrome_accept!(zstd, headers);
    headers.insert(
        HeaderName::from_static("priority"),
        HeaderValue::from_static("u=0, i"),
    );
    headers
}

