//! Emulation for different browsers.

#[macro_use]
mod macros;
mod curves;
pub mod chrome;
pub mod firefox;
pub mod okhttp;
pub mod opera;
pub mod safari;

// curves module is kept private - each browser defines its own curves

pub use typed_builder::TypedBuilder;
#[cfg(all(
    feature = "emulation-gzip",
    feature = "emulation-deflate",
    feature = "emulation-brotli"
))]
pub use rquest::header::ACCEPT_ENCODING;

// Re-export types from parent
#[allow(unused_imports)]
pub use crate::emulation::{
    AlpnProtos, AlpsProtos, CertificateCompressionAlgorithm, EmulationOS, EmulationOption,
    Priority, PseudoOrder, SettingsOrder, StreamDependency, StreamId, TlsConfig, TlsVersion,
};
pub use rquest::http2::Http2Options;
pub use rquest::{TlsOptions, Priorities};
// Re-export enum variants for use in macros
#[allow(unused_imports)]
pub use rquest::http2::{PseudoId, SettingId};
#[allow(unused_imports)]
pub use rquest::http2::{PseudoId::*, SettingId::*};

// Import types from rquest
pub use rquest::{
    ExtensionType, SslCurve,
    header::{ACCEPT, ACCEPT_LANGUAGE, HeaderMap, HeaderName, HeaderValue, USER_AGENT},
};

