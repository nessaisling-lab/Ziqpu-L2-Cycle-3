//! Legend — a self-contained, always-available dictionary of the language the engine speaks: the
//! planets and points it weighs, the aspects it measures, what "flowing" vs "friction" means, and
//! the four fit bands. Plain-language definitions written from standard public astrology knowledge
//! (nothing fetched, nothing quoted). Rendered as a collapsed `<details>` parchment card so it never
//! competes with the reading — it opens when the seeker wants it.

use dioxus::prelude::*;

/// A single glossary row: a glyph, a term, and its plain-language meaning.
fn entry(glyph: &str, term: &str, meaning: &str) -> Element {
    rsx! {
        div { class: "legend-row",
            span { class: "legend-glyph", "aria-hidden": "true", "{glyph}" }
            div { class: "legend-def",
                span { class: "legend-term", "{term}" }
                span { class: "legend-meaning", "{meaning}" }
            }
        }
    }
}

#[component]
pub fn Legend() -> Element {
    // The planets + calculated points the engine emits (Sun … Pluto, the lunar nodes, Chiron).
    let planets = [
        (
            "☉",
            "Sun",
            "Core self — vitality, identity, the will and purpose you radiate.",
        ),
        (
            "☽",
            "Moon",
            "Emotion and instinct — inner needs, moods, what makes you feel safe.",
        ),
        (
            "☿",
            "Mercury",
            "Mind — how you think, learn, speak, and connect ideas.",
        ),
        (
            "♀",
            "Venus",
            "Values and attraction — love, taste, pleasure, what you find beautiful.",
        ),
        (
            "♂",
            "Mars",
            "Drive — energy, assertion, desire, how you act and pursue.",
        ),
        (
            "♃",
            "Jupiter",
            "Growth — expansion, optimism, opportunity, faith, and luck.",
        ),
        (
            "♄",
            "Saturn",
            "Limits and structure — discipline, responsibility, time, and maturity.",
        ),
        (
            "♅",
            "Uranus",
            "Change — disruption, independence, sudden insight, the unexpected.",
        ),
        (
            "♆",
            "Neptune",
            "Dreams — intuition, imagination, compassion, dissolving of boundaries.",
        ),
        (
            "♇",
            "Pluto",
            "Intensity — power, depth, and slow, thorough transformation.",
        ),
        (
            "☊",
            "Lunar Nodes",
            "The Moon's nodes — a directional axis of pull and release (weighed lightly).",
        ),
        (
            "⚷",
            "Chiron",
            "The wound that teaches — where hurt becomes hard-won skill (weighed lightly).",
        ),
    ];

    // The five Ptolemaic aspects the engine measures — angular relationships between two bodies.
    let aspects = [
        (
            "☌",
            "Conjunction · 0°",
            "Fusion — two forces sit together and amplify each other.",
        ),
        (
            "⚹",
            "Sextile · 60°",
            "Flowing — easy opportunity and gentle, willing support.",
        ),
        (
            "△",
            "Trine · 120°",
            "Flowing — natural harmony and ease; talent that comes freely.",
        ),
        (
            "□",
            "Square · 90°",
            "Friction — tension that demands effort and forces growth.",
        ),
        (
            "☍",
            "Opposition · 180°",
            "Friction — a push-pull between two ends seeking balance.",
        ),
    ];

    rsx! {
        details { class: "legend",
            summary { class: "legend-summary",
                span { class: "legend-summary__mark", "aria-hidden": "true", "✦" }
                span { "Legend — the dictionary of the sky" }
                span { class: "caret legend-summary__caret", "aria-hidden": "true", "›" }
            }

            div { class: "legend-body",
                p { class: "legend-intro",
                    "Every reading is built from these. A chart weighs the "
                    em { "planets" }
                    " of two moments against one another through the "
                    em { "aspects" }
                    " between them — and the balance of flowing versus friction sets the fit."
                }

                section { class: "legend-section",
                    h4 { class: "legend-heading", "Planets & points" }
                    div { class: "legend-grid",
                        {planets.iter().map(|(g, t, m)| rsx! {
                            {entry(g, t, m)}
                        })}
                    }
                }

                section { class: "legend-section",
                    h4 { class: "legend-heading", "Aspects" }
                    div { class: "legend-grid",
                        {aspects.iter().map(|(g, t, m)| rsx! {
                            {entry(g, t, m)}
                        })}
                    }
                }

                section { class: "legend-section",
                    h4 { class: "legend-heading", "Flowing vs friction" }
                    div { class: "legend-grid",
                        div { class: "legend-row",
                            span { class: "legend-glyph legend-glyph--flow", "aria-hidden": "true", "↝" }
                            div { class: "legend-def",
                                span { class: "legend-term", "Flowing" }
                                span { class: "legend-meaning",
                                    "Sextiles, trines, and easy conjunctions — energy moves without resistance. Comfort, support, natural talent."
                                }
                            }
                        }
                        div { class: "legend-row",
                            span { class: "legend-glyph legend-glyph--fric", "aria-hidden": "true", "↯" }
                            div { class: "legend-def",
                                span { class: "legend-term", "Friction" }
                                span { class: "legend-meaning",
                                    "Squares and oppositions — energy meets resistance. Tension and challenge; the grit that forces growth."
                                }
                            }
                        }
                    }
                }

                section { class: "legend-section",
                    h4 { class: "legend-heading", "Fit bands" }
                    div { class: "legend-grid",
                        div { class: "legend-row",
                            span { class: "legend-band", style: "--b:var(--band-strong)", "aria-hidden": "true" }
                            div { class: "legend-def",
                                span { class: "legend-term", "Strongly Aligned · 75–100" }
                                span { class: "legend-meaning", "Flowing contacts dominate with little friction — the pairing sings together." }
                            }
                        }
                        div { class: "legend-row",
                            span { class: "legend-band", style: "--b:var(--band-aligned)", "aria-hidden": "true" }
                            div { class: "legend-def",
                                span { class: "legend-term", "Aligned · 60–74" }
                                span { class: "legend-meaning", "Mostly flowing with some friction — a solid, workable match." }
                            }
                        }
                        div { class: "legend-row",
                            span { class: "legend-band", style: "--b:var(--band-mixed)", "aria-hidden": "true" }
                            div { class: "legend-def",
                                span { class: "legend-term", "Mixed · 40–59" }
                                span { class: "legend-meaning", "Flowing and friction roughly balanced — real strengths and real snags." }
                            }
                        }
                        div { class: "legend-row",
                            span { class: "legend-band", style: "--b:var(--band-misalign)", "aria-hidden": "true" }
                            div { class: "legend-def",
                                span { class: "legend-term", "Misaligned · 0–39" }
                                span { class: "legend-meaning", "Friction dominates — the two pull in different directions." }
                            }
                        }
                    }
                }

                p { class: "legend-foot", "measured, not fate — a symbolic lens, not financial advice." }
            }
        }
    }
}
