//! Object FC (Function Compute) Access Point operations.
//!
//! Object FC access points route GetObject requests through a Function
//! Compute function so the response body can be transformed on the fly
//! (image resizing, PII redaction, etc.).

mod create_access_point_for_object_process;
mod delete_access_point_for_object_process;
mod delete_access_point_policy_for_object_process;
mod get_access_point_config_for_object_process;
mod get_access_point_for_object_process;
mod get_access_point_policy_for_object_process;
mod list_access_points_for_object_process;
mod put_access_point_config_for_object_process;
mod put_access_point_policy_for_object_process;

pub use create_access_point_for_object_process::*;
pub use delete_access_point_for_object_process::*;
pub use delete_access_point_policy_for_object_process::*;
pub use get_access_point_config_for_object_process::*;
pub use get_access_point_for_object_process::*;
pub use get_access_point_policy_for_object_process::*;
pub use list_access_points_for_object_process::*;
pub use put_access_point_config_for_object_process::*;
pub use put_access_point_policy_for_object_process::*;

pub trait BucketObjectFcAccessPointOperations:
    CreateAccessPointForObjectProcessOps
    + GetAccessPointForObjectProcessOps
    + DeleteAccessPointForObjectProcessOps
    + ListAccessPointsForObjectProcessOps
    + PutAccessPointConfigForObjectProcessOps
    + GetAccessPointConfigForObjectProcessOps
    + PutAccessPointPolicyForObjectProcessOps
    + GetAccessPointPolicyForObjectProcessOps
    + DeleteAccessPointPolicyForObjectProcessOps
{
}

impl<T> BucketObjectFcAccessPointOperations for T where
    T: CreateAccessPointForObjectProcessOps
        + GetAccessPointForObjectProcessOps
        + DeleteAccessPointForObjectProcessOps
        + ListAccessPointsForObjectProcessOps
        + PutAccessPointConfigForObjectProcessOps
        + GetAccessPointConfigForObjectProcessOps
        + PutAccessPointPolicyForObjectProcessOps
        + GetAccessPointPolicyForObjectProcessOps
        + DeleteAccessPointPolicyForObjectProcessOps
{
}
