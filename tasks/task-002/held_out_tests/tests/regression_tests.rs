use candidate_task::{Job, Scheduler, SubmitError};
#[test]
fn existing_bounded_queue_contract_remains() {
    let mut scheduler = Scheduler::new(1); scheduler.submit(Job { id: 1 }).unwrap();
    assert_eq!(scheduler.submit(Job { id: 2 }), Err(SubmitError::Full));
    assert_eq!(scheduler.take().unwrap().id, 1);
}
