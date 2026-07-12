## What & why

<!-- What does this PR change, and why? -->

## Checklist

- [ ] Commits are signed off (`git commit -s`) — DCO
- [ ] `cargo fmt --all -- --check` clean
- [ ] `cargo clippy --workspace --all-features -- -D warnings` clean
- [ ] `cargo test --workspace --all-features` green
- [ ] `cargo deny check` green (advisories + licenses/bans/sources; no AGPL/GPL added)
- [ ] No secrets added; `.env` not committed
- [ ] CI is green on macOS + Windows + Linux

## Base branch

<!-- Contributions target `nightfall` (see CONTRIBUTING.md). A `nightfall → main` promotion is
     merged with a merge commit, not squash (see RELEASING.md). -->
