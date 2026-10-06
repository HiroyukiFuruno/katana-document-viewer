use super::PersistentCacheError;
use serde::{Serialize, de::DeserializeOwned};

pub(super) struct PayloadCodec;

impl PayloadCodec {
    pub(super) fn encode<T: Serialize>(
        metadata: &T,
        payload: &[u8],
    ) -> Result<Vec<u8>, PersistentCacheError> {
        let json = serde_json::to_vec(metadata).map_err(|_| PersistentCacheError::Corrupt)?;
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
        let value = serde_json::from_slice(metadata).map_err(|_| PersistentCacheError::Corrupt)?;
        Ok((value, &bytes[end..]))
    }
}
