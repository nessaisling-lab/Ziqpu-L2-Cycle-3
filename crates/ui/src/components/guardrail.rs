//! Guardrail — the persistent no-advice surface. Advice-seeking questions get an explicit refusal
//! (never a signal); anything else, a plain reflection. The refusal is enforced in the loop's code,
//! not in a prompt.

use dioxus::prelude::*;

use agents::Answer;

use crate::state::AppCtx;

#[component]
pub fn Guardrail() -> Element {
    let ctx = use_context::<AppCtx>();
    let mut question = use_signal(|| "Should I buy AAPL?".to_string());
    let answer = ctx.answer.read().clone();

    rsx! {
        section { class: "card guardrail",
            h2 { "Ask" }
            div { class: "ask-row",
                input {
                    class: "ask-input",
                    value: "{question}",
                    oninput: move |e| question.set(e.value()),
                }
                button {
                    class: "primary",
                    onclick: {
                        let mut ctx = ctx.clone();
                        move |_| {
                            let q = question.read().clone();
                            let pair = match ctx.session.borrow().ask(&q) {
                                Answer::Refusal(m) => (true, m),
                                Answer::Reflection(m) => (false, m),
                            };
                            ctx.answer.set(Some(pair));
                        }
                    },
                    "Ask"
                }
            }

            {
                match answer {
                    Some((refusal, msg)) => {
                        let cls = if refusal { "answer refusal" } else { "answer reflection" };
                        rsx! { p { class: "{cls}", "{msg}" } }
                    }
                    None => rsx! {},
                }
            }

            footer { class: "disclaimer", "measured, not fate — not financial advice." }
        }
    }
}
