use rquest::SslCurve;

/// Commonly used curve configurations as static arrays
pub const CURVES_1: &[SslCurve] = &[SslCurve::X25519, SslCurve::SECP256R1, SslCurve::SECP384R1];

// Note: Post-quantum curves like X25519MLKEM768 are not yet supported in rquest.


