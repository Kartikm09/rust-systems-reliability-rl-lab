use criterion::{Criterion, criterion_group, criterion_main};
use protocol::Job;
use scheduler::Scheduler;

fn scheduler_round_trip(criterion: &mut Criterion) {
    criterion.bench_function("scheduler_round_trip", |bencher| {
        bencher.iter(|| {
            let scheduler = Scheduler::new(32);
            for id in 0..32 {
                scheduler
                    .submit(Job {
                        id,
                        attempts: 0,
                        payload: "payload".to_owned(),
                    })
                    .unwrap();
            }
            while scheduler.try_take().is_some() {}
        });
    });
}
criterion_group!(benches, scheduler_round_trip);
criterion_main!(benches);
