#[test]
fn atomic_shutdown_flag_has_consistent_visibility() {
    loom::model(|| {
        use loom::sync::Arc;
        use loom::sync::atomic::{AtomicBool, Ordering};
        use loom::thread;
        let accepting = Arc::new(AtomicBool::new(true));
        let writer = Arc::clone(&accepting);
        let handle = thread::spawn(move || writer.store(false, Ordering::Release));
        handle.join().unwrap();
        assert!(!accepting.load(Ordering::Acquire));
    });
}
