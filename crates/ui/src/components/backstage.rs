//! Backstage — the receipts behind a card: the choice's full synastry measures as a raw mono table
//! (every cross-aspect, marked flowing/friction) and the live tool-call order as chips (the
//! `propose` and `pull_grounded` beats tagged distinctly).

use dioxus::prelude::*;

use crate::state::AppCtx;

#[component]
pub fn Backstage(choice: String) -> Element {
    let ctx = use_context::<AppCtx>();

    let measures = ctx.measures.read().get(&choice).cloned();
    let calls = ctx.calls.read().clone();

    let table = match measures {
        Some(m) if !m.aspects.is_empty() => {
            let rows = m
                .aspects
                .iter()
                .enumerate()
                .map(|(i, a)| {
                    let nature = if a.harmonious { "flowing" } else { "friction" };
                    let nature_cls = if a.harmonious { "flow" } else { "fric" };
                    let body_a = a.body_a.clone();
                    let aspect = a.aspect.clone();
                    let body_b = a.body_b.clone();
                    let orb = format!("{:.2}°", a.orb);
                    rsx! {
                        tr { key: "{i}",
                            td { class: "b", "{body_a}" }
                            td { "{aspect}" }
                            td { class: "b", "{body_b}" }
                            td { "{orb}" }
                            td { span { class: "{nature_cls}", "{nature}" } }
                        }
                    }
                })
                .collect::<Vec<_>>();

            rsx! {
                div { class: "bs__scroll",
                    table {
                        thead {
                            tr {
                                th { "Your body" }
                                th { "Aspect" }
                                th { "{choice} body" }
                                th { "Orb" }
                                th { "Nature" }
                            }
                        }
                        tbody { {rows.into_iter()} }
                    }
                }
            }
        }
        Some(_) => rsx! {
            p { class: "measured", span { class: "orb", "No cross-aspects within orb." } }
        },
        None => rsx! {
            p { class: "measured", span { class: "orb", "No measures for this choice." } }
        },
    };

    let chips = if calls.is_empty() {
        rsx! { li { "—" } }
    } else {
        let items = calls
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let cls = if c == "propose" {
                    "propose"
                } else if c.starts_with("pull_grounded") {
                    "pull"
                } else {
                    ""
                };
                rsx! { li { key: "{i}", class: "{cls}", "{c}" } }
            })
            .collect::<Vec<_>>();
        rsx! { {items.into_iter()} }
    };

    rsx! {
        details { class: "bs",
            summary { class: "bs__h",
                span { class: "caret", "▸" }
                " Backstage — the measures & tool order"
            }
            div { class: "bs__body",
                {table}
                div { class: "trace",
                    div { class: "eyebrow", style: "color:var(--ink-faint)", "Tool order" }
                    ol { {chips} }
                }
            }
        }
    }
}
