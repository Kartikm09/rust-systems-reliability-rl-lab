use candidate_task::{Job, Scheduler};
fn main() {
    let mut scheduler = Scheduler::new();
    for id in 0..100 { scheduler.submit(&Job { id, payload: "x".repeat(128) }); }
    while scheduler.take().is_some() {}
    println!(r#"{{"metric":"clone_work","value":{}}}"#, scheduler.clone_work());
}
