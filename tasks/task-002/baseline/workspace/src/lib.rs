#![forbid(unsafe_code)]
use std::collections::VecDeque;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Job { pub id: u64 }
#[derive(Debug, Eq, PartialEq)]
pub enum SubmitError { Full, ShuttingDown }
pub struct Scheduler { capacity: usize, queue: VecDeque<Job>, accepting: bool }
impl Scheduler {
    pub fn new(capacity: usize) -> Self { Self { capacity, queue: VecDeque::new(), accepting: true } }
    pub fn submit(&mut self, job: Job) -> Result<(), SubmitError> {
        if !self.accepting { return Err(SubmitError::ShuttingDown); }
        if self.queue.len() == self.capacity { return Err(SubmitError::Full); }
        self.queue.push_back(job); Ok(())
    }
    pub fn take(&mut self) -> Option<Job> { self.queue.pop_front() }
    pub fn stop(&mut self) { self.accepting = false; self.queue.clear(); }
}
