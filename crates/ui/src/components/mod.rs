//! One component per file, wired to the loop's phases.

mod backstage;
mod briefing;
mod checkpoint;
mod fit_card;
mod guardrail;
mod ranked;
mod setup;

pub use backstage::Backstage;
pub use briefing::Briefing;
pub use checkpoint::Checkpoint;
pub use fit_card::FitCard;
pub use guardrail::Guardrail;
pub use ranked::Ranked;
pub use setup::Setup;
