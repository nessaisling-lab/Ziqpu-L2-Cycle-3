//! Guardrail — the persistent no-advice surface. Advice-seeking questions get an explicit refusal
//! (never a signal); anything else, a plain reflection. The refusal is enforced in the loop's code,
//! not in a prompt.

use dioxus::prelude::*;

use agents::Answer;

use crate::state::AppCtx;

#[component]
pub fn Guardrail() -> Element {
    let ctx = use_context::<AppCtx>();
    let mut question = use_signal(|| "Should I buy Tesla?".to_string());
    let answer = ctx.answer.read().clone();

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
                }
                button {
                    class: "btn btn--go",
                    r#type: "button",
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
                        let cls = if refusal { "refusal" } else { "reflection" };
                        rsx! { p { class: "{cls}", "{msg}" } }
                    }
                    None => rsx! {},
                }
            }
        }

        p { class: "reminder", "measured, not fate — not financial advice" }
    }
}
