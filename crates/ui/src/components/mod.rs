//! One component per file, wired to the loop's phases.

mod backstage;
mod birth_input;
mod briefing;
mod checkpoint;
mod fit_card;
mod guardrail;
mod legend;
mod ranked;
mod settings;
mod setup;

pub use backstage::Backstage;
pub use birth_input::{draft_to_moment, BirthInputForm};
pub use briefing::Briefing;
pub use checkpoint::Checkpoint;
pub use fit_card::FitCard;
pub use guardrail::Guardrail;
pub use legend::Legend;
pub use ranked::Ranked;
pub use settings::SettingsButton;
pub use setup::Setup;
