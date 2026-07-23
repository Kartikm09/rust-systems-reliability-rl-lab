use candidate_task::Scheduler;
#[test]
fn empty_queue_and_metric_are_stable() { let mut scheduler = Scheduler::new(); assert!(scheduler.take().is_none()); assert_eq!(scheduler.clone_work(), 0); }
