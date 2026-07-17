<!--
Prepended to every GitHub Release body by .github/workflows/release.yml.

This file exists because the workflow used to publish with `--generate-notes` alone, which emits an
auto-generated commit list and nothing else — while a comment in the workflow claimed the Gatekeeper
workaround was "documented in the release notes". It wasn't, anywhere a downloader would look. A
first-launch prerequisite that only the maintainer knows is a broken download to everyone else.

Keep it short, keep it true, and keep every "known limitation" honest — someone is reading this
because the app didn't start.
-->

## Install

Ziqpu is a single application. No account, no database, no Docker. Your chart and your keys stay on
your machine.

### Windows — `Ziqpu-<version>-windows-x64.zip`

Unzip anywhere and run `ziqpu-ui.exe`.

- **Needs the Microsoft Edge WebView2 Runtime — and Ziqpu handles this itself.** Windows 11 has it
  built in, and so do the vast majority of Windows 10 machines (it can be missing on a clean
  Windows 10 install, Windows Server, or LTSC editions). If it's missing, Ziqpu offers at startup
  to download Microsoft's official installer (~2 MB) and set it up — per-user, no administrator
  rights, about a minute. Prefer to do it yourself? Install from
  <https://developer.microsoft.com/microsoft-edge/webview2/> and start Ziqpu again.
- **SmartScreen** will warn ("Windows protected your PC") because the executable is not
  code-signed. *More info → Run anyway.*

### macOS — `Ziqpu-<version>-macos-universal.dmg`

Open the `.dmg` and drag Ziqpu out.

- **Universal binary** — runs natively on both Apple Silicon (arm64) and Intel (x86_64) Macs,
  macOS 11 Big Sur or newer.
- **Unsigned and un-notarized**, so Gatekeeper blocks the first launch — often claiming the app is
  "damaged", which it is not. **Right-click the app → Open**, then confirm. If macOS still refuses:
  `xattr -dr com.apple.quarantine /path/to/Ziqpu.app`

### Linux — `Ziqpu-<version>-linux-x64.tar.gz`

`tar -xzf` and run `./ziqpu-ui`.

- **Needs WebKitGTK at runtime.** If it exits with a missing-library error, install it:
  - Debian / Ubuntu — `sudo apt install libwebkit2gtk-4.1-0 libgtk-3-0`
  - Fedora — `sudo dnf install webkit2gtk4.1 gtk3`
  - Arch — `sudo pacman -S webkit2gtk-4.1 gtk3`
- A desktop session is required; there is no headless mode.

## What you get with nothing set up

Ziqpu runs **offline, out of the box** — the chart math and the company data are compiled into the
binary. With no API key and no local model you get **Raw**: real charts, real synastry, deterministic
readings written from templates.

For written readings from a model, three roads:

- **Built-in free tier** — this build ships one. Pick "Use Ziqpu built-in (free)" and readings run
  through Ziqpu's own key proxy: no key of yours, no account. It is rate-limited and can be paused;
  when it's over its budget the app says so honestly instead of failing quietly.
- **Your own key** (Anthropic or OpenRouter) — stored in your OS keychain, never in a file, never
  shown back to you.
- **A local model** — Ziqpu benchmarks your machine, installs the right llama.cpp build for your
  GPU **automatically** (one-time, per-user, no admin rights), and offers only models your machine
  can actually run. Below the minimum floor it tells you plainly and installs nothing.

## Known limitations in this build

- **Unsigned binaries.** SmartScreen (Windows) and Gatekeeper (macOS) warn on first launch — the
  workarounds above are the whole fix; nothing is wrong with the download.
- Reading data comes from **SEC EDGAR filings and Wikipedia**, only after you approve it at the
  checkpoint.

**Ziqpu is reflection, not advice** — measured, not fate. Not financial advice.
