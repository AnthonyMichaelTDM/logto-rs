//! # Core
//! This crate contains the core functionality needed
//! to use LogTo auth in your application, on the server-side.
//! checkout the examples to see how to use this in web-frameworks like Actix-web
//!
//! because this crate needs to use your LogTo api key to interface with LogTo, you should
//! not use it in user facing code (such as a website frontend, desktop app, etc.).
//! instead, you should use the [logto-client](https://crates.io/crates/logto-client) crate, which
//! is designed to be used in user facing code.
//!
//! ## Functionality
//! - Types
//!     - [ ] OidcConfigResponse
//!     - [ ] CodeTokenResponse
//!     - [ ] RefreshTokenResponse
//!     - [ ] IdTokenClaims
//! - Utility Functions
//!     - [ ] generateCodeVerifier
//!     - [ ] generateCodeChallenge
//!     - [ ] generateState
//!     - [ ] decodeIdToken
//!     - [ ] verifyIdToken
//!     - [ ] verifyAndParseCodeFromCallbackUri
//! - Core Functions
//!     - [ ] fetchOidcConfig
//!     - [ ] generateSignInUri
//!     - [ ] generateSignOutUri
//!     - [ ] fetchTokenByAuthorizationCode
//!     - [ ] fetchTokenByRefreshToken
//!     - [ ] revoke

mod errors;
