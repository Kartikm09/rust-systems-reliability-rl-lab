#![forbid(unsafe_code)]

//! Shared protocol and job value objects.

/// A decoded protocol frame.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Frame {
    /// Deterministic event identifier.
    pub id: u64,
    /// UTF-8 payload.
    pub payload: String,
}

/// A schedulable job.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Job {
    /// Stable job identifier.
    pub id: u64,
    /// Number of retries already attempted.
    pub attempts: u8,
    /// Job payload.
    pub payload: String,
}
