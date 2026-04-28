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
pub mod vectors;

pub use bucket::*;
pub use index::*;
pub use vectors::*;

/// Aggregate supertrait for all 13 vector-bucket APIs.
pub trait VectorBucketOperations:
    PutVectorBucketOps
    + GetVectorBucketOps
    + ListVectorBucketsOps
    + DeleteVectorBucketOps
    + PutVectorIndexOps
    + GetVectorIndexOps
    + ListVectorIndexesOps
    + DeleteVectorIndexOps
    + PutVectorsOps
    + GetVectorsOps
    + ListVectorsOps
    + DeleteVectorsOps
    + QueryVectorsOps
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
        + PutVectorsOps
        + GetVectorsOps
        + ListVectorsOps
        + DeleteVectorsOps
        + QueryVectorsOps
{
}
