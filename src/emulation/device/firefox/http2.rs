// Firefox HTTP/2 configuration macros
// Note: Macros resolve types at the expansion site (in mod.rs)

macro_rules! headers_priority {
    () => {
        StreamDependency::new(StreamId::from(13), 41, false)
    };
}

macro_rules! pseudo_order {
    () => {
        [PseudoOrder::Method, PseudoOrder::Path, PseudoOrder::Authority, PseudoOrder::Scheme]
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
            .initial_stream_window_size(131072)
            .initial_connection_window_size(12582912)
            .max_frame_size(16384)
            .headers_priority(headers_priority!())
            .headers_pseudo_order(pseudo_order!())
            .settings_order(settings_order!())
    };

    (1) => {
        http2_options!(@base Http2Config::builder())
            .header_table_size(65536)
            .enable_push(false)
            .build()
    };
    (2) => {
        http2_options!(@base Http2Config::builder())
            .initial_stream_id(15)
            .header_table_size(65536)
            .priority(vec![
                Priority::new(
                    StreamId::from(3),
                    StreamDependency::new(StreamId::from(0), 200, false),
                ),
                Priority::new(
                    StreamId::from(5),
                    StreamDependency::new(StreamId::from(0), 100, false),
                ),
                Priority::new(
                    StreamId::from(7),
                    StreamDependency::new(StreamId::from(0), 0, false),
                ),
                Priority::new(
                    StreamId::from(9),
                    StreamDependency::new(StreamId::from(7), 0, false),
                ),
                Priority::new(
                    StreamId::from(11),
                    StreamDependency::new(StreamId::from(3), 0, false),
                ),
                Priority::new(
                    StreamId::from(13),
                    StreamDependency::new(StreamId::from(0), 240, false),
                ),
            ])
            .build()
    };
    (3) => {
        Http2Config::builder()
            .initial_stream_window_size(131072)
            .initial_connection_window_size(12582912)
            .max_frame_size(16384)
            .header_table_size(65536)
            .enable_push(false)
            .max_concurrent_streams(0)
            .headers_priority(headers_priority!())
            .headers_pseudo_order(pseudo_order!())
            .settings_order(settings_order!())
            .build()
    };
    (4) => {
        Http2Config::builder()
            .initial_stream_window_size(32768)
            .initial_connection_window_size(12582912)
            .max_frame_size(16384)
            .header_table_size(4096)
            .enable_push(false)
            .headers_priority(headers_priority!())
            .headers_pseudo_order(pseudo_order!())
            .settings_order(settings_order!())
            .build()
    };
}

