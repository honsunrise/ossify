//! Binary frame-stream decoder for `SelectObject` and
//! `CreateSelectObjectMeta`.
//!
//! Wire format (all multi-byte integers are **big-endian**):
//!
//! ```text
//! +---------+-------------+-------------+----------+---------+----------+
//! | version | frame type  | payload len | header   | payload | payload  |
//! | (1 B)   | (3 B)       | (4 B)       | CRC-32   |         | CRC-32   |
//! |         |             |             | (4 B)    | (var)   | (4 B)    |
//! +---------+-------------+-------------+----------+---------+----------+
//! ```
//!
//! The first four bytes are read as a big-endian `u32` whose top byte (the
//! protocol version, currently `1`) is masked to `0` before matching the
//! frame-type constants. All payloads start with an 8-byte `offset`
//! indicating how far into the source object the server has scanned.
//!
//! Frame types recognised:
//!
//! | value      | meaning                                            |
//! |------------|----------------------------------------------------|
//! | `0x800001` | [`SelectFrame::Data`] — query output chunk         |
//! | `0x800004` | [`SelectFrame::Continuous`] — heartbeat (no data)  |
//! | `0x800005` | [`SelectFrame::End`] — terminal status / error     |
//! | `0x800006` | [`SelectFrame::MetaEndCsv`] — meta-index result    |
//! | `0x800007` | [`SelectFrame::MetaEndJson`] — meta-index result   |
//!
//! The payload CRC-32 is the IEEE CRC of the *entire* payload (including
//! the 8-byte offset prefix). Header CRC is the CRC of `frame_type ||
//! payload_length` but the server occasionally leaves either checksum
//! zeroed out, which the SDKs treat as "skip verification" by convention.

use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::{Buf, Bytes, BytesMut};
use futures::{Stream, StreamExt};

use crate::{Error, Result};

/// Frame-type codes (with the version byte cleared).
const FRAME_DATA: u32 = 0x0080_0001;
const FRAME_CONTINUOUS: u32 = 0x0080_0004;
const FRAME_END: u32 = 0x0080_0005;
const FRAME_META_END_CSV: u32 = 0x0080_0006;
const FRAME_META_END_JSON: u32 = 0x0080_0007;

const MIN_PAYLOAD_LEN: usize = 8; // all payloads start with an 8-byte offset

/// One decoded frame from a `SelectObject` / `CreateSelectObjectMeta`
/// response stream.
#[derive(Debug, Clone)]
pub enum SelectFrame {
    /// Data chunk frame. `data` is a slice of the query output.
    Data {
        /// Number of bytes of the source object scanned so far.
        offset: u64,
        data: Bytes,
    },
    /// Heartbeat / keep-alive frame with no data.
    Continuous { offset: u64 },
    /// Terminal frame. `status` is the final HTTP-style status code —
    /// callers MUST check this and not rely on the outer HTTP response.
    End {
        offset: u64,
        total_scanned: u64,
        status: u32,
        /// "ErrorCode1,…,ErrorCodeN.Message" when `status` is non-2xx.
        error_message: String,
    },
    /// Meta-index result for `CreateSelectObjectMeta` on CSV objects.
    MetaEndCsv {
        offset: u64,
        total_scanned: u64,
        status: u32,
        splits_count: u32,
        rows_count: u64,
        cols_count: u32,
        error_message: String,
    },
    /// Meta-index result for `CreateSelectObjectMeta` on JSON objects.
    /// (No `cols_count` because JSON objects have no column concept.)
    MetaEndJson {
        offset: u64,
        total_scanned: u64,
        status: u32,
        splits_count: u32,
        rows_count: u64,
        error_message: String,
    },
}

impl SelectFrame {
    /// Decode a single frame from the start of `buf`. Returns
    /// `Ok(Some((frame, consumed)))` when a full frame is available,
    /// `Ok(None)` when more bytes are needed, or an error on corruption.
    ///
    /// If `verify_payload_crc` is true (set by the caller when the request
    /// carried `EnablePayloadCrc=true`), the payload CRC-32 is checked
    /// against the trailing 4 bytes; a zero server-side CRC is treated as
    /// "skip verification" to match the behaviour of the Go/Python SDKs.
    pub fn decode(buf: &[u8], verify_payload_crc: bool) -> Result<Option<(Self, usize)>> {
        // Fixed-size header: frame_type(4) + payload_len(4) + header_crc(4) = 12 bytes,
        // plus at least 8 bytes of payload (the offset) before we can even
        // identify the frame. So the minimum pre-payload read is 20 bytes.
        if buf.len() < 20 {
            return Ok(None);
        }

        // Parse the frame type with the version byte masked out.
        let type_bytes = [0, buf[1], buf[2], buf[3]];
        let frame_type = u32::from_be_bytes(type_bytes);

        let payload_len = u32::from_be_bytes([buf[4], buf[5], buf[6], buf[7]]) as usize;
        // header CRC at buf[8..12] — documented but not verified by SDKs.

        if payload_len < MIN_PAYLOAD_LEN {
            return Err(Error::Other(format!(
                "select frame payload too short: {payload_len} < {MIN_PAYLOAD_LEN}"
            )));
        }

        let total_len = 12 + payload_len + 4; // header + payload + payload CRC
        if buf.len() < total_len {
            return Ok(None);
        }

        let payload = &buf[12..12 + payload_len];
        let server_crc = u32::from_be_bytes([
            buf[12 + payload_len],
            buf[12 + payload_len + 1],
            buf[12 + payload_len + 2],
            buf[12 + payload_len + 3],
        ]);

        if verify_payload_crc && server_crc != 0 {
            let mut hasher = crc32fast::Hasher::new();
            hasher.update(payload);
            let client_crc = hasher.finalize();
            if client_crc != server_crc {
                return Err(Error::Other(format!(
                    "select payload CRC mismatch: server={server_crc:#010x}, \
                     client={client_crc:#010x}"
                )));
            }
        }

        let frame = decode_payload(frame_type, payload)?;
        Ok(Some((frame, total_len)))
    }

    /// The `status` field of an [`SelectFrame::End`] / [`SelectFrame::MetaEndCsv`]
    /// / [`SelectFrame::MetaEndJson`] frame. `None` for data/continuous frames.
    pub fn terminal_status(&self) -> Option<u32> {
        match self {
            SelectFrame::Data { .. } | SelectFrame::Continuous { .. } => None,
            SelectFrame::End { status, .. }
            | SelectFrame::MetaEndCsv { status, .. }
            | SelectFrame::MetaEndJson { status, .. } => Some(*status),
        }
    }
}

fn decode_payload(frame_type: u32, payload: &[u8]) -> Result<SelectFrame> {
    let offset = u64::from_be_bytes(payload[0..8].try_into().unwrap());
    let rest = &payload[8..];

    match frame_type {
        FRAME_DATA => Ok(SelectFrame::Data {
            offset,
            data: Bytes::copy_from_slice(rest),
        }),
        FRAME_CONTINUOUS => {
            if !rest.is_empty() {
                return Err(Error::Other("select continuous frame has trailing bytes".into()));
            }
            Ok(SelectFrame::Continuous { offset })
        },
        FRAME_END => {
            if rest.len() < 12 {
                return Err(Error::Other("select end frame truncated".into()));
            }
            let total_scanned = u64::from_be_bytes(rest[0..8].try_into().unwrap());
            let status = u32::from_be_bytes(rest[8..12].try_into().unwrap());
            let error_message = String::from_utf8_lossy(&rest[12..]).into_owned();
            Ok(SelectFrame::End {
                offset,
                total_scanned,
                status,
                error_message,
            })
        },
        FRAME_META_END_CSV => {
            // total_scanned(8) + status(4) + splits(4) + rows(8) + cols(4) = 28
            if rest.len() < 28 {
                return Err(Error::Other("select meta-end CSV frame truncated".into()));
            }
            let total_scanned = u64::from_be_bytes(rest[0..8].try_into().unwrap());
            let status = u32::from_be_bytes(rest[8..12].try_into().unwrap());
            let splits_count = u32::from_be_bytes(rest[12..16].try_into().unwrap());
            let rows_count = u64::from_be_bytes(rest[16..24].try_into().unwrap());
            let cols_count = u32::from_be_bytes(rest[24..28].try_into().unwrap());
            let error_message = String::from_utf8_lossy(&rest[28..]).into_owned();
            Ok(SelectFrame::MetaEndCsv {
                offset,
                total_scanned,
                status,
                splits_count,
                rows_count,
                cols_count,
                error_message,
            })
        },
        FRAME_META_END_JSON => {
            // total_scanned(8) + status(4) + splits(4) + rows(8) = 24
            if rest.len() < 24 {
                return Err(Error::Other("select meta-end JSON frame truncated".into()));
            }
            let total_scanned = u64::from_be_bytes(rest[0..8].try_into().unwrap());
            let status = u32::from_be_bytes(rest[8..12].try_into().unwrap());
            let splits_count = u32::from_be_bytes(rest[12..16].try_into().unwrap());
            let rows_count = u64::from_be_bytes(rest[16..24].try_into().unwrap());
            let error_message = String::from_utf8_lossy(&rest[24..]).into_owned();
            Ok(SelectFrame::MetaEndJson {
                offset,
                total_scanned,
                status,
                splits_count,
                rows_count,
                error_message,
            })
        },
        other => Err(Error::Other(format!("unknown select frame type: {other:#010x}"))),
    }
}

/// Streaming decoder that adapts a raw byte stream (typically a
/// `reqwest::Response::bytes_stream()`) into a stream of [`SelectFrame`]s.
pub struct SelectFrameStream<S> {
    inner: S,
    buffer: BytesMut,
    exhausted: bool,
    verify_payload_crc: bool,
}

impl<S> SelectFrameStream<S> {
    pub fn new(inner: S, verify_payload_crc: bool) -> Self {
        Self {
            inner,
            buffer: BytesMut::new(),
            exhausted: false,
            verify_payload_crc,
        }
    }
}

impl<S> Stream for SelectFrameStream<S>
where
    S: Stream<Item = std::result::Result<Bytes, reqwest::Error>> + Unpin,
{
    type Item = Result<SelectFrame>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            // Try to decode a frame from what we already buffered.
            match SelectFrame::decode(&self.buffer, self.verify_payload_crc) {
                Ok(Some((frame, consumed))) => {
                    self.buffer.advance(consumed);
                    return Poll::Ready(Some(Ok(frame)));
                },
                Ok(None) => {
                    if self.exhausted {
                        return Poll::Ready(None);
                    }
                    // fall through to read more bytes
                },
                Err(e) => return Poll::Ready(Some(Err(e))),
            }

            match self.inner.poll_next_unpin(cx) {
                Poll::Ready(Some(Ok(chunk))) => {
                    self.buffer.extend_from_slice(&chunk);
                },
                Poll::Ready(Some(Err(e))) => {
                    return Poll::Ready(Some(Err(e.into())));
                },
                Poll::Ready(None) => {
                    self.exhausted = true;
                    if self.buffer.is_empty() {
                        return Poll::Ready(None);
                    }
                    // Loop once more; decode will turn remaining trailing
                    // garbage into an error or succeed with the final frame.
                },
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Encode one frame with correct CRCs. Helper for tests only.
    fn encode_frame(frame_type: u32, payload: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(12 + payload.len() + 4);
        let type_with_version = frame_type | (1u32 << 24);
        out.extend_from_slice(&type_with_version.to_be_bytes());
        out.extend_from_slice(&(payload.len() as u32).to_be_bytes());
        // Header CRC: CRC of frame_type_bytes || payload_len_bytes
        let mut header_hasher = crc32fast::Hasher::new();
        header_hasher.update(&out[0..8]);
        out.extend_from_slice(&header_hasher.finalize().to_be_bytes());
        out.extend_from_slice(payload);
        // Payload CRC
        let mut payload_hasher = crc32fast::Hasher::new();
        payload_hasher.update(payload);
        out.extend_from_slice(&payload_hasher.finalize().to_be_bytes());
        out
    }

    #[test]
    fn decode_data_frame() {
        let offset: u64 = 1234;
        let mut payload = Vec::new();
        payload.extend_from_slice(&offset.to_be_bytes());
        payload.extend_from_slice(b"hello, world");
        let bytes = encode_frame(FRAME_DATA, &payload);
        let (frame, consumed) = SelectFrame::decode(&bytes, true).unwrap().unwrap();
        assert_eq!(consumed, bytes.len());
        match frame {
            SelectFrame::Data { offset: o, data } => {
                assert_eq!(o, 1234);
                assert_eq!(&data[..], b"hello, world");
            },
            _ => panic!("expected Data frame"),
        }
    }

    #[test]
    fn decode_continuous_frame() {
        let payload = 42u64.to_be_bytes();
        let bytes = encode_frame(FRAME_CONTINUOUS, &payload);
        let (frame, _) = SelectFrame::decode(&bytes, false).unwrap().unwrap();
        matches!(frame, SelectFrame::Continuous { offset: 42 });
    }

    #[test]
    fn decode_end_frame_success() {
        let mut payload = Vec::new();
        payload.extend_from_slice(&100u64.to_be_bytes()); // offset
        payload.extend_from_slice(&200u64.to_be_bytes()); // total_scanned
        payload.extend_from_slice(&200u32.to_be_bytes()); // status
        // no error message
        let bytes = encode_frame(FRAME_END, &payload);
        let (frame, _) = SelectFrame::decode(&bytes, false).unwrap().unwrap();
        match frame {
            SelectFrame::End {
                offset,
                total_scanned,
                status,
                error_message,
            } => {
                assert_eq!(offset, 100);
                assert_eq!(total_scanned, 200);
                assert_eq!(status, 200);
                assert!(error_message.is_empty());
            },
            _ => panic!("expected End frame"),
        }
    }

    #[test]
    fn decode_meta_end_csv() {
        let mut payload = Vec::new();
        payload.extend_from_slice(&0u64.to_be_bytes()); // offset
        payload.extend_from_slice(&1024u64.to_be_bytes()); // total_scanned
        payload.extend_from_slice(&200u32.to_be_bytes()); // status
        payload.extend_from_slice(&4u32.to_be_bytes()); // splits
        payload.extend_from_slice(&10000u64.to_be_bytes()); // rows
        payload.extend_from_slice(&5u32.to_be_bytes()); // cols
        let bytes = encode_frame(FRAME_META_END_CSV, &payload);
        let (frame, _) = SelectFrame::decode(&bytes, false).unwrap().unwrap();
        match frame {
            SelectFrame::MetaEndCsv {
                splits_count,
                rows_count,
                cols_count,
                ..
            } => {
                assert_eq!(splits_count, 4);
                assert_eq!(rows_count, 10000);
                assert_eq!(cols_count, 5);
            },
            _ => panic!("expected MetaEndCsv"),
        }
    }

    #[test]
    fn decode_meta_end_json() {
        let mut payload = Vec::new();
        payload.extend_from_slice(&0u64.to_be_bytes()); // offset
        payload.extend_from_slice(&2048u64.to_be_bytes()); // total_scanned
        payload.extend_from_slice(&200u32.to_be_bytes()); // status
        payload.extend_from_slice(&8u32.to_be_bytes()); // splits
        payload.extend_from_slice(&500u64.to_be_bytes()); // rows
        let bytes = encode_frame(FRAME_META_END_JSON, &payload);
        let (frame, _) = SelectFrame::decode(&bytes, true).unwrap().unwrap();
        match frame {
            SelectFrame::MetaEndJson {
                splits_count,
                rows_count,
                ..
            } => {
                assert_eq!(splits_count, 8);
                assert_eq!(rows_count, 500);
            },
            _ => panic!("expected MetaEndJson"),
        }
    }

    #[test]
    fn decode_returns_none_when_incomplete() {
        let mut payload = Vec::new();
        payload.extend_from_slice(&0u64.to_be_bytes());
        payload.extend_from_slice(b"data");
        let bytes = encode_frame(FRAME_DATA, &payload);
        // Feed only the first few bytes — should request more data.
        assert!(SelectFrame::decode(&bytes[..10], false).unwrap().is_none());
        assert!(SelectFrame::decode(&bytes[..20], false).unwrap().is_none());
        // Full buffer decodes successfully.
        assert!(SelectFrame::decode(&bytes, false).unwrap().is_some());
    }

    #[test]
    fn decode_rejects_unknown_frame_type() {
        // Build a frame with an unrecognised type.
        let mut payload = Vec::new();
        payload.extend_from_slice(&0u64.to_be_bytes());
        let bytes = encode_frame(0x00AA_BBCC, &payload);
        let err = SelectFrame::decode(&bytes, false).unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("unknown select frame type"), "msg={msg}");
    }

    #[test]
    fn payload_crc_mismatch_is_detected() {
        let mut payload = Vec::new();
        payload.extend_from_slice(&0u64.to_be_bytes());
        payload.extend_from_slice(b"data");
        let mut bytes = encode_frame(FRAME_DATA, &payload);
        // Corrupt the payload CRC trailer.
        let len = bytes.len();
        bytes[len - 1] ^= 0xff;
        let err = SelectFrame::decode(&bytes, true).unwrap_err();
        assert!(format!("{err}").contains("CRC mismatch"));
    }

    #[test]
    fn payload_crc_zero_is_accepted() {
        let mut payload = Vec::new();
        payload.extend_from_slice(&0u64.to_be_bytes());
        payload.extend_from_slice(b"data");
        let mut bytes = encode_frame(FRAME_DATA, &payload);
        // Zero out the payload CRC: server convention for "do not verify".
        let len = bytes.len();
        bytes[len - 4..].copy_from_slice(&[0, 0, 0, 0]);
        let result = SelectFrame::decode(&bytes, true).unwrap();
        assert!(result.is_some());
    }
}
