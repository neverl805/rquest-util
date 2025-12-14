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

