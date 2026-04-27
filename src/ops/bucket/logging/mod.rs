//! Bucket access-log operations.

mod delete_bucket_logging;
mod delete_user_defined_log_fields_config;
mod get_bucket_logging;
mod get_user_defined_log_fields_config;
mod put_bucket_logging;
mod put_user_defined_log_fields_config;

pub use delete_bucket_logging::*;
pub use delete_user_defined_log_fields_config::*;
pub use get_bucket_logging::*;
pub use get_user_defined_log_fields_config::*;
pub use put_bucket_logging::*;
pub use put_user_defined_log_fields_config::*;

pub trait BucketLoggingOperations:
    PutBucketLoggingOps
    + GetBucketLoggingOps
    + DeleteBucketLoggingOps
    + PutUserDefinedLogFieldsConfigOps
    + GetUserDefinedLogFieldsConfigOps
    + DeleteUserDefinedLogFieldsConfigOps
{
}
impl<T> BucketLoggingOperations for T where
    T: PutBucketLoggingOps
        + GetBucketLoggingOps
        + DeleteBucketLoggingOps
        + PutUserDefinedLogFieldsConfigOps
        + GetUserDefinedLogFieldsConfigOps
        + DeleteUserDefinedLogFieldsConfigOps
{
}
