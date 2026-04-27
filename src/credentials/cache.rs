use std::time::Duration;

use tokio::sync::RwLock;

use crate::Result;
use crate::credentials::{Credentials, CredentialsProvider};

/// Default refresh-skew: refresh credentials when they are within 5 minutes of
/// expiration.
const DEFAULT_REFRESH_SKEW: Duration = Duration::from_secs(5 * 60);

/// Wraps another [`CredentialsProvider`] with an in-memory cache.
///
/// Credentials are only fetched from the underlying provider when the cache
/// is empty or when the current credentials have expired (or will expire
/// within `refresh_skew`). Concurrent callers share the same refresh via a
/// `tokio::sync::RwLock`.
#[derive(Debug)]
pub struct CachingCredentialsProvider<P> {
    inner: P,
    cache: RwLock<Option<Credentials>>,
    refresh_skew: Duration,
}

impl<P> CachingCredentialsProvider<P> {
    pub fn new(inner: P) -> Self {
        Self {
            inner,
            cache: RwLock::new(None),
            refresh_skew: DEFAULT_REFRESH_SKEW,
        }
    }

    pub fn with_refresh_skew(mut self, skew: Duration) -> Self {
        self.refresh_skew = skew;
        self
    }
}

impl<P> CredentialsProvider for CachingCredentialsProvider<P>
where
    P: CredentialsProvider,
{
    async fn get_credentials(&self) -> Result<Credentials> {
        // Fast path: read lock
        {
            let cached = self.cache.read().await;
            if let Some(c) = cached.as_ref()
                && !c.is_expired_within(self.refresh_skew)
            {
                return Ok(c.clone());
            }
        }

        // Slow path: acquire write lock and refresh
        let mut cached = self.cache.write().await;
        // Double-check after acquiring the write lock
        if let Some(c) = cached.as_ref()
            && !c.is_expired_within(self.refresh_skew)
        {
            return Ok(c.clone());
        }

        let fresh = self.inner.get_credentials().await?;
        *cached = Some(fresh.clone());
        Ok(fresh)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use jiff::{Timestamp, ToSpan};

    use super::*;

    #[derive(Debug)]
    struct CountingProvider {
        calls: AtomicUsize,
        expires_in: Option<jiff::Span>,
    }

    impl CountingProvider {
        fn new(expires_in: Option<jiff::Span>) -> Self {
            Self {
                calls: AtomicUsize::new(0),
                expires_in,
            }
        }

        fn call_count(&self) -> usize {
            self.calls.load(Ordering::SeqCst)
        }
    }

    impl CredentialsProvider for CountingProvider {
        async fn get_credentials(&self) -> Result<Credentials> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            let expiration = self
                .expires_in
                .map(|span| Timestamp::now().checked_add(span).unwrap());
            Ok(Credentials::with_sts("ak", "sk", "token", expiration))
        }
    }

    #[tokio::test]
    async fn test_cache_reuses_non_expired_credentials() {
        let inner = CountingProvider::new(Some(1.hour()));
        let cache = CachingCredentialsProvider::new(inner);

        let c1 = cache.get_credentials().await.unwrap();
        let c2 = cache.get_credentials().await.unwrap();
        assert_eq!(c1.access_key_id, c2.access_key_id);
        assert_eq!(cache.inner.call_count(), 1);
    }

    #[tokio::test]
    async fn test_cache_refetches_when_expiring() {
        let inner = CountingProvider::new(Some(1.second()));
        // force refresh_skew larger than expiration => always "expiring"
        let cache = CachingCredentialsProvider::new(inner).with_refresh_skew(Duration::from_secs(60));

        cache.get_credentials().await.unwrap();
        cache.get_credentials().await.unwrap();
        assert_eq!(cache.inner.call_count(), 2);
    }

    #[tokio::test]
    async fn test_cache_permanent_credentials_reused() {
        // No expiration => always reused.
        struct Permanent(AtomicUsize);
        impl std::fmt::Debug for Permanent {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("Permanent").finish()
            }
        }
        impl CredentialsProvider for Permanent {
            async fn get_credentials(&self) -> Result<Credentials> {
                self.0.fetch_add(1, Ordering::SeqCst);
                Ok(Credentials::new("ak", "sk"))
            }
        }

        let cache = CachingCredentialsProvider::new(Permanent(AtomicUsize::new(0)));
        cache.get_credentials().await.unwrap();
        cache.get_credentials().await.unwrap();
        cache.get_credentials().await.unwrap();
        assert_eq!(cache.inner.0.load(Ordering::SeqCst), 1);
    }
}
