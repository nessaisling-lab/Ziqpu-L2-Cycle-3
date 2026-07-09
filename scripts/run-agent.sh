#!/usr/bin/env bash
# Ziqpu — run the agent with Hamun-ana pointed at the local LLM hub.
#
# Single source of truth for the hub: LOCAL_LLM_BASE_URL (an OpenAI-compatible /v1 base URL).
# Default = LM Studio (http://localhost:1234/v1). To move the whole stack to Ollama, set:
#   LOCAL_LLM_BASE_URL=http://localhost:11434/v1   # Ollama speaks the same /v1 API
#   LOCAL_LLM_MODEL=<an ollama tag, e.g. qwen2.5:3b>
# — no code changes; only the URL + model name move. (Ungasaga stays Claude via ANTHROPIC_API_KEY,
# or the deterministic template if unset; the hub only drives Hamun-ana, the local measurer.)
set -euo pipefail

base="${LOCAL_LLM_BASE_URL:-http://localhost:1234/v1}"
model="${LOCAL_LLM_MODEL:-gemma-4-e4b-it}"

export ZIQPU_LOCAL_LLM=1
export ZIQPU_LLM_PROVIDER=openai   # OpenAI-compatible — works for BOTH LM Studio and Ollama
export ZIQPU_LLM_URL="$base"
export ZIQPU_LLM_MODEL="$model"

echo "[hub] Hamun-ana -> $base  (model: $model)"

repo="$(cd "$(dirname "$0")/.." && pwd)"
exe="$repo/target/debug/ziqpu-agent"
[ -x "$exe" ] || exe="$repo/target/release/ziqpu-agent"
[ -x "$exe" ] || exe="$exe.exe"                       # Windows/Git-Bash
[ -x "$exe" ] || { echo "build first:  cargo build -p agents --bin ziqpu-agent"; exit 1; }

exec "$exe" "$@"
