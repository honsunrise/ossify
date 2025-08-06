mod body;
mod credential;
mod error;
pub mod ops;
mod query_auth_option;
mod response;
mod ser;
mod utils;

use std::borrow::Cow;
use std::collections::HashSet;
use std::time::Duration;

use http::HeaderMap;
use http::header::HOST;
use serde::Serialize;
use tracing::trace;
use url::Url;

use self::body::MakeBody;
use self::credential::{Credential, SignContext};
pub use self::error::Error;
use self::error::Result;
pub use self::query_auth_option::{QueryAuthOptions, QueryAuthOptionsBuilder};
use self::response::ResponseProcessor;
use self::utils::escape_path;

pub(crate) trait Ops: Sized {
    const PRODUCT: &'static str = "oss";
    const USE_BUCKET: bool = true;

    type Query;
    type Body: MakeBody;
    type Response;

    /// The HTTP Method used for this operation (e.g. GET, PATCH, DELETE)
    fn method(&self) -> http::Method;

    /// The Key for this operation
    fn key<'a>(&'a self) -> Option<Cow<'a, str>> {
        None
    }

    /// Additional headers used for signature calculation
    fn additional_headers(&self) -> HashSet<String> {
        HashSet::new()
    }

    /// Additional headers
    fn headers(&self) -> Result<Option<HeaderMap>> {
        Ok(None)
    }

    /// The query string for the request, if any
    fn query(&self) -> Option<&Self::Query> {
        None
    }

    /// The body of the request, if any
    fn body(&self) -> Option<&<Self::Body as MakeBody>::Body> {
        None
    }
}

pub(crate) trait Request<P> {
    type Response;

    fn request(&self, ops: P) -> impl Future<Output = Result<Self::Response>>;

    fn presign(
        &self,
        ops: P,
        public: bool,
        query_auth_options: Option<QueryAuthOptions>,
    ) -> impl Future<Output = Result<String>>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UrlStyle {
    #[default]
    VirtualHosted,
    Path,
    CName,
}

/// Configuration for the API client.
/// Allows users to customize its behaviors.
pub struct ClientConfig {
    /// The maximum time limit for an request.
    pub http_timeout: Duration,
    /// A default set of HTTP headers which will be sent with each API request.
    pub default_headers: http::HeaderMap,
    /// The URL style to use for the API client that uses internal endpoint.
    pub url_style: UrlStyle,
    /// The URL style to use for the API client that uses public endpoint.
    pub public_url_style: UrlStyle,
}

impl Default for ClientConfig {
    fn default() -> Self {
        ClientConfig {
            http_timeout: Duration::from_secs(30),
            default_headers: http::HeaderMap::default(),
            url_style: UrlStyle::default(),
            public_url_style: UrlStyle::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    http_client: reqwest::Client,

    raw_internal_host: String,
    raw_internal_scheme: String,
    raw_public_host: String,
    raw_public_scheme: String,
    region: String,
    bucket: String,
    url_style: UrlStyle,
    public_url_style: UrlStyle,
    credentials: Credential,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    fn build_url<'a>(
        &'a self,
        bucket: Option<Cow<'a, str>>,
        key: Option<Cow<'a, str>>,
        public: bool,
    ) -> (Cow<'a, str>, Cow<'a, str>) {
        let host = if public {
            self.raw_public_host.as_str()
        } else {
            self.raw_internal_host.as_str()
        };

        let url_style = if public {
            self.public_url_style
        } else {
            self.url_style
        };

        let (host, paths) = match (bucket, url_style) {
            (Some(bucket_name), UrlStyle::VirtualHosted) => {
                (Cow::Owned(format!("{bucket_name}.{host}")), None)
            },
            (Some(bucket_name), UrlStyle::Path) => {
                let mut paths = Vec::with_capacity(2);
                paths.push(bucket_name);
                if key.is_none() {
                    paths.push(Cow::Borrowed(""));
                }
                (Cow::Borrowed(host), Some(paths))
            },
            (None, _) | (Some(_), UrlStyle::CName) => (Cow::Borrowed(host), None),
        };

        let path = match (paths, key.as_ref().map(|k| k.trim().trim_start_matches('/'))) {
            (Some(paths), Some(key_str)) => {
                let mut paths = paths.clone();
                paths.push(escape_path(key_str).into());
                Cow::Owned(format!("/{}", paths.join("/")))
            },
            (Some(paths), None) => Cow::Owned(format!("/{}", paths.join("/"))),
            (None, Some(key_str)) => Cow::Owned(format!("/{}", escape_path(key_str))),
            (None, None) => Cow::Borrowed("/"),
        };

        (host, path)
    }

    fn prepare_request<P>(
        &self,
        ops: P,
        public: bool,
        query_auth_options: Option<QueryAuthOptions>,
    ) -> Result<reqwest::Request>
    where
        P: Ops + Send + 'static,
        P::Query: Serialize + Send,
        P::Response: ResponseProcessor + Send,
        P::Body: MakeBody + Send,
    {
        let method = ops.method();
        let bucket = P::USE_BUCKET.then_some(Cow::Borrowed(self.bucket.as_str()));

        // Build the request
        let (host, path) = self.build_url(bucket.clone(), ops.key(), public);
        let scheme = if public {
            &self.raw_public_scheme
        } else {
            &self.raw_internal_scheme
        };
        let request_url = format!("{scheme}://{host}{path}");
        let mut request = self.http_client.request(method.clone(), request_url).build()?;

        // Prepare additional headers
        let mut additional_headers = ops.additional_headers();

        // Fill the body if any
        if let Some(body) = ops.body() {
            P::Body::make_body(body, &mut request)?;
            additional_headers.insert("content-length".to_string());
        }

        let headers = request.headers_mut();
        // Fill the headers if any
        if let Some(extra_headers) = ops.headers()? {
            headers.extend(extra_headers);
        }

        headers.insert(HOST, host.parse()?);

        // Prepare sign context
        let sign_context = SignContext {
            region: Cow::Borrowed(&self.region),
            product: Cow::Borrowed(P::PRODUCT),
            bucket,
            key: ops.key(),
            query: ops.query(),
            additional_headers,
        };

        // Authenticate the request
        self.credentials
            .auth_to(&mut request, sign_context, query_auth_options)?;

        Ok(request)
    }
}

impl<P> Request<P> for Client
where
    P: Ops + Send + 'static,
    P::Query: Serialize + Send,
    P::Response: ResponseProcessor + Send,
    P::Body: MakeBody + Send,
{
    type Response = <P::Response as ResponseProcessor>::Output;

    async fn request(&self, ops: P) -> Result<Self::Response> {
        let request = self.prepare_request(ops, false, None)?;

        // Send the request
        trace!("Sending request: {request:?}");
        let resp = self.http_client.execute(request).await?;

        // Parse the response
        P::Response::from_response(resp).await
    }

    async fn presign(
        &self,
        ops: P,
        public: bool,
        query_auth_options: Option<QueryAuthOptions>,
    ) -> Result<String> {
        let request = self.prepare_request(ops, public, query_auth_options)?;

        let sign_url = request.url().to_string();
        Ok(sign_url)
    }
}

pub struct ClientBuilder {
    config: ClientConfig,
    endpoint: Option<String>,
    public_endpoint: Option<String>,
    region: Option<String>,
    bucket: Option<String>,
    access_key_id: Option<String>,
    access_key_secret: Option<String>,
    security_token: Option<String>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            config: ClientConfig::default(),
            endpoint: None,
            public_endpoint: None,
            region: None,
            bucket: None,
            access_key_id: None,
            access_key_secret: None,
            security_token: None,
        }
    }

    /// Set the OSS endpoint URL
    pub fn endpoint<T: AsRef<str>>(mut self, endpoint: T) -> Self {
        self.endpoint = Some(endpoint.as_ref().to_string());
        self
    }

    /// Set the public OSS endpoint URL (optional, defaults to endpoint if not set)
    pub fn public_endpoint<T: AsRef<str>>(mut self, public_endpoint: T) -> Self {
        self.public_endpoint = Some(public_endpoint.as_ref().to_string());
        self
    }

    /// Set the OSS region
    pub fn region<T: AsRef<str>>(mut self, region: T) -> Self {
        self.region = Some(region.as_ref().to_string());
        self
    }

    /// Set the bucket name
    pub fn bucket<T: AsRef<str>>(mut self, bucket: T) -> Self {
        self.bucket = Some(bucket.as_ref().to_string());
        self
    }

    /// Set the access key ID for authentication
    pub fn access_key_id<T: AsRef<str>>(mut self, access_key_id: T) -> Self {
        self.access_key_id = Some(access_key_id.as_ref().to_string());
        self
    }

    /// Set the access key secret for authentication
    pub fn access_key_secret<T: AsRef<str>>(mut self, access_key_secret: T) -> Self {
        self.access_key_secret = Some(access_key_secret.as_ref().to_string());
        self
    }

    /// Set the security token (optional, for temporary credentials)
    pub fn security_token<T: AsRef<str>>(mut self, security_token: T) -> Self {
        self.security_token = Some(security_token.as_ref().to_string());
        self
    }

    /// Set the HTTP timeout for requests
    pub fn http_timeout(mut self, timeout: Duration) -> Self {
        self.config.http_timeout = timeout;
        self
    }

    /// Set default headers to be sent with each request
    pub fn default_headers(mut self, headers: http::HeaderMap) -> Self {
        self.config.default_headers = headers;
        self
    }

    /// Set the URL style for requests that use internal endpoint
    pub fn url_style(mut self, style: UrlStyle) -> Self {
        self.config.url_style = style;
        self
    }

    /// Set the URL style for requests that use public endpoint
    pub fn public_url_style(mut self, style: UrlStyle) -> Self {
        self.config.public_url_style = style;
        self
    }

    /// Build the Client with the configured parameters
    pub fn build(self) -> Result<Client> {
        // Validate required fields
        let endpoint = self
            .endpoint
            .ok_or_else(|| Error::InvalidArgument("endpoint is required".to_string()))?;
        let region = self
            .region
            .ok_or_else(|| Error::InvalidArgument("region is required".to_string()))?;
        let bucket = self
            .bucket
            .ok_or_else(|| Error::InvalidArgument("bucket is required".to_string()))?;
        let access_key_id = self
            .access_key_id
            .ok_or_else(|| Error::InvalidArgument("access_key_id is required".to_string()))?;
        let access_key_secret = self
            .access_key_secret
            .ok_or_else(|| Error::InvalidArgument("access_key_secret is required".to_string()))?;

        // Build HTTP client
        let http_client = reqwest::Client::builder()
            .default_headers(self.config.default_headers)
            .timeout(self.config.http_timeout)
            .build()?;

        // Parse endpoint URL
        let endpoint_url = Url::parse(&endpoint)?;
        let raw_internal_host = endpoint_url.host_str().ok_or(Error::MissingHost)?.to_owned();
        let raw_internal_scheme = endpoint_url.scheme().to_owned();

        // Parse public endpoint URL (use internal endpoint if not specified)
        let public_endpoint_str = self.public_endpoint.as_ref().unwrap_or(&endpoint);
        let public_endpoint_url = Url::parse(public_endpoint_str)?;
        let raw_public_host = public_endpoint_url
            .host_str()
            .ok_or(Error::MissingHost)?
            .to_owned();
        let raw_public_scheme = public_endpoint_url.scheme().to_owned();

        // Build credentials
        let credentials = Credential {
            security_token: self.security_token,
            access_key_id,
            access_key_secret,
        };

        Ok(Client {
            region,
            bucket,
            raw_internal_host,
            raw_internal_scheme,
            raw_public_host,
            raw_public_scheme,
            url_style: self.config.url_style,
            public_url_style: self.config.public_url_style,
            credentials,
            http_client,
        })
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
