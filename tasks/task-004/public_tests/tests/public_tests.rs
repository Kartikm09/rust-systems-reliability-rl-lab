use candidate_task::{Job, Scheduler};
#[test]
fn fifo_and_history_are_preserved() {
    let mut scheduler = Scheduler::new();
    scheduler.submit(Job { id: 1, payload: "a".into() }); scheduler.submit(Job { id: 2, payload: "b".into() });
    assert_eq!(scheduler.take().unwrap().id, 1); assert_eq!(scheduler.take().unwrap().id, 2); assert_eq!(scheduler.history_ids(), vec![1, 2]);
}
