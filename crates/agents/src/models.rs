//! **Live** model discovery for the hosted providers — the catalog behind the in-app model picker.
//!
//! Nothing here hardcodes a model list. Each provider's own API is the source of truth, so the
//! picker shows what is actually callable *today* with the seeker's key. (The one hardcoded id left
//! in the codebase is the per-provider *fallback default* in `interpret_llm`, used only when no
//! choice has been made — and this module is how a seeker replaces it.)
//!
//! Three catalogs, three auth shapes:
//! - **Anthropic** — `GET /v1/models`, authed with the seeker's `x-api-key`. Paginated with
//!   `after_id`/`before_id` + `has_more` (NOT the `page`/`next_page` scheme other endpoints use);
//!   one `limit=100` page covers the whole catalog today, so we don't walk the cursor.
//! - **OpenRouter** — `GET /api/v1/models`, **keyless** (a public catalog), so the picker can
//!   populate before the seeker has pasted anything.
//! - **Ziqpu built-in** — the key proxy's own `/v1/models`, which echoes its `ALLOWED_MODELS`
//!   allowlist. The proxy refuses any model outside that list, so asking it is the only honest way
//!   to know what the built-in tier can serve (see `proxy/README.md`).
//!
//! Every fetch is a blocking `ureq` call — callers must run it off the UI thread.

use crate::interpret_llm::DEFAULT_ANTHROPIC_MODEL;
use crate::llm_http::get_json;

/// How well a model suits **this** job: writing Ungasaga's reading — a warm 4–7 sentence narrative
/// that obeys a fixed shape, names no jargon, gives no advice, and fits a 1536-token cap.
///
/// Every verdict is derived from the provider's own live fields (never a hand-kept list), so it
/// stays true as catalogs change. See [`score_openrouter_fit`] for the rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fit {
    /// Ranked best for readings: clears every rule below AND scores near the top of the catalog's
    /// published intelligence benchmark (our closest live proxy for "follows a careful brief and
    /// writes good prose" — borne out in testing, where small models leak jargon the brief forbids).
    Best,
    /// The strongest of the **free** models. A separate badge because the free tier tops out well
    /// below the paid frontier (≈38 vs ≈59 on the catalog's index today), so the outright picks all
    /// cost money — a seeker who doesn't want a bill still deserves a pointed recommendation rather
    /// than being told to scroll.
    BestFree,
    /// Usable, no known caveat.
    Ok,
    /// Usable, but the seeker should know something first.
    Caution(&'static str),
    /// **Shown and marked, but not selectable yet.** The model is real and worth knowing about — we
    /// just can't drive it today (image/audio generation, and other tool-shaped work the agent will
    /// grow into). Listing it keeps the catalog honest about what exists and flags what's coming,
    /// rather than pretending these models don't exist.
    Unsupported(&'static str),
    /// Hidden — actively going away, or can't honor the limits a reading depends on.
    Unfit(&'static str),
}

impl Fit {
    /// Short badge text, or `None` for the unremarkable middle.
    pub fn badge(&self) -> Option<&'static str> {
        match self {
            Fit::Best => Some("best for readings"),
            Fit::BestFree => Some("best free"),
            Fit::Caution(why) | Fit::Unsupported(why) | Fit::Unfit(why) => Some(why),
            Fit::Ok => None,
        }
    }

    /// Should this model appear in the list at all? Everything except the actively-retiring and the
    /// outright incapable — including [`Fit::Unsupported`], which is listed precisely so the seeker
    /// can see it's coming.
    pub fn listed(&self) -> bool {
        !matches!(self, Fit::Unfit(_))
    }

    /// Can the seeker actually pick it today?
    pub fn selectable(&self) -> bool {
        !matches!(self, Fit::Unfit(_) | Fit::Unsupported(_))
    }

    /// Is this one of the two recommendation badges?
    pub fn is_pick(&self) -> bool {
        matches!(self, Fit::Best | Fit::BestFree)
    }
}

/// One selectable model from a provider's live catalog.
#[derive(Debug, Clone, PartialEq)]
pub struct ModelOption {
    /// The exact id to send as the request's `model` field.
    pub id: String,
    /// Human label for the dropdown (the provider's own display name).
    pub label: String,
    /// Short sub-label — served context, and "free" where the provider says so.
    pub note: String,
    /// Whether the provider prices this model at zero. Drives free-first ordering.
    pub free: bool,
    /// Artificial Analysis' published intelligence index, when the catalog carries one (~99 of
    /// OpenRouter's ~344). Third-party and live — we don't compute or estimate it.
    pub intelligence: Option<f64>,
    /// Whether this is the highest-scoring model in the catalog on that index.
    pub top_quality: bool,
    /// Suitability for writing a Ziqpu reading.
    pub fit: Fit,
}

/// Fetch the catalog for a provider slug (`"anthropic"` / `"openrouter"` / `"built_in"`), reading
/// the credentials it needs from the environment. `Err` carries a short, **key-free** reason fit to
/// show in the UI.
pub fn list_for_provider(slug: &str) -> Result<Vec<ModelOption>, String> {
    match slug {
        "anthropic" => {
            let key = env_nonempty("ANTHROPIC_API_KEY")
                .ok_or("Add your Anthropic key to see its models.")?;
            list_anthropic(&key)
        }
        "openrouter" => list_openrouter(),
        "built_in" => {
            let url = env_nonempty("ZIQPU_PROXY_URL")
                .ok_or("The built-in reader isn't configured in this build.")?;
            let token = env_nonempty("ZIQPU_PROXY_TOKEN")
                .ok_or("The built-in reader isn't configured in this build.")?;
            list_proxy(&url, &token)
        }
        other => Err(format!("Unknown provider: {other}")),
    }
}

/// Anthropic's catalog, authed with the seeker's own key. One page of 100 covers it today; if
/// `has_more` ever goes true we'd walk `after_id` — for now an unwalked cursor just means the
/// picker shows the first 100, which is the whole list.
pub fn list_anthropic(api_key: &str) -> Result<Vec<ModelOption>, String> {
    let text = get_json(
        "https://api.anthropic.com/v1/models?limit=100",
        &[("x-api-key", api_key), ("anthropic-version", "2023-06-01")],
    )
    .ok_or("Couldn't reach Anthropic — check the key and your connection.")?;
    parse_anthropic(&text)
}

/// OpenRouter's catalog. **Keyless** — the model list is public, so this populates before a key is
/// entered (the key is only needed to actually run a reading).
pub fn list_openrouter() -> Result<Vec<ModelOption>, String> {
    let text = get_json("https://openrouter.ai/api/v1/models", &[])
        .ok_or("Couldn't reach OpenRouter — check your connection.")?;
    parse_openrouter(&text)
}

/// The built-in tier's allowlist, straight from the proxy. Derives `/v1/models` from the configured
/// `/v1/messages` URL so one setting drives both.
pub fn list_proxy(messages_url: &str, token: &str) -> Result<Vec<ModelOption>, String> {
    let url = models_url_from(messages_url);
    let text = get_json(&url, &[("authorization", &format!("Bearer {token}"))])
        .ok_or("Couldn't reach the Ziqpu reader — check your connection.")?;
    parse_anthropic(&text)
}

/// `…/v1/messages` → `…/v1/models`. Falls back to appending `/v1/models` to the origin if the URL
/// doesn't end the way we expect, so a hand-edited proxy URL still has a chance of resolving.
fn models_url_from(messages_url: &str) -> String {
    match messages_url.strip_suffix("/v1/messages") {
        Some(base) => format!("{base}/v1/models"),
        None => format!("{}/v1/models", messages_url.trim_end_matches('/')),
    }
}

/// Parse the Anthropic `/v1/models` shape — `{"data":[{"id","display_name","max_input_tokens",…}]}`.
/// The proxy echoes the same shape for the built-in tier, so both paths share this.
fn parse_anthropic(body: &str) -> Result<Vec<ModelOption>, String> {
    let value: serde_json::Value =
        serde_json::from_str(body).map_err(|_| "The model list came back unreadable.")?;
    if value.get("type").and_then(|t| t.as_str()) == Some("error") {
        return Err("That key was rejected — check it and try again.".to_string());
    }
    let data = value
        .get("data")
        .and_then(|d| d.as_array())
        .ok_or("The model list came back empty.")?;

    let mut out: Vec<ModelOption> = data
        .iter()
        .filter_map(|m| {
            let id = m.get("id")?.as_str()?.to_string();
            let label = m
                .get("display_name")
                .and_then(|n| n.as_str())
                .unwrap_or(&id)
                .to_string();
            let note = m
                .get("max_input_tokens")
                .and_then(|c| c.as_u64())
                .map(|c| format!("{} context", human_tokens(c)))
                .unwrap_or_default();
            // Anthropic's catalog publishes no benchmark scores, and every model in it is a strong
            // instruction-follower that holds the reading's shape — so there is no honest way (and
            // no need) to rank them against each other here. The one we mark is the model the app
            // already defaults to; the rest are simply `Ok`.
            let fit = if id == DEFAULT_ANTHROPIC_MODEL {
                Fit::Best
            } else {
                Fit::Ok
            };
            Some(ModelOption {
                id,
                label,
                note,
                free: false,
                intelligence: None,
                top_quality: false,
                fit,
            })
        })
        .collect();
    if out.is_empty() {
        return Err("No models available for that key.".to_string());
    }
    // Our pick first, then alphabetical — stable across fetches so the picker doesn't reshuffle.
    out.sort_by(|a, b| {
        (b.fit == Fit::Best)
            .cmp(&(a.fit == Fit::Best))
            .then_with(|| a.label.cmp(&b.label))
    });
    Ok(out)
}

/// Parse OpenRouter's `/api/v1/models` shape. Keeps only models that can **emit text** (the catalog
/// also carries image/audio-output models, which can't write a reading) and marks the zero-priced
/// ones so the picker can lead with them.
fn parse_openrouter(body: &str) -> Result<Vec<ModelOption>, String> {
    let value: serde_json::Value =
        serde_json::from_str(body).map_err(|_| "The model list came back unreadable.")?;
    let data = value
        .get("data")
        .and_then(|d| d.as_array())
        .ok_or("The model list came back empty.")?;

    let mut out: Vec<ModelOption> = data
        .iter()
        .filter_map(|m| {
            let id = m.get("id")?.as_str()?.to_string();
            let label = m
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or(&id)
                .to_string();
            // Pricing is a string of dollars-per-token ("0" for free tiers).
            let free = m
                .get("pricing")
                .and_then(|p| p.get("prompt"))
                .and_then(|p| p.as_str())
                .map(is_zero_price)
                .unwrap_or(false);
            // `top_provider.context_length` is what the serving provider ACTUALLY gives you;
            // `context_length` is the model's advertised maximum. They differ on ~36 models —
            // nemotron advertises 1M but is served 262K — so quoting the advertised number would
            // promise context the seeker won't get. Prefer the served figure.
            let ctx = m
                .get("top_provider")
                .and_then(|t| t.get("context_length"))
                .and_then(|c| c.as_u64())
                .or_else(|| m.get("context_length").and_then(|c| c.as_u64()))
                .map(|c| format!("{} context", human_tokens(c)));
            let note = match (free, ctx) {
                (true, Some(c)) => format!("free · {c}"),
                (true, None) => "free".to_string(),
                (false, Some(c)) => c,
                (false, None) => String::new(),
            };
            let intelligence = m
                .get("benchmarks")
                .and_then(|b| b.get("artificial_analysis"))
                .and_then(|a| a.get("intelligence_index"))
                .and_then(|s| s.as_f64());
            let fit = score_openrouter_fit(m);
            Some(ModelOption {
                id,
                label,
                note,
                free,
                intelligence,
                top_quality: false,
                fit,
            })
        })
        // Retiring / incapable models are dropped; not-yet-supported ones stay, marked.
        .filter(|m| m.fit.listed())
        .collect();
    if out.is_empty() {
        return Err("No models available.".to_string());
    }
    mark_best(&mut out);
    // Picks first, then free, then strongest-first, then alphabetical — the catalog runs to ~325
    // models, so neither our recommendations nor the zero-cost options should be buried mid-scroll,
    // and within each tier the better model still leads. Two deliberate sinks: models we can't drive
    // yet go last (present for awareness, not in the way), and unscored models sort after scored
    // ones rather than jumping the queue on a missing number.
    out.sort_by(|a, b| {
        a.fit
            .selectable()
            .cmp(&b.fit.selectable())
            .reverse()
            .then_with(|| b.fit.is_pick().cmp(&a.fit.is_pick()))
            .then_with(|| (b.fit == Fit::Best).cmp(&(a.fit == Fit::Best)))
            .then_with(|| b.free.cmp(&a.free))
            .then_with(|| {
                b.intelligence
                    .unwrap_or(f64::MIN)
                    .partial_cmp(&a.intelligence.unwrap_or(f64::MIN))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .then_with(|| a.label.cmp(&b.label))
    });
    Ok(out)
}

/// How many models earn the "best for readings" badge. Small enough that the badge means something.
const BEST_PICKS: usize = 3;
/// How many free models earn the "best free" badge.
const BEST_FREE_PICKS: usize = 2;
/// A declared end-of-life this close counts as retiring — don't offer it. Generous enough to catch
/// the real sunsets (several land within days) without hiding a model over a far-future placeholder
/// date some providers publish (one lists 2098).
const RETIRING_WITHIN_DAYS: i64 = 120;

/// Days until a model's declared end-of-life, or `None` when it publishes no date (the common case)
/// or the date is unparseable. Dates arrive as plain `YYYY-MM-DD`.
fn days_until_expiry(m: &serde_json::Value) -> Option<i64> {
    let raw = m.get("expiration_date")?.as_str()?;
    let date = chrono::NaiveDate::parse_from_str(raw, "%Y-%m-%d").ok()?;
    Some((date - chrono::Utc::now().date_naive()).num_days())
}

/// Does this model emit text and nothing but text?
///
/// "Contains text" is the trap: the media models list `text` alongside their real output — Lyria
/// (music) is `["text","audio"]`, the Gemini/GPT image models are `["image","text"]`. A
/// contains-check reads a music generator as a writer. Requiring text-only keeps every
/// vision-INPUT→text model, since what those emit is text.
///
/// Absent modality info → assume text (the field is newer than some entries).
fn outputs_only_text(m: &serde_json::Value) -> bool {
    m.get("architecture")
        .and_then(|a| a.get("output_modalities"))
        .and_then(|o| o.as_array())
        .map(|mods| {
            let out: Vec<&str> = mods.iter().filter_map(|x| x.as_str()).collect();
            !out.is_empty() && out.iter().all(|m| *m == "text")
        })
        .unwrap_or(true)
}

/// Rate one OpenRouter model against what a Ziqpu reading actually needs. Pure — every input is a
/// live catalog field, so this can't rot into a stale opinion. Each rule maps to a failure we have
/// actually seen or would predictably hit:
///
/// - **No `max_tokens` support** → the model ignores our 1536 cap, so a reading can run away.
///   Unfit (5 of ~344 today).
/// - **Mandatory reasoning** (`reasoning.mandatory: true`, 63 of ~344) → the model always writes its
///   chain-of-thought, and our `reasoning: {exclude: true}` cannot switch off what's mandatory. This
///   is exactly the leak that put pages of "We must write 4-7 sentences. Let's craft…" ahead of a
///   real reading (`strip_reasoning` exists because of it). Usable — we strip it — but caveated.
/// - **Unguarded variant** (abliterated / uncensored / …) → Ungasaga's no-advice guardrail is
///   code-enforced so these stay *safe*, but they're off-brand for the voice and carry
///   provenance/licensing questions. Caveated, never "best".
///
/// Everything else is `Ok`; [`mark_best`] then promotes the strongest of them.
fn score_openrouter_fit(m: &serde_json::Value) -> Fit {
    // Declared end-of-life beats everything: a model that stops answering next week is not a choice
    // worth offering, however good it looks today. 16 of ~344 carry a date — including a free Llama
    // that expires within days while still reading as a strong free pick on every other signal.
    if let Some(days) = days_until_expiry(m) {
        if days <= RETIRING_WITHIN_DAYS {
            return Fit::Unfit("retiring");
        }
    }

    // Emits something other than text (image / audio generation). Not hidden — the seeker should
    // see these exist, and the agent is expected to grow into driving them; it just can't today.
    if !outputs_only_text(m) {
        return Fit::Unsupported("image/audio — not supported yet");
    }

    let supports_max_tokens = m
        .get("supported_parameters")
        .and_then(|p| p.as_array())
        .map(|p| p.iter().any(|x| x.as_str() == Some("max_tokens")))
        .unwrap_or(true);
    if !supports_max_tokens {
        return Fit::Unfit("ignores length limits");
    }

    let id = m.get("id").and_then(|i| i.as_str()).unwrap_or_default();
    if is_unguarded_variant(id) {
        return Fit::Caution("unguarded variant — off-brand voice");
    }

    let mandatory_reasoning = m
        .get("reasoning")
        .and_then(|r| r.get("mandatory"))
        .and_then(|b| b.as_bool())
        .unwrap_or(false);
    if mandatory_reasoning {
        return Fit::Caution("always shows its work — we trim it");
    }

    Fit::Ok
}

/// Promote the strongest `Ok` models to [`Fit::Best`], and flag the catalog's top scorer.
///
/// Ranking uses the catalog's published intelligence index rather than anything we invent. It is a
/// proxy, not a prose score — but it is the honest one available, and it matches what testing showed:
/// small models leak the planet/aspect names the brief forbids, larger ones hold the shape. A model
/// with no published score is never promoted (we won't guess on its behalf) — it stays selectable.
fn mark_best(out: &mut [ModelOption]) {
    if let Some(top) = out
        .iter()
        .filter_map(|m| m.intelligence)
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    {
        for m in out.iter_mut() {
            if m.intelligence == Some(top) {
                m.top_quality = true;
            }
        }
    }

    // Rank only the clean, scored models — a caveated or unscored model never wins a badge.
    let ranked = |out: &[ModelOption], free_only: bool| -> Vec<usize> {
        let mut v: Vec<(usize, f64)> = out
            .iter()
            .enumerate()
            .filter(|(_, m)| m.fit == Fit::Ok && (!free_only || m.free))
            .filter_map(|(i, m)| m.intelligence.map(|s| (i, s)))
            .collect();
        v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        v.into_iter().map(|(i, _)| i).collect()
    };

    // Outright picks first, so a free model strong enough to place overall wins the better badge
    // rather than being filed under "best free".
    for i in ranked(out, false).into_iter().take(BEST_PICKS) {
        out[i].fit = Fit::Best;
    }
    // Then the best of what's left among the free tier.
    for i in ranked(out, true).into_iter().take(BEST_FREE_PICKS) {
        out[i].fit = Fit::BestFree;
    }
}

/// Does this model id name a guardrail-stripped variant? Mirrors the `model` crate's agent-fit
/// filter for local GGUFs — same reasoning, same vocabulary, applied to the hosted catalog.
fn is_unguarded_variant(id: &str) -> bool {
    const MARKERS: [&str; 6] = [
        "abliterated",
        "uncensored",
        "derestricted",
        "unfiltered",
        "heretic",
        "-unalign",
    ];
    let id = id.to_ascii_lowercase();
    MARKERS.iter().any(|m| id.contains(m))
}

/// Is this OpenRouter price string zero? Prices arrive as decimal strings ("0", "0.0",
/// "0.000003"); a plain `== "0"` misses the padded forms.
fn is_zero_price(p: &str) -> bool {
    p.parse::<f64>().map(|v| v == 0.0).unwrap_or(false)
}

/// 1000000 → "1M", 128000 → "128K". Used for the dropdown's context sub-label.
fn human_tokens(n: u64) -> String {
    if n >= 1_000_000 {
        let m = n as f64 / 1_000_000.0;
        if (m - m.round()).abs() < 0.05 {
            format!("{}M", m.round() as u64)
        } else {
            format!("{m:.1}M")
        }
    } else if n >= 1_000 {
        format!("{}K", n / 1_000)
    } else {
        n.to_string()
    }
}

fn env_nonempty(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|v| !v.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_anthropic_catalog_shape() {
        // The documented shape: data[] of {id, display_name, max_input_tokens, ...} + has_more.
        let body = r#"{
            "data": [
                {"type":"model","id":"claude-sonnet-5","display_name":"Claude Sonnet 5",
                 "max_input_tokens":1000000,"max_tokens":128000},
                {"type":"model","id":"claude-haiku-4-5","display_name":"Claude Haiku 4.5",
                 "max_input_tokens":200000,"max_tokens":64000}
            ],
            "has_more": false, "first_id": "a", "last_id": "b"
        }"#;
        let got = parse_anthropic(body).expect("valid catalog");
        assert_eq!(got.len(), 2);
        // Sorted by label: Haiku before Sonnet.
        assert_eq!(got[0].id, "claude-haiku-4-5");
        assert_eq!(got[0].label, "Claude Haiku 4.5");
        assert_eq!(got[0].note, "200K context");
        assert_eq!(got[1].note, "1M context");
        assert!(!got[1].free);
    }

    #[test]
    fn anthropic_error_body_is_a_key_free_message() {
        let body = r#"{"type":"error","error":{"type":"authentication_error","message":"x"}}"#;
        let err = parse_anthropic(body).unwrap_err();
        assert!(err.contains("rejected"), "{err}");
    }

    #[test]
    fn parses_openrouter_and_leads_with_free() {
        // Real-shape sample: a paid model, a free one, and an image-output model to exclude.
        let body = r#"{
            "data": [
                {"id":"vendor/paid-model","name":"Zebra Paid","context_length":128000,
                 "pricing":{"prompt":"0.000003","completion":"0.000015"},
                 "architecture":{"output_modalities":["text"]}},
                {"id":"nvidia/nemotron-3-super-120b-a12b:free","name":"Nemotron 3 Super",
                 "context_length":128000,"pricing":{"prompt":"0","completion":"0"},
                 "architecture":{"output_modalities":["text"]}},
                {"id":"vendor/image-only","name":"Painter","context_length":4096,
                 "pricing":{"prompt":"0"},
                 "architecture":{"output_modalities":["image"]}}
            ]
        }"#;
        let got = parse_openrouter(body).expect("valid catalog");
        // The image-output model is LISTED (so its existence is visible and its future support is
        // flagged) but marked and not selectable, and it sinks below the real writers.
        assert_eq!(got.len(), 3);
        let img = got
            .iter()
            .find(|m| m.id == "vendor/image-only")
            .expect("listed");
        assert_eq!(img.fit, Fit::Unsupported("image/audio — not supported yet"));
        assert!(!img.fit.selectable());
        assert_eq!(
            got.last().unwrap().id,
            "vendor/image-only",
            "sinks to the bottom"
        );
        // Free leads, even though "Nemotron" sorts after "Zebra"... no: free-first is the rule.
        assert_eq!(got[0].id, "nvidia/nemotron-3-super-120b-a12b:free");
        assert!(got[0].free);
        assert_eq!(got[0].note, "free · 128K context");
        assert!(!got[1].free);
        assert_eq!(got[1].note, "128K context");
    }

    /// Regression: media models list `text` ALONGSIDE their real output, so a "contains text"
    /// filter admits a music generator into the list of models that can write a reading. These are
    /// the exact `output_modalities` shapes observed in the live catalog.
    #[test]
    fn media_models_that_also_claim_text_output_are_excluded() {
        let body = r#"{
            "data": [
                {"id":"google/lyria-3-clip-preview","name":"Lyria 3 Clip","context_length":1000000,
                 "pricing":{"prompt":"0"},
                 "architecture":{"output_modalities":["text","audio"]}},
                {"id":"google/gemini-3-pro-image","name":"Gemini 3 Pro Image","context_length":32000,
                 "pricing":{"prompt":"0.000003"},
                 "architecture":{"output_modalities":["image","text"]}},
                {"id":"openai/gpt-audio","name":"GPT Audio","context_length":32000,
                 "pricing":{"prompt":"0.000003"},
                 "architecture":{"output_modalities":["text","audio"]}},
                {"id":"vendor/writer","name":"Writer","context_length":128000,
                 "pricing":{"prompt":"0.000001"},
                 "architecture":{"input_modalities":["text","image"],"output_modalities":["text"]}}
            ]
        }"#;
        let got = parse_openrouter(body).expect("valid catalog");
        // All four are listed, but only the text-only writer is selectable — and a vision-INPUT
        // model still qualifies, because what it emits is text.
        assert_eq!(got.len(), 4, "got: {got:?}");
        let selectable: Vec<&str> = got
            .iter()
            .filter(|m| m.fit.selectable())
            .map(|m| m.id.as_str())
            .collect();
        assert_eq!(selectable, vec!["vendor/writer"]);
        for id in ["google/lyria-3-clip-preview", "openai/gpt-audio"] {
            let m = got.iter().find(|m| m.id == id).expect("listed");
            assert!(matches!(m.fit, Fit::Unsupported(_)), "{id}: {:?}", m.fit);
        }
    }

    /// A declared end-of-life outranks every other signal — including a very strong score.
    /// Regression: a free Llama with 2.1M HF downloads read as a top free pick while expiring
    /// within days.
    #[test]
    fn retiring_models_are_dropped_but_far_future_dates_are_not() {
        let soon = (chrono::Utc::now().date_naive() + chrono::Duration::days(3))
            .format("%Y-%m-%d")
            .to_string();
        let never = "2098-12-31";
        let body = format!(
            r#"{{"data":[
                {{"id":"vendor/dying","name":"Dying","pricing":{{"prompt":"0"}},
                 "architecture":{{"output_modalities":["text"]}},
                 "supported_parameters":["max_tokens"],
                 "expiration_date":"{soon}",
                 "benchmarks":{{"artificial_analysis":{{"intelligence_index":99.0}}}}}},
                {{"id":"vendor/evergreen","name":"Evergreen","pricing":{{"prompt":"0.1"}},
                 "architecture":{{"output_modalities":["text"]}},
                 "supported_parameters":["max_tokens"],
                 "expiration_date":"{never}",
                 "benchmarks":{{"artificial_analysis":{{"intelligence_index":50.0}}}}}}
            ]}}"#
        );
        let got = parse_openrouter(&body).expect("valid");
        // Gone despite scoring 99 — it stops answering in 3 days.
        assert!(!got.iter().any(|m| m.id == "vendor/dying"));
        // A far-future placeholder is not a sunset.
        assert!(got.iter().any(|m| m.id == "vendor/evergreen"));
    }

    /// The free tier tops out far below the paid frontier, so "best free" must be its own badge or
    /// a cost-conscious seeker gets no recommendation at all.
    #[test]
    fn best_free_is_awarded_separately_from_the_outright_picks() {
        // Catalog-shaped: enough paid models to fill the three outright picks, mirroring the real
        // catalog where the frontier (≈55-59) sits well above the best free model (≈38).
        let body = r#"{"data":[
            {"id":"paid/strong","name":"Paid Strong","pricing":{"prompt":"0.1"},
             "architecture":{"output_modalities":["text"]},"supported_parameters":["max_tokens"],
             "benchmarks":{"artificial_analysis":{"intelligence_index":59.0}}},
            {"id":"paid/b","name":"Paid B","pricing":{"prompt":"0.1"},
             "architecture":{"output_modalities":["text"]},"supported_parameters":["max_tokens"],
             "benchmarks":{"artificial_analysis":{"intelligence_index":58.0}}},
            {"id":"paid/c","name":"Paid C","pricing":{"prompt":"0.1"},
             "architecture":{"output_modalities":["text"]},"supported_parameters":["max_tokens"],
             "benchmarks":{"artificial_analysis":{"intelligence_index":57.0}}},
            {"id":"free/best","name":"Free Best","pricing":{"prompt":"0"},
             "architecture":{"output_modalities":["text"]},"supported_parameters":["max_tokens"],
             "benchmarks":{"artificial_analysis":{"intelligence_index":38.0}}},
            {"id":"free/weaker","name":"Free Weaker","pricing":{"prompt":"0"},
             "architecture":{"output_modalities":["text"]},"supported_parameters":["max_tokens"],
             "benchmarks":{"artificial_analysis":{"intelligence_index":20.0}}},
            {"id":"free/weakest","name":"Free Weakest","pricing":{"prompt":"0"},
             "architecture":{"output_modalities":["text"]},"supported_parameters":["max_tokens"],
             "benchmarks":{"artificial_analysis":{"intelligence_index":10.0}}}
        ]}"#;
        let got = parse_openrouter(body).expect("valid");
        let fit = |id: &str| got.iter().find(|m| m.id == id).unwrap().fit.clone();
        // The paid model takes the outright badge...
        assert_eq!(fit("paid/strong"), Fit::Best);
        // ...and the free tier still gets a pointed recommendation of its own.
        assert_eq!(fit("free/best"), Fit::BestFree);
        assert_eq!(fit("free/weaker"), Fit::BestFree); // BEST_FREE_PICKS = 2
        assert_eq!(fit("free/weakest"), Fit::Ok);
    }

    /// A free model strong enough to place overall should win the better badge, not be filed under
    /// "best free".
    #[test]
    fn a_free_model_that_places_overall_wins_the_outright_badge() {
        // The three outright picks are the monster + two paid models, so the weaker free model
        // can't back into `Best` just by filling an empty slot.
        let body = r#"{"data":[
            {"id":"free/monster","name":"Free Monster","pricing":{"prompt":"0"},
             "architecture":{"output_modalities":["text"]},"supported_parameters":["max_tokens"],
             "benchmarks":{"artificial_analysis":{"intelligence_index":99.0}}},
            {"id":"paid/a","name":"Paid A","pricing":{"prompt":"0.1"},
             "architecture":{"output_modalities":["text"]},"supported_parameters":["max_tokens"],
             "benchmarks":{"artificial_analysis":{"intelligence_index":80.0}}},
            {"id":"paid/b","name":"Paid B","pricing":{"prompt":"0.1"},
             "architecture":{"output_modalities":["text"]},"supported_parameters":["max_tokens"],
             "benchmarks":{"artificial_analysis":{"intelligence_index":70.0}}},
            {"id":"free/second","name":"Free Second","pricing":{"prompt":"0"},
             "architecture":{"output_modalities":["text"]},"supported_parameters":["max_tokens"],
             "benchmarks":{"artificial_analysis":{"intelligence_index":10.0}}}
        ]}"#;
        let got = parse_openrouter(body).expect("valid");
        assert_eq!(got[0].id, "free/monster");
        assert_eq!(got[0].fit, Fit::Best, "not demoted to best-free");
        let second = got.iter().find(|m| m.id == "free/second").unwrap();
        assert_eq!(second.fit, Fit::BestFree);
    }

    /// Each fit rule maps to a real failure mode; lock them all in one catalog.
    #[test]
    fn fit_rules_reflect_what_a_reading_actually_needs() {
        let body = r#"{
            "data": [
                {"id":"vendor/no-cap","name":"No Cap","pricing":{"prompt":"0"},
                 "architecture":{"output_modalities":["text"]},
                 "supported_parameters":["temperature"],
                 "benchmarks":{"artificial_analysis":{"intelligence_index":99.0}}},

                {"id":"vendor/model-abliterated","name":"Unguarded","pricing":{"prompt":"0"},
                 "architecture":{"output_modalities":["text"]},
                 "supported_parameters":["max_tokens"],
                 "benchmarks":{"artificial_analysis":{"intelligence_index":90.0}}},

                {"id":"vendor/thinker","name":"Thinker","pricing":{"prompt":"0.1"},
                 "architecture":{"output_modalities":["text"]},
                 "supported_parameters":["max_tokens"],
                 "reasoning":{"mandatory":true},
                 "benchmarks":{"artificial_analysis":{"intelligence_index":80.0}}},

                {"id":"vendor/strong","name":"Strong","pricing":{"prompt":"0.1"},
                 "architecture":{"output_modalities":["text"]},
                 "supported_parameters":["max_tokens"],
                 "reasoning":{"mandatory":false},
                 "benchmarks":{"artificial_analysis":{"intelligence_index":70.0}},
                 "top_provider":{"context_length":128000}},

                {"id":"vendor/unscored","name":"Unscored","pricing":{"prompt":"0.1"},
                 "architecture":{"output_modalities":["text"]},
                 "supported_parameters":["max_tokens"]}
            ]
        }"#;
        let got = parse_openrouter(body).expect("valid catalog");
        let by = |id: &str| got.iter().find(|m| m.id == id).cloned();

        // Ignores our length cap → not offered at all, despite the catalog's top score.
        assert!(by("vendor/no-cap").is_none());

        // Guardrail-stripped → selectable, caveated, never "best" — even at score 90.
        let ung = by("vendor/model-abliterated").expect("offered");
        assert!(matches!(ung.fit, Fit::Caution(_)));

        // Mandatory chain-of-thought → caveated (we strip it, but the seeker should know).
        let think = by("vendor/thinker").expect("offered");
        assert_eq!(
            think.fit,
            Fit::Caution("always shows its work — we trim it")
        );

        // The clean, scored model wins the badge despite NOT being the catalog's top scorer —
        // fit beats raw score, which is the whole point of the badge.
        let strong = by("vendor/strong").expect("offered");
        assert_eq!(strong.fit, Fit::Best);
        assert_eq!(strong.note, "128K context");

        // Unscored → never promoted on a guess, but still selectable.
        let uns = by("vendor/unscored").expect("offered");
        assert_eq!(uns.fit, Fit::Ok);

        // "Top quality" is the catalog's own highest score, independent of fit — and it is NOT
        // the same model as our pick here, so the two badges stay distinct.
        assert!(by("vendor/model-abliterated").unwrap().top_quality);
        assert!(!strong.top_quality);

        // Our pick sorts first.
        assert_eq!(got[0].id, "vendor/strong");
    }

    /// The served context is what the seeker actually gets — quoting the advertised max would
    /// promise 1M when nemotron is really served 262K.
    #[test]
    fn context_note_quotes_the_served_window_not_the_advertised_one() {
        let body = r#"{"data":[
            {"id":"v/m","name":"M","pricing":{"prompt":"0"},
             "architecture":{"output_modalities":["text"]},
             "supported_parameters":["max_tokens"],
             "context_length":1000000,
             "top_provider":{"context_length":262144}}
        ]}"#;
        let got = parse_openrouter(body).expect("valid");
        // 262144 served, not the 1000000 advertised. (Quoted decimally — 262K — to match how the
        // providers themselves quote windows, e.g. Anthropic's "200K".)
        assert_eq!(got[0].note, "free · 262K context");
    }

    #[test]
    fn anthropic_marks_the_app_default_as_the_pick() {
        let body = format!(
            r#"{{"data":[
                {{"type":"model","id":"{DEFAULT_ANTHROPIC_MODEL}","display_name":"Zzz Default","max_input_tokens":1000000}},
                {{"type":"model","id":"claude-haiku-4-5","display_name":"Aaa Haiku","max_input_tokens":200000}}
            ]}}"#
        );
        let got = parse_anthropic(&body).expect("valid");
        // The pick leads even though its label sorts last — and it's the SAME id the interpreter
        // falls back to, so the recommendation can't drift from the default.
        assert_eq!(got[0].id, DEFAULT_ANTHROPIC_MODEL);
        assert_eq!(got[0].fit, Fit::Best);
        assert_eq!(got[1].fit, Fit::Ok);
    }

    #[test]
    fn unguarded_variants_are_spotted_by_name() {
        assert!(is_unguarded_variant("vendor/llama-abliterated"));
        assert!(is_unguarded_variant("vendor/Model-UNCENSORED"));
        assert!(is_unguarded_variant("vendor/x-heretic"));
        assert!(!is_unguarded_variant(
            "nvidia/nemotron-3-super-120b-a12b:free"
        ));
        assert!(!is_unguarded_variant("anthropic/claude-opus-4.8"));
    }

    #[test]
    fn zero_price_tolerates_padded_decimals() {
        assert!(is_zero_price("0"));
        assert!(is_zero_price("0.0"));
        assert!(is_zero_price("0.000000"));
        assert!(!is_zero_price("0.000003"));
        assert!(!is_zero_price("nonsense"));
    }

    #[test]
    fn human_tokens_reads_like_a_spec_sheet() {
        assert_eq!(human_tokens(1_000_000), "1M");
        assert_eq!(human_tokens(200_000), "200K");
        assert_eq!(human_tokens(128_000), "128K");
        assert_eq!(human_tokens(900), "900");
    }

    /// Hits the REAL OpenRouter catalog. `#[ignore]`d so CI stays offline and deterministic; run it
    /// by hand when you want to prove the parser still matches the live shape:
    /// `cargo test -p agents live_openrouter -- --ignored --nocapture`
    #[test]
    #[ignore = "network: hits the live OpenRouter catalog"]
    fn live_openrouter_catalog_parses() {
        let got = list_openrouter().expect("live catalog");
        assert!(got.len() > 50, "expected a real catalog, got {}", got.len());
        assert_eq!(got[0].fit, Fit::Best, "our picks must lead the list");
        assert!(
            got.iter().any(|m| m.id.contains('/')),
            "OpenRouter ids are namespaced vendor/model"
        );
        eprintln!("{} models offered. Ziqpu's picks:", got.len());
        for m in got.iter().filter(|m| m.fit == Fit::Best) {
            eprintln!(
                "  ✦ {} — {} [{}] intelligence={:?}",
                m.label, m.note, m.id, m.intelligence
            );
        }
        eprintln!("Best FREE:");
        for m in got.iter().filter(|m| m.fit == Fit::BestFree) {
            eprintln!(
                "  ✧ {} — {} [{}] intelligence={:?}",
                m.label, m.note, m.id, m.intelligence
            );
        }
        eprintln!("Top quality (catalog's own highest score):");
        for m in got.iter().filter(|m| m.top_quality) {
            eprintln!("  ★ {} [{}] {:?}", m.label, m.id, m.intelligence);
        }
        eprintln!("Listed but not drivable yet (support planned):");
        for m in got.iter().filter(|m| !m.fit.selectable()).take(4) {
            eprintln!("  ▪ {} [{}] — {:?}", m.label, m.id, m.fit.badge());
        }
        eprintln!(
            "caveated: {}, unguarded: {}, not-yet-supported: {}",
            got.iter()
                .filter(|m| matches!(m.fit, Fit::Caution(_)))
                .count(),
            got.iter().filter(|m| is_unguarded_variant(&m.id)).count(),
            got.iter().filter(|m| !m.fit.selectable()).count()
        );
        // Every badge must land on something real, or it's decoration.
        assert!(got.iter().any(|m| m.fit == Fit::Best), "no picks");
        assert!(got.iter().any(|m| m.fit == Fit::BestFree), "no free pick");
        assert!(got.iter().any(|m| m.top_quality), "no top-quality mark");
        // The retiring filter must be PRECISE, not broad. The catalog carries two Llama 3.2 3B
        // variants: the `:free` one declares an end-of-life days away (and read as a strong free
        // pick on every other signal), while the paid sibling has no expiry and should stay.
        assert!(
            !got.iter()
                .any(|m| m.id == "meta-llama/llama-3.2-3b-instruct:free"),
            "an expiring model is still being offered"
        );
        assert!(
            got.iter()
                .any(|m| m.id == "meta-llama/llama-3.2-3b-instruct"),
            "the non-expiring sibling was dropped — the filter is too broad"
        );
    }

    #[test]
    fn proxy_models_url_derives_from_the_messages_url() {
        assert_eq!(
            models_url_from("https://p.example.workers.dev/v1/messages"),
            "https://p.example.workers.dev/v1/models"
        );
        // A hand-edited URL still resolves somewhere sane.
        assert_eq!(
            models_url_from("https://p.example.workers.dev/"),
            "https://p.example.workers.dev/v1/models"
        );
    }
}
