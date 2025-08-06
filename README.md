# Ossify - Alibaba Cloud OSS SDK for Rust

[![Rust](https://img.shields.io/badge/rust-nightly-brightgreen.svg)](https://www.rust-lang.org)
[![Crates.io](https://img.shields.io/crates/v/ossify.svg)](https://crates.io/crates/ossify)
[![Documentation](https://docs.rs/ossify/badge.svg)](https://docs.rs/ossify)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/ossify.svg)](LICENSE-APACHE)

A **modern**, **easy-to-use**, and **reqwest-powered** Rust SDK for Alibaba Cloud Object Storage Service (OSS). Built with developer experience in mind, this SDK provides a clean, intuitive API that makes working with OSS straightforward and enjoyable.

## ✨ Key Features

- **🚀 Reqwest-Powered**: Built on top of the battle-tested `reqwest` HTTP client with full async/await support
- **🎯 Developer-Friendly**: Clean, intuitive API designed for ease of use and developer productivity
- **🔐 Secure by Default**: Full support for OSS authentication, including temporary credentials and STS tokens
- **📦 Complete Coverage**: Comprehensive support for bucket operations, object management, and multipart uploads
- **🛡️ Type-Safe**: Leverages Rust's type system to prevent common errors at compile time
- **⚡ Performance**: Optimized for speed with streaming support and efficient memory usage
- **🔧 Flexible**: Multiple URL styles (Virtual Hosted, Path-style, CNAME) and customizable configurations

## 🚀 Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
ossify = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

### Basic Example

```rust

use ossify::Client;
use ossify::ops::bucket::BucketOperations;
use ossify::ops::object::base::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = Client::builder()
        .endpoint("https://oss-cn-hangzhou.aliyuncs.com")
        .region("cn-hangzhou")
        .bucket("my-bucket")
        .access_key_id("your-access-key-id")
        .access_key_secret("your-access-key-secret")
        .build()?;

    // Upload a file
    let data = b"Hello, OSS!";
    let response = client.put_object("hello.txt", data, None).await?;
    println!("Upload successful! ETag: {}", response.etag);

    // Download the file
    let content = client.get_object("hello.txt", Default::default(), None).await?;
    println!("Downloaded content: {}", String::from_utf8_lossy(&content));

    Ok(())
}
```

## 📚 Core Operations

### Bucket Operations

```rust
use ossify::ops::bucket::*;

// Create a bucket
let config = PutBucketConfiguration::new();
client.put_bucket(config, None).await?;

// List objects
let options = ListObjectsOptions::new().max_keys(100);
let result = client.list_objects(Some(options)).await?;

// Get bucket info
let info = client.get_bucket_info().await?;
println!("Bucket: {}, Location: {}", info.name, info.location);

// Delete bucket
client.delete_bucket().await?;
```

### Object Operations

```rust
use ossify::ops::object::base::*;

// Upload object with options
let options = PutObjectOptions::new()
    .content_type("text/plain")
    .storage_class(StorageClass::Standard);
client.put_object("file.txt", data, Some(options)).await?;

// Download with range
let params = GetObjectParams::new();
let options = GetObjectOptions::new().range("bytes=0-1023");
let content = client.get_object("file.txt", params, Some(options)).await?;

// Get object metadata
let metadata = client.head_object("file.txt", None).await?;
println!("Size: {}, Last Modified: {:?}", metadata.content_length, metadata.last_modified);

// Delete object
client.delete_object("file.txt", None).await?;
```

### Multipart Upload

```rust
use ossify::ops::object::multipart_upload::*;

// Initialize multipart upload
let result = client.initiate_multipart_upload("large-file.bin", None).await?;
let upload_id = result.upload_id;

// Upload parts
let part1 = client.upload_part("large-file.bin", &upload_id, 1, &chunk1).await?;
let part2 = client.upload_part("large-file.bin", &upload_id, 2, &chunk2).await?;

// Complete upload
let parts = vec![
    Part::new(1, part1.etag),
    Part::new(2, part2.etag),
];
let options = CompleteMultipartUploadOptions::new().parts(parts);
client.complete_multipart_upload("large-file.bin", &upload_id, Some(options)).await?;
```

## 🔧 Configuration

### Client Builder

The SDK provides a flexible builder pattern for configuration:

```rust
use ossify::{Client, UrlStyle};
use std::time::Duration;

let client = Client::builder()
    .endpoint("https://oss-cn-hangzhou.aliyuncs.com")
    .public_endpoint("https://oss-cn-hangzhou.aliyuncs.com") // Optional
    .region("cn-hangzhou")
    .bucket("my-bucket")
    .access_key_id("your-access-key-id")
    .access_key_secret("your-access-key-secret")
    .security_token("your-sts-token") // Optional, for temporary credentials
    .http_timeout(Duration::from_secs(30))
    .url_style(UrlStyle::VirtualHosted) // VirtualHosted, Path, or CName
    .build()?;
```

### URL Styles

- **VirtualHosted** (default): `https://bucket.oss-cn-hangzhou.aliyuncs.com/object`
- **Path**: `https://oss-cn-hangzhou.aliyuncs.com/bucket/object`
- **CNAME**: `https://custom-domain.com/object`

### Authentication

The SDK supports multiple authentication methods:

```rust
// Basic credentials
let client = Client::builder()
    .access_key_id("your-key")
    .access_key_secret("your-secret")
    .build()?;

// With STS token (temporary credentials)
let client = Client::builder()
    .access_key_id("your-key")
    .access_key_secret("your-secret")
    .security_token("your-sts-token")
    .build()?;
```

## 🌟 Advanced Features

### Presigned URLs

Generate presigned URLs for secure, temporary access:

```rust
use ossify::QueryAuthOptions;

let auth_options = QueryAuthOptions::builder()
    .expires_in(3600) // 1 hour
    .build()?;

let url = client.presign_get_object(
    "private-file.jpg",
    true, // public endpoint
    GetObjectParams::new(),
    None,
    auth_options
).await?;

println!("Presigned URL: {}", url);
```

### Streaming Support

Efficient handling of large files with streaming:

```rust
// The SDK uses reqwest's streaming capabilities internally
// for efficient memory usage with large objects
```

### Error Handling

Comprehensive error types for robust applications:

```rust
use ossify::Error;

match client.get_object("nonexistent.txt", Default::default(), None).await {
    Ok(content) => println!("File content: {:?}", content),
    Err(Error::HttpError(status)) if status.as_u16() == 404 => {
        println!("File not found");
    },
    Err(e) => eprintln!("Error: {}", e),
}
```

## 🏗️ Architecture

This SDK is built with modern Rust practices:

- **Async/Await**: Full async support powered by `tokio` and `reqwest`
- **Type Safety**: Leverages Rust's type system to prevent errors
- **Zero-Copy**: Efficient memory usage with `Cow` and `Bytes`
- **Modular Design**: Clean separation of concerns with trait-based architecture
- **Comprehensive**: Covers all major OSS operations

### Core Dependencies

- **reqwest**: HTTP client with async support and robust error handling
- **tokio**: Async runtime for high-performance I/O
- **serde**: Serialization/deserialization for API requests/responses
- **chrono**: Date/time handling for OSS operations
- **bytes**: Efficient byte buffer management

## 📖 Documentation

For detailed documentation and examples, visit:

- [API Documentation](https://docs.rs/ossify)
- [Official OSS Documentation](https://www.alibabacloud.com/help/en/oss/)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [reqwest](https://github.com/seanmonstar/reqwest) - the amazing HTTP client for Rust
- Inspired by the need for a modern, easy-to-use OSS SDK in the Rust ecosystem
- Thanks to the Alibaba Cloud team for providing comprehensive OSS documentation

---

<p align="center">
<strong>Happy coding with OSS and Rust! 🦀✨</strong>
</p>
