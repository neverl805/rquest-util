// Safari HTTP/2 configuration macros
// Note: Macros are hygienic and resolve types at the expansion site

macro_rules! headers_priority {
    (1) => {
        StreamDependency::new(StreamId::zero(), 255, true)
    };
    (2) => {
        StreamDependency::new(StreamId::zero(), 255, false)
    };
}

macro_rules! pseudo_order {
    (1) => {
        [PseudoOrder::Method, PseudoOrder::Scheme, PseudoOrder::Path, PseudoOrder::Authority]
    };
    (2) => {
        [PseudoOrder::Method, PseudoOrder::Scheme, PseudoOrder::Authority, PseudoOrder::Path]
    };
}

macro_rules! settings_order {
    (1) => {
        [
            SettingsOrder::HeaderTableSize,
            SettingsOrder::EnablePush,
            SettingsOrder::InitialWindowSize,
            SettingsOrder::MaxConcurrentStreams,
            SettingsOrder::MaxFrameSize,
            SettingsOrder::MaxHeaderListSize,
            SettingsOrder::UnknownSetting8,
            SettingsOrder::UnknownSetting9,
        ]
    };
    (2) => {
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

macro_rules! http2_options {
    (@base $builder:expr, $headers_priority:expr, $pseudo_order:expr, $settings_order:expr) => {
        $builder
            .max_concurrent_streams(100)
            .headers_priority($headers_priority)
            .headers_pseudo_order($pseudo_order)
            .settings_order($settings_order)
    };

    (1) => {
        http2_options!(@base Http2Config::builder(), headers_priority!(1), pseudo_order!(1), settings_order!(1))
            .initial_stream_window_size(2097152)
            .initial_connection_window_size(10551295)
            .build()
    };
    (2) => {
        http2_options!(@base Http2Config::builder(), headers_priority!(1), pseudo_order!(1), settings_order!(1))
            .initial_stream_window_size(2097152)
            .initial_connection_window_size(10551295)
            .enable_push(false)
            .build()
    };
    (3) => {
        http2_options!(@base Http2Config::builder(), headers_priority!(2), pseudo_order!(2), settings_order!(2))
            .initial_stream_window_size(2097152)
            .initial_connection_window_size(10485760)
            .enable_push(false)
            .unknown_setting8(true)
            .unknown_setting9(true)
            .build()
    };
    (4) => {
        http2_options!(@base Http2Config::builder(), headers_priority!(1), pseudo_order!(1), settings_order!(1))
            .initial_stream_window_size(4194304)
            .initial_connection_window_size(10551295)
            .build()
    };
    (5) => {
        http2_options!(@base Http2Config::builder(), headers_priority!(1), pseudo_order!(1), settings_order!(1))
            .initial_stream_window_size(4194304)
            .initial_connection_window_size(10551295)
            .enable_push(false)
            .build()
    };
    (6) => {
        http2_options!(@base Http2Config::builder(), headers_priority!(2), pseudo_order!(2), settings_order!(2))
            .initial_stream_window_size(2097152)
            .initial_connection_window_size(10485760)
            .enable_push(false)
            .unknown_setting9(true)
            .build()
    };
}

