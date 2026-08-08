# `mcp` — Ziqpu over the Model Context Protocol

Phase 1b portability: drive the two-vizier loop from **any MCP host** (Claude Desktop, an IDE), so
the agent runs outside the Ziqpu app and **travels with the user** via a portable profile.

MCP's stdio transport is newline-delimited JSON-RPC 2.0, so this is a small hand-rolled server over
`serde_json` — no extra dependency, no pre-1.0 SDK. The request handler ([`handle`](src/lib.rs)) is
pure, so it's unit-tested without any stdio.

## Tools

| Tool | What it does |
|---|---|
| `make_profile` | Build a portable birth profile (birth data only) — the thing the agent travels with |
| `chart` | The natal chart of a seeded choice (real ephemeris) — `AAPL MSFT TSLA KO JNJ` |
| `recommend` | OBSERVE + DECIDE: ranked synastry fit reads for a profile. **Costs money** — one hosted-model call per choice, on your key. Pauses first |
| `pull_grounded_signals` | Reaches external public sources (SEC EDGAR, SEC XBRL, Wikidata, Wikipedia, FDA, NHTSA vPIC). Keyless, but discloses the identifier. Pauses first |

Both costed tools return `PENDING_APPROVAL` first, naming exactly what the call would spend, and do
nothing until re-called with `acknowledged: true`. (`approved` is accepted as a deprecated alias so
existing host configs keep working.) The no-advice guardrail and honesty rules carry through the
loop unchanged.

### What this server cannot do

**It cannot verify that a human approved anything.** The `acknowledged` flag is set by the calling
model — the same model that reads the `PENDING_APPROVAL` response. A one-time token would not fix
that: the model can read the token out of the response and hand it straight back. That is the model
shaking hands with itself, and shipping it would be worse than the gap, because it would *look* like
verification.

**On this surface, your MCP host's own approval prompt is the gate.** Ziqpu's part is to make sure
that prompt has what a person needs to decide:

- the consent sentence — naming every source a call would spend — lives in the tool **description**,
  which is what hosts show in the approval dialog;
- both costed tools declare `openWorldHint: true` and `readOnlyHint: false`, which hosts use to
  decide when to ask;
- `PENDING_APPROVAL` forces a second round trip, so there is a moment for the host to ask at all.

**If your host has auto-approve enabled for this server, there is no gate.** We cannot detect that
from here. If that matters for your use, run the desktop app instead — its checkpoint is a real
button pressed by a real person.

The protocol's `elicitation` capability *would* close this properly: it lets a server ask the host to
put a question directly to the human, out of band from the model. We detect whether your host
supports it. Using it needs a bidirectional transport this server does not yet have, so it is
declared, measured, and not yet claimed. That work is tracked rather than pretended.

## Run

```bash
cargo run -p mcp          # speaks JSON-RPC on stdin/stdout
ZIQPU_LIVE=1 cargo run -p mcp   # real sources AND the real hosted model — this one costs money
```

Register with an MCP host (e.g. Claude Desktop `claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "ziqpu": { "command": "/path/to/target/release/ziqpu-mcp" }
  }
}
```

Then, in the host: `make_profile` → `recommend` → (on a pick) `pull_grounded_signals`. To have
Ungasaga speak as a live model, set an interpreter key — `OPENROUTER_API_KEY` / `OPENAI_API_KEY`
(tried first) or `ANTHROPIC_API_KEY` (Claude); with none set, the deterministic reader is used.
