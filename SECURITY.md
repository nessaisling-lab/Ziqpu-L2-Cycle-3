# Security Policy

## Reporting a vulnerability

Please report security issues privately to the maintainer rather than opening a public
issue. Use GitHub's **Report a vulnerability** (Security → Advisories) on this repository,
or email the maintainer. You'll get an acknowledgement within a few days.

## Secret hygiene (project rules)

- **No secrets in the repo, ever.** `.env` is gitignored; only `.env.example` is tracked.
- `ANTHROPIC_API_KEY` is **server-side only** — it must never appear in a client/WASM bundle.
- Real data (`DATABASE_URL`) lives in the private Nisaba Postgres and in GitHub Actions
  **Secrets**; public CI runs on fixtures and never sees it.
- CI enforces this: the `security` job runs `gitleaks` (secret scan), `cargo audit`
  (advisories), and `cargo deny` (license/ban policy). GitHub secret scanning + push
  protection are enabled on the repository.

## Accepted advisories

- **RUSTSEC-2023-0071** (`rsa` "Marvin" timing attack) — pulled transitively by `sqlx-postgres`;
  no fixed `rsa` release exists. Not exploitable here: the sidecar authenticates to a local/private
  Postgres via SCRAM/md5 and never performs RSA key operations. Scoped-ignored in `.cargo/audit.toml`
  so the audit still fails on any *other* advisory. To be removed when a patched `rsa` ships.

## Agent-to-agent handoff channel (defense-in-depth)

When a reading is grounded, the **local** model drafts a compact handoff brief that the **live**
model consumes (see the layered pipeline). The plan is to encode that brief as **structured English
with optional prompt compression** (a terse, domain-specific schema — the "compact handoff
encoding"). This carries two *defense-in-depth* security properties, both real but neither a
guarantee:

- **Channel obfuscation, not encryption.** A compressed, telegraphic handoff is not plain prose, so
  casual inspection or naive scraping yields less. It is **not** cryptographically protected — any
  capable model can decode it — so treat it as obscurity/hardening, never as confidentiality.
- **Reduced prompt-injection surface.** The handoff is terse, structured, and every token is
  load-bearing, so foreign natural-language content tends to be conspicuous, break parsing, or be
  dropped by compression rather than blend in and hijack the instruction. This *complements* — it
  does not replace — the primary defenses: a choice's name/notes and any fetched text are treated as
  **data, not instructions**, and no costed external action runs without a human **approval token**
  (see [`crates/agents/README.md`](crates/agents/README.md) → *Responsibility · blast radius · prompt
  injection*).

The untrusted-input handling and the approval gate remain the load-bearing controls; the compact
handoff is an additional layer, not a substitute.

## Note on imported material

Source archives used to seed this project contained committed credentials. Those keys were
treated as compromised and rotated, and are **not** present in this repository. Do not
re-import any `.env` from those archives.
