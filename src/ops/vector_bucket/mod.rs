//! Vector Bucket operations (JSON-based, dedicated `oss-vectors` endpoint).
//!
//! All APIs in this tree target a completely separate endpoint family
//! (`{region}.oss-vectors.aliyuncs.com`), use `application/json` bodies with
//! camelCase field names (except the bucket-info responses, which preserve
//! the PascalCase scheme from legacy OSS XML for compatibility), and share
//! nothing with regular OSS APIs besides the V4 signing algorithm.
//!
//! To call these APIs, construct a `Client` whose endpoint points at the
//! vector service (for example `https://cn-hangzhou.oss-vectors.aliyuncs.com`
//! or the bucket-scoped variant for bucket-level calls).
//!
//! Official category index:
//! <https://www.alibabacloud.com/help/en/oss/developer-reference/apis-for-operations-on-vector-buckets/>

pub mod bucket;
pub mod index;

pub use bucket::*;
pub use index::*;

/// Aggregate supertrait for vector-bucket operations.
///
/// Batches 22a–22b cover bucket-lifecycle and index operations; vector
/// data-plane operations (Put/Get/List/Delete/Query vectors) will be added
/// in batch 22c.
pub trait VectorBucketOperations:
    PutVectorBucketOps
    + GetVectorBucketOps
    + ListVectorBucketsOps
    + DeleteVectorBucketOps
    + PutVectorIndexOps
    + GetVectorIndexOps
    + ListVectorIndexesOps
    + DeleteVectorIndexOps
{
}
impl<T> VectorBucketOperations for T where
    T: PutVectorBucketOps
        + GetVectorBucketOps
        + ListVectorBucketsOps
        + DeleteVectorBucketOps
        + PutVectorIndexOps
        + GetVectorIndexOps
        + ListVectorIndexesOps
        + DeleteVectorIndexOps
{
}
