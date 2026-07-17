//! Make a failure to start **visible** — and, on Windows, fix the common one before it happens.
//!
//! ## The bug this exists for
//!
//! A release build links as `windows_subsystem = "windows"` (no console — see `main.rs`), which is
//! correct: a GUI app should not flash a terminal. But it means a panic before the window opens has
//! nowhere to print. And there is a very ordinary way to panic before the window opens: Dioxus
//! desktop draws through the system webview, and on Windows that is the **Edge WebView2 Runtime**,
//! which is not present on every machine. Microsoft's own list of where it may be missing is
//! exactly three cases: **clean Windows 10 installs, Windows Server, and LTSC editions**. When it is
//! missing, `wry` fails to build the webview and `dioxus-desktop`'s `webview.rs` does
//! `.build().unwrap()`.
//!
//! (An earlier version of this file also blamed **N/KN editions**. That was wrong, and it shipped:
//! Microsoft's N/KN exclusion list is *media technology only* — Media Player, Groove, Movies & TV.
//! Edge itself is present in N editions and WebView2 appears nowhere on that list. Guessing at
//! another vendor's edition matrix and putting the guess in a user-facing dialog is how you send a
//! stranger to fix the wrong thing.)
//!
//! Panic → stderr → a console that does not exist → exit 101. The user double-clicks the exe and
//! **nothing happens at all**. No window, no error, no log. They double-click again, conclude the
//! download is corrupt, and leave. It is the worst failure mode we can ship: indistinguishable from
//! a broken file, and completely silent.
//!
//! ## What this does — two layers
//!
//! 1. **Proactive (Windows): [`ensure_webview2`].** Before launching, read the registry the way
//!    Microsoft documents for deployment detection: the `pv` (product version) value under the
//!    WebView2 Runtime client GUID, in all three places an install can register — per-machine on
//!    64-bit Windows (`HKLM\SOFTWARE\WOW6432Node\…`), per-machine on 32-bit (`HKLM\SOFTWARE\…`),
//!    and **per-user** (`HKCU\Software\…`). Missing, empty, or `0.0.0.0` all mean "not installed".
//!    If absent, offer to download Microsoft's official Evergreen Bootstrapper (~2 MB) and run it
//!    `/silent /install` — silent mode performs a *per-user* install that needs **no administrator
//!    rights**, which is exactly why we don't use the bootstrapper's own UI (its interactive mode
//!    goes per-machine and raises a UAC prompt a non-admin evaluator can't accept). Every prompt
//!    here is a native `MessageBoxW`, because a progress *window* would itself need the webview —
//!    the component being installed.
//!
//! 2. **The floor: [`install_startup_dialog`].** A panic hook that puts the message in a native
//!    dialog box. It does not prevent the panic — it makes the panic *legible*, which is the
//!    difference between "this app is broken" and "oh, I need to install that". Deliberately a
//!    catch-all rather than a probe: it covers every other pre-window failure too (a missing DLL,
//!    a broken profile dir), including anything layer 1 got wrong.
//!
//! Not needed on macOS (WKWebView is part of the OS) and not possible on Linux for the equivalent
//! problem: a missing `libwebkit2gtk` fails in the dynamic loader **before `main`**, so no Rust
//! code of ours ever runs and no hook could fire. That one can only be solved by packaging (see
//! `.github/INSTALL_NOTES.md`).

/// UTF-16, NUL-terminated — what the W APIs want.
#[cfg(windows)]
fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Show `text` in a native modal box titled `title` with the given `MB_*` style, and return the
/// pressed-button id (`IDYES` etc.). Null owner HWND is correct — ours never opened.
#[cfg(windows)]
fn message_box(title: &str, text: &str, flags: u32) -> i32 {
    use windows_sys::Win32::UI::WindowsAndMessaging::MessageBoxW;
    // SAFETY: both pointers are valid NUL-terminated UTF-16 buffers that outlive the call, and a
    // null HWND is documented as "no owner window".
    unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            wide(text).as_ptr(),
            wide(title).as_ptr(),
            flags,
        )
    }
}

/// Show `text` in a native error dialog, best-effort.
///
/// Windows only; a no-op elsewhere, where a panic reaches a terminal the user can actually see.
#[cfg(windows)]
fn dialog(title: &str, text: &str) {
    use windows_sys::Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MB_OK};
    message_box(title, text, MB_OK | MB_ICONERROR);
}

#[cfg(not(windows))]
fn dialog(_title: &str, _text: &str) {}

/// Ask a Yes/No question in a native dialog; `true` = the user pressed **Yes**.
#[cfg(windows)]
fn ask_yes_no(title: &str, text: &str) -> bool {
    use windows_sys::Win32::UI::WindowsAndMessaging::{IDYES, MB_ICONINFORMATION, MB_YESNO};
    message_box(title, text, MB_YESNO | MB_ICONINFORMATION) == IDYES
}

/// The message shown when startup dies. Names the overwhelmingly likely cause first, because a
/// stranger reading this has no way to know what a "webview" is and needs the fix, not a diagnosis.
#[cfg(windows)]
const STARTUP_HELP: &str = "Ziqpu couldn't start.\n\n\
     This is almost always a missing Microsoft Edge WebView2 Runtime — the component Ziqpu draws \
     its window with. Windows 11 has it built in and the vast majority of Windows 10 machines do \
     too; it can be missing on a clean Windows 10 install, on Windows Server, or on LTSC \
     editions.\n\n\
     Install the WebView2 Runtime, then start Ziqpu again:\n\
     https://developer.microsoft.com/microsoft-edge/webview2/\n\n\
     Your download is not corrupt, and nothing is wrong with your machine.\n\n\
     Technical detail:\n";

#[cfg(not(windows))]
const STARTUP_HELP: &str = "Ziqpu couldn't start.\n\nTechnical detail:\n";

/// Route panics to a dialog **in release builds only**.
///
/// Debug keeps the default handler: the console is there, and a dialog in the middle of a
/// development loop is worse than a backtrace. Chains to the previous hook so the message still
/// reaches stderr wherever stderr exists (a terminal launch, a CI run, a redirected log).
///
/// Call first thing in `main`, before anything that can fail.
pub fn install_startup_dialog() {
    if cfg!(debug_assertions) {
        return;
    }
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        // Keep the normal behaviour too — it costs nothing and is what a developer running from a
        // terminal expects to see.
        previous(info);
        dialog("Ziqpu", &format!("{STARTUP_HELP}{info}"));
    }));
}

// ---------------------------------------------------------------------------
// Layer 1: proactive WebView2 detect + install (Windows only; no-op elsewhere)
// ---------------------------------------------------------------------------

/// Where a stranger installs WebView2 by hand — shown whenever the automatic path can't finish.
#[cfg(windows)]
const MANUAL_URL: &str = "https://developer.microsoft.com/microsoft-edge/webview2/";

/// Microsoft's permalink to the Evergreen Bootstrapper (`MicrosoftEdgeWebview2Setup.exe`, ~2 MB).
/// The fwlink redirects to the current signed build on Microsoft's CDN; certificate validation on
/// the HTTPS hop is what authenticates it.
#[cfg(windows)]
const BOOTSTRAPPER_URL: &str = "https://go.microsoft.com/fwlink/p/?LinkId=2124703";

/// Read a `REG_SZ` value, or `None` if the key/value is missing or unreadable — for our purposes
/// "can't read it" and "isn't there" both mean "not installed".
#[cfg(windows)]
fn reg_sz(
    root: windows_sys::Win32::System::Registry::HKEY,
    subkey: &str,
    value: &str,
) -> Option<String> {
    use windows_sys::Win32::System::Registry::{RegGetValueW, RRF_RT_REG_SZ};

    let subkey_w = wide(subkey);
    let value_w = wide(value);
    // Version strings are short ("120.0.2210.61"); 128 UTF-16 units is generous. A longer value
    // fails with ERROR_MORE_DATA, which the `!= 0` below treats as absent — fine for a probe.
    let mut buf = [0u16; 128];
    let mut size_bytes = (std::mem::size_of_val(&buf)) as u32;
    // SAFETY: all pointers are valid for the duration of the call; `buf`/`size_bytes` describe a
    // real writable buffer, and RRF_RT_REG_SZ makes the API NUL-terminate what it writes.
    let rc = unsafe {
        RegGetValueW(
            root,
            subkey_w.as_ptr(),
            value_w.as_ptr(),
            RRF_RT_REG_SZ,
            std::ptr::null_mut(),
            buf.as_mut_ptr().cast(),
            &mut size_bytes,
        )
    };
    if rc != 0 {
        return None; // ERROR_SUCCESS is 0; anything else = treat as not present
    }
    // The returned byte count includes the terminating NUL; drop it.
    let units = (size_bytes as usize / 2).saturating_sub(1);
    Some(String::from_utf16_lossy(&buf[..units.min(buf.len())]))
}

/// Microsoft documents `pv` **missing, empty, or literally `0.0.0.0`** as "not installed".
#[cfg(windows)]
fn pv_means_installed(pv: &str) -> bool {
    !pv.is_empty() && pv != "0.0.0.0"
}

/// The installed WebView2 Runtime version, or `None` if it isn't installed.
///
/// This is the registry check Microsoft documents for deployment detection: the `pv` value under
/// the Runtime's client GUID. An install can register in three places — per-machine on 64-bit
/// Windows (EdgeUpdate is a 32-bit product, so it lands under `WOW6432Node`), per-machine on
/// 32-bit Windows, and **per-user** (which is what our own non-elevated bootstrapper run
/// produces). Any hit counts.
#[cfg(windows)]
fn webview2_version() -> Option<String> {
    use windows_sys::Win32::System::Registry::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};

    const CLIENT: &str = r"Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}";
    let candidates = [
        (
            HKEY_LOCAL_MACHINE,
            format!(r"SOFTWARE\WOW6432Node\{CLIENT}"),
        ),
        (HKEY_LOCAL_MACHINE, format!(r"SOFTWARE\{CLIENT}")),
        (HKEY_CURRENT_USER, format!(r"Software\{CLIENT}")),
    ];
    candidates
        .into_iter()
        .find_map(|(root, subkey)| reg_sz(root, &subkey, "pv").filter(|pv| pv_means_installed(pv)))
}

/// Download the Evergreen Bootstrapper to the temp dir and run it `/silent /install`.
///
/// Silent mode is the point, not a nicety: it performs a **per-user** install with no UAC prompt,
/// so a non-admin user (a work laptop, a school machine) still succeeds. The bootstrapper then
/// downloads the full runtime itself, so this can take a minute or two on the user's connection —
/// the offer dialog says so. Returns a human-readable reason on failure.
#[cfg(windows)]
fn install_webview2() -> Result<(), String> {
    use std::io::Read;

    // Download in-process over HTTPS (rustls, bundled roots — same discipline as the live LLM
    // calls). The 32 MB cap and the 100 KB floor are sanity rails: the bootstrapper is ~2 MB, so
    // anything tiny is a captive-portal/error page, not the installer.
    let agent = ureq::AgentBuilder::new()
        .timeout(std::time::Duration::from_secs(180))
        .build();
    let resp = agent
        .get(BOOTSTRAPPER_URL)
        .call()
        .map_err(|e| format!("the download from Microsoft failed ({e})"))?;
    let mut bytes = Vec::new();
    resp.into_reader()
        .take(32 * 1024 * 1024)
        .read_to_end(&mut bytes)
        .map_err(|e| format!("the download from Microsoft was interrupted ({e})"))?;
    if bytes.len() < 100 * 1024 {
        return Err(format!(
            "the download looks wrong ({} bytes — expected an installer of about 2 MB)",
            bytes.len()
        ));
    }

    let dest = std::env::temp_dir().join("MicrosoftEdgeWebview2Setup.exe");
    std::fs::write(&dest, &bytes)
        .map_err(|e| format!("couldn't save the installer to {} ({e})", dest.display()))?;

    let status = crate::no_window(std::process::Command::new(&dest))
        .args(["/silent", "/install"])
        .status()
        .map_err(|e| format!("couldn't start the installer ({e})"));
    let _ = std::fs::remove_file(&dest); // best-effort cleanup either way
    let status = status?;
    if !status.success() {
        return Err(format!("the installer reported a failure ({status})"));
    }
    Ok(())
}

/// Detect WebView2 before launching; if it's missing, offer to install it. Windows only — a no-op
/// everywhere else. Call after [`install_startup_dialog`] and before the Dioxus launch.
///
/// Outcomes: **present** → return silently (the overwhelmingly common case: one registry read).
/// **User declines** → explain the manual path and exit cleanly. **Install fails** → explain what
/// happened + the manual path, exit. **Install succeeds** → return and launch; if registration
/// hasn't landed yet we warn but still try, and the panic-hook floor catches a real failure.
#[cfg(windows)]
pub fn ensure_webview2() {
    if webview2_version().is_some() {
        return;
    }

    let offer = "Welcome to Ziqpu.\n\n\
         Ziqpu draws its window with the Microsoft Edge WebView2 Runtime, and this machine \
         doesn't have it yet. (Windows 11 ships it built in; a clean Windows 10 install, Windows \
         Server, or an LTSC edition may not.)\n\n\
         Install it now?\n\n\
         Yes — Ziqpu downloads the official installer from Microsoft (about 2 MB) and runs it. \
         No administrator rights needed. This usually takes a minute or two, and Ziqpu opens \
         when it finishes.\n\n\
         No — Ziqpu closes, since it can't show a window without WebView2.";
    if !ask_yes_no("Ziqpu — one-time setup", offer) {
        dialog(
            "Ziqpu",
            &format!(
                "Ziqpu will close now.\n\nWhen you're ready, install the WebView2 Runtime \
                 yourself from:\n{MANUAL_URL}\n\nthen start Ziqpu again."
            ),
        );
        std::process::exit(0);
    }

    match install_webview2() {
        Ok(()) => {
            if webview2_version().is_none() {
                // The installer said success but the registration isn't visible yet. Try anyway —
                // worst case the panic-hook floor turns the failure into a readable dialog.
                dialog(
                    "Ziqpu",
                    &format!(
                        "The installer finished, but Windows hasn't registered the WebView2 \
                         Runtime yet. Ziqpu will try to start anyway.\n\nIf no window appears, \
                         start Ziqpu once more; if that still fails, install manually \
                         from:\n{MANUAL_URL}"
                    ),
                );
            }
        }
        Err(why) => {
            dialog(
                "Ziqpu",
                &format!(
                    "The WebView2 install didn't finish: {why}.\n\nYou can install it yourself \
                     from:\n{MANUAL_URL}\n\nthen start Ziqpu again. Nothing is wrong with your \
                     Ziqpu download."
                ),
            );
            std::process::exit(1);
        }
    }
}

#[cfg(not(windows))]
pub fn ensure_webview2() {}

#[cfg(all(test, windows))]
mod tests {
    use super::*;
    use windows_sys::Win32::System::Registry::HKEY_LOCAL_MACHINE;

    /// The absent-markers Microsoft documents: missing (None upstream), empty, and `0.0.0.0`.
    #[test]
    fn pv_absent_markers_mean_not_installed() {
        assert!(!pv_means_installed(""));
        assert!(!pv_means_installed("0.0.0.0"));
        assert!(pv_means_installed("120.0.2210.61"));
        // A real update in flight can leave odd-but-nonzero versions; those still count.
        assert!(pv_means_installed("1.0.0.0"));
    }

    /// `reg_sz` against a value guaranteed on every Windows since forever — proves the FFI call,
    /// the byte/UTF-16 accounting, and the NUL strip, without depending on WebView2 being present
    /// on the machine running the tests.
    #[test]
    fn reg_sz_reads_a_guaranteed_value() {
        let product = reg_sz(
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion",
            "ProductName",
        )
        .expect("ProductName exists on every Windows");
        assert!(product.starts_with("Windows"), "got {product:?}");
        assert!(!product.ends_with('\0'), "NUL must be stripped");
    }

    /// A missing key is `None`, never a panic or a garbage string.
    #[test]
    fn reg_sz_missing_key_is_none() {
        assert_eq!(
            reg_sz(HKEY_LOCAL_MACHINE, r"SOFTWARE\Ziqpu\DoesNotExist", "pv"),
            None
        );
    }
}
