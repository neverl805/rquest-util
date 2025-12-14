use super::*;

// Opera uses Chromium-based headers

// Macro definitions must come before use
macro_rules! header_opera_sec_ch_ua {
    ($headers:expr, $ua:expr, $platform:expr, $is_mobile:expr) => {
        let mobile = if $is_mobile { "?1" } else { "?0" };
        $headers.insert("sec-ch-ua", HeaderValue::from_static($ua));
        $headers.insert("sec-ch-ua-mobile", HeaderValue::from_static(mobile));
        $headers.insert("sec-ch-ua-platform", HeaderValue::from_static($platform));
    };
}

macro_rules! header_opera_sec_fetch {
    ($headers:expr) => {
        $headers.insert("sec-fetch-dest", HeaderValue::from_static("document"));
        $headers.insert("sec-fetch-mode", HeaderValue::from_static("navigate"));
        $headers.insert("sec-fetch-site", HeaderValue::from_static("none"));
    };
}

macro_rules! header_opera_ua {
    ($headers:expr, $ua:expr) => {
        $headers.insert(USER_AGENT, HeaderValue::from_static($ua));
    };
}

#[allow(unused_macro_rules)]
macro_rules! header_opera_accept {
    ($headers:expr) => {
        $headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"));
        #[cfg(all(feature = "emulation-gzip", feature = "emulation-deflate", feature = "emulation-brotli"))]
        $headers.insert(
            ACCEPT_ENCODING,
            HeaderValue::from_static("gzip, deflate, br"),
        );
        $headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
    };
    (zstd, $headers:expr) => {
        $headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"));
        #[cfg(all(
            feature = "emulation-gzip",
            feature = "emulation-deflate",
            feature = "emulation-brotli",
            feature = "emulation-zstd"
        ))]
        $headers.insert(
            ACCEPT_ENCODING,
            HeaderValue::from_static("gzip, deflate, br, zstd"),
        );
        $headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
    };
}

pub fn header_initializer(
    sec_ch_ua: &'static str,
    user_agent: &'static str,
    emulation_os: EmulationOS,
) -> HeaderMap {
    let mut headers = HeaderMap::new();

    // Sec-CH-UA headers (Opera uses similar format to Chrome)
    header_opera_sec_ch_ua!(
        headers,
        sec_ch_ua,
        emulation_os.platform(),
        emulation_os.is_mobile()
    );

    // User-Agent
    header_opera_ua!(headers, user_agent);

    // Accept headers
    header_opera_accept!(headers);

    // Sec-Fetch headers
    header_opera_sec_fetch!(headers);

    headers
}
