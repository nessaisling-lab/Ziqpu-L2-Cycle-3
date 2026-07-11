//! "Your sky" — the seeker's own daily + weekly self-reading, shown under the Identity card in
//! Setup. It's the *you* half of the product: before weighing any choice, here is today's tightest
//! transit and the shape of your coming week.
//!
//! The engine methods ([`agents::Session::daily_reading`] / `weekly_reading`) are deterministic and
//! take the date as a parameter — they never read the clock, so the graded loop stays reproducible.
//! The clock lives *here*, at the UI edge: `chrono::Local::now()` supplies "today", from which the
//! week runs. Both methods take `&self` and record no graded tool call, so borrowing the shared
//! session to compute them is safe.

use dioxus::prelude::*;

use crate::state::AppCtx;

#[component]
pub fn YourSky() -> Element {
    let ctx = use_context::<AppCtx>();
    let seeker = ctx.seeker.read().clone();

    // Today (local) drives both reads; the week runs the seven days from today. The engine stays
    // clock-free — this is the one place the real date enters.
    let today = chrono::Local::now().date_naive();
    let daily = ctx.session.borrow().daily_reading(&seeker, today);
    let weekly = ctx.session.borrow().weekly_reading(&seeker, today);

    rsx! {
        section { class: "your-sky",
            p { class: "eyebrow", "Your sky" }
            div { class: "sky-read",
                p { class: "sky-line", "{daily.reading}" }
                p { class: "sky-line", "{weekly.reading}" }
            }
        }
    }
}
