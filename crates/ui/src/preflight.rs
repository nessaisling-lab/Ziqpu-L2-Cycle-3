//! Make a failure to start **visible**.
//!
//! ## The bug this exists for
//!
//! A release build links as `windows_subsystem = "windows"` (no console — see `main.rs`), which is
//! correct: a GUI app should not flash a terminal. But it means a panic before the window opens has
//! nowhere to print. And there is a very ordinary way to panic before the window opens: Dioxus
//! desktop draws through the system webview, and on Windows that is the **Edge WebView2 Runtime**,
//! which is *not* present on every machine — notably LTSC, N/KN editions, and freshly-imaged
//! enterprise installs. When it is missing, `wry` fails to build the webview and
//! `dioxus-desktop`'s `webview.rs` does `.build().unwrap()`.
//!
//! Panic → stderr → a console that does not exist → exit 101. The user double-clicks the exe and
//! **nothing happens at all**. No window, no error, no log. They double-click again, conclude the
//! download is corrupt, and leave. It is the worst failure mode we can ship: indistinguishable from
//! a broken file, and completely silent.
//!
//! ## What this does
//!
//! Installs a panic hook that puts the message in a native dialog box. It does not prevent the
//! panic — it makes the panic *legible*, which is the difference between "this app is broken" and
//! "oh, I need to install that".
//!
//! Deliberately a **catch-all rather than a WebView2 probe**: it needs no registry knowledge, no
//! version detection, and no new dependency, and it covers every other pre-window failure too (a
//! missing DLL, a broken profile dir). A proactive check — detect WebView2 *before* launching and
//! offer to install it — is strictly better UX and is the next step; this is the floor beneath it,
//! and the floor is what stops the silent case.
//!
//! Not needed on Linux for the equivalent problem: a missing `libwebkit2gtk` fails in the dynamic
//! loader **before `main`**, so no Rust code of ours ever runs and no hook could fire. That one can
//! only be solved by packaging (see `.github/INSTALL_NOTES.md`).

/// Show `text` in a native modal dialog titled `title`, best-effort.
///
/// Windows only; a no-op elsewhere, where a panic reaches a terminal the user can actually see.
#[cfg(windows)]
fn dialog(title: &str, text: &str) {
    use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK};

    /// UTF-16, NUL-terminated — what the W APIs want.
    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    // SAFETY: both pointers are valid NUL-terminated UTF-16 buffers that outlive the call, and a
    // null HWND is documented as "no owner window" (correct — ours never opened).
    unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            wide(text).as_ptr(),
            wide(title).as_ptr(),
            MB_OK | MB_ICONERROR,
        );
    }
}

#[cfg(not(windows))]
fn dialog(_title: &str, _text: &str) {}

/// The message shown when startup dies. Names the overwhelmingly likely cause first, because a
/// stranger reading this has no way to know what a "webview" is and needs the fix, not a diagnosis.
#[cfg(windows)]
const STARTUP_HELP: &str = "Ziqpu couldn't start.\n\n\
     This is almost always a missing Microsoft Edge WebView2 Runtime — the component Ziqpu draws \
     its window with. Most Windows 11 and Windows 10 machines already have it; some (LTSC, N/KN, \
     and freshly-imaged work laptops) do not.\n\n\
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
