# Acceptance Criteria

- Reject lengths above 4096 before allocation.
- Decode escaped separators correctly.
- Never panic on malformed UTF-8 or truncated input.
- Preserve valid frame behavior.
