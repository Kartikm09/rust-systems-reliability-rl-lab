#![forbid(unsafe_code)]

//! Deterministic persistence abstraction.

use protocol::Job;
use std::collections::BTreeMap;
use std::sync::Mutex;

/// Persistence port for accepted and dead-letter jobs.
pub trait JobStore: Send + Sync {
    /// Persist an accepted job.
    fn save(&self, job: Job);
    /// Retrieve a job by identifier.
    fn get(&self, id: u64) -> Option<Job>;
}

/// In-memory deterministic implementation.
#[derive(Debug, Default)]
pub struct MemoryStore {
    jobs: Mutex<BTreeMap<u64, Job>>,
}
impl JobStore for MemoryStore {
    fn save(&self, job: Job) {
        self.jobs
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(job.id, job);
    }
    fn get(&self, id: u64) -> Option<Job> {
        self.jobs
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(&id)
            .cloned()
    }
}
