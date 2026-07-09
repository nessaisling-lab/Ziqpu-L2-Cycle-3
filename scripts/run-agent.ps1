# Ziqpu — run the agent with Hamun-ana pointed at the local LLM hub.
#
# Single source of truth for the hub: $env:LOCAL_LLM_BASE_URL (an OpenAI-compatible /v1 base URL).
# Default = LM Studio (http://localhost:1234/v1). To move the whole stack to Ollama, set:
#   $env:LOCAL_LLM_BASE_URL = 'http://localhost:11434/v1'   # Ollama speaks the same /v1 API
#   $env:LOCAL_LLM_MODEL    = '<an ollama tag, e.g. qwen2.5:3b>'
# — no code changes; only the URL + model name move. (Ungasaga stays Claude via ANTHROPIC_API_KEY,
# or the deterministic template if unset; the hub only drives Hamun-ana, the local measurer.)
$ErrorActionPreference = 'Stop'

$base  = if ($env:LOCAL_LLM_BASE_URL) { $env:LOCAL_LLM_BASE_URL } else { 'http://localhost:1234/v1' }
$model = if ($env:LOCAL_LLM_MODEL)    { $env:LOCAL_LLM_MODEL }    else { 'gemma-4-e4b-it' }

$env:ZIQPU_LOCAL_LLM    = '1'
$env:ZIQPU_LLM_PROVIDER = 'openai'   # OpenAI-compatible — works for BOTH LM Studio and Ollama
$env:ZIQPU_LLM_URL      = $base
$env:ZIQPU_LLM_MODEL    = $model

Write-Host "[hub] Hamun-ana -> $base  (model: $model)"

$repo = Split-Path $PSScriptRoot -Parent
$exe  = @("$repo\target\debug\ziqpu-agent.exe", "$repo\target\release\ziqpu-agent.exe") |
        Where-Object { Test-Path $_ } | Select-Object -First 1
if (-not $exe) { Write-Host "build first:  cargo build -p agents --bin ziqpu-agent"; exit 1 }

& $exe @args
exit $LASTEXITCODE
