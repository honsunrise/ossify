# Ossify — Alibaba Cloud OSS SDK for Rust

[![Rust](https://img.shields.io/badge/rust-1.88%2B-brightgreen.svg)](https://www.rust-lang.org)
[![Crates.io](https://img.shields.io/crates/v/ossify.svg)](https://crates.io/crates/ossify)
[![Documentation](https://docs.rs/ossify/badge.svg)](https://docs.rs/ossify)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/ossify.svg)](LICENSE-APACHE)

A **modern**, **complete**, and **reqwest-powered** Rust SDK for Alibaba
Cloud Object Storage Service (OSS). Ossify aims for parity with the
official Python / Go v2 SDKs while keeping the API ergonomic and
type-safe.

## ✨ Highlights

- **🎯 Full API surface** — **~190 operations** across every category listed
  in the [official OSS operations index](https://www.alibabacloud.com/help/en/oss/developer-reference/list-of-operations-by-function):
  bucket lifecycle, object CRUD, multipart upload, lifecycle, versioning,
  replication, inventory, logging, retention, access points, data indexing,
  DDoS protection, LiveChannel streaming, Resource Pool QoS, Vector Bucket
  (with ANN search), SelectObject SQL, and more.
- **🚀 reqwest-powered** — built on `reqwest` 0.13 with full async/await,
  streaming uploads/downloads, and `rustls` by default.
- **🔐 V4 signing + flexible credentials** — OSS V4 signature for both
  request signing and presigned URLs, with a default credentials chain
  covering explicit AK/SK, environment variables, and RRSA/OIDC for ACK.
- **🛡️ Type-safe** — every API is one `struct` + one sub-trait; responses
  are strongly typed (deserialized from XML or JSON as required).
- **🧱 Modular** — one file per API under `src/ops/{bucket,object,service,
vector_bucket}/<category>/`, with `Operations` aggregate supertraits for
  convenient blanket imports.
- **🎨 Consistent ergonomics** — request parameters use builder methods,
  path/query segments are escaped correctly, and single-byte delimiters or
  base64 SQL expressions are encoded for you.

## 🚀 Quick start

```toml
[dependencies]
ossify = "0.4"
tokio = { version = "1", features = ["full"] }
bytes = "1"
```

### Hello, OSS

```rust,no_run
use bytes::Bytes;
use ossify::Client;
use ossify::ops::object::base::{GetObjectOps, GetObjectParams, PutObjectOps};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .endpoint("https://oss-cn-hangzhou.aliyuncs.com")
        .region("cn-hangzhou")
        .bucket("my-bucket")
        .access_key_id("your-access-key-id")
        .access_key_secret("your-access-key-secret")
        .build()?;

    // Upload
    let put = client
        .put_object("hello.txt", Bytes::from_static(b"Hello, OSS!"), None)
        .await?;
    println!("ETag = {}", put.etag);

    // Download
    let body = client
        .get_object("hello.txt", GetObjectParams::new(), None)
        .await?;
    println!("{}", String::from_utf8_lossy(&body));

    Ok(())
}
```

## 📚 API coverage at a glance

Every entry below maps to one or more ready-to-call methods. Click
through to the module to discover the exact parameter and response types.

### Service-level (`ossify::ops::service`)

| Group                            | APIs                                                                                                                                                                                                            |
| -------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Regions & buckets                | `DescribeRegions`, `ListBuckets`                                                                                                                                                                                |
| User-level Anti-DDoS             | `InitUserAntiDDosInfo`, `UpdateUserAntiDDosInfo`, `GetUserAntiDDosInfo`                                                                                                                                         |
| User-level PublicAccessBlock     | `PutPublicAccessBlock`, `GetPublicAccessBlock`, `DeletePublicAccessBlock`                                                                                                                                       |
| Data redundancy                  | `ListUserDataRedundancyTransition`                                                                                                                                                                              |
| Resource Pool (`resource_pool/`) | `ListResourcePools`, `GetResourcePoolInfo`, `ListResourcePoolBuckets`, Put/Get/List/Delete `ResourcePoolRequesterQoSInfo`, `ListResourcePoolBucketGroups`, Put/Get/List/Delete `ResourcePoolBucketGroupQoSInfo` |

### Bucket-level (`ossify::ops::bucket`)

| Category                                          | APIs                                                                                                                                                |
| ------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| `base/`                                           | `PutBucket`, `GetBucketInfo`, `GetBucketLocation`, `GetBucketStat`, `DeleteBucket`, `ListObjects` (v2), `ListObjectsV1`, `ListObjectVersions`       |
| `acl/`                                            | `PutBucketACL`, `GetBucketACL`                                                                                                                      |
| `access_monitor/`                                 | Put/Get                                                                                                                                             |
| `access_point/`                                   | CreateAccessPoint, GetAccessPoint, DeleteAccessPoint, ListAccessPoints, Put/Get/Delete AccessPointPolicy                                            |
| `object_fc_access_point/`                         | Create/Get/Delete/ListAccessPointForObjectProcess, Put/Get/DeleteAccessPointConfigForObjectProcess, Put/Get/DeleteAccessPointPolicyForObjectProcess |
| `anti_ddos/`                                      | Init/Update/Get BucketAntiDDosInfo                                                                                                                  |
| `archive_direct_read/`                            | Put/Get                                                                                                                                             |
| `cname/`                                          | Create/Delete/List CnameToken, Put/Get BucketCnameToken                                                                                             |
| `cors/`                                           | Put/Get/Delete + GetBucketOptions (preflight)                                                                                                       |
| `data_accelerator/`                               | Put/Get/Delete                                                                                                                                      |
| `data_indexing/`                                  | Open/Get/Close/DoMetaQuery                                                                                                                          |
| `encryption/`                                     | Put/Get/Delete                                                                                                                                      |
| `https_config/`                                   | Put/Get                                                                                                                                             |
| `inventory/`                                      | Put/Get/List/Delete                                                                                                                                 |
| `lifecycle/`                                      | Put/Get/Delete                                                                                                                                      |
| `live_channel/`                                   | Put/Put-status/Get-info/Get-stat/Get-history/List/Delete LiveChannel, Post/Get VodPlaylist                                                          |
| `logging/`                                        | Put/Get/Delete + CNAME variants                                                                                                                     |
| `policy/`                                         | Put/Get/Delete/GetPolicyStatus                                                                                                                      |
| `public_access_block/`                            | Put/Get/Delete (+ access-point variants)                                                                                                            |
| `qos/` + `requester_qos/`                         | Bucket-total QoS and per-requester QoS (Put/Get/List/Delete)                                                                                        |
| `redundancy_transition/`                          | Create/Get/Delete/List BucketDataRedundancyTransition                                                                                               |
| `referer/`                                        | Put/Get                                                                                                                                             |
| `replication/`                                    | Put/Get/Delete, ListReplicationLocation, ListReplicationRules, GetBucketReplicationProgress                                                         |
| `request_payment/`                                | Put/Get                                                                                                                                             |
| `resource_group/` + `resource_pool_bucket_group/` | Put/Get resource group; Put bucket→pool-group mapping                                                                                               |
| `retention/`                                      | InitiateBucketWorm, CompleteBucketWorm, AbortBucketWorm, ExtendBucketWormWorm, GetBucketWorm                                                        |
| `style/`                                          | Put/Get/List/Delete                                                                                                                                 |
| `tagging/`                                        | Put/Get/Delete                                                                                                                                      |
| `transfer_acceleration/`                          | Put/Get                                                                                                                                             |
| `versioning/`                                     | Put/Get + RestoreObject (bucket-scope)                                                                                                              |
| `website/`                                        | Put/Get/Delete                                                                                                                                      |

### Object-level (`ossify::ops::object`)

| Category            | APIs                                                                                                                                                                                                                                                        |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `base/`             | PutObject (body / stream), GetObject (with `Range`, presigned URL), HeadObject, GetObjectMeta, CopyObject, DeleteObject, DeleteMultipleObjects, AppendObject, SealAppendObject, RestoreObject, CleanRestoredObject, PostObject (helpers) + Callback helpers |
| `acl/`              | PutObjectACL, GetObjectACL                                                                                                                                                                                                                                  |
| `folder/` (HNS)     | CreateDirectory, Rename, DeleteDirectory                                                                                                                                                                                                                    |
| `multipart_upload/` | InitiateMultipartUpload, UploadPart, UploadPartCopy, ListMultipartUploads, ListParts, CompleteMultipartUpload, AbortMultipartUpload                                                                                                                         |
| `select/`           | SelectObject (CSV & JSON, SQL → frame stream), CreateSelectCsvObjectMeta, CreateSelectJsonObjectMeta                                                                                                                                                        |
| `symbolic_link/`    | PutSymlink, GetSymlink                                                                                                                                                                                                                                      |
| `tagging/`          | Put/Get/Delete                                                                                                                                                                                                                                              |

### Vector Bucket (`ossify::ops::vector_bucket`, JSON on `oss-vectors.aliyuncs.com`)

| Category   | APIs                                                                                                    |
| ---------- | ------------------------------------------------------------------------------------------------------- |
| `bucket/`  | PutVectorBucket, GetVectorBucket, ListVectorBuckets, DeleteVectorBucket                                 |
| `index/`   | PutVectorIndex, GetVectorIndex, ListVectorIndexes, DeleteVectorIndex                                    |
| `vectors/` | PutVectors, GetVectors, ListVectors, DeleteVectors, QueryVectors (top-K ANN with MongoDB-style filters) |

## 🧭 Usage guide by feature

### Buckets and objects

```rust,no_run
use bytes::Bytes;
use ossify::Client;
use ossify::ops::bucket::base::{
    DeleteBucketOps, ListObjectsOps, PutBucketConfiguration, PutBucketOps,
};
use ossify::ops::common::{DataRedundancyType, StorageClass};
use ossify::ops::object::base::{
    DeleteObjectOps, GetObjectOps, GetObjectOptions, GetObjectParams, HeadObjectOps,
    PutObjectOps, PutObjectOptions,
};

# async fn demo(client: &Client) -> Result<(), ossify::Error> {
// Create a bucket (LRS Standard)
let config = PutBucketConfiguration {
    storage_class: Some(StorageClass::Standard),
    data_redundancy_type: Some(DataRedundancyType::LocallyRedundantStorage),
};
client.put_bucket(config, None).await?;

// Upload with options
let options = PutObjectOptions::new()
    .content_type("text/plain")
    .storage_class(StorageClass::InfrequentAccess);
client
    .put_object("file.txt", Bytes::from_static(b"hello"), Some(options))
    .await?;

// Range download
let range_opts = GetObjectOptions::new().range("bytes=0-1023");
let chunk = client
    .get_object("file.txt", GetObjectParams::new(), Some(range_opts))
    .await?;

// Metadata
let meta = client.head_object("file.txt", None).await?;
println!("Content-Length: {:?}", meta.content_length);

// List (v2)
let page = client.list_objects(None).await?;
println!("Found {} objects", page.contents.len());

// Clean up
client.delete_object("file.txt", None).await?;
client.delete_bucket().await?;
# Ok(()) }
```

### Multipart upload

```rust,no_run
use bytes::Bytes;
use ossify::Client;
use ossify::ops::object::multipart_upload::{
    AbortMultipartUploadOperations, CompleteMultipartUploadOperations,
    InitiateMultipartUploadOperations, Part, UploadPartOperations,
};

# async fn multipart(client: &Client, chunk1: Bytes, chunk2: Bytes)
#   -> Result<(), ossify::Error> {
let init = client.initiate_multipart_upload("big.bin", None).await?;
let upload_id = init.upload_id;

let p1 = client.upload_part("big.bin", &upload_id, 1, chunk1).await?;
let p2 = client.upload_part("big.bin", &upload_id, 2, chunk2).await?;

let parts = vec![Part::new(1, p1.etag), Part::new(2, p2.etag)];
client
    .complete_multipart_upload("big.bin", &upload_id, parts, None)
    .await?;
# Ok(()) }
```

### Vector Bucket (ANN search)

Vector bucket APIs live on a dedicated `{region}.oss-vectors.aliyuncs.com`
endpoint family. Point a separate `Client` at that endpoint to use them:

```rust,no_run
use ossify::Client;
use ossify::ops::common::{VectorData, VectorDistanceMetric};
use ossify::ops::vector_bucket::{
    PutVectorBucketOps, PutVectorIndexOps, PutVectorIndexRequest, PutVectorsOps,
    PutVectorsRequest, QueryVectorsOps, QueryVectorsRequest, VectorFilter,
};
use ossify::ops::common::Vector;

# async fn vectors(client: &Client) -> Result<(), ossify::Error> {
client.put_vector_bucket().await?;

client
    .put_vector_index(PutVectorIndexRequest::new(
        "docs",
        1024,
        VectorDistanceMetric::Cosine,
    ))
    .await?;

client
    .put_vectors(PutVectorsRequest {
        index_name: "docs".into(),
        vectors: vec![Vector {
            key: "doc-1".into(),
            data: Some(VectorData::new(vec![0.1; 1024])),
            metadata: None,
            distance: None,
        }],
    })
    .await?;

let hits = client
    .query_vectors(
        QueryVectorsRequest::new("docs", VectorData::new(vec![0.1; 1024]), 5)
            .filter(VectorFilter::eq("lang", "en"))
            .return_distance(true),
    )
    .await?;
for hit in hits.vectors {
    println!("{} -> distance={:?}", hit.key, hit.distance);
}
# Ok(()) }
```

### SelectObject (SQL over CSV / JSON)

`SelectObject` returns a stream of frames — data, heartbeats, and a
terminal status frame that must be inspected for errors:

```rust,no_run
use futures::StreamExt;
use ossify::Client;
use ossify::ops::object::select::{
    CsvInputSerialization, CsvOutputSerialization, SelectFrame, SelectObjectOps, SelectRequest,
};

# async fn run_query(client: &Client) -> Result<(), ossify::Error> {
let req = SelectRequest::new_csv(
    "select _1, _2 from ossobject where _3 > 100",
    CsvInputSerialization::default(),
    CsvOutputSerialization::default(),
)
.with_payload_crc(true);

let mut frames = client.select_object_csv("data.csv", req).await?;

while let Some(frame) = frames.next().await {
    match frame? {
        SelectFrame::Data { data, .. } => print!("{}", String::from_utf8_lossy(&data)),
        SelectFrame::End { status, error_message, .. } => {
            if status != 200 && status != 206 {
                eprintln!("select failed: {status} {error_message}");
            }
            break;
        }
        _ => {}
    }
}
# Ok(()) }
```

## 🔧 Configuration

### Client builder

```rust,no_run
use std::time::Duration;

use ossify::{Client, UrlStyle};

# fn demo() -> Result<(), ossify::Error> {
let client = Client::builder()
    .endpoint("https://oss-cn-hangzhou-internal.aliyuncs.com")
    .public_endpoint("https://oss-cn-hangzhou.aliyuncs.com")
    .region("cn-hangzhou")
    .bucket("my-bucket")
    .access_key_id("your-access-key-id")
    .access_key_secret("your-access-key-secret")
    .security_token("your-sts-token")           // optional
    .http_timeout(Duration::from_secs(30))
    .url_style(UrlStyle::VirtualHosted)         // VirtualHosted | Path | CName
    .build()?;
# Ok(()) }
```

URL styles:

- **VirtualHosted** (default): `https://bucket.oss-cn-hangzhou.aliyuncs.com/object`
- **Path**: `https://oss-cn-hangzhou.aliyuncs.com/bucket/object`
- **CName**: `https://custom-domain.com/object`

### Credentials

Explicit AK/SK, STS token, RRSA (ACK OIDC), or the default chain — all
via the same `Client::builder()`:

```rust,no_run
use ossify::Client;
use ossify::credentials::RrsaCredentialsProvider;

# fn demo() -> Result<(), ossify::Error> {
// ACK RRSA pod — reads ALIBABA_CLOUD_ROLE_ARN /
// ALIBABA_CLOUD_OIDC_PROVIDER_ARN / ALIBABA_CLOUD_OIDC_TOKEN_FILE.
let http = reqwest::Client::new();
let rrsa = RrsaCredentialsProvider::from_env(http).expect("RRSA env set");

let client = Client::builder()
    .endpoint("https://oss-cn-hangzhou-internal.aliyuncs.com")
    .region("cn-hangzhou")
    .bucket("my-bucket")
    .credentials_provider(rrsa)
    .build()?;
# Ok(()) }
```

If neither `access_key_id` / `access_key_secret` nor an explicit
`credentials_provider` is supplied, the builder falls back to
`DefaultCredentialsChain`, which walks:

1. `EnvironmentCredentialsProvider` — reads
   `ALIBABA_CLOUD_ACCESS_KEY_ID` / `ALIBABA_CLOUD_ACCESS_KEY_SECRET`
   (+ optional `ALIBABA_CLOUD_SECURITY_TOKEN`) or the `OSS_*` equivalents.
2. `RrsaCredentialsProvider::from_env` — RRSA/OIDC if its env vars exist.

## 🌟 Advanced features

### Presigned URLs

```rust,no_run
use ossify::{Client, QueryAuthOptions};
use ossify::ops::object::base::{GetObjectOps, GetObjectParams};

# async fn presign(client: &Client) -> Result<(), ossify::Error> {
let opts = QueryAuthOptions::builder().expires_in(3600).build()?;
let url = client
    .presign_get_object(
        "private-file.jpg",
        true, // public endpoint
        GetObjectParams::new(),
        None,
        opts,
    )
    .await?;
println!("presigned = {url}");
# Ok(()) }
```

### Streaming uploads

`put_object_stream` / `upload_part_stream` accept any
`futures::TryStream<Ok = bytes::Bytes>` so large files and back-pressured
producers (e.g. channels, `reqwest::Response::bytes_stream()`) can be
forwarded without buffering into memory.

### Error handling

```rust,no_run
use ossify::{Client, Error};
use ossify::ops::object::base::{GetObjectOps, GetObjectParams};

# async fn handle(client: &Client) {
match client
    .get_object("missing.txt", GetObjectParams::new(), None)
    .await
{
    Ok(bytes) => println!("{} bytes", bytes.len()),
    Err(Error::ApiError { status_code, message }) if status_code.as_u16() == 404 => {
        println!("not found: {message:?}");
    }
    Err(e) => eprintln!("other error: {e}"),
}
# }
```

## 🏗️ Architecture

- **One API = one file**: every OSS operation lives in its own
  `src/ops/.../<operation>.rs` exposing a `struct`, optional
  `Params`/`Options`, a `<Name>Ops` sub-trait, and a blanket `impl for Client`. Category `mod.rs` files aggregate sub-traits into
  `Operations` supertraits for convenient mass-imports.
- **Shared XML / JSON types** live in `src/ops/common/`.
- **Uniform request pipeline** (`src/lib.rs`): every `Ops` declares a
  `Body` (`NoneBody` | `ZeroBody` | `XMLBody<T>` | `JSONBody<T>` |
  `BytesBody` | `StreamBody<S>`), a `Response` processor
  (`EmptyResponseProcessor` | `BodyResponseProcessor<T>` |
  `HeaderResponseProcessor<T>` | `BinaryResponseProcessor` |
  `StreamResponseProcessor`), and a `Query` type that serialises to an
  alphabetically-sorted query string.
- **Signing**: OSS V4 (`OSS4-HMAC-SHA256`) used for both request
  `Authorization` headers and presigned URLs.
- **Single-endpoint client**: one `Client` instance targets one endpoint
  family. Vector Bucket uses a dedicated `oss-vectors` endpoint, so keep
  a separate `Client` for vector operations.

### Core dependencies

- [`reqwest`](https://docs.rs/reqwest) — HTTP with `rustls-no-provider` and streaming.
- [`tokio`](https://docs.rs/tokio) / [`futures`](https://docs.rs/futures) — async runtime and stream primitives.
- [`serde`](https://docs.rs/serde) + [`serde_json`](https://docs.rs/serde_json)
- [`quick-xml`](https://docs.rs/quick-xml) — request/response encoding.
- [`jiff`](https://docs.rs/jiff) — timestamps and durations.
- [`bytes`](https://docs.rs/bytes) — zero-copy byte buffers.
- [`hmac`](https://docs.rs/hmac) + [`sha2`](https://docs.rs/sha2) +
- [`sha1`](https://docs.rs/sha1) + [`crc32fast`](https://docs.rs/crc32fast) — signing and SelectObject payload CRC.

## 📖 Documentation

- [API docs on docs.rs](https://docs.rs/ossify)
- [OSS operations index](https://www.alibabacloud.com/help/en/oss/developer-reference/list-of-operations-by-function)

## 🤝 Contributing

PRs are very welcome — please open an issue first for large changes.
Every API file should include unit tests for (a) query-parameter
serialisation and (b) XML/JSON round-tripping; `cargo test --lib` +
`cargo clippy --lib --tests` must both be clean.

## 📄 License

Licensed under either of

- Apache License 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

---

<p align="center"><strong>Happy coding with OSS and Rust! 🦀✨</strong></p>
