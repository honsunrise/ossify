use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct QueryAuthOptions {
    pub x_oss_expires: u32,
    pub response_content_type: Option<String>,
    pub response_content_language: Option<String>,
    pub response_content_disposition: Option<String>,
    pub response_content_encoding: Option<String>,
    pub version_id: Option<String>,
    pub x_oss_process: Option<String>,
    #[serde(flatten)]
    pub additional_parameters: HashMap<String, String>,
}

impl QueryAuthOptions {
    pub fn builder() -> QueryAuthOptionsBuilder {
        QueryAuthOptionsBuilder::default()
    }
}

/// Builder for `PresignOptions`
#[derive(Debug, Default)]
pub struct QueryAuthOptionsBuilder {
    x_oss_expires: u32,
    response_content_type: Option<String>,
    response_content_language: Option<String>,
    response_content_disposition: Option<String>,
    response_content_encoding: Option<String>,
    version_id: Option<String>,
    x_oss_process: Option<String>,
    additional_parameters: HashMap<String, String>,
}

impl QueryAuthOptionsBuilder {
    pub fn new(x_oss_expires: u32) -> Self {
        Self {
            x_oss_expires,
            ..Default::default()
        }
    }

    pub fn x_oss_expires(mut self, x_oss_expires: u32) -> Self {
        self.x_oss_expires = x_oss_expires;
        self
    }

    pub fn response_content_type(mut self, content_type: impl Into<String>) -> Self {
        self.response_content_type = Some(content_type.into());
        self
    }

    pub fn response_content_language(mut self, language: impl Into<String>) -> Self {
        self.response_content_language = Some(language.into());
        self
    }

    pub fn response_content_disposition(mut self, disposition: impl Into<String>) -> Self {
        self.response_content_disposition = Some(disposition.into());
        self
    }

    pub fn response_content_encoding(mut self, encoding: impl Into<String>) -> Self {
        self.response_content_encoding = Some(encoding.into());
        self
    }

    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }

    pub fn x_oss_process(mut self, x_oss_process: impl Into<String>) -> Self {
        self.x_oss_process = Some(x_oss_process.into());
        self
    }

    pub fn additional_parameter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.additional_parameters.insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> QueryAuthOptions {
        QueryAuthOptions {
            x_oss_expires: self.x_oss_expires,
            response_content_type: self.response_content_type,
            response_content_language: self.response_content_language,
            response_content_disposition: self.response_content_disposition,
            response_content_encoding: self.response_content_encoding,
            version_id: self.version_id,
            x_oss_process: self.x_oss_process,
            additional_parameters: self.additional_parameters,
        }
    }
}
