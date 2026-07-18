# Ziqpu key proxy

The server-side holder of Ziqpu's **shared** Anthropic key, so the desktop app can offer live
readings out of the box **without shipping the key**.

## Why

A key baked into a distributed desktop binary is extractable by anyone who has the binary — no amount
of obfuscation changes that. So the Ziqpu app **never contains the Anthropic key**. It calls this tiny
Cloudflare Worker with a low-value, revocable **app token**; the Worker holds the real
`ANTHROPIC_API_KEY` as an encrypted platform secret and forwards the reading request to Anthropic.

Reverse-engineer the client completely and the Anthropic key still never leaks. The worst an attacker
gets is the app token — which is rate-limited, spend-capped, and revocable with one command.

> Users who bring **their own** key don't touch this proxy at all — their key goes straight into their
> OS keychain (`crates/ui/src/vault.rs`) and the app calls Anthropic/OpenRouter directly. The proxy is
> only the *default, no-setup* path.

## Security controls (all enforced in `src/worker.js`)

| # | Control | How |
|---|---------|-----|
| 1 | Secret key | `ANTHROPIC_API_KEY` is a Worker secret — never in code, repo, or the app binary |
| 2 | TLS | Cloudflare terminates HTTPS; upstream to Anthropic is HTTPS |
| 3 | App-token gate | timing-safe compare of `Authorization: Bearer <APP_TOKEN>` |
| 4 | Rate limiting | per-IP daily request cap (KV counter) |
| 5 | Spend cap | monthly token budget (KV counter); refuse once exceeded |
| 6 | Input limits | model allowlist, `max_tokens` clamp, 64 KB body cap, POST + `/v1/messages` only |
| 7 | Privacy | request/response **bodies are never logged** (the birth chart is sensitive) |
| 8 | Kill switch | set `ENABLED="false"` → the free tier is off instantly |
| 9 | Rotation | rotate the key with one command; the app references the URL, not the key |

The KV counters are approximate (KV isn't atomic), so treat the spend cap as a *soft* limit. The
**hard** money backstop is a **budget limit on the Anthropic account itself** — set one; see below.

## Deploy (one time)

Prerequisites: a Cloudflare account (free tier is enough) and Node.js.

```bash
cd proxy
npm install                       # installs wrangler locally
npx wrangler login                # opens a browser to authorize your Cloudflare account

# 1. Create the KV namespace for the rate-limit + spend counters, then paste the printed id
#    into wrangler.toml (the id = line under [[kv_namespaces]]).
npx wrangler kv namespace create ZIQPU_RL

# 2. Set the two secrets (encrypted at rest; never written to the repo).
npx wrangler secret put ANTHROPIC_API_KEY   # paste your real Anthropic key when prompted
npx wrangler secret put APP_TOKEN           # paste a long random string (see below) — this is what the app sends

# 3. Deploy. Prints your Worker URL, e.g. https://ziqpu-key-proxy.<subdomain>.workers.dev
npx wrangler deploy
```

Generate a strong `APP_TOKEN` (any long random string works):

```bash
openssl rand -hex 32
```

### Then wire the app to it

The app reads two values (build-time constants / env — see `crates/agents` proxy interpreter):

- `ZIQPU_PROXY_URL` = the Worker URL + `/v1/messages` (e.g. `https://ziqpu-key-proxy.<sub>.workers.dev/v1/messages`)
- `ZIQPU_PROXY_TOKEN` = the same `APP_TOKEN` you set above

Set those for the build that ships the built-in free tier. Leave them empty and the app simply omits
the built-in option (users bring their own key). **Never put the Anthropic key here — only the URL and
the app token.**

## Set the hard money backstop (do this)

In the Anthropic Console, set a **monthly spend limit / budget** on the account whose key this proxy
holds. The in-proxy `MONTHLY_TOKEN_BUDGET` is a soft cap; the account budget is the guarantee that a
leaked app token can never run up an unbounded bill.

## Rotate / revoke

- **Rotate the Anthropic key** (e.g. suspected exposure): create a new key in the Anthropic Console,
  `npx wrangler secret put ANTHROPIC_API_KEY` with the new value, then revoke the old key. **No app
  update needed** — the app never had the key.
- **Rotate the app token** (e.g. abuse): `npx wrangler secret put APP_TOKEN` with a new value and ship
  a build carrying the new `ZIQPU_PROXY_TOKEN`. Old builds get `401` until updated.
- **Emergency off:** set `ENABLED="false"` in `wrangler.toml` and `npx wrangler deploy` (or toggle the
  var in the Cloudflare dashboard for an instant effect).

## Local dev

```bash
cd proxy
cp .dev.vars.example .dev.vars    # then fill in real values locally (this file is gitignored)
npx wrangler dev                  # runs the Worker at http://localhost:8787
```

## Tuning (`wrangler.toml` [vars])

| var | default | meaning |
|-----|---------|---------|
| `ENABLED` | `true` | kill switch |
| `ALLOWED_MODELS` | `claude-opus-4-8` | comma-separated model allowlist |
| `MAX_TOKENS_CAP` | `1536` | per-request output ceiling |
| `PER_CLIENT_DAILY_REQUESTS` | `50` | per-IP daily request cap |
| `MONTHLY_TOKEN_BUDGET` | `2000000` | soft monthly token spend cap |

## Alternatives

The app talks to the proxy over plain HTTPS with a bearer token, so the host is swappable. If you'd
rather stay all-Rust (matching the N4 roadmap), the same contract can be served by an `axum` service on
Fly.io / Render — port `src/worker.js`'s logic and point `ZIQPU_PROXY_URL` at it. Cloudflare Workers is
the recommended default only for its zero-ops, always-on, free-tier footprint.
