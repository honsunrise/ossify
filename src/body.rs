use std::marker::PhantomData;

use bytes::Bytes;
use futures::TryStream;
use http::HeaderValue;
use http::header::{CONTENT_LENGTH, CONTENT_TYPE};
use reqwest::Body;
use serde::Serialize;

use crate::BoxError;
use crate::error::Result;

pub trait MakeBody {
    type Body;

    fn make_body(body: Self::Body, request: &mut reqwest::Request) -> Result<()>;
}

pub struct NoneBody;

impl MakeBody for NoneBody {
    type Body = ();

    fn make_body(_body: Self::Body, _request: &mut reqwest::Request) -> Result<()> {
        Ok(())
    }
}

pub struct ZeroBody;

impl MakeBody for ZeroBody {
    type Body = ();

    fn make_body(_body: Self::Body, request: &mut reqwest::Request) -> Result<()> {
        let headers = request.headers_mut();
        headers.insert(CONTENT_LENGTH, HeaderValue::from_static("0"));
        Ok(())
    }
}

pub struct XMLBody<T>(PhantomData<T>);

impl<T> MakeBody for XMLBody<T>
where
    T: Serialize,
{
    type Body = T;

    fn make_body(body: Self::Body, request: &mut reqwest::Request) -> Result<()> {
        let body = quick_xml::se::to_string(&body)?;
        let headers = request.headers_mut();
        headers.insert(CONTENT_LENGTH, HeaderValue::from_str(&body.len().to_string())?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/xml"));
        request.body_mut().replace(Body::from(body));
        Ok(())
    }
}

pub struct StreamBody<S>(PhantomData<S>);

impl<S> MakeBody for StreamBody<S>
where
    S: TryStream + Send + 'static,
    S::Error: Into<BoxError>,
    Bytes: From<S::Ok>,
{
    type Body = S;

    fn make_body(body: Self::Body, request: &mut reqwest::Request) -> Result<()> {
        request.body_mut().replace(Body::wrap_stream(body));
        Ok(())
    }
}
