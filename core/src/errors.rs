//! Errors that can occur in the core crate

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /// An error occurred with the reqwest crate, which is used for making HTTP requests to the
    /// LogTo API
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    /// An error occurred with the chrono crate
    #[error(transparent)]
    Chrono(#[from] chrono::ParseError),
    /// An error occurred with the serde crate
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    /// An error occurred with the url crate
    #[error(transparent)]
    Url(#[from] url::ParseError),
    /// An error occurred with the base64 crate
    #[error(transparent)]
    Base64(#[from] base64::DecodeError),
    /// An error occurred with validating a token
    #[error(transparent)]
    Token(#[from] TokenError),
    /// An error occurred with validating a callback uri
    #[error(transparent)]
    Callback(#[from] CallbackError),
    /// Another error occurred
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
/// Errors that can occur while validating a token
pub enum TokenError {
    #[error("issuer mismatch: expected {expected}, got {actual}")]
    IssuerMismatch {
        /// the expected issuer
        expected: String,
        /// the actual issuer
        actual: String,
    },
    #[error("audience mismatch: expected {expected}, got {actual}")]
    AudienceMismatch {
        /// the expected audience
        expected: String,
        /// the actual audience
        actual: String,
    },
    #[error("token is expired, expired on {} UTC", .0.to_rfc3339())]
    Expired(chrono::DateTime<chrono::Utc>),
    #[error("token was issued in the future: {} UTC", .0.to_rfc3339())]
    IssuedInTheFuture(chrono::DateTime<chrono::Utc>),
    #[error("token was issued too far in the past: {} UTC", .0.to_rfc3339())]
    IssuedTooFarInThePast(chrono::DateTime<chrono::Utc>),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
/// Errors that can occur relating to Callback URIs
pub enum CallbackError {
    /// provided state does not match the state in the Callback URI
    #[error("state mismatch, expected: {in_uri}, got: {provided}")]
    StateMismatch {
        /// the state in the Callback URI
        in_uri: String,
        /// the state provided by the user
        provided: String,
    },
    /// the Callback URI is missing a code
    #[error("missing code in callback URI: {0}")]
    MissingCode(String),
    /// callback uri does not match redirect uri
    #[error("callback uri does not match redirect uri: {callback} != {redirect}")]
    UriMismatch {
        /// the callback uri
        callback: String,
        /// the redirect uri
        redirect: String,
    },
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
