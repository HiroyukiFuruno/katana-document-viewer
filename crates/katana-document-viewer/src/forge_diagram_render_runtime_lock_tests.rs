use super::super::{recover_mutex_guard, with_krr_plantuml_render_lock};
use std::sync::{
    Arc, Barrier, Mutex,
    atomic::{AtomicUsize, Ordering},
};
use std::thread;
use std::time::Duration;

#[test]
fn plantuml_render_lock_serializes_renderer_calls() -> Result<(), String> {
    let active = Arc::new(AtomicUsize::new(0));
    let max_active = Arc::new(AtomicUsize::new(0));
    let start = Arc::new(Barrier::new(3));
    let handles = (0..2)
        .map(|_| {
            spawn_plantuml_render_lock_probe(
                Arc::clone(&active),
                Arc::clone(&max_active),
                Arc::clone(&start),
            )
        })
        .collect::<Vec<_>>();

    start.wait();
    for handle in handles {
        join_plantuml_render_lock_probe(handle)?;
    }

    assert_eq!(1, max_active.load(Ordering::SeqCst));
    assert_eq!(0, active.load(Ordering::SeqCst));
    Ok(())
}

#[test]
fn plantuml_render_lock_recovers_from_poisoned_mutex() {
    let mutex = Arc::new(Mutex::new(()));
    let poison_target = Arc::clone(&mutex);
    let poisoned = thread::spawn(move || poison_mutex(&poison_target))
        .join()
        .is_err();

    assert!(poisoned);
    assert!(mutex.is_poisoned());
    drop(recover_mutex_guard(&mutex));
}

#[test]
fn plantuml_render_lock_acquires_healthy_mutex() {
    let mutex = Mutex::new(());

    drop(recover_mutex_guard(&mutex));
    assert!(!mutex.is_poisoned());
}

fn poison_mutex(mutex: &Mutex<()>) {
    let _guard = recover_mutex_guard(mutex);
    assert!(mutex.is_poisoned(), "intentional poison");
}

fn spawn_plantuml_render_lock_probe(
    active: Arc<AtomicUsize>,
    max_active: Arc<AtomicUsize>,
    start: Arc<Barrier>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        start.wait();
        with_krr_plantuml_render_lock(|| {
            let now = active.fetch_add(1, Ordering::SeqCst) + 1;
            max_active.fetch_max(now, Ordering::SeqCst);
            thread::sleep(Duration::from_millis(20));
            active.fetch_sub(1, Ordering::SeqCst);
        });
    })
}

fn join_plantuml_render_lock_probe(handle: thread::JoinHandle<()>) -> Result<(), String> {
    match handle.join() {
        Ok(()) => Ok(()),
        Err(_) => Err("plantuml render lock probe panicked".to_string()),
    }
}
