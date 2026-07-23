use candidate_task::{Job, Scheduler, SubmitError};
#[test]
fn shutdown_drains_a_bounded_fifo_prefix() {
    let mut scheduler = Scheduler::new(4);
    for id in 1..=3 { scheduler.submit(Job { id }).unwrap(); }
    let summary = scheduler.shutdown_and_drain(2);
    assert_eq!(summary.completed.iter().map(|job| job.id).collect::<Vec<_>>(), vec![1, 2]);
    assert_eq!(summary.remaining, 1);
    assert_eq!(scheduler.submit(Job { id: 4 }), Err(SubmitError::ShuttingDown));
}
