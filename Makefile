PYTHON ?= python3
TASK ?= task-001
PATCH ?= tasks/$(TASK)/golden/solution.patch
REPORT ?= reports/evaluations/$(TASK)/manual
.PHONY: setup format lint test benchmark audit evaluator-test evaluate verify-evaluations security-scan verify-all
setup:
	rustc --version
	cargo --version
format:
	cargo fmt --all -- --check
lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings
test:
	cargo test --workspace --all-features
	cargo test --doc --workspace
benchmark:
	cargo bench --workspace
audit:
	cargo audit
	cargo deny check
evaluator-test:
	PYTHONPATH=evaluator/src $(PYTHON) -m unittest discover -s evaluator/tests -v
evaluate:
	PYTHONPATH=evaluator/src $(PYTHON) -m rl_evaluator.cli evaluate --repo-root . --task $(TASK) --patch $(PATCH) --output $(REPORT)
verify-evaluations:
	$(PYTHON) scripts/verify_evaluations.py
security-scan:
	$(PYTHON) scripts/secret_scan.py
verify-all: format lint test evaluator-test verify-evaluations security-scan
	$(PYTHON) scripts/validate_repository.py
