//! Guardrail — the persistent no-advice surface, and the working Ask box. Advice-seeking questions
//! get an explicit refusal (enforced in the loop's code, never a prompt); a question that *names a
//! company* is measured against the seeker's chart on a throwaway session and answered with a warm
//! reading; anything else gets a helpful nudge. The refusal is never a signal.
//!
//! The Ask box is a real `form`: its `onsubmit` fires for both the Ask button and the Enter key
//! (one shared `submit` closure), which is the webview-reliable way to submit on Enter. The ask
//! always pulls a *real* model — Raw upgrades to Live, Local/Live pass through — never the offline
//! template. The reading's live-interpreter call runs on a worker thread, so a live model never
//! freezes the window — the answer area shows a brief "Consulting the viziers…" state until the
//! prose arrives back through `ctx.ask_reader`.

use agents::{is_advice_seeking, Answer, Fit, Measures, ReadMode};
use dioxus::prelude::*;

use crate::state::{measures_for, AnswerView, AppCtx};

/// The line appended under an advice-seeking question's measured reading — points the seeker at the
/// human checkpoint where the real record can be added, without ever calling a trade.
const CHECKPOINT_NOTE: &str = "approve grounding at the checkpoint to add the real record";

/// Fetch a company's reading off the event-loop thread (a live model call blocks), threading the
/// current [`ReadMode`] through [`agents::reading_for_mode`]; the prose returns via `ctx.ask_reader`
/// and fills the in-flight [`AnswerView::Reading`]. The closure moves only owned, `Send` values —
/// never a `Signal` or the `!Send` session.
fn spawn_reading(
    ctx: &AppCtx,
    m: Measures,
    fit: Fit,
    name: String,
    ticker: String,
    mode: ReadMode,
) {
    let tx = ctx.ask_reader.tx();
    std::thread::spawn(move || {
        let (prose, _model) = agents::reading_for_mode(&m, fit, &name, mode);
        let _ = tx.unbounded_send((ticker, prose));
    });
}

/// Strip a leading redundant `FIT: …` line from an interpreter's prose — the Ask box's reading
/// header already shows the band, so the warm reading + why + REMINDER is all that should remain.
fn strip_fit_header(prose: String) -> String {
    match prose.split_once('\n') {
        Some((first, rest)) if first.trim_start().starts_with("FIT:") => {
            rest.trim_start().to_string()
        }
        _ => prose,
    }
}

#[component]
pub fn Guardrail() -> Element {
    let ctx = use_context::<AppCtx>();
    let mut question = use_signal(|| "Should I buy Tesla?".to_string());
    let answer = ctx.answer.read().clone();

    // One submit path, shared by the button and the Enter key. Routes the question by the two
    // orthogonal signals — is it advice-seeking, and does it name a measurable company:
    //   1. advice + company → the enforced-in-code redirect, FOLLOWED BY that company's measured
    //      reading (off-thread) + a checkpoint note. An honest answer plus the data — never a trade.
    //   2. advice, no company → the redirect alone.
    //   3. non-advice + company → the measured reading (no redirect, no note).
    //   4. otherwise → a helpful nudge.
    let submit = {
        let ctx = ctx.clone();
        move |raw: String| {
            let q = raw.trim().to_string();
            if q.is_empty() {
                return;
            }
            let mut ctx = ctx.clone();
            // The Ask box always pulls a REAL model: if the current display mode is the deterministic
            // Raw template, upgrade the ask to Live; Local stays Local, Live stays Live. (Product owner:
            // "the ask should always do a live or local pull.")
            let mode = match *ctx.mode.read() {
                ReadMode::Raw => ReadMode::Live,
                other => other,
            };

            // Resolve a named company to a datable choice + its measured band on a THROWAWAY session
            // (never recorded on the graded log). `None` when no measurable company was named.
            let measured = tickers::find_in_text(&q).and_then(|row| {
                let choice = tickers::choice(&row.ticker)?;
                let seeker = ctx.seeker.read().clone();
                let measures = measures_for(&seeker, std::slice::from_ref(&choice));
                let m = measures.get(&choice.ticker)?.clone();
                let fit = Fit::from_score(m.score);
                Some((row, m, fit))
            });

            if is_advice_seeking(&q) {
                // The redirect is minted in the graded session's code (never a prompt). Extract the
                // owned message so no RefCell borrow lingers past the match.
                let redirect = match ctx.session.borrow().ask(&q) {
                    Answer::Refusal(m) => m,
                    Answer::Reflection(m) => m,
                };
                if let Some((row, m, fit)) = measured {
                    // Honest answer + the data: the redirect above, the measured reading below.
                    ctx.answer.set(Some(AnswerView::Reading {
                        name: row.name.clone(),
                        ticker: row.ticker.clone(),
                        label: fit.label().to_string(),
                        pending: true,
                        text: String::new(),
                        redirect: Some(redirect),
                        note: Some(CHECKPOINT_NOTE.to_string()),
                    }));
                    spawn_reading(&ctx, m, fit, row.name, row.ticker, mode);
                } else {
                    // Advice with no measurable company → the redirect alone.
                    ctx.answer.set(Some(AnswerView::Refusal(redirect)));
                }
                return;
            }

            if let Some((row, m, fit)) = measured {
                // A named company, non-advice: paint the "measuring…" state, fill the prose off-thread.
                ctx.answer.set(Some(AnswerView::Reading {
                    name: row.name.clone(),
                    ticker: row.ticker.clone(),
                    label: fit.label().to_string(),
                    pending: true,
                    text: String::new(),
                    redirect: None,
                    note: None,
                }));
                spawn_reading(&ctx, m, fit, row.name, row.ticker, mode);
                return;
            }

            ctx.answer.set(Some(AnswerView::Nudge(
                "Name one of your choices — e.g. Coca-Cola — and I'll measure it.".to_string(),
            )));
        }
    };

    rsx! {
        p { class: "eyebrow", "Enforced in code, not in a prompt" }
        p { class: "lead",
            "Name a company or ticker and I'll measure how you "
            em { "fit" }
            " it. Ask whether to "
            em { "buy" }
            " it and I won't."
        }

        div { class: "rail",
            // A real `form`, so Enter submits reliably in WebView2 (the old onkeydown handler was
            // unreliable). `onsubmit` fires for BOTH the Enter key and the submit button; we
            // `prevent_default()` to stop the webview's default page-reload, then run the shared closure.
            form { class: "ask",
                onsubmit: {
                    let submit = submit.clone();
                    move |e: FormEvent| {
                        e.prevent_default();
                        submit(question.read().clone());
                    }
                },
                input {
                    value: "{question}",
                    "aria-label": "Ask about a company or ticker",
                    placeholder: "Ask about a company — e.g. 'how do I fit with AMD?'",
                    oninput: move |e| question.set(e.value()),
                }
                button {
                    class: "btn btn--go",
                    r#type: "submit",
                    "Ask"
                }
            }

            {
                match answer {
                    None => rsx! {},
                    Some(AnswerView::Refusal(msg)) => rsx! { p { class: "refusal", "{msg}" } },
                    Some(AnswerView::Nudge(msg)) => rsx! { p { class: "nudge", "{msg}" } },
                    Some(AnswerView::Reading { name, ticker, label, pending, text, redirect, note }) => {
                        let body = if pending { String::new() } else { strip_fit_header(text) };
                        rsx! {
                            // The enforced-in-code redirect (bold no-advice voice), when this reading
                            // answered an advice-seeking question. Rendered above the reading.
                            {redirect.map(|r| rsx! { p { class: "refusal", "{r}" } })}
                            if pending {
                                div { class: "ask-reading ask-reading--pending",
                                    span { class: "wheat-caption",
                                        "Consulting the viziers… measuring {name} ({ticker})"
                                    }
                                }
                            } else {
                                div { class: "ask-reading",
                                    div { class: "ask-reading__head",
                                        span { class: "ask-reading__name", "{name}" }
                                        span { class: "ticker", "{ticker}" }
                                        span { class: "badge", "{label}" }
                                    }
                                    p { class: "reading", "{body}" }
                                }
                            }
                            // The checkpoint pointer, when set — where the real record gets added.
                            {note.map(|n| rsx! { p { class: "measured grounded-reminder", "{n}" } })}
                        }
                    }
                }
            }
        }

        p { class: "reminder", "measured, not fate — not financial advice" }
        p { class: "liability",
            "We are not responsible for any financial or personal decisions you make based on these readings."
        }
    }
}
