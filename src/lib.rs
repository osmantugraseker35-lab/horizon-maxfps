//! Horizon Max FPS reduces optional rendering work to improve frame rate.

use mod_api::{
    ClientHost, Hooks, Registrar, RegistrationContext, RenderGuiPreEvent, RenderHandEvent,
    RenderItemInFrameEvent, RenderLivingPreEvent, SettingSpec, runtime::Flow,
};

#[derive(Default)]
struct MaxFpsMod;

impl MaxFpsMod {
    fn enabled(host: &dyn ClientHost) -> bool {
        host.setting_bool("enable_mod").unwrap_or(true)
    }

    fn should_skip(host: &dyn ClientHost, setting: &str, default: bool) -> Flow {
        if Self::enabled(host) && host.setting_bool(setting).unwrap_or(default) {
            Flow::Cancel
        } else {
            Flow::Continue
        }
    }
}

#[mod_api::horizon_mod]
impl Hooks for MaxFpsMod {
    fn register(&self, registrar: &mut dyn Registrar, _context: &RegistrationContext) {
        let _ = registrar.register_setting(SettingSpec::toggle("enable_mod", "Enable Horizon Max FPS", true));
        let _ = registrar.register_setting(SettingSpec::toggle("hide_first_person_hand", "Skip first-person hand and held-item rendering", true));
        let _ = registrar.register_setting(SettingSpec::toggle("hide_item_frame_contents", "Skip item-frame content rendering", true));
        let _ = registrar.register_setting(SettingSpec::toggle("minimal_hud", "Extreme: skip the entire HUD", false));
        let _ = registrar.register_setting(SettingSpec::toggle("hide_living_entities", "Extreme: skip all living-entity rendering", false));
    }

    fn render_hand(&self, host: &dyn ClientHost, _event: &mut RenderHandEvent<'_, '_>) -> Flow {
        Self::should_skip(host, "hide_first_person_hand", true)
    }

    fn render_item_in_frame(&self, host: &dyn ClientHost, _event: &mut RenderItemInFrameEvent<'_>) -> Flow {
        Self::should_skip(host, "hide_item_frame_contents", true)
    }

    fn render_gui_pre(&self, host: &dyn ClientHost, _event: &mut RenderGuiPreEvent) -> Flow {
        Self::should_skip(host, "minimal_hud", false)
    }

    fn render_living_pre(&self, host: &dyn ClientHost, _event: &mut RenderLivingPreEvent<'_>) -> Flow {
        Self::should_skip(host, "hide_living_entities", false)
    }
}
