// Chrome HTTP/2 configuration macros
// Note: Macros resolve types at the expansion site (in mod.rs)

macro_rules! headers_priority {
    () => {
        StreamDependency::new(StreamId::from(0), 255, true)
    };
}

macro_rules! pseudo_order {
    () => {
        [PseudoOrder::Method, PseudoOrder::Authority, PseudoOrder::Scheme, PseudoOrder::Path]
    };
}

macro_rules! settings_order {
    () => {
        [
            SettingsOrder::HeaderTableSize,
            SettingsOrder::EnablePush,
            SettingsOrder::MaxConcurrentStreams,
            SettingsOrder::InitialWindowSize,
            SettingsOrder::MaxFrameSize,
            SettingsOrder::MaxHeaderListSize,
            SettingsOrder::UnknownSetting8,
            SettingsOrder::UnknownSetting9,
        ]
    };
}

#[allow(unused_macro_rules)]
macro_rules! http2_options {
    (@base $builder:expr) => {
        $builder
            .initial_stream_window_size(6291456)
            .initial_connection_window_size(15728640)
            .max_header_list_size(262144)
            .header_table_size(65536)
            .headers_priority(headers_priority!())
            .headers_pseudo_order(pseudo_order!())
            .settings_order(settings_order!())
    };

    (1) => {
        http2_options!(@base Http2Config::builder())
            .max_concurrent_streams(1000)
            .build()
    };
    (2) => {
        http2_options!(@base Http2Config::builder())
            .max_concurrent_streams(1000)
            .enable_push(false)
            .build()
    };
    (3) => {
        http2_options!(@base Http2Config::builder())
            .enable_push(false)
            .build()
    };
}

