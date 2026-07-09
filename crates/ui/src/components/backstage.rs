//! Backstage — the receipts behind a card: the choice's full synastry measures (every cross-aspect,
//! marked flowing/friction) and the live tool-call order from the graded session.

use dioxus::prelude::*;

use crate::state::AppCtx;

#[component]
pub fn Backstage(choice: String) -> Element {
    let ctx = use_context::<AppCtx>();

    let measures = ctx.measures.read().get(&choice).cloned();
    let calls = ctx.calls.read().clone();

    let body = match measures {
        Some(m) => {
            let score = m.score;
            let rows = m
                .aspects
                .iter()
                .enumerate()
                .map(|(i, a)| {
                    let flow = if a.harmonious { "flowing" } else { "friction" };
                    let flow_cls = if a.harmonious {
                        "flow-good"
                    } else {
                        "flow-bad"
                    };
                    let body_a = a.body_a.clone();
                    let aspect = a.aspect.clone();
                    let body_b = a.body_b.clone();
                    let orb_val = a.orb;
                    let orb = format!("{orb_val:.2}°");
                    rsx! {
                        tr { key: "{i}",
                            td { "{body_a}" }
                            td { "{aspect}" }
                            td { "{body_b}" }
                            td { class: "orb", "{orb}" }
                            td { class: "{flow_cls}", "{flow}" }
                        }
                    }
                })
                .collect::<Vec<_>>();

            rsx! {
                div { class: "backstage-body",
                    div { class: "measure-score", "Synastry score: {score} / 100" }
                    if rows.is_empty() {
                        p { class: "muted", "No cross-aspects within orb." }
                    } else {
                        table { class: "aspect-table",
                            thead {
                                tr {
                                    th { "You" }
                                    th { "Aspect" }
                                    th { "Choice" }
                                    th { "Orb" }
                                    th { "Flow" }
                                }
                            }
                            tbody { {rows.into_iter()} }
                        }
                    }
                }
            }
        }
        None => rsx! {
            div { class: "backstage-body muted", "No measures for this choice." }
        },
    };

    let tool_order = if calls.is_empty() {
        rsx! { span { class: "muted", "—" } }
    } else {
        let items = calls
            .iter()
            .enumerate()
            .map(|(i, c)| rsx! { li { key: "{i}", "{c}" } })
            .collect::<Vec<_>>();
        rsx! { ol { class: "calls", {items.into_iter()} } }
    };

    rsx! {
        details { class: "backstage",
            summary { "Backstage — the measures & tool order" }
            {body}
            div { class: "tool-order",
                span { class: "tool-label", "Tool order" }
                {tool_order}
            }
        }
    }
}
