# `model` — Ziqpu's local-model brain

Decides **which** open-weight model fits a machine, then **fetches + serves** it via `llama.cpp` on an
OpenAI-compatible port — so a **Local** reading runs on the user's own GPU with no CLI and no cloud.
Powers the UI's in-app model panel; also a standalone `ziqpu-model` binary.

## What it does

- **Two-layer benchmark.** (1) *Device → tier*: `detect_spec` (RAM via `sysinfo`, cores, GPU/VRAM) →
  `recommend_for` returns `Local(ModelPick)` or `NoLocal { reason, fallback }` against a capability floor
  (8 GB / 4 cores / AVX2 on x86; Apple/ARM never AVX2-gated). (2) *Online best-GGUF*: pick the largest
  **quant** that fits the VRAM budget from the repo's real GGUFs, degrading to a static pick offline.
- **Runtime resolution.** `resolve_llama_server` prefers a managed CUDA build, then PATH/winget;
  `Backend`/`select_device`/`gpu_serve_args` pin a **discrete** GPU (`-ngl 99 -dev <CUDA0/Vulkan1/…>`);
  `SERVE_CTX_SIZE` caps the KV cache so weights+context fit in VRAM.
- **Serve, not re-serve.** `running_server_port` reconnects to a healthy server (`/health`=ok) instead
  of reloading; `model_cached` detects an already-downloaded model; serve is **single-active** (stops a
  prior llama-server first, or they stack and hang the machine).

## Use

```bash
cargo run -p model -- benchmark      # tier + GPU/RAM + the resolved `llama-server -hf <repo>:<quant>`
cargo run -p model -- plan           # dry-run the serve pick (device + quant), no download
cargo run -p model -- get            # resolve + check the llama.cpp runtime + print next steps
cargo run -p model -- serve          # download (first run) + serve OpenAI-compatible on :1234
cargo run -p model -- resolve <term> # list raw HF GGUF candidates for a search term
```

The served port is the app's **Local** default, so serving is zero-config for a reading.

## Gotchas (don't re-learn these)

- **GPU backend.** The winget `llama.cpp` is a **Vulkan** build; on a hybrid laptop it can grab the
  *integrated* GPU and OOM. `select_device` prefers a discrete CUDA/Metal/ROCm device — and a discrete
  GPU even within Vulkan, by name. CUDA-on-NVIDIA is faster than Vulkan-on-NVIDIA.
- **Real OOM cause was context, not weights** — an uncapped serve context loads the model's full
  trained window (32k+) as KV. `SERVE_CTX_SIZE` caps it; the VRAM weight budget is `0.80`.
- **Quant ≠ file size.** MoE/native-quant repos have similar-sized files across quants; rank by quant
  quality (`quant_rank`: Q8 > Q6 > Q5 > Q4 …), tie-break smaller.
- Model weights are **never bundled or redistributed** — recommended + fetched by the user's own runtime
  at download time, under each model's own license (see the repo `NOTICE`).

## Status

Live on the dev machine: **Qwen3-14B** (Medium/16 GB tier, Q6_K) serves on the discrete NVIDIA GPU, no
OOM. **Next:** `ensure_runtime` — the app auto-downloads the right per-vendor `llama.cpp` build itself
(NVIDIA → CUDA, Apple → Metal, else Vulkan/CPU) into an app-created dir, so partners with no `llama.cpp`
installed get a working Local path cross-platform.
