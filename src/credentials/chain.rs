use std::time::Duration;

use tracing::debug;

use crate::Result;
use crate::credentials::env::EnvironmentCredentialsProvider;
use crate::credentials::rrsa::RrsaCredentialsProvider;
use crate::credentials::{
    CachingCredentialsProvider,
    Credentials,
    CredentialsProvider,
    DynCredentialsProvider,
};

/// Walks a list of providers in order, returning credentials from the first
/// provider that succeeds.
///
/// Errors from individual providers are logged at `debug` level and do not
/// abort the chain; if every provider fails, the error from the last one is
/// returned.
pub struct CredentialsChain {
    providers: Vec<(String, DynCredentialsProvider)>,
}

impl std::fmt::Debug for CredentialsChain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names: Vec<_> = self.providers.iter().map(|(n, _)| n.as_str()).collect();
        f.debug_struct("CredentialsChain")
            .field("providers", &names)
            .finish()
    }
}

impl CredentialsChain {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }

    pub fn push<P>(mut self, name: impl Into<String>, provider: P) -> Self
    where
        P: CredentialsProvider + 'static,
    {
        self.providers
            .push((name.into(), DynCredentialsProvider::new(provider)));
        self
    }
}

impl Default for CredentialsChain {
    fn default() -> Self {
        Self::new()
    }
}

impl CredentialsProvider for CredentialsChain {
    async fn get_credentials(&self) -> Result<Credentials> {
        let mut last_error = None;
        for (name, provider) in &self.providers {
            match provider.get_credentials().await {
                Ok(c) => return Ok(c),
                Err(e) => {
                    debug!(target: "ossify::credentials", "provider `{name}` failed: {e}");
                    last_error = Some(e);
                },
            }
        }
        Err(last_error.unwrap_or(crate::Error::InvalidCredentials))
    }
}

/// A sensible default credentials chain that follows the order recommended by
/// the Alibaba Cloud SDK documentation:
///
/// 1. `EnvironmentCredentialsProvider` – reads `ALIBABA_CLOUD_ACCESS_KEY_ID`
///    etc. from the environment.
/// 2. `RrsaCredentialsProvider::from_env` – resolves RRSA/OIDC config from
///    `ALIBABA_CLOUD_ROLE_ARN`, `ALIBABA_CLOUD_OIDC_PROVIDER_ARN`, and
///    `ALIBABA_CLOUD_OIDC_TOKEN_FILE`.
///
/// The result is cached via [`CachingCredentialsProvider`] so temporary
/// credentials are reused until they approach expiration.
#[derive(Debug)]
pub struct DefaultCredentialsChain {
    inner: CachingCredentialsProvider<CredentialsChain>,
}

/// Builder for [`DefaultCredentialsChain`].
#[derive(Debug)]
pub struct DefaultCredentialsChainBuilder {
    http_client: Option<reqwest::Client>,
    refresh_skew: Option<Duration>,
}

impl DefaultCredentialsChainBuilder {
    pub fn new() -> Self {
        Self {
            http_client: None,
            refresh_skew: None,
        }
    }

    /// Reuse the provided `reqwest::Client` for STS calls made by the RRSA
    /// provider inside the chain.
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = Some(client);
        self
    }

    /// How long before expiration to proactively refresh credentials.
    ///
    /// Defaults to 5 minutes. Applies to the outer
    /// [`CachingCredentialsProvider`] that wraps the whole chain.
    pub fn refresh_skew(mut self, skew: Duration) -> Self {
        self.refresh_skew = Some(skew);
        self
    }

    pub fn build(self) -> DefaultCredentialsChain {
        let http_client = self.http_client.unwrap_or_default();
        let mut chain = CredentialsChain::new().push("environment", EnvironmentCredentialsProvider::new());

        if let Some(rrsa) = RrsaCredentialsProvider::from_env(http_client) {
            chain = chain.push("rrsa", rrsa);
        }

        let mut caching = CachingCredentialsProvider::new(chain);
        if let Some(skew) = self.refresh_skew {
            caching = caching.with_refresh_skew(skew);
        }
        DefaultCredentialsChain { inner: caching }
    }
}

impl Default for DefaultCredentialsChainBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultCredentialsChain {
    /// Build the default chain using the process's current environment
    /// variables and a fresh `reqwest::Client` (for STS calls, when needed).
    pub fn new() -> Self {
        Self::builder().build()
    }

    /// Same as [`Self::new`], but reuses the provided `reqwest::Client` for
    /// STS requests.
    pub fn with_http_client(http_client: reqwest::Client) -> Self {
        Self::builder().http_client(http_client).build()
    }

    /// Returns a builder for fine-grained configuration of the default chain.
    pub fn builder() -> DefaultCredentialsChainBuilder {
        DefaultCredentialsChainBuilder::new()
    }
}

impl Default for DefaultCredentialsChain {
    fn default() -> Self {
        Self::new()
    }
}

impl CredentialsProvider for DefaultCredentialsChain {
    async fn get_credentials(&self) -> Result<Credentials> {
        self.inner.get_credentials().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::credentials::StaticCredentialsProvider;

    #[derive(Debug)]
    struct FailingProvider;

    impl CredentialsProvider for FailingProvider {
        async fn get_credentials(&self) -> Result<Credentials> {
            Err(crate::Error::InvalidCredentials)
        }
    }

    #[tokio::test]
    async fn test_chain_returns_first_success() {
        let chain = CredentialsChain::new()
            .push("failing", FailingProvider)
            .push("static", StaticCredentialsProvider::new("ak", "sk"));
        let c = chain.get_credentials().await.unwrap();
        assert_eq!(c.access_key_id, "ak");
    }

    #[tokio::test]
    async fn test_chain_empty_returns_invalid_credentials() {
        let chain = CredentialsChain::new();
        let err = chain.get_credentials().await.unwrap_err();
        assert!(matches!(err, crate::Error::InvalidCredentials));
    }

    #[tokio::test]
    async fn test_chain_all_failing_returns_last_error() {
        let chain = CredentialsChain::new()
            .push("a", FailingProvider)
            .push("b", FailingProvider);
        let err = chain.get_credentials().await.unwrap_err();
        assert!(matches!(err, crate::Error::InvalidCredentials));
    }

    #[test]
    fn test_default_chain_builder_refresh_skew() {
        let builder = DefaultCredentialsChain::builder().refresh_skew(Duration::from_secs(120));
        assert_eq!(builder.refresh_skew, Some(Duration::from_secs(120)));
    }
}
