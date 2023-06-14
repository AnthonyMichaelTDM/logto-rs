//! This module encapsulates functionality related to the configuration of OpenID Connect providers.
//!
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use url::Url;

/// The configuration of the identity provider, which can be retrieved via /oidc/.well-known/openid-configuration API.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
pub struct OidcConfigResponse {
    authorization_endpoint: Url,
    token_endpoint: Url,
    end_session_endpoint: Url,
    revocation_endpoint: Url,
    jwks_uri: Url,
    issuer: String,
}

