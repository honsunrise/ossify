use std::marker::PhantomData;

use bytes::Bytes;
use http::HeaderValue;
use http::header::{CONTENT_LENGTH, CONTENT_TYPE};
use reqwest::Body;
use serde::Serialize;

use crate::error::Result;

pub(crate) trait MakeBody {
    type Body;

    fn make_body(body: &Self::Body, request: &mut reqwest::Request) -> Result<()>;
}

pub(crate) struct EmptyBody;

impl MakeBody for EmptyBody {
    type Body = ();

    fn make_body(_body: &Self::Body, _request: &mut reqwest::Request) -> Result<()> {
        Ok(())
    }
}

pub(crate) struct XMLBody<T>(PhantomData<T>);

impl<T> MakeBody for XMLBody<T>
where
    T: Serialize,
{
    type Body = T;

    fn make_body(body: &Self::Body, request: &mut reqwest::Request) -> Result<()> {
        let body = quick_xml::se::to_string(&body)?;
        let headers = request.headers_mut();
        headers.insert(CONTENT_LENGTH, HeaderValue::from_str(&body.len().to_string())?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/xml"));
        request.body_mut().replace(Body::from(body));
        Ok(())
    }
}

pub(crate) struct BinaryBody;

impl MakeBody for BinaryBody {
    type Body = Bytes;

    fn make_body(body: &Self::Body, request: &mut reqwest::Request) -> Result<()> {
        let len = body.len();
        let headers = request.headers_mut();
        headers.insert(CONTENT_LENGTH, HeaderValue::from_str(&len.to_string())?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/octet-stream"));
        request.body_mut().replace(Body::from(body.clone()));
        Ok(())
    }
}
