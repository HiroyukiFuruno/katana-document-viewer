use super::PersistentCacheError;
use serde::{Serialize, de::DeserializeOwned};

pub(super) struct PayloadCodec;

pub(super) const MAX_PAYLOAD_BYTES: usize = 128 * 1024 * 1024;
pub(super) const MAX_METADATA_BYTES: usize = 1024 * 1024;
pub(super) const PREFIX_BYTES: u64 = 8;

fn validate_lengths(metadata: usize, payload: usize) -> Result<(), PersistentCacheError> {
    if metadata > MAX_METADATA_BYTES || payload > MAX_PAYLOAD_BYTES {
        return Err(PersistentCacheError::Capacity);
    }
    Ok(())
}

pub(super) fn validate_framed_payload(bytes: &[u8]) -> bool {
    PayloadCodec::decode::<serde_json::Value>(bytes).is_ok()
}

impl PayloadCodec {
    pub(super) fn encode<T: Serialize>(
        metadata: &T,
        payload: &[u8],
    ) -> Result<Vec<u8>, PersistentCacheError> {
        let json = serde_json::to_vec(metadata).map_err(|_| PersistentCacheError::Corrupt)?;
        validate_lengths(json.len(), payload.len())?;
        let mut bytes = Vec::with_capacity(8 + json.len() + payload.len());
        bytes.extend_from_slice(&(json.len() as u64).to_le_bytes());
        bytes.extend_from_slice(&json);
        bytes.extend_from_slice(payload);
        Ok(bytes)
    }

    pub(super) fn decode<T: DeserializeOwned>(
        bytes: &[u8],
    ) -> Result<(T, &[u8]), PersistentCacheError> {
        let prefix = bytes.get(..8).ok_or(PersistentCacheError::Corrupt)?;
        let length = u64::from_le_bytes(
            prefix
                .try_into()
                .map_err(|_| PersistentCacheError::Corrupt)?,
        );
        let end = usize::try_from(length)
            .ok()
            .and_then(|n| n.checked_add(8))
            .ok_or(PersistentCacheError::Corrupt)?;
        let metadata = bytes.get(8..end).ok_or(PersistentCacheError::Corrupt)?;
        if metadata.len() > MAX_METADATA_BYTES {
            return Err(PersistentCacheError::Capacity);
        }
        if bytes.len().saturating_sub(end) > MAX_PAYLOAD_BYTES {
            return Err(PersistentCacheError::Capacity);
        }
        let value = serde_json::from_slice(metadata).map_err(|_| PersistentCacheError::Corrupt)?;
        Ok((value, &bytes[end..]))
    }
}
