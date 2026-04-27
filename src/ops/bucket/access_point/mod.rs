//! Bucket access-point operations.

mod create_access_point;
mod delete_access_point;
mod delete_access_point_policy;
mod get_access_point;
mod get_access_point_policy;
mod list_access_points;
mod put_access_point_policy;

pub use create_access_point::*;
pub use delete_access_point::*;
pub use delete_access_point_policy::*;
pub use get_access_point::*;
pub use get_access_point_policy::*;
pub use list_access_points::*;
pub use put_access_point_policy::*;

pub trait BucketAccessPointOperations:
    CreateAccessPointOps
    + GetAccessPointOps
    + DeleteAccessPointOps
    + ListAccessPointsOps
    + PutAccessPointPolicyOps
    + GetAccessPointPolicyOps
    + DeleteAccessPointPolicyOps
{
}

impl<T> BucketAccessPointOperations for T where
    T: CreateAccessPointOps
        + GetAccessPointOps
        + DeleteAccessPointOps
        + ListAccessPointsOps
        + PutAccessPointPolicyOps
        + GetAccessPointPolicyOps
        + DeleteAccessPointPolicyOps
{
}
