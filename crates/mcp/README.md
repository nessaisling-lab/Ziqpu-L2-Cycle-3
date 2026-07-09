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
| `recommend` | OBSERVE + DECIDE: ranked synastry fit reads for a profile, then proposes grounding |
| `pull_grounded_signals` | The **checkpoint** — without `approved:true` it returns `PENDING_APPROVAL` and fetches nothing; with it, the gated SEC EDGAR pull runs |

The approval checkpoint (PRD §18) surfaces at the MCP boundary: the host must re-call with
`approved:true` before anything external happens. The no-advice guardrail and honesty rules carry
through the loop unchanged.

## Run

```bash
cargo run -p mcp          # speaks JSON-RPC on stdin/stdout
ZIQPU_LIVE=1 cargo run -p mcp   # real SEC EDGAR for the grounded pull (needs curl + network)
```

Register with an MCP host (e.g. Claude Desktop `claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "ziqpu": { "command": "/path/to/target/release/ziqpu-mcp" }
  }
}
```

Then, in the host: `make_profile` → `recommend` → (on a pick) `pull_grounded_signals`. Set
`ANTHROPIC_API_KEY` to have Ungasaga speak as Claude; otherwise the deterministic reader is used.
