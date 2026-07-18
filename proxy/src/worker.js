// Ziqpu key proxy — the server-side holder of the shared Anthropic key.
//
// WHY THIS EXISTS: a key bundled in the distributed desktop app is extractable by anyone with the
// binary. So the app NEVER contains the Anthropic key. Instead it calls this Worker with a low-value,
// revocable APP_TOKEN; the Worker holds the real ANTHROPIC_API_KEY as a platform secret and forwards
// the reading request. The client can be fully reverse-engineered and the Anthropic key still never
// leaks — the worst an attacker gets is the throttled, capped, revocable app token.
//
// SECURITY CONTROLS (all mandatory for the "actual product" posture):
//   1. Secret key      — ANTHROPIC_API_KEY is a Worker secret, never in code/repo/binary.
//   2. TLS             — Cloudflare terminates HTTPS; upstream to Anthropic is HTTPS.
//   3. App-token gate  — timing-safe check of `Authorization: Bearer <APP_TOKEN>`.
//   4. Rate limiting   — per-token/IP daily request cap (KV counter).
//   5. Spend cap       — monthly token budget (KV counter); refuse once exceeded. Anthropic account
//                        budget is the hard backstop behind this soft cap.
//   6. Input limits    — model allowlist, max_tokens clamp, body-size limit, POST + one path only.
//   7. Privacy         — request/response BODIES are never logged (the chart is sensitive).
//   8. Kill switch     — set ENABLED="false" to disable the free tier instantly.
//   9. Rotation        — rotate ANTHROPIC_API_KEY with `wrangler secret put`; the app references this
//                        URL, not the key, so rotation needs zero client changes.
//
// Bindings (see wrangler.toml + README):
//   secrets : ANTHROPIC_API_KEY, APP_TOKEN
//   KV      : ZIQPU_RL   (rate-limit + spend counters)
//   vars    : ENABLED, ALLOWED_MODELS, MAX_TOKENS_CAP, PER_CLIENT_DAILY_REQUESTS, MONTHLY_TOKEN_BUDGET

const ANTHROPIC_URL = "https://api.anthropic.com/v1/messages";
const ANTHROPIC_VERSION = "2023-06-01";
const MAX_BODY_BYTES = 64 * 1024; // a reading request is small; anything bigger is abuse.

export default {
  async fetch(request, env) {
    try {
      return await handle(request, env);
    } catch (_err) {
      // Never leak internals (or anything key-adjacent) in an error body.
      return json({ error: "proxy_error" }, 502);
    }
  },
};

async function handle(request, env) {
  // Kill switch — flip ENABLED to "false" to turn the free tier off immediately.
  if (String(env.ENABLED ?? "true").toLowerCase() === "false") {
    return json({ error: "service_disabled" }, 503);
  }

  // Two routes, nothing else — no open relay surface.
  //   POST /v1/messages : the reading call
  //   GET  /v1/models   : which models this proxy will actually serve (the allowlist)
  const url = new URL(request.url);
  const isMessages = request.method === "POST" && url.pathname === "/v1/messages";
  const isModels = request.method === "GET" && url.pathname === "/v1/models";
  if (!isMessages && !isModels) {
    // Distinguish "wrong verb on a real path" from "no such path", as clients expect.
    const knownPath = url.pathname === "/v1/messages" || url.pathname === "/v1/models";
    return json({ error: knownPath ? "method_not_allowed" : "not_found" }, knownPath ? 405 : 404);
  }

  // --- (3) App-token gate: timing-safe compare against the APP_TOKEN secret. Both routes. ---
  const presented = bearer(request.headers.get("authorization"));
  if (!presented || !env.APP_TOKEN || !timingSafeEqual(presented, env.APP_TOKEN)) {
    return json({ error: "unauthorized" }, 401);
  }

  if (isModels) return await listModels(env);

  // --- (6) Body-size limit before we read/parse. ---
  const raw = await request.text();
  if (raw.length > MAX_BODY_BYTES) return json({ error: "payload_too_large" }, 413);

  let body;
  try {
    body = JSON.parse(raw);
  } catch (_e) {
    return json({ error: "invalid_json" }, 400);
  }

  // --- (6) Model allowlist + max_tokens clamp. Same list GET /v1/models advertises. ---
  const allowed = allowedModels(env);
  if (!allowed.includes(body.model)) {
    return json({ error: "model_not_allowed" }, 400);
  }
  const cap = int(env.MAX_TOKENS_CAP, 1536);
  if (!Number.isFinite(body.max_tokens) || body.max_tokens > cap) {
    body.max_tokens = cap; // clamp rather than reject — the app always wants a reading back.
  }

  // Rate/spend keys: prefer the (shared) app token bucket, but also bound by IP so one leaked token
  // can't be fanned out across machines without each IP hitting its own limit.
  const ip = request.headers.get("cf-connecting-ip") ?? "0.0.0.0";
  const today = utcDate();
  const month = today.slice(0, 7);

  // --- (5) Spend cap: refuse once this month's token budget is already spent. ---
  const budget = int(env.MONTHLY_TOKEN_BUDGET, 2_000_000);
  const spent = int(await env.ZIQPU_RL.get(`spend:${month}`), 0);
  if (spent >= budget) return json({ error: "monthly_budget_exhausted" }, 429);

  // --- (4) Rate limit: per-IP daily request cap. ---
  const perDay = int(env.PER_CLIENT_DAILY_REQUESTS, 50);
  const rlKey = `req:${ip}:${today}`;
  const used = int(await env.ZIQPU_RL.get(rlKey), 0);
  if (used >= perDay) return json({ error: "rate_limited" }, 429);
  // Count the attempt (TTL ~2 days so the key self-cleans). KV isn't atomic, so this is approximate —
  // fine for abuse-throttling; the spend cap + Anthropic account budget are the hard money backstops.
  await env.ZIQPU_RL.put(rlKey, String(used + 1), { expirationTtl: 60 * 60 * 48 });

  // --- (1)(2) Forward to Anthropic with the REAL key injected server-side. ---
  const upstream = await fetch(ANTHROPIC_URL, {
    method: "POST",
    headers: {
      "x-api-key": env.ANTHROPIC_API_KEY,
      "anthropic-version": ANTHROPIC_VERSION,
      "content-type": "application/json",
    },
    body: JSON.stringify(body),
  });

  const text = await upstream.text();

  // --- (5) Meter spend from the response usage (best-effort; never logs the body). ---
  if (upstream.ok) {
    try {
      const usage = JSON.parse(text).usage ?? {};
      const used_tokens = int(usage.input_tokens, 0) + int(usage.output_tokens, 0);
      if (used_tokens > 0) {
        await env.ZIQPU_RL.put(`spend:${month}`, String(spent + used_tokens), {
          expirationTtl: 60 * 60 * 24 * 40, // ~40 days: covers the month, then self-cleans.
        });
      }
    } catch (_e) {
      /* usage parse failure is non-fatal — don't block the reading */
    }
  }

  // Pass the upstream status + JSON straight back; strip everything except content-type.
  return new Response(text, {
    status: upstream.status,
    headers: { "content-type": "application/json" },
  });
}

/**
 * GET /v1/models — what the built-in tier can actually serve.
 *
 * Rather than hardcode names, ask Anthropic for the real catalog (with the server-side key) and
 * return only the entries this proxy's ALLOWED_MODELS permits. The app gets true display names and
 * context windows, and the list can never drift from what `/v1/messages` will accept — both read
 * the same allowlist. The Anthropic key never leaves this Worker.
 */
async function listModels(env) {
  const allowed = allowedModels(env);
  const upstream = await fetch("https://api.anthropic.com/v1/models?limit=100", {
    headers: {
      "x-api-key": env.ANTHROPIC_API_KEY,
      "anthropic-version": ANTHROPIC_VERSION,
    },
  });
  if (!upstream.ok) return json({ error: "upstream_unavailable" }, 502);

  let catalog;
  try {
    catalog = JSON.parse(await upstream.text());
  } catch (_e) {
    return json({ error: "upstream_unreadable" }, 502);
  }
  const data = (catalog.data ?? []).filter((m) => allowed.includes(m.id));
  return json({ data, has_more: false }, 200);
}

// ---- helpers ----

/** The configured model allowlist, parsed once per request. */
function allowedModels(env) {
  return String(env.ALLOWED_MODELS ?? "claude-opus-4-8")
    .split(",")
    .map((s) => s.trim())
    .filter(Boolean);
}

/** Extract the token from an `Authorization: Bearer <token>` header, or null. */
function bearer(header) {
  if (!header) return null;
  const m = /^Bearer\s+(.+)$/i.exec(header.trim());
  return m ? m[1] : null;
}

/** Length-safe, constant-time string compare (avoids leaking the token via response timing). */
function timingSafeEqual(a, b) {
  const ea = new TextEncoder().encode(a);
  const eb = new TextEncoder().encode(b);
  if (ea.length !== eb.length) return false;
  let diff = 0;
  for (let i = 0; i < ea.length; i++) diff |= ea[i] ^ eb[i];
  return diff === 0;
}

function int(v, fallback) {
  const n = parseInt(v, 10);
  return Number.isFinite(n) ? n : fallback;
}

/** Current UTC date as YYYY-MM-DD (rate-limit + spend buckets are UTC-day/month aligned). */
function utcDate() {
  return new Date().toISOString().slice(0, 10);
}

function json(obj, status) {
  return new Response(JSON.stringify(obj), {
    status,
    headers: { "content-type": "application/json" },
  });
}
