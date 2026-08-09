//! Built-in free-tier health, surfaced honestly.
//!
//! The built-in Live tier is the Ziqpu key proxy: rate-limited, with a monthly spend cap and a kill
//! switch. When it declines, the proxy answers with a specific JSON error and status —
//! `monthly_budget_exhausted` / `rate_limited` (429) or `service_disabled` (503). Without this
//! module the app's HTTP layer collapses every non-2xx to "no reading", so the seeker silently gets
//! the offline template with **no way to know the free tier is simply exhausted for now** — the exact
//! "failing quietly" the changelog promised we don't do.
//!
//! This classifies that response and latches the last-observed state (process-global, thread-safe)
//! so the UI can show one honest line. **Only the built-in proxy path records here** — a user's own
//! key or OpenRouter is not "the free tier" and never touches the latch. And [`reset`] clears it at
//! the start of each ranking, so a state observed under the built-in tier can't linger on the screen
//! after the seeker switches to their own key (whose path never records).

use std::sync::atomic::{AtomicU8, Ordering};

/// The built-in tier's last-observed disposition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltInTier {
    /// Nothing observed this window — fresh, or the built-in tier wasn't the active source.
    Unknown,
    /// A reading came back — the tier is serving.
    Ready,
    /// The monthly token budget is spent (429 `monthly_budget_exhausted`).
    OverBudget,
    /// The per-IP daily request cap is hit (429 `rate_limited`).
    RateLimited,
    /// The operator flipped the kill switch off (503 `service_disabled`).
    Paused,
}

impl BuiltInTier {
    fn code(self) -> u8 {
        match self {
            BuiltInTier::Unknown => 0,
            BuiltInTier::Ready => 1,
            BuiltInTier::OverBudget => 2,
            BuiltInTier::RateLimited => 3,
            BuiltInTier::Paused => 4,
        }
    }
    fn from_code(c: u8) -> Self {
        match c {
            1 => BuiltInTier::Ready,
            2 => BuiltInTier::OverBudget,
            3 => BuiltInTier::RateLimited,
            4 => BuiltInTier::Paused,
            _ => BuiltInTier::Unknown,
        }
    }
}

/// Classify a built-in-proxy response by `(HTTP status, body)`. Returns the tier-specific disposition
/// only when the body carries one of the proxy's known error strings; otherwise [`BuiltInTier::Unknown`]
/// — a generic 500, an Anthropic upstream error passed through, or an offline transport failure must
/// **not** masquerade as "over budget". Pure; unit-tested. Keys on the error string (the proxy's
/// explicit contract), with the status corroborating.
pub fn classify(status: u16, body: &str) -> BuiltInTier {
    let err = serde_json::from_str::<serde_json::Value>(body)
        .ok()
        .and_then(|v| {
            v.get("error")
                .and_then(|e| e.as_str())
                .map(|s| s.to_string())
        });
    match (status, err.as_deref()) {
        (429, Some("monthly_budget_exhausted")) => BuiltInTier::OverBudget,
        (429, Some("rate_limited")) => BuiltInTier::RateLimited,
        (503, Some("service_disabled")) => BuiltInTier::Paused,
        _ => BuiltInTier::Unknown,
    }
}

/// The one honest line for a paused/exhausted tier, or `None` when it's serving (or was never the
/// active source this window). Pure — the UI banner reads it via [`notice`]; kept separate so it is
/// testable without touching the global latch.
pub fn message(t: BuiltInTier) -> Option<String> {
    let line = match t {
        BuiltInTier::OverBudget => {
            "The built-in free readings are over their budget for now, so this reading used the \
             offline template. Add your own API key or run a local model for live readings."
        }
        BuiltInTier::RateLimited => {
            "The built-in free readings have hit today's shared limit — try again later, or add \
             your own API key or a local model for live readings."
        }
        BuiltInTier::Paused => {
            "The built-in free readings are paused right now. Add your own API key or run a local \
             model for live readings."
        }
        BuiltInTier::Ready | BuiltInTier::Unknown => return None,
    };
    Some(line.to_string())
}

static STATE: AtomicU8 = AtomicU8::new(0); // 0 = Unknown

/// Record an observation of the built-in tier. An actionable state (`Ready` or any paused/exhausted
/// state) overwrites; [`BuiltInTier::Unknown`] is a no-op — a generic transport blip must not erase a
/// real "over budget" we already saw, nor invent one.
pub fn record(t: BuiltInTier) {
    if t != BuiltInTier::Unknown {
        STATE.store(t.code(), Ordering::Relaxed);
    }
}

/// Clear the latch. Called at the start of each ranking so the banner reflects only THIS ranking's
/// built-in attempts — and vanishes once the seeker switches to their own key (that path never
/// records, so the latch stays [`BuiltInTier::Unknown`] → no banner).
pub fn reset() {
    STATE.store(0, Ordering::Relaxed);
}

/// The last-observed disposition.
pub fn latest() -> BuiltInTier {
    BuiltInTier::from_code(STATE.load(Ordering::Relaxed))
}

/// The user-facing notice for the current state, or `None` when the tier is serving / not in play.
pub fn notice() -> Option<String> {
    message(latest())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_matches_the_proxy_contract() {
        assert_eq!(
            classify(429, r#"{"error":"monthly_budget_exhausted"}"#),
            BuiltInTier::OverBudget
        );
        assert_eq!(
            classify(429, r#"{"error":"rate_limited"}"#),
            BuiltInTier::RateLimited
        );
        assert_eq!(
            classify(503, r#"{"error":"service_disabled"}"#),
            BuiltInTier::Paused
        );
    }

    #[test]
    fn a_generic_failure_never_claims_over_budget() {
        // A 500, an unknown error string, a passed-through Anthropic error, garbage, and the RIGHT
        // error string on the WRONG status all stay Unknown — honesty cuts both ways.
        assert_eq!(
            classify(500, r#"{"error":"proxy_error"}"#),
            BuiltInTier::Unknown
        );
        assert_eq!(
            classify(429, r#"{"error":"something_else"}"#),
            BuiltInTier::Unknown
        );
        assert_eq!(
            classify(
                400,
                r#"{"type":"error","error":{"type":"invalid_request_error"}}"#
            ),
            BuiltInTier::Unknown
        );
        assert_eq!(classify(502, "not json at all"), BuiltInTier::Unknown);
        assert_eq!(
            classify(200, r#"{"error":"monthly_budget_exhausted"}"#),
            BuiltInTier::Unknown,
            "the error string on a 200 is not a real refusal"
        );
    }

    #[test]
    fn only_paused_states_produce_a_notice() {
        assert!(message(BuiltInTier::OverBudget).unwrap().contains("budget"));
        assert!(message(BuiltInTier::RateLimited).unwrap().contains("limit"));
        assert!(message(BuiltInTier::Paused).unwrap().contains("paused"));
        assert_eq!(message(BuiltInTier::Ready), None);
        assert_eq!(message(BuiltInTier::Unknown), None);
        // Every notice points the seeker at a real alternative (own key or local).
        for t in [
            BuiltInTier::OverBudget,
            BuiltInTier::RateLimited,
            BuiltInTier::Paused,
        ] {
            let m = message(t).unwrap();
            assert!(m.contains("own API key") && m.contains("local"), "{m}");
        }
    }
}
