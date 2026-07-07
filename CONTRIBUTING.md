# Contributing to Ziqpu

Thanks for helping build Ziqpu. A few rules keep the project healthy and keep the door open
to a future commercial edition.

## Developer Certificate of Origin (DCO) — required

Every commit must be signed off. By signing off you certify the
[Developer Certificate of Origin](https://developercertificate.org/) and agree to the
contribution terms in [CLA.md](CLA.md).

```bash
git commit -s -m "your message"     # appends: Signed-off-by: Your Name <you@example.com>
```

The **DCO** CI check fails any PR whose commits lack a `Signed-off-by` line.

## Workflow

1. Branch off `main` (`feature/…`, `fix/…`).
2. Keep diffs small and reviewable.
3. Before pushing, run the local gates:
   ```bash
   cargo fmt --all -- --check
   cargo clippy --workspace --all-features -- -D warnings
   cargo test --workspace --all-features
   cargo deny check licenses bans sources
   ```
4. Open a PR. **All CI jobs must be green** — `test`, `stability`, `smoke`, `security` on
   macOS/Windows/Linux, plus `DCO` — before merge. This is the phase gate.

## Dependencies

The public tree is **permissive-only**. Do not add AGPL/GPL/SSPL dependencies — `cargo deny`
will reject them. MIT/Apache-2.0/BSD/ISC/MPL-2.0 are fine.

## Report what you verified

State what you *verified* vs. what you *assumed*. Never claim a check passed without running it.
