# Structured domain-error refactor

Replace stringly typed errors with a domain enum while preserving messages and CLI exit codes.

Submit a unified diff against the candidate workspace. The patch is evaluated
with public tests first and held-out correctness and regression tests only in an
internal copy. Include `CANDIDATE_NOTES.md` with the invariant, compatibility
impact, and commands used.
