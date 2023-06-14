//! This module encapsulates functionality related to the configuration of OpenID Connect providers.

use crate::errors::Error;
use serde::{Deserialize, Serialize};
use url::Url;

/// The configuration of the identity provider, which can be retrieved via /.well-known/openid-configuration API
/// of your LogTo instance (logto.dev if you're using the hosted version)
///
/// get the configuration for your LogTo instance by calling `fetch_oidc_config`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OidcConfigResponse {
    authorization_endpoint: Url,
    token_endpoint: Url,
    end_session_endpoint: Url,
    revocation_endpoint: Url,
    jwks_uri: Url,
    issuer: String,
}

pub async fn fetch_oidc_config(endpoint: Url) -> Result<OidcConfigResponse, Error> {
    reqwest::get(endpoint)
        .await
        .map_err(Error::from)?
        .error_for_status()
        .map_err(Error::from)?
        .json::<OidcConfigResponse>()
        .await
        .map_err(Error::from)
}

#[cfg(test)]
mod test {
    use super::*;
    use httpmock::prelude::*;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use tokio_test;

    macro_rules! blocking {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_fetch_oidc_config() -> Result<(), Error> {
        // Start a lightweight mock server.
        let server = MockServer::start();

        let mock_response = OidcConfigResponse {
            authorization_endpoint: Url::parse(&server.url("/oidc/authorize")).unwrap(),
            token_endpoint: Url::parse(&server.url("/oidc/token")).unwrap(),
            end_session_endpoint: Url::parse(&server.url("/oidc/logout")).unwrap(),
            revocation_endpoint: Url::parse(&server.url("/oidc/revoke")).unwrap(),
            jwks_uri: Url::parse(&server.url("/oidc/jwks")).unwrap(),
            issuer: server.url("/oidc").to_string(),
        };

        // Create a mock on the server.
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/oidc/.well-known/openid-configuration");
            then.status(200)
                .header("content-type", "text/json; charset=UTF-8")
                .json_body(json!(mock_response));
        });

        // make the request
        let oidc_config = blocking!(fetch_oidc_config(
            Url::parse(&server.url("/oidc/.well-known/openid-configuration")).unwrap()
        ))?;

        // Ensure the mock server recorded the request.
        mock.assert();

        // Ensure the response was as we expected.
        assert_eq!(oidc_config, mock_response);

        Ok(())
    }

    #[test]
    fn test_fetch_oidc_config_error() -> Result<(), Error> {
        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/oidc/.well-known/openid-configuration");
            then.status(500);
        });

        // make the request
        let oidc_config = blocking!(fetch_oidc_config(
            Url::parse(&server.url("/oidc/.well-known/openid-configuration")).unwrap()
        ));

        // Ensure the mock server recorded the request.
        mock.assert();

        // Ensure the response was as we expected.
        assert!(oidc_config.is_err());
        // check that the error type
        assert!(match oidc_config.unwrap_err() {
            Error::Reqwest(e) => e.is_status(),
            _ => false,
        });

        Ok(())
    }

    #[test]
    fn test_fetch_oidc_config_bad_format() -> Result<(), Error> {
        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/oidc/.well-known/openid-configuration");
            then.status(200)
                .header("content-type", "text/json; charset=UTF-8")
                .body("not json");
        });

        // make the request
        let oidc_config = blocking!(fetch_oidc_config(
            Url::parse(&server.url("/oidc/.well-known/openid-configuration")).unwrap()
        ));

        // Ensure the mock server recorded the request.
        mock.assert();

        // Ensure the response was as we expected.
        assert!(oidc_config.is_err());
        // check error type
        assert!(match oidc_config.unwrap_err() {
            Error::Reqwest(e) => e.is_decode(),
            _ => false,
        });

        Ok(())
    }
}
