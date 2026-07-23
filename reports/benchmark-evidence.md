# Benchmark Evidence

Recorded on 2026-07-23 with Rust 1.97.1 and Criterion 0.8.2 on an Apple M1 host.
These local measurements are reproducibility evidence, not cross-machine guarantees.

## Application Benchmark

Command: `cargo bench --workspace`

```text
scheduler_round_trip  time: [1.5154 us 1.5308 us 1.5460 us]
```

## Task 004 Before And After

| Workspace | `clone_work` | Result |
| --- | ---: | --- |
| Published baseline | 300 | Reproduced with `scripts/run_benchmark.sh` |
| Golden patch | 0 | Accepted; configured threshold is 0 |

The metric counts avoidable payload-clone work and is deterministic. The accepted
evaluation in `reports/evaluations/task-004/solution/result.json` also passed Clippy,
build, public, held-out, regression, and determinism stages.
