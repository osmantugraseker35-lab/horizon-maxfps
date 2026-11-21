//! Horizon Max FPS provides optional, user-controlled rendering reductions.
//! It never hides the first-person hand or held item.

use mod_api::{
    ClientHost, Hooks, Registrar, RegistrationContext, RenderGuiPreEvent,
    RenderItemInFrameEvent, RenderLivingPreEvent, SettingSpec, runtime::Flow,
};

#[derive(Default)]
struct MaxFpsMod;

impl MaxFpsMod {
    fn enabled(host: &dyn ClientHost) -> bool {
        host.setting_bool("enable_mod").unwrap_or(true)
    }

    fn optional_skip(host: &dyn ClientHost, setting: &str) -> Flow {
        if Self::enabled(host) && host.setting_bool(setting).unwrap_or(false) {
            Flow::Cancel
        } else {
            Flow::Continue
        }
    }
}

#[mod_api::horizon_mod]
impl Hooks for MaxFpsMod {
    fn register(&self, registrar: &mut dyn Registrar, _context: &RegistrationContext) {
        let _ = registrar.register_setting(SettingSpec::toggle(
            "enable_mod",
            "Enable Horizon Max FPS",
            true,
        ));
        let _ = registrar.register_setting(SettingSpec::toggle(
            "hide_item_frame_contents",
            "Performance mode: skip item-frame contents",
            false,
        ));
        let _ = registrar.register_setting(SettingSpec::toggle(
            "minimal_hud",
            "Extreme mode: hide the HUD",
            false,
        ));
        let _ = registrar.register_setting(SettingSpec::toggle(
            "hide_living_entities",
            "Extreme mode: hide living entities",
            false,
        ));
    }

    fn render_item_in_frame(
        &self,
        host: &dyn ClientHost,
        _event: &mut RenderItemInFrameEvent<'_>,
    ) -> Flow {
        Self::optional_skip(host, "hide_item_frame_contents")
    }

    fn render_gui_pre(&self, host: &dyn ClientHost, _event: &mut RenderGuiPreEvent) -> Flow {
        Self::optional_skip(host, "minimal_hud")
    }

    fn render_living_pre(
        &self,
        host: &dyn ClientHost,
        _event: &mut RenderLivingPreEvent<'_>,
    ) -> Flow {
        Self::optional_skip(host, "hide_living_entities")
    }
}
