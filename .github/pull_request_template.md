## What & why

<!-- What does this PR change, and which phase / exit-gate does it advance? -->

## Checklist

- [ ] Commits are signed off (`git commit -s`) — DCO
- [ ] `cargo fmt --all -- --check` clean
- [ ] `cargo clippy --workspace --all-features -- -D warnings` clean
- [ ] `cargo test --workspace --all-features` green
- [ ] `cargo deny check licenses bans sources` green (no AGPL/GPL added)
- [ ] No secrets added; `.env` not committed
- [ ] CI is green on macOS + Windows + Linux

## Phase gate

<!-- If this closes a phase, note the exit gate met and the vphase-N tag to cut. -->
