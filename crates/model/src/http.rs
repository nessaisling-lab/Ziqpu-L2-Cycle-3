//! In-process HTTP for the model crate — the last place Ziqpu shelled out to fetch something it
//! then trusted.
//!
//! # Why this replaced `curl`
//!
//! Five call sites here spawned `curl.exe`, each carrying its own copy of the same discipline:
//! System32-pinned path, `--max-time`, `--max-filesize`, `--proto =https`, a User-Agent. Their doc
//! comments called that "the repo's cross-platform, no-HTTP-crate convention".
//!
//! That convention had already been abandoned everywhere else. `crates/agents` moved to `ureq`
//! (pure-Rust rustls, bundled roots) and its own module says why: with `curl -H`, an API key rides
//! on a **process command line**, visible to any process listing on the machine. This crate never
//! followed, so a stale sentence in five doc comments kept asserting a rule the codebase had
//! stopped following — and the subprocess stayed.
//!
//! Nothing here sends a secret, so that specific exposure was not the risk. The risks that were
//! real: the app depended on a binary it does not ship (the code admits `running_server_port`
//! silently finds nothing on a machine with no `curl`), and every fetch spawned a process whose
//! argv the OS logs.
//!
//! # Two things this does better than the curl it replaces
//!
//! **The host check now covers redirects.** `download_to`'s comment conceded its host allowlist was
//! "deliberately the *weaker* of the two checks — `curl -L` follows redirects" to any host it is
//! sent to. In-process, the URL *after* redirects is inspectable, so [`get_to_file`] re-checks it
//! and refuses a redirect that lands somewhere unexpected. The bytes are still digest-verified
//! afterwards; this simply stops them being fetched at all.
//!
//! **The size cap fails instead of truncating.** `--max-filesize` aborts the transfer; a naive
//! `Read::take(n)` would hand back a silently truncated body, and a truncated JSON parses to "no
//! candidates" rather than to an error. So the readers below take `limit + 1` and treat the extra
//! byte as the failure signal.

use std::io::Read;
use std::time::Duration;

/// Identifies Ziqpu to the APIs it calls, as the curl `-A` flag did.
pub(crate) const USER_AGENT: &str = crate::HF_USER_AGENT;

/// Build an agent with a whole-request timeout.
fn agent(timeout: Duration) -> ureq::Agent {
    ureq::AgentBuilder::new()
        .timeout(timeout)
        .user_agent(USER_AGENT)
        .build()
}

/// Read a response body, failing rather than truncating past `limit`.
fn read_capped(resp: ureq::Response, limit: usize) -> Result<String, String> {
    let mut buf = Vec::new();
    resp.into_reader()
        // `limit + 1`: if the extra byte arrives, the body exceeded the cap and the result is a
        // failure, not a shortened success.
        .take(limit as u64 + 1)
        .read_to_end(&mut buf)
        .map_err(|e| format!("read failed ({e})"))?;
    if buf.len() > limit {
        return Err(format!("response exceeded the {limit}-byte cap"));
    }
    String::from_utf8(buf).map_err(|e| format!("response was not UTF-8 ({e})"))
}

/// GET an **HTTPS** URL and return the body, bounded by `timeout` and `limit` bytes.
///
/// `None` on any failure — offline, DNS, non-2xx, oversized, non-UTF-8 — so callers keep the
/// "degrade quietly to the static pick" behaviour the curl versions had.
pub(crate) fn get_text(url: &str, timeout: Duration, limit: usize) -> Option<String> {
    // The `--proto =https` guarantee, checked before a connection is opened.
    if !url.starts_with("https://") {
        return None;
    }
    let resp = agent(timeout).get(url).call().ok()?;
    read_capped(resp, limit).ok()
}

/// GET a **loopback** URL — the local `llama-server` health probe, which is plain HTTP on
/// `127.0.0.1` by design and must not be held to the HTTPS rule.
///
/// Restricted to loopback rather than merely allowing `http://`, so this cannot become a general
/// plaintext escape hatch for some later caller.
pub(crate) fn get_text_loopback(url: &str, timeout: Duration, limit: usize) -> Option<String> {
    if !url.starts_with("http://127.0.0.1:") && !url.starts_with("http://localhost:") {
        return None;
    }
    let resp = agent(timeout).get(url).call().ok()?;
    read_capped(resp, limit).ok()
}

/// Stream an HTTPS URL to `dest`, bounded by `timeout` and `limit` bytes.
///
/// `allow_host` is consulted twice: on the URL given, and again on the URL actually reached after
/// redirects. That second check is what `curl -L` could not do.
pub(crate) fn get_to_file(
    url: &str,
    dest: &std::path::Path,
    timeout: Duration,
    limit: u64,
    allow_host: impl Fn(&str) -> bool,
) -> Result<(), String> {
    if !url.starts_with("https://") {
        return Err(format!("refusing a non-HTTPS download URL: {url}"));
    }
    if !allow_host(url) {
        return Err(format!(
            "refusing to download the runtime from an unexpected host: {url}"
        ));
    }

    let resp = agent(timeout)
        .get(url)
        .call()
        .map_err(|e| format!("download failed ({e})"))?;

    // Where the redirects actually landed. A release URL that redirects to a CDN is normal; one
    // that redirects somewhere the allowlist does not name is not, and the bytes are refused before
    // any of them reach the disk.
    let final_url = resp.get_url().to_string();
    if !final_url.starts_with("https://") || !allow_host(&final_url) {
        return Err(format!(
            "the download redirected to an unexpected host: {final_url}"
        ));
    }

    let mut reader = resp.into_reader().take(limit + 1);
    let mut file = std::fs::File::create(dest)
        .map_err(|e| format!("couldn't create {} ({e})", dest.display()))?;
    let copied = std::io::copy(&mut reader, &mut file)
        .map_err(|e| format!("download failed while writing ({e})"))?;

    if copied > limit {
        // Remove the partial file: leaving it behind would let a later run mistake it for a
        // complete download. The digest check would catch that, but only after an extraction step
        // that should never have been reached.
        let _ = std::fs::remove_file(dest);
        return Err(format!("download exceeded the {limit}-byte cap"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The scheme rules are enforced before any connection, so they are testable offline — which is
    /// the point of checking them in code rather than delegating to a `--proto` flag.
    #[test]
    fn plaintext_and_odd_schemes_are_refused_without_a_connection() {
        let t = Duration::from_millis(1);
        assert_eq!(get_text("http://example.com/x", t, 10), None);
        assert_eq!(get_text("ftp://example.com/x", t, 10), None);
        assert_eq!(get_text("file:///etc/passwd", t, 10), None);

        // And the loopback helper does not become a general plaintext door.
        assert_eq!(get_text_loopback("http://example.com/health", t, 10), None);
        assert_eq!(
            get_text_loopback("http://127.0.0.1.evil.com/health", t, 10),
            None
        );
    }

    /// The host allowlist is consulted before the request, so a rejected host never opens a socket.
    #[test]
    fn a_disallowed_host_is_refused_before_any_bytes() {
        let dest = std::env::temp_dir().join("ziqpu-http-test-never-written");
        let _ = std::fs::remove_file(&dest);

        let err = get_to_file(
            "https://evil.example.com/runtime.zip",
            &dest,
            Duration::from_millis(1),
            10,
            |_| false,
        )
        .expect_err("a disallowed host must be refused");
        assert!(err.contains("unexpected host"), "{err}");
        assert!(!dest.exists(), "nothing may be written for a refused host");

        let err = get_to_file(
            "http://plain.example.com/runtime.zip",
            &dest,
            Duration::from_millis(1),
            10,
            |_| true,
        )
        .expect_err("plaintext must be refused");
        assert!(err.contains("non-HTTPS"), "{err}");
    }
}
