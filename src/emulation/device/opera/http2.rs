// Opera uses the same HTTP/2 configuration as Chrome (Chromium-based)
#[allow(unused_macro_rules)]
macro_rules! http2_options {
    (1) => {{
        Http2Config::builder()
            .initial_stream_window_size(6291456)
            .initial_connection_window_size(15728640)
            .max_concurrent_streams(1000)
            .max_header_list_size(262144)
            .header_table_size(65536)
            .build()
    }};
    (2) => {{
        Http2Config::builder()
            .initial_stream_window_size(6291456)
            .initial_connection_window_size(15728640)
            .max_concurrent_streams(1000)
            .max_header_list_size(262144)
            .header_table_size(65536)
            .enable_push(false)
            .build()
    }};
    (3) => {{
        Http2Config::builder()
            .initial_stream_window_size(6291456)
            .initial_connection_window_size(15728640)
            .max_header_list_size(262144)
            .header_table_size(65536)
            .enable_push(false)
            .build()
    }};
}
