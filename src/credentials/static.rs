use crate::Result;
use crate::credentials::{Credentials, CredentialsProvider};

/// A credentials provider that always returns the same hard-coded credentials.
///
/// Useful for long-term AK/SK pairs or for tests. For temporary STS tokens
/// obtained out-of-band, pass the optional `security_token`.
#[derive(Clone, Debug)]
pub struct StaticCredentialsProvider {
    credentials: Credentials,
}

impl StaticCredentialsProvider {
    pub fn new(access_key_id: impl Into<String>, access_key_secret: impl Into<String>) -> Self {
        Self {
            credentials: Credentials::new(access_key_id, access_key_secret),
        }
    }

    pub fn with_security_token(
        access_key_id: impl Into<String>,
        access_key_secret: impl Into<String>,
        security_token: impl Into<String>,
    ) -> Self {
        Self {
            credentials: Credentials::with_sts(access_key_id, access_key_secret, security_token, None),
        }
    }

    pub fn from_credentials(credentials: Credentials) -> Self {
        Self { credentials }
    }
}

impl CredentialsProvider for StaticCredentialsProvider {
    async fn get_credentials(&self) -> Result<Credentials> {
        Ok(self.credentials.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_static_provider_returns_credentials() {
        let p = StaticCredentialsProvider::new("ak", "sk");
        let c = p.get_credentials().await.unwrap();
        assert_eq!(c.access_key_id, "ak");
        assert_eq!(c.access_key_secret, "sk");
        assert!(c.security_token.is_none());
        assert!(c.expiration.is_none());
    }

    #[tokio::test]
    async fn test_static_provider_with_sts() {
        let p = StaticCredentialsProvider::with_security_token("ak", "sk", "tok");
        let c = p.get_credentials().await.unwrap();
        assert_eq!(c.security_token.as_deref(), Some("tok"));
    }
}
