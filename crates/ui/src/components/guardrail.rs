//! Guardrail — the persistent no-advice surface, and the working Ask box. Advice-seeking questions
//! get an explicit refusal (enforced in the loop's code, never a prompt); a question that *names a
//! company* is measured against the seeker's chart on a throwaway session and answered with a warm
//! reading; anything else gets a helpful nudge. The refusal is never a signal.
//!
//! The Ask box submits on both the button and the Enter key (one shared `submit` closure). The
//! reading's live-interpreter call runs on a worker thread, so a live model never freezes the
//! window — the answer area shows a brief "Consulting the viziers…" state until the prose arrives
//! back through `ctx.ask_reader`.

use agents::{is_advice_seeking, Answer, Fit};
use dioxus::prelude::*;

use crate::state::{measures_for, AnswerView, AppCtx};

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

    // One submit path, shared by the button and the Enter key. Routes the question:
    //   1. advice-seeking             → the enforced-in-code Refusal (`Session::ask`)
    //   2. names a measurable company → measure it (throwaway session) + a warm reading, off-thread
    //   3. otherwise                  → a helpful nudge
    let submit = {
        let ctx = ctx.clone();
        move |raw: String| {
            let q = raw.trim().to_string();
            if q.is_empty() {
                return;
            }
            let mut ctx = ctx.clone();

            if is_advice_seeking(&q) {
                // Keep the current path: the refusal is minted in the graded session's code.
                let view = match ctx.session.borrow().ask(&q) {
                    Answer::Refusal(m) => AnswerView::Refusal(m),
                    Answer::Reflection(m) => AnswerView::Reflection(m),
                };
                ctx.answer.set(Some(view));
                return;
            }

            if let Some(row) = tickers::find_in_text(&q) {
                // A named company: date it, measure it against the seeker on a THROWAWAY session
                // (never recorded on the graded log), band the score, and fetch the reading off-thread.
                if let Some(choice) = tickers::choice(&row.ticker) {
                    let seeker = ctx.seeker.read().clone();
                    let measures = measures_for(&seeker, std::slice::from_ref(&choice));
                    if let Some(m) = measures.get(&choice.ticker) {
                        let fit = Fit::from_score(m.score);
                        // Paint the "measuring…" state immediately; the prose fills in when it lands.
                        ctx.answer.set(Some(AnswerView::Reading {
                            name: row.name.clone(),
                            ticker: row.ticker.clone(),
                            label: fit.label().to_string(),
                            pending: true,
                            text: String::new(),
                        }));
                        // The live interpreter call (a blocking `curl`) runs on a worker thread so it
                        // never freezes the window; the prose returns via `ctx.ask_reader`. The thread
                        // moves only owned, `Send` values — never a Signal or the `!Send` session.
                        let tx = ctx.ask_reader.tx();
                        let m = m.clone();
                        let name = row.name.clone();
                        let ticker = row.ticker.clone();
                        std::thread::spawn(move || {
                            let (prose, _model) = agents::reading_for(&m, fit, &name);
                            let _ = tx.unbounded_send((ticker, prose));
                        });
                    }
                }
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
            "Ask how you "
            em { "fit" }
            " a choice and I'll measure it. Ask whether to "
            em { "buy" }
            " it and I won't."
        }

        div { class: "rail",
            div { class: "ask",
                input {
                    value: "{question}",
                    "aria-label": "Ask about a choice",
                    oninput: move |e| question.set(e.value()),
                    onkeydown: {
                        let submit = submit.clone();
                        move |e: KeyboardEvent| {
                            if e.key() == Key::Enter {
                                e.prevent_default();
                                submit(question.read().clone());
                            }
                        }
                    },
                }
                button {
                    class: "btn btn--go",
                    r#type: "button",
                    onclick: {
                        let submit = submit.clone();
                        move |_| submit(question.read().clone())
                    },
                    "Ask"
                }
            }

            {
                match answer {
                    None => rsx! {},
                    Some(AnswerView::Refusal(msg)) => rsx! { p { class: "refusal", "{msg}" } },
                    Some(AnswerView::Reflection(msg)) => rsx! { p { class: "reflection", "{msg}" } },
                    Some(AnswerView::Nudge(msg)) => rsx! { p { class: "nudge", "{msg}" } },
                    Some(AnswerView::Reading { name, ticker, label, pending, text }) => {
                        if pending {
                            rsx! {
                                div { class: "ask-reading ask-reading--pending",
                                    span { class: "wheat-caption",
                                        "Consulting the viziers… measuring {name} ({ticker})"
                                    }
                                }
                            }
                        } else {
                            let body = strip_fit_header(text);
                            rsx! {
                                div { class: "ask-reading",
                                    div { class: "ask-reading__head",
                                        span { class: "ask-reading__name", "{name}" }
                                        span { class: "ticker", "{ticker}" }
                                        span { class: "badge", "{label}" }
                                    }
                                    p { class: "reading", "{body}" }
                                }
                            }
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
