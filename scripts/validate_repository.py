from __future__ import annotations
import json
from pathlib import Path
root = Path(__file__).resolve().parents[1]
tasks = sorted((root / "tasks").glob("task-*"))
if len(tasks) != 4: raise SystemExit(f"expected four tasks, found {len(tasks)}")
required = ["README.md", "LICENSE", "Cargo.toml", "Dockerfile", "evaluator", ".github/workflows/ci.yml"]
missing = [path for path in required if not (root / path).exists()]
for task in tasks:
    for path in ["task.yaml", "evaluator_config.json", "golden/solution.patch", "public_tests", "held_out_tests"]:
        if not (task / path).exists(): missing.append(str((task / path).relative_to(root)))
    json.loads((task / "evaluator_config.json").read_text(encoding="utf-8"))
if missing: raise SystemExit("missing: " + ", ".join(missing))
print("repository structure validation passed")
