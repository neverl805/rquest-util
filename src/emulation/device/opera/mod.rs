#[macro_use]
mod http2;
#[macro_use]
mod tls;
mod header;

use header::*;
use tls::*;

use super::*;
use crate::emulation::device::curves::CURVES_1;

// Opera 116 - Latest with MLKEM768
mod_generator!(
    opera116,
    tls_options!(7, CURVES_1),
    http2_options!(3),
    header_initializer,
    [
        (
            Windows,
            r#""Chromium";v="116", "Not)A;Brand";v="24", "Opera";v="102""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 OPR/102.0.0.0"
        ),
        (
            MacOS,
            r#""Chromium";v="116", "Not)A;Brand";v="24", "Opera";v="102""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 OPR/102.0.0.0"
        ),
        (
            Linux,
            r#""Chromium";v="116", "Not)A;Brand";v="24", "Opera";v="102""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36 OPR/102.0.0.0"
        )
    ]
);

// Opera 117
mod_generator!(
    opera117,
    opera116::build_emulation,
    header_initializer,
    [
        (
            Windows,
            r#""Chromium";v="117", "Not;A=Brand";v="24", "Opera";v="103""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36 OPR/103.0.0.0"
        ),
        (
            MacOS,
            r#""Chromium";v="117", "Not;A=Brand";v="24", "Opera";v="103""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36 OPR/103.0.0.0"
        ),
        (
            Linux,
            r#""Chromium";v="117", "Not;A=Brand";v="24", "Opera";v="103""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36 OPR/103.0.0.0"
        )
    ]
);

// Opera 118
mod_generator!(
    opera118,
    opera116::build_emulation,
    header_initializer,
    [
        (
            Windows,
            r#""Chromium";v="118", "Opera GX";v="104", "Not=A?Brand";v="99""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36 OPR/104.0.0.0"
        ),
        (
            MacOS,
            r#""Chromium";v="118", "Opera GX";v="104", "Not=A?Brand";v="99""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36 OPR/104.0.0.0"
        ),
        (
            Linux,
            r#""Chromium";v="118", "Opera GX";v="104", "Not=A?Brand";v="99""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36 OPR/104.0.0.0"
        )
    ]
);

// Opera 119
mod_generator!(
    opera119,
    opera116::build_emulation,
    header_initializer,
    [
        (
            Windows,
            r#""Chromium";v="119", "Not?A_Brand";v="24", "Opera";v="105""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36 OPR/105.0.0.0"
        ),
        (
            MacOS,
            r#""Chromium";v="119", "Not?A_Brand";v="24", "Opera";v="105""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36 OPR/105.0.0.0"
        ),
        (
            Linux,
            r#""Chromium";v="119", "Not?A_Brand";v="24", "Opera";v="105""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36 OPR/105.0.0.0"
        )
    ]
);
