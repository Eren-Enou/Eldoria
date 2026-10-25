//! Optional transport only. The uncompressed v1 commit and world payload are unchanged.
use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    borrow::Cow,
    io::{Cursor, Read, Write},
};
const LIMIT: usize = 160 * 1024 * 1024;
#[derive(Clone, Copy, Debug)]
pub enum Codec {
    Raw,
    Zlib,
}
#[derive(Deserialize)]
struct Header {
    schema: String,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Packed {
    schema: String,
    codec: String,
    uncompressed_bytes: u64,
    uncompressed_sha256: String,
    #[serde(with = "serde_bytes")]
    payload: Vec<u8>,
}
fn parse<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, String> {
    let mut cursor = Cursor::new(bytes);
    let value = ciborium::from_reader(&mut cursor).map_err(|e| e.to_string())?;
    if cursor.position() != bytes.len() as u64 {
        return Err("trailing storage bytes".into());
    }
    Ok(value)
}
fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub fn encode(raw: &[u8], codec: Codec) -> Result<Vec<u8>, String> {
    if raw.len() > LIMIT {
        return Err("uncompressed commit exceeds160MiB".into());
    }
    match codec {
        Codec::Raw => Ok(raw.to_vec()),
        Codec::Zlib => {
            let mut encoder = ZlibEncoder::new(vec![], Compression::new(6));
            encoder.write_all(raw).map_err(|e| e.to_string())?;
            let packed = Packed {
                schema: "woi-world-storage/v2".into(),
                codec: "zlib".into(),
                uncompressed_bytes: raw.len() as u64,
                uncompressed_sha256: hash(raw),
                payload: encoder.finish().map_err(|e| e.to_string())?,
            };
            let mut bytes = vec![];
            ciborium::into_writer(&packed, &mut bytes).map_err(|e| e.to_string())?;
            Ok(bytes)
        }
    }
}
pub fn decode(bytes: &[u8]) -> Result<Cow<'_, [u8]>, String> {
    if bytes.len() > LIMIT {
        return Err("storage container exceeds160MiB".into());
    }
    let header: Header = parse(bytes)?;
    match header.schema.as_str() {
        "woi-world-commit/v1" => Ok(Cow::Borrowed(bytes)),
        "woi-world-storage/v2" => {
            let packed: Packed = parse(bytes)?;
            if packed.codec != "zlib" || packed.uncompressed_bytes > LIMIT as u64 {
                return Err("unsupported codec or decompression size".into());
            }
            let mut decoder = ZlibDecoder::new(packed.payload.as_slice());
            let mut raw = vec![];
            (&mut decoder)
                .take(packed.uncompressed_bytes + 1)
                .read_to_end(&mut raw)
                .map_err(|e| e.to_string())?;
            if raw.len() as u64 != packed.uncompressed_bytes
                || decoder.total_in() != packed.payload.len() as u64
                || hash(&raw) != packed.uncompressed_sha256
            {
                return Err("compressed commit length/checksum/trailing-data mismatch".into());
            }
            let inner: Header = parse(&raw)?;
            if inner.schema != "woi-world-commit/v1" {
                return Err("nested/unsupported compressed commit".into());
            }
            Ok(Cow::Owned(raw))
        }
        _ => Err("unsupported storage schema".into()),
    }
}
