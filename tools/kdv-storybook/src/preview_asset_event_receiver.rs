use super::{AssetWorkerMessage, MAX_INCREMENTAL_ASSET_WORKERS};
use crate::preview::PreviewBuilder;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;

const READY_BATCH_LIMIT: usize = 4;

impl PreviewBuilder {
    pub(super) fn receive_asset_worker_messages(
        receiver: mpsc::Receiver<Result<AssetWorkerMessage, String>>,
        expected_count: usize,
        cancel_token: &AtomicBool,
        accept: &mut impl FnMut(Vec<AssetWorkerMessage>, bool) -> Result<(), String>,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let mut received = 0;
        while let Ok(first) = receiver.recv() {
            if cancel_token.load(Ordering::Relaxed) {
                break;
            }
            let batch_limit = Self::ready_batch_limit(expected_count, received);
            let mut batch = vec![Self::decode_worker_message(first)?];
            received += 1;
            while batch.len() < batch_limit
                && let Ok(message) = receiver.try_recv()
            {
                if cancel_token.load(Ordering::Relaxed) {
                    break;
                }
                batch.push(Self::decode_worker_message(message)?);
                received += 1;
            }
            accept(batch, received == expected_count).map_err(std::io::Error::other)?;
        }
        if !cancel_token.load(Ordering::Relaxed) && received < expected_count {
            return Err(std::io::Error::other(format!(
                "asset worker channel closed before all assets completed: received={received}, expected={expected_count}",
            ))
            .into());
        }
        Ok(received)
    }

    fn decode_worker_message(
        message: Result<AssetWorkerMessage, String>,
    ) -> Result<AssetWorkerMessage, std::io::Error> {
        message.map_err(std::io::Error::other)
    }

    pub(super) fn incremental_asset_worker_count(request_count: usize) -> usize {
        request_count.min(MAX_INCREMENTAL_ASSET_WORKERS)
    }

    fn ready_batch_limit(expected_count: usize, received_count: usize) -> usize {
        if expected_count > 1 && received_count == 0 {
            return 1;
        }
        READY_BATCH_LIMIT
    }
}
