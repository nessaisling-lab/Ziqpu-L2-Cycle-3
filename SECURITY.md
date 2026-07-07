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

## Note on imported material

Source archives used to seed this project contained committed credentials. Those keys were
treated as compromised and rotated, and are **not** present in this repository. Do not
re-import any `.env` from those archives.
