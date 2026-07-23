use candidate_task::{Job, Scheduler};
#[test]
fn zero_and_large_limits_are_accounted() {
    let mut first = Scheduler::new(2); first.submit(Job { id: 1 }).unwrap();
    let zero = first.shutdown_and_drain(0); assert!(zero.completed.is_empty()); assert_eq!(zero.remaining, 1);
    let mut second = Scheduler::new(2); second.submit(Job { id: 1 }).unwrap();
    let all = second.shutdown_and_drain(10); assert_eq!(all.completed.len(), 1); assert_eq!(all.remaining, 0);
}
