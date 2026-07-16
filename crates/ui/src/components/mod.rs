//! One component per file, wired to the loop's phases.

mod backstage;
mod birth_input;
mod briefing;
mod checkpoint;
mod fit_card;
mod guardrail;
mod identity;
mod legend;
mod lock;
mod model_panel;
mod model_picker;
mod onboarding;
mod ranked;
mod settings;
mod setup;
mod wheat_loader;
mod your_sky;

pub use backstage::Backstage;
pub use birth_input::{draft_to_moment, BirthInputForm};
pub use briefing::Briefing;
pub use checkpoint::Checkpoint;
pub use fit_card::FitCard;
pub use guardrail::Guardrail;
pub use identity::Identity;
pub use legend::Legend;
pub use lock::PremiumLock;
pub use model_panel::ModelPanel;
pub use model_picker::ModelPicker;
pub use onboarding::Onboarding;
pub use ranked::Ranked;
pub use settings::SettingsButton;
pub use setup::Setup;
pub use wheat_loader::{WheatLoader, WheatPhase};
pub use your_sky::YourSky;
