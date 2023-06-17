pub mod connection;
pub mod mctypes;
pub mod packet;
pub mod stream;

// TODO: The eventual goal is to support multiple versions using macros to generate
// packet structures per-version, though this may or may not be feasible. External
// codegen (in conjunction with macros) might be possible, too.

/// 761 = 1.19.3 https://wiki.vg/Protocol_version_numbers
#[allow(dead_code)]
pub const PROTOCOL_VERSION: i32 = 761;
