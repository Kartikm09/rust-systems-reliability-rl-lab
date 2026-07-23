#![forbid(unsafe_code)]

//! Event ingestion service composition.

use parser::{ParseError, parse_frame};
use protocol::Job;
use scheduler::{Scheduler, SubmitError};
use std::sync::Arc;
use storage::JobStore;

/// Service errors preserve parser and scheduler categories.
#[derive(Debug, Eq, PartialEq)]
pub enum ServiceError {
    Parse(ParseError),
    Submit(SubmitError),
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(error) => write!(formatter, "parse error: {error}"),
            Self::Submit(error) => write!(formatter, "submit error: {error}"),
        }
    }
}
impl std::error::Error for ServiceError {}

/// Coordinates parsing, persistence, and scheduling.
pub struct Engine<S: JobStore> {
    scheduler: Arc<Scheduler>,
    store: Arc<S>,
}
impl<S: JobStore> Engine<S> {
    #[must_use]
    pub fn new(scheduler: Arc<Scheduler>, store: Arc<S>) -> Self {
        Self { scheduler, store }
    }
    pub fn ingest(&self, input: &[u8]) -> Result<u64, ServiceError> {
        let frame = parse_frame(input).map_err(ServiceError::Parse)?;
        let job = Job {
            id: frame.id,
            attempts: 0,
            payload: frame.payload,
        };
        self.scheduler
            .submit(job.clone())
            .map_err(ServiceError::Submit)?;
        self.store.save(job);
        Ok(frame.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use storage::MemoryStore;
    #[test]
    fn ingestion_persists_and_schedules() {
        let scheduler = Arc::new(Scheduler::new(2));
        let store = Arc::new(MemoryStore::default());
        let engine = Engine::new(Arc::clone(&scheduler), Arc::clone(&store));
        assert_eq!(engine.ingest(b"7:3:job"), Ok(7));
        assert_eq!(scheduler.try_take().unwrap().id, 7);
        assert_eq!(store.get(7).unwrap().payload, "job");
    }
}
