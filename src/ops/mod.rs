pub mod bucket;
pub mod object;
pub mod service;

use serde::Deserialize;

#[derive(Default, Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Owner {
    pub id: Option<String>,
    pub display_name: Option<String>,
}
