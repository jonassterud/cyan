/// Error type.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid system time")]
    SystemTime(#[from] std::time::SystemTimeError),
    #[error("failed casting integer")]
    TryFromIntError(#[from] std::num::TryFromIntError),
    #[error("failed hashing / signing")]
    Secp256k1(#[from] secp256k1::Error),
    #[error("wrong value (expected {expected:?}, found {found:?})")]
    ExpectedFound { expected: String, found: String },
}
