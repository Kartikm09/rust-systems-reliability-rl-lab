set -euo pipefail
cargo run --quiet --bin benchmark_probe > benchmark_result.json
cat benchmark_result.json
