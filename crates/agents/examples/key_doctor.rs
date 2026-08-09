//! Diagnose a stored provider key **without ever revealing it**.
//!
//! ```text
//! cargo run -p agents --example key_doctor
//! ```
//!
//! # Why shape, and only shape
//!
//! The standing rule on this project is that a key is never shown — the UI reports presence, never
//! the value, and a source-level test enforces that no surface calls `get_key`. That rule is what
//! makes a 401 hard to diagnose: "the key is present" and "the key is correct" look identical from
//! outside, which is exactly the confusion that let a rejected key sit in the vault for a month
//! while every screen said "live".
//!
//! Shape is the middle ground. Length, prefix, and whether stray whitespace crept in are enough to
//! separate the common causes — a truncated paste, a key with a trailing newline, a value from the
//! wrong provider — from "the key is well-formed and the server still refuses it", which means
//! revoked or rotated. None of that discloses the secret: a length is not a key, and the prefix is
//! printed on the provider's own dashboard.
//!
//! What it deliberately does **not** print: any part of the key body, and any full character count
//! that could narrow a guess meaningfully — the length is reported against the expected length, not
//! as a fingerprint.

use agents::vault::{key_source, Provider};

fn main() {
    println!("Provider key diagnosis — shape only, no value is read out.\n");

    for provider in [Provider::Anthropic, Provider::OpenRouter] {
        println!("── {}", provider.label());
        println!("   source   {:?}", key_source(provider));
        println!("   env var  {}", provider.env_var());

        match agents::vault::key_shape(provider) {
            None => println!("   shape    (nothing stored)\n"),
            Some(shape) => {
                println!("   shape    {shape}");
                for note in shape.diagnosis(provider) {
                    println!("            !! {note}");
                }
                println!();
            }
        }
    }

    println!(
        "A well-formed key that still returns 401 has been revoked or rotated on the provider's\n\
         side — the fix is a new key, not a repair. Check the dashboard's \"last used\" column: a\n\
         key that never shows recent use was never the one authenticating."
    );
}
