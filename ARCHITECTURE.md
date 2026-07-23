# Architecture

`protocol` owns value types; `parser` validates untrusted bytes; `scheduler` owns bounded
queue and shutdown behavior; `storage` defines deterministic persistence; `service`
composes ports; `cli` is the only process boundary. Dependencies point inward and unsafe
code is forbidden workspace-wide.

Task workspaces are independent reduced slices. The Python evaluator builds a public
candidate copy and a separate internal held-out copy. See `DECISIONS.md`.
