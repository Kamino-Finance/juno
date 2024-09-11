use solana_client::client_error::ClientError;
use solana_program::{
    instruction::InstructionError, message::SanitizeMessageError, sanitize::SanitizeError,
};
use solana_sdk::pubkey::ParsePubkeyError;
use thiserror::Error;

/// A `Result` alias where the `Err` case is `jupv4-openapi::Error`.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("io: {0:?}")]
    Io(#[from] std::io::Error),

    #[error("invalid pubkey in response data: {0}")]
    ParsePubkey(#[from] ParsePubkeyError),

    #[error("base64: {0}")]
    Base64Decode(#[from] base64::DecodeError),

    #[error("bincode: {0}")]
    Bincode(#[from] bincode::Error),

    #[error("Jupiter API: {0}")]
    JupiterApi(String),

    #[error("No route found for the requested swap")]
    NoValidRoute,

    #[error("Price impact too high")]
    PriceImpactTooHigh(f32),

    #[error("serde_json: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Error while decompiling provided transaction: {0:?}")]
    SanitizeMessageError(#[from] SanitizeMessageError),

    #[error("Error while decompiling provided transaction: {0:?}")]
    SanitizeError(#[from] SanitizeError),

    #[error("Lookup table deserialization failed: {0:?}")]
    LookupTableDeserialize(#[from] InstructionError),

    #[error("Lookup table account was not found onchain")]
    LookupTableAccountNotFound,

    #[error("Solana client error: {0:?}")]
    SolanaRpcError(#[from] ClientError),

    #[error("Solana transaction compile error")]
    SolanaCompileError,

    #[error("Response type conversion error")]
    ResponseTypeConversionError,

    #[error("Base URL already set")]
    BaseUrlAlreadySet,
}

impl<T> From<crate::apis::Error<T>> for Error
where
    T: core::fmt::Debug,
{
    fn from(api_error: crate::apis::Error<T>) -> Self {
        match api_error {
            crate::apis::Error::Reqwest(e) => Self::Reqwest(e),
            crate::apis::Error::Serde(e) => Self::SerdeJson(e),
            crate::apis::Error::Io(e) => Self::Io(e),
            crate::apis::Error::ResponseError(e) => Self::JupiterApi(format!("{:?}", e)),
        }
    }
}
