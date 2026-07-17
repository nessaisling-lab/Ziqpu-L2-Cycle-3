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

- **Needs the Microsoft Edge WebView2 Runtime.** Windows 11 has it built in, and so do the vast
  majority of Windows 10 machines. It can be missing on a clean Windows 10 install, on Windows
  Server, or on LTSC editions. If the app does nothing at all when you double-click it, this is why:
  install the **Evergreen Bootstrapper** from
  <https://developer.microsoft.com/microsoft-edge/webview2/> and try again.
- **SmartScreen** will warn ("Windows protected your PC") because the executable is not
  code-signed. *More info → Run anyway.*

### macOS — `Ziqpu-<version>-macos.dmg`

Open the `.dmg` and drag Ziqpu out.

- **Apple Silicon only** (arm64). It will not run on an Intel Mac.
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

To get written readings from a model you can either paste your own API key (Anthropic or OpenRouter
— stored in your OS keychain, never in a file, never shown back to you) or run a model locally.

## Known limitations in this build

- **No built-in free tier.** The key-proxy tier is written and tested but is not configured in
  published builds, so it does not appear. Bring your own key, or use a local model.
- **Local models need llama.cpp installed yourself.** Ziqpu can benchmark your machine and
  recommend a model, but it does not yet install the runtime for you.
- Reading data comes from **SEC EDGAR filings and Wikipedia**, only after you approve it at the
  checkpoint.

**Ziqpu is reflection, not advice** — measured, not fate. Not financial advice.
