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

Ziqpu ships on **two tracks**: `main` is the protected, all-green **stable** line, and **`nightfall`**
is the integration branch where in-progress work soaks before being promoted to `main`. Contributions
target **`nightfall`** — see [RELEASING.md](RELEASING.md) for the full track model.

1. Branch off **`nightfall`**. Name your branch **`feat/<name>`** or **`<yourfirstname>/<topic>`**
   (e.g. `nathan/airlines-v1`). **You cannot push to `main`** — it is protected. The owner pushes
   build-ahead work to `nightfall` directly; contributors open PRs into it.
2. Keep diffs small and reviewable.
3. Before pushing, run the local gates:
   ```bash
   cargo fmt --all -- --check
   cargo clippy --workspace --all-features -- -D warnings
   cargo test --workspace --all-features --locked
   cargo deny check          # advisories + licenses + bans + sources
   ```
4. Open a pull request **into `nightfall`**. **All CI jobs must be green** — `test`, `stability`,
   `smoke`, `security`, `desktop`, `integration`, and `anise cross-check` across macOS/Windows/Linux,
   plus `DCO` — and **the repo owner approves and merges.**

## Datasets

Contributing domain data (not code)? See [datasets/README.md](datasets/README.md) for the shared
"birth moment" schema and per-domain folders. Same flow: your `<firstname>/<topic>` branch → PR → owner approves.

## Dependencies

The public tree is **permissive-only**. Do not add AGPL/GPL/SSPL dependencies — `cargo deny`
will reject them. MIT/Apache-2.0/BSD/ISC/MPL-2.0 are fine.

## Report what you verified

State what you *verified* vs. what you *assumed*. Never claim a check passed without running it.
