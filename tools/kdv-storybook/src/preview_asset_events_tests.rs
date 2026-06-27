use super::{AssetWorkerMessage, PreviewBuilder};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;

#[test]
fn receive_asset_worker_messages_counts_none_results_as_processed()
-> Result<(), Box<dyn std::error::Error>> {
    let (sender, receiver) = mpsc::channel();
    sender.send(Ok(AssetWorkerMessage { artifact: None }))?;
    drop(sender);
    let cancel_token = AtomicBool::new(false);
    let mut batches = Vec::new();

    let received = PreviewBuilder::receive_asset_worker_messages(
        receiver,
        1,
        &cancel_token,
        &mut |messages, complete| {
            batches.push((messages.len(), complete));
            Ok(())
        },
    )?;

    assert_eq!(1, received);
    assert_eq!(vec![(1, true)], batches);
    Ok(())
}

#[test]
fn receive_asset_worker_messages_batches_ready_results() -> Result<(), Box<dyn std::error::Error>> {
    let (sender, receiver) = mpsc::channel();
    sender.send(Ok(AssetWorkerMessage { artifact: None }))?;
    sender.send(Ok(AssetWorkerMessage { artifact: None }))?;
    sender.send(Ok(AssetWorkerMessage { artifact: None }))?;
    drop(sender);
    let cancel_token = AtomicBool::new(false);
    let mut batches = Vec::new();

    let received = PreviewBuilder::receive_asset_worker_messages(
        receiver,
        3,
        &cancel_token,
        &mut |messages, complete| {
            batches.push((messages.len(), complete));
            Ok(())
        },
    )?;

    assert_eq!(3, received);
    assert_eq!(vec![(1, false), (2, true)], batches);
    Ok(())
}

#[test]
fn receive_asset_worker_messages_fails_when_channel_closes_before_expected_assets()
-> Result<(), Box<dyn std::error::Error>> {
    let (sender, receiver) = mpsc::channel();
    sender.send(Ok(AssetWorkerMessage { artifact: None }))?;
    drop(sender);
    let cancel_token = AtomicBool::new(false);
    let mut batches = Vec::new();

    let error = match PreviewBuilder::receive_asset_worker_messages(
        receiver,
        2,
        &cancel_token,
        &mut |messages, complete| {
            batches.push((messages.len(), complete));
            Ok(())
        },
    ) {
        Ok(_) => return Err(std::io::Error::other("partial channel close must fail").into()),
        Err(error) => error,
    };

    assert!(
        error
            .to_string()
            .contains("asset worker channel closed before all assets completed")
    );
    assert_eq!(vec![(1, false)], batches);
    Ok(())
}

#[test]
fn incremental_asset_worker_count_uses_parallel_workers_without_exceeding_cap() {
    assert_eq!(0, PreviewBuilder::incremental_asset_worker_count(0));
    assert_eq!(1, PreviewBuilder::incremental_asset_worker_count(1));
    assert_eq!(4, PreviewBuilder::incremental_asset_worker_count(4));
    assert_eq!(5, PreviewBuilder::incremental_asset_worker_count(5));
    assert_eq!(16, PreviewBuilder::incremental_asset_worker_count(32));
}

#[test]
fn receive_asset_worker_messages_stops_without_complete_after_cancel()
-> Result<(), Box<dyn std::error::Error>> {
    let (sender, receiver) = mpsc::channel();
    sender.send(Ok(AssetWorkerMessage { artifact: None }))?;
    sender.send(Ok(AssetWorkerMessage { artifact: None }))?;
    drop(sender);
    let cancel_token = AtomicBool::new(true);
    let mut batches = Vec::new();

    let received = PreviewBuilder::receive_asset_worker_messages(
        receiver,
        2,
        &cancel_token,
        &mut |messages, complete| {
            batches.push((messages.len(), complete));
            Ok(())
        },
    )?;

    assert_eq!(0, received);
    assert!(batches.is_empty());
    assert!(cancel_token.load(Ordering::Relaxed));
    Ok(())
}
