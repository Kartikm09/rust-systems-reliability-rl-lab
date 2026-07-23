use service::Engine;
use std::sync::Arc;
use storage::{JobStore, MemoryStore};

#[test]
fn invalid_frame_has_no_persistence_side_effect() {
    let scheduler = Arc::new(scheduler::Scheduler::new(1));
    let store = Arc::new(MemoryStore::default());
    let engine = Engine::new(scheduler, Arc::clone(&store));
    assert!(engine.ingest(b"1:9:x").is_err());
    assert!(store.get(1).is_none());
}
