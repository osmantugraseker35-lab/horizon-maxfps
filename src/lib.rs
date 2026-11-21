//! Horizon Max FPS safety release.
//!
//! This build deliberately registers no runtime rendering or frame-rate hooks.
//! It exists only to replace older installations that could reduce performance.

use mod_api::{Hooks, Registrar, RegistrationContext};

#[derive(Default)]
struct MaxFpsMod;

#[mod_api::horizon_mod]
impl Hooks for MaxFpsMod {
    fn register(&self, _registrar: &mut dyn Registrar, _context: &RegistrationContext) {}
}
