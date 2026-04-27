//! GetBucketDataAccelerator.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketdataaccelerator>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

pub use super::put_bucket_data_accelerator::AcceleratePaths;
use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketDataAcceleratorParams {
    #[serde(rename = "dataAccelerator")]
    data_accelerator: OnlyKeyField,
    #[serde(
        rename = "x-oss-datalake-cache-available-zone",
        skip_serializing_if = "Option::is_none"
    )]
    pub available_zone: Option<String>,
    /// Include the `?verbose` sub-resource to request full path details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) verbose: Option<OnlyKeyField>,
}

impl GetBucketDataAcceleratorParams {
    pub fn new(available_zone: Option<String>, verbose: bool) -> Self {
        Self {
            data_accelerator: OnlyKeyField,
            available_zone,
            verbose: if verbose { Some(OnlyKeyField) } else { None },
        }
    }
}

/// Basic information about a configured OSS accelerator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "BasicInfomation", rename_all = "PascalCase")]
pub struct DataAcceleratorBasicInformation {
    pub quota: u64,
    pub available_zone: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accelerate_paths: Option<AcceleratePaths>,
    pub creation_date: u64,
    pub quota_frozen_until: u64,
}

/// Response body: `<DataAccelerator>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "DataAccelerator", rename_all = "PascalCase")]
pub struct DataAccelerator {
    pub name: String,
    pub bucket_name: String,
    #[serde(rename = "BasicInfomation")]
    pub basic_information: DataAcceleratorBasicInformation,
}

pub struct GetBucketDataAccelerator {
    pub available_zone: Option<String>,
    pub verbose: bool,
}

impl Ops for GetBucketDataAccelerator {
    type Response = BodyResponseProcessor<DataAccelerator>;
    type Body = NoneBody;
    type Query = GetBucketDataAcceleratorParams;

    fn prepare(self) -> Result<Prepared<GetBucketDataAcceleratorParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketDataAcceleratorParams::new(self.available_zone, self.verbose)),
            ..Default::default()
        })
    }
}

pub trait GetBucketDataAcceleratorOps {
    /// Query the OSS accelerator configuration of a bucket.
    ///
    /// * `available_zone` narrows to a specific zone (`cn-wulanchabu-b`,
    ///   `cn-beijing-h`, …); omit to return every zone.
    /// * `verbose = true` requests full acceleration-path details.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketdataaccelerator>
    fn get_bucket_data_accelerator(
        &self,
        available_zone: Option<String>,
        verbose: bool,
    ) -> impl Future<Output = Result<DataAccelerator>>;
}

impl GetBucketDataAcceleratorOps for Client {
    async fn get_bucket_data_accelerator(
        &self,
        available_zone: Option<String>,
        verbose: bool,
    ) -> Result<DataAccelerator> {
        self.request(GetBucketDataAccelerator {
            available_zone,
            verbose,
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize_default() {
        assert_eq!(
            crate::ser::to_string(&GetBucketDataAcceleratorParams::default()).unwrap(),
            "dataAccelerator"
        );
    }

    #[test]
    fn params_serialize_full() {
        let q = crate::ser::to_string(&GetBucketDataAcceleratorParams::new(
            Some("cn-wulanchabu-b".to_string()),
            true,
        ))
        .unwrap();
        assert_eq!(q, "dataAccelerator&verbose&x-oss-datalake-cache-available-zone=cn-wulanchabu-b");
    }

    #[test]
    fn response_round_trip() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<DataAccelerator>
  <Name>mybucket_data-acc</Name>
  <BucketName>mybucket</BucketName>
  <BasicInfomation>
    <Quota>200</Quota>
    <AvailableZone>cn-wulanchabu-b</AvailableZone>
    <AcceleratePaths>
      <DefaultCachePolicy>write-back</DefaultCachePolicy>
      <Path>
        <Name>AccelerationPath</Name>
        <CachePolicy>sync-warmup</CachePolicy>
      </Path>
    </AcceleratePaths>
    <CreationDate>1751013420658</CreationDate>
    <QuotaFrozenUntil>1751017020624</QuotaFrozenUntil>
  </BasicInfomation>
</DataAccelerator>"#;
        let parsed: DataAccelerator = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.name, "mybucket_data-acc");
        assert_eq!(parsed.bucket_name, "mybucket");
        assert_eq!(parsed.basic_information.quota, 200);
        assert_eq!(parsed.basic_information.creation_date, 1751013420658);
        let ap = parsed.basic_information.accelerate_paths.as_ref().unwrap();
        assert_eq!(ap.paths.len(), 1);
        assert_eq!(ap.paths[0].name, "AccelerationPath");
    }
}
