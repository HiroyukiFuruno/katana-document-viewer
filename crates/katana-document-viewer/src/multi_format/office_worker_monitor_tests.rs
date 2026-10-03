use super::MacOsMemoryMonitor;
use std::time::{Duration, Instant};

const FINISH_WATCHDOG: Duration = Duration::from_secs(2);

struct SynchronizedMonitor {
    monitor: MacOsMemoryMonitor,
    ready: std::sync::mpsc::Receiver<()>,
    worker_thread: std::thread::Thread,
    stop: std::sync::Arc<std::sync::atomic::AtomicBool>,
    park_after_stop: bool,
}

fn spawn_synchronized_worker(
    stop: std::sync::Arc<std::sync::atomic::AtomicBool>,
    exceeded: std::sync::Arc<std::sync::atomic::AtomicBool>,
    ready: std::sync::mpsc::Sender<()>,
    park_after_stop: bool,
    mark_exceeded: bool,
) -> std::thread::JoinHandle<()> {
    use std::sync::atomic::Ordering;

    std::thread::spawn(move || {
        if mark_exceeded {
            exceeded.store(true, Ordering::Release);
        }
        let _ = ready.send(());
        if park_after_stop {
            let deadline = Instant::now() + FINISH_WATCHDOG;
            while !stop.load(Ordering::Acquire) && Instant::now() < deadline {
                std::thread::yield_now();
            }
        }
        std::thread::park();
        while !stop.load(Ordering::Acquire) {
            std::thread::park();
        }
    })
}

fn synchronized_monitor(park_after_stop: bool, mark_exceeded: bool) -> SynchronizedMonitor {
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::sync::mpsc;

    let stop = Arc::new(AtomicBool::new(false));
    let exceeded = Arc::new(AtomicBool::new(false));
    let (ready_tx, ready_rx) = mpsc::channel();
    let worker = spawn_synchronized_worker(
        Arc::clone(&stop),
        Arc::clone(&exceeded),
        ready_tx,
        park_after_stop,
        mark_exceeded,
    );
    let worker_thread = worker.thread().clone();
    let monitor = MacOsMemoryMonitor {
        stop: Arc::clone(&stop),
        exceeded: Arc::clone(&exceeded),
        worker,
    };
    SynchronizedMonitor {
        monitor,
        ready: ready_rx,
        worker_thread,
        stop,
        park_after_stop,
    }
}

fn start_finish(
    monitor: MacOsMemoryMonitor,
) -> (std::sync::mpsc::Receiver<bool>, std::thread::JoinHandle<()>) {
    use std::sync::mpsc;

    let (finished_tx, finished_rx) = mpsc::channel();
    let finisher = std::thread::spawn(move || {
        let result = monitor.finish();
        let _ = finished_tx.send(result);
    });
    (finished_rx, finisher)
}

fn await_stop(stop: &std::sync::atomic::AtomicBool) {
    let deadline = Instant::now() + FINISH_WATCHDOG;
    while !stop.load(std::sync::atomic::Ordering::Acquire) && Instant::now() < deadline {
        std::thread::yield_now();
    }
}

fn collect_finish(
    finished_rx: std::sync::mpsc::Receiver<bool>,
    worker_thread: std::thread::Thread,
) -> (Option<bool>, bool) {
    let completed = finished_rx.recv_timeout(FINISH_WATCHDOG);
    let completed_without_fallback = completed.is_ok();
    let finish_result = match completed {
        Ok(result) => Some(result),
        Err(_) => {
            // 旧finishのjoin待ちでもworkerを回収し、漏れなくREDにする。
            worker_thread.unpark();
            finished_rx.recv_timeout(FINISH_WATCHDOG).ok()
        }
    };
    (finish_result, completed_without_fallback)
}

fn finish_synchronized_monitor(test_monitor: SynchronizedMonitor) -> (bool, bool) {
    assert!(test_monitor.ready.recv_timeout(FINISH_WATCHDOG).is_ok());
    let (finished_rx, finisher) = start_finish(test_monitor.monitor);
    if test_monitor.park_after_stop {
        await_stop(&test_monitor.stop);
    }
    let (finish_result, completed_without_fallback) =
        collect_finish(finished_rx, test_monitor.worker_thread);
    if finish_result.is_some() {
        assert!(finisher.join().is_ok());
    }
    assert!(
        finish_result.is_some(),
        "worker did not finish after fallback"
    );
    (finish_result == Some(true), completed_without_fallback)
}

fn finish_with_parked_worker(park_after_stop: bool, mark_exceeded: bool) -> (bool, bool) {
    finish_synchronized_monitor(synchronized_monitor(park_after_stop, mark_exceeded))
}

#[test]
fn finish_retains_stop_notification_received_before_park() {
    let (exceeded, completed_without_fallback) = finish_with_parked_worker(true, false);

    assert!(
        completed_without_fallback,
        "finish required fallback unpark"
    );
    assert!(!exceeded);
}

#[test]
fn finish_notifies_a_worker_entering_wait() {
    let (exceeded, completed_without_fallback) = finish_with_parked_worker(false, false);

    assert!(
        completed_without_fallback,
        "finish required fallback unpark"
    );
    assert!(!exceeded);
}

#[test]
fn finish_preserves_exceeded_state_while_unparking_worker() {
    let (exceeded, completed_without_fallback) = finish_with_parked_worker(false, true);

    assert!(
        completed_without_fallback,
        "finish required fallback unpark"
    );
    assert!(exceeded);
}

#[test]
fn monitor_marks_and_terminates_a_process_above_the_limit() {
    let child = std::process::Command::new("/bin/sleep").arg("5").spawn();
    assert!(child.is_ok());
    if let Ok(mut child) = child {
        let monitor = MacOsMemoryMonitor::start(child.id(), 0);
        let deadline = Instant::now() + Duration::from_secs(2);
        while !monitor.exceeded() && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(10));
        }
        assert!(monitor.finish());
        let _ = child.wait();
    }
}

#[test]
fn monitor_finishes_without_exceeding_for_stopped_and_missing_processes() {
    let monitor = MacOsMemoryMonitor::start(std::process::id(), usize::MAX);
    std::thread::sleep(Duration::from_millis(20));
    assert!(!monitor.finish());
    let monitor = MacOsMemoryMonitor::start(u32::MAX, usize::MAX);
    std::thread::sleep(Duration::from_millis(20));
    assert!(!monitor.finish());
}
