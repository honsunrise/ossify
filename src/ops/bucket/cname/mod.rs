//! Bucket CNAME (custom domain mapping) operations.

mod create_cname_token;
mod delete_cname;
mod get_cname_token;
mod list_cname;
mod put_cname;

pub use create_cname_token::*;
pub use delete_cname::*;
pub use get_cname_token::*;
pub use list_cname::*;
pub use put_cname::*;

pub trait BucketCnameOperations:
    CreateCnameTokenOps + GetCnameTokenOps + PutCnameOps + ListCnameOps + DeleteCnameOps
{
}
impl<T> BucketCnameOperations for T where
    T: CreateCnameTokenOps + GetCnameTokenOps + PutCnameOps + ListCnameOps + DeleteCnameOps
{
}
