//! Horizon Max FPS requests a high foreground frame-rate limit without
//! intercepting rendering. It never hides or cancels visual elements.

use mod_api::{
    ClientHost, FramerateLimitEvent, Hooks, Registrar, RegistrationContext, SettingSpec,
};

const TARGET_FPS: i32 = 1000;

#[derive(Default)]
struct MaxFpsMod;

#[mod_api::horizon_mod]
impl Hooks for MaxFpsMod {
    fn register(&self, registrar: &mut dyn Registrar, _context: &RegistrationContext) {
        let _ = registrar.register_setting(SettingSpec::toggle(
            "enable_mod",
            "Request a 1000 FPS limit",
            true,
        ));
    }

    fn framerate_limit(&self, host: &dyn ClientHost, event: &mut FramerateLimitEvent) {
        if !host.setting_bool("enable_mod").unwrap_or(true) {
            return;
        }

        if event.focused && !event.minimized {
            event.limit = event.limit.max(TARGET_FPS);
        }
    }
}
