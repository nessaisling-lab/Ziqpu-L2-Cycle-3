//! One component per file, wired to the loop's phases.

mod backstage;
mod birth_input;
mod briefing;
mod checkpoint;
mod fit_card;
mod guardrail;
mod ranked;
mod setup;

pub use backstage::Backstage;
pub use birth_input::{draft_to_moment, BirthInputForm};
pub use briefing::Briefing;
pub use checkpoint::Checkpoint;
pub use fit_card::FitCard;
pub use guardrail::Guardrail;
pub use ranked::Ranked;
pub use setup::Setup;
