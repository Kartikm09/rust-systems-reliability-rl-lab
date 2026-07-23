#![forbid(unsafe_code)]

//! Bounded FIFO scheduler with explicit shutdown accounting.

use protocol::Job;
use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use thiserror::Error;

/// Submission failures.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum SubmitError {
    /// Queue reached its configured capacity.
    #[error("scheduler queue is full")]
    Full,
    /// Scheduler no longer accepts work.
    #[error("scheduler is shutting down")]
    ShuttingDown,
}

#[derive(Debug)]
struct State {
    queue: VecDeque<Job>,
    accepting: bool,
}

/// Thread-safe bounded scheduler.
#[derive(Debug)]
pub struct Scheduler {
    capacity: usize,
    state: Mutex<State>,
    available: Condvar,
}

impl Scheduler {
    /// Create a scheduler with a positive capacity.
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be positive");
        Self {
            capacity,
            state: Mutex::new(State {
                queue: VecDeque::new(),
                accepting: true,
            }),
            available: Condvar::new(),
        }
    }

    /// Submit a job without blocking.
    pub fn submit(&self, job: Job) -> Result<(), SubmitError> {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if !state.accepting {
            return Err(SubmitError::ShuttingDown);
        }
        if state.queue.len() == self.capacity {
            return Err(SubmitError::Full);
        }
        state.queue.push_back(job);
        self.available.notify_one();
        Ok(())
    }

    /// Take one available job.
    pub fn try_take(&self) -> Option<Job> {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .queue
            .pop_front()
    }

    /// Stop accepting jobs and drain at most `completion_limit` jobs.
    pub fn shutdown_and_drain(&self, completion_limit: usize) -> ShutdownSummary {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        state.accepting = false;
        let mut completed = Vec::new();
        while completed.len() < completion_limit {
            let Some(job) = state.queue.pop_front() else {
                break;
            };
            completed.push(job);
        }
        ShutdownSummary {
            completed,
            remaining: state.queue.len(),
        }
    }
}

/// Deterministic shutdown result.
#[derive(Debug, Eq, PartialEq)]
pub struct ShutdownSummary {
    pub completed: Vec<Job>,
    pub remaining: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn queue_is_bounded_and_fifo() {
        let scheduler = Scheduler::new(1);
        scheduler
            .submit(Job {
                id: 1,
                attempts: 0,
                payload: "a".into(),
            })
            .unwrap();
        assert_eq!(
            scheduler.submit(Job {
                id: 2,
                attempts: 0,
                payload: "b".into()
            }),
            Err(SubmitError::Full)
        );
        assert_eq!(scheduler.try_take().unwrap().id, 1);
    }
}
