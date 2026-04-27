//! DescribeRegions: query endpoints of all supported OSS regions or a specific
//! region.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/describeregions>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Prepared, Request};

/// Query parameters for [`DescribeRegions`].
///
/// * When `region` is `None`, the request is serialized as `?regions` and OSS
///   returns every supported region.
/// * When `region` is `Some("oss-cn-hangzhou")`, the request is serialized as
///   `?regions=oss-cn-hangzhou` and OSS returns the single region's endpoints.
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRegionsParams {
    #[serde(rename = "regions", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl DescribeRegionsParams {
    /// Query all regions (serializes to `?regions`).
    pub fn all() -> Self {
        Self::default()
    }

    /// Query a specific region (serializes to `?regions=<region>`).
    pub fn region(region: impl Into<String>) -> Self {
        Self {
            region: Some(region.into()),
        }
    }
}

/// Single region entry returned by OSS.
#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct RegionInfo {
    /// OSS region ID, e.g. `oss-cn-hangzhou`.
    pub region: String,
    /// Public (internet) endpoint, e.g. `oss-cn-hangzhou.aliyuncs.com`.
    pub internet_endpoint: String,
    /// Internal (VPC) endpoint, e.g. `oss-cn-hangzhou-internal.aliyuncs.com`.
    pub internal_endpoint: String,
    /// Acceleration endpoint. Always `oss-accelerate.aliyuncs.com` today but
    /// OSS docs do not guarantee this forever.
    pub accelerate_endpoint: String,
}

/// Response body for [`DescribeRegions`].
///
/// Matches the XML root `<RegionInfoList>`.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegionInfoList {
    /// Each `<RegionInfo>` child becomes one entry. `default` yields an empty
    /// `Vec` if the list is missing entirely.
    #[serde(default, rename = "RegionInfo")]
    pub regions: Vec<RegionInfo>,
}

/// The `DescribeRegions` operation.
pub struct DescribeRegions {
    pub params: DescribeRegionsParams,
}

impl Ops for DescribeRegions {
    type Response = BodyResponseProcessor<RegionInfoList>;
    type Body = NoneBody;
    type Query = DescribeRegionsParams;

    const USE_BUCKET: bool = false;

    fn prepare(self) -> Result<Prepared<DescribeRegionsParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(self.params),
            ..Default::default()
        })
    }
}

/// Trait for the `DescribeRegions` operation.
pub trait DescribeRegionsOps {
    /// Query endpoints of all supported OSS regions.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/describeregions>
    fn describe_regions(&self) -> impl Future<Output = Result<RegionInfoList>>;

    /// Query endpoints of a single OSS region.
    fn describe_region(&self, region: impl Into<String>) -> impl Future<Output = Result<RegionInfoList>>;
}

impl DescribeRegionsOps for Client {
    async fn describe_regions(&self) -> Result<RegionInfoList> {
        let ops = DescribeRegions {
            params: DescribeRegionsParams::all(),
        };
        self.request(ops).await
    }

    async fn describe_region(&self, region: impl Into<String>) -> Result<RegionInfoList> {
        let ops = DescribeRegions {
            params: DescribeRegionsParams::region(region),
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describe_regions_params_serialize_all() {
        let q = crate::ser::to_string(&DescribeRegionsParams::all()).unwrap();
        assert_eq!(q, "");
    }

    #[test]
    fn describe_regions_params_serialize_single() {
        let q = crate::ser::to_string(&DescribeRegionsParams::region("oss-cn-hangzhou")).unwrap();
        assert_eq!(q, "regions=oss-cn-hangzhou");
    }

    #[test]
    fn describe_regions_prepared_method() {
        let prepared = DescribeRegions {
            params: DescribeRegionsParams::all(),
        }
        .prepare()
        .unwrap();
        assert_eq!(prepared.method, Method::GET);
        assert!(prepared.key.is_none());
    }

    #[test]
    fn describe_regions_use_bucket_false() {
        const _: () = assert!(!<DescribeRegions as Ops>::USE_BUCKET);
    }

    #[test]
    fn region_info_list_parses_single_region() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<RegionInfoList>
  <RegionInfo>
    <Region>oss-cn-hangzhou</Region>
    <InternetEndpoint>oss-cn-hangzhou.aliyuncs.com</InternetEndpoint>
    <InternalEndpoint>oss-cn-hangzhou-internal.aliyuncs.com</InternalEndpoint>
    <AccelerateEndpoint>oss-accelerate.aliyuncs.com</AccelerateEndpoint>
  </RegionInfo>
</RegionInfoList>"#;
        let parsed: RegionInfoList = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.regions.len(), 1);
        assert_eq!(parsed.regions[0].region, "oss-cn-hangzhou");
        assert_eq!(parsed.regions[0].internet_endpoint, "oss-cn-hangzhou.aliyuncs.com");
    }

    #[test]
    fn region_info_list_parses_multiple_regions() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<RegionInfoList>
  <RegionInfo>
    <Region>oss-cn-hangzhou</Region>
    <InternetEndpoint>oss-cn-hangzhou.aliyuncs.com</InternetEndpoint>
    <InternalEndpoint>oss-cn-hangzhou-internal.aliyuncs.com</InternalEndpoint>
    <AccelerateEndpoint>oss-accelerate.aliyuncs.com</AccelerateEndpoint>
  </RegionInfo>
  <RegionInfo>
    <Region>oss-cn-shanghai</Region>
    <InternetEndpoint>oss-cn-shanghai.aliyuncs.com</InternetEndpoint>
    <InternalEndpoint>oss-cn-shanghai-internal.aliyuncs.com</InternalEndpoint>
    <AccelerateEndpoint>oss-accelerate.aliyuncs.com</AccelerateEndpoint>
  </RegionInfo>
</RegionInfoList>"#;
        let parsed: RegionInfoList = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.regions.len(), 2);
        assert_eq!(parsed.regions[1].region, "oss-cn-shanghai");
    }

    #[test]
    fn region_info_list_parses_empty() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<RegionInfoList></RegionInfoList>"#;
        let parsed: RegionInfoList = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.regions.len(), 0);
    }
}
