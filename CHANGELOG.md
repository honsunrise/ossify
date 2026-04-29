# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] – 2026-04-28

### Overview

`v0.4.0` is a **full-surface** release: ossify now covers every API
listed in the [official OSS operations index](https://www.alibabacloud.com/help/en/oss/developer-reference/list-of-operations-by-function)
plus the dedicated Vector Bucket endpoint family. The release adds ~170
new operations across a dozen categories, generalises the transport
plumbing to support JSON bodies and binary frame streams, and includes
several infrastructure improvements.

**Breaking changes**: this is a pre-1.0 release; several signatures have
been refined (see below) compared to `v0.3.x`.

### Added – categories & APIs

#### Service (account-level, `USE_BUCKET=false`)
- `DescribeRegions`
- `ListUserDataRedundancyTransition`
- `InitUserAntiDDosInfo`, `UpdateUserAntiDDosInfo`, `GetUserAntiDDosInfo`
- `PutPublicAccessBlock`, `GetPublicAccessBlock`, `DeletePublicAccessBlock`
- Resource Pool basics + per-requester QoS + bucket-group QoS
  (12 operations under `ops::service::resource_pool`)

#### Bucket
- `access_monitor/` — Put/Get
- `access_point/` — Create/Get/Delete/ListAccessPoints, Put/Get/Delete
  AccessPointPolicy
- `object_fc_access_point/` — Object-FC access point (9 operations)
- `anti_ddos/` — Init/Update/Get
- `archive_direct_read/` — Put/Get
- `cname/` — Create/Delete/List CnameToken, Put/Get BucketCnameToken
- `cors/` — Put/Get/Delete + GetBucketOptions (preflight)
- `data_accelerator/` — Put/Get/Delete
- `data_indexing/` — Open/Get/Close + DoMetaQuery
- `encryption/` — Put/Get/Delete
- `https_config/` — Put/Get
- `inventory/` — Put/Get/List/Delete
- `lifecycle/` — Put/Get/Delete
- `live_channel/` — 9 operations (RTMP / HLS / VOD playlist)
- `logging/` — Put/Get/Delete + CNAME variants
- `policy/` — Put/Get/Delete/GetPolicyStatus
- `public_access_block/` — Put/Get/Delete (+ access-point variants)
- `qos/` + `requester_qos/` — 7 operations
- `redundancy_transition/` — Create/Get/Delete/List
- `referer/` — Put/Get
- `replication/` — 6 operations
- `request_payment/` — Put/Get
- `resource_group/`, `resource_pool_bucket_group/` — 3 operations
- `retention/` — 5 WORM operations
- `style/` — Put/Get/List/Delete
- `tagging/` — Put/Get/Delete
- `transfer_acceleration/` — Put/Get
- `versioning/` — Put/Get + RestoreObject
- `website/` — Put/Get/Delete

#### Object
- `base/` — AppendObject, SealAppendObject, GetObjectMeta,
  DeleteMultipleObjects, RestoreObject, CleanRestoredObject,
  PostObject (V1 + V4 signing helpers) and Callback helpers
- `acl/` — Put/Get
- `folder/` (HNS) — CreateDirectory, Rename, DeleteDirectory
- `multipart_upload/` — ListMultipartUploads, ListParts, UploadPartCopy
- `select/` — **SelectObject** and **CreateSelectObjectMeta** with a
  full binary frame-stream decoder (`SelectFrame`, `SelectFrameStream`)
  supporting CSV + JSON inputs, optional payload CRC-32, and typed
  request/response models
- `symbolic_link/` — Put/Get
- `tagging/` — Put/Get/Delete

#### Vector Bucket (`ops::vector_bucket`, dedicated `oss-vectors` endpoint)
- `bucket/` — Put/Get/List/Delete VectorBucket (13 JSON APIs total)
- `index/` — Put/Get/List/Delete VectorIndex
- `vectors/` — Put/Get/List/Delete/Query Vectors with a typed
  MongoDB-style filter DSL (`VectorFilter::{eq,ne,is_in,not_in,exists,
  and,or,Raw}`) and `top_k` ANN search

### Added – infrastructure
- `body::JSONBody<T>` — JSON request body encoder (for Vector Bucket).
- `response::StreamResponseProcessor` — returns the raw
  `reqwest::Response` so callers can consume streamed / framed bodies
  (used by SelectObject and CreateSelectObjectMeta).
- `ops::common::qos::{QoSConfiguration, RequesterQoSInfo}`,
  `ops::common::resource_pool::*`, `ops::common::live_channel::*`,
  `ops::common::vector::*`, `ops::common::acl::ObjectAcl` and many more
  shared XML/JSON types.
- `SelectFrame` wire-format decoder with IEEE CRC-32 payload
  verification (via `crc32fast`).

### Added – credentials
- `RrsaCredentialsProvider` — OIDC-token → STS credentials exchange
  with automatic refresh, for ACK (Alibaba Cloud Kubernetes) pods.
- `DefaultCredentialsChain` — walks environment credentials and RRSA
  when neither explicit AK/SK nor a provider is supplied.

### Changed
- `Error::ApiError { status_code, message }` replaces the older
  opaque HTTP-error variant; consumers can now pattern-match on OSS
  response codes directly.
- README re-written to cover every category and every advanced
  feature; code snippets updated to compile against the current API.

### Fixed
- Query-string serializer: a sequence of `(key, value)` tuples (for
  example `Vec<(&str, &str)>`) now serialises to
  `"key1=value1&key2=value2"` instead of concatenating the raw
  values. Two bugs in `src/ser/` were addressed:
  1. `PairSerializer` dropped the key when it got the value — it
     wrote only `value` instead of `key=value`.
  2. `SeqSerializer` / `TupleSerializer` never emitted `&` between
     elements and never skipped empty `(key, None)` pairs.
  The serializer now writes each pair into a scratch buffer, skips
  entries whose value is `None` / unit, and inserts `&` between
  non-empty pairs. Unblocks the previously long-standing
  `ser::tests::test_serialize_tags` failure.

### Dependencies
- Added `crc32fast = "1.5"`.
- Kept in sync with `reqwest = "0.13"` (`rustls-no-provider`),
  `quick-xml = "0.39"`, `serde_with = "3.14"`, `jiff = "0.2"`.

### Test & quality
- Lib test count grew from the `v0.3.x` baseline to **530 passing
  tests** (all green — the `ser::tests::test_serialize_tags` failure
  inherited from earlier releases was fixed as part of this cycle).
- `cargo clippy --lib --tests` is clean.

## [0.3.x] and earlier

Earlier versions bootstrapped the signing pipeline, covered
`PutBucket` / `GetBucketInfo` / `ListObjects`, basic object CRUD and
multipart uploads, presigned URLs, and the core credential providers.
See the git history for details.

[0.4.0]: https://github.com/honsunrise/ossify/releases/tag/v0.4.0
