use candidate_task::{Job, Scheduler};
#[test]
fn payload_ownership_preserves_values() {
    let mut scheduler = Scheduler::new(); scheduler.submit(Job { id: 9, payload: "payload".into() }); assert_eq!(scheduler.take().unwrap().payload, "payload");
}
