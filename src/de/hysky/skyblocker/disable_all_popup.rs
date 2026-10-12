//! Ported from de.hysky.skyblocker.DisableAllPopup by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::disable_all::DisableAll;
use crate::de::hysky::skyblocker::utils::countdown_component::CountdownComponent;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::toasts::system_toast;
use mcrust::net::minecraft::client::gui::components::toasts::system_toast::SystemToastId;
use mcrust::net::minecraft::client::gui::gui;
use mcrust::net::minecraft::client::gui::screens::screen;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final long DELAY_S = 10;
pub const DELAY_S: i64 = ((10) as i64) as i64;
/// java: private static final long DELAY_MS = DELAY_S * 1000;
pub static DELAY_MS: LazyLock<i64> = LazyLock::new(|| DELAY_S.wrapping_mul(1000 as i64));
/// java: private static final SystemToast.SystemToastId TOAST_ID = new SystemToast.SystemToastId();
pub static TOAST_ID: compat::MainThread<SystemToastId> = compat::MainThread::new(|| SystemToastId::new());

/// java: public final class DisableAllPopup
#[derive(Clone)]
pub struct DisableAllPopup {
    /// java: private final long end;
    pub end: i64,
}

impl DisableAllPopup {
    /// java: public DisableAllPopup()
    pub fn new() -> Self {
        let __init_end = (compat::millis()).wrapping_add(*DELAY_MS);
        let mut this = Self {
            end: __init_end,
        };
        this
    }

    /// java: public void open(Screen parent)
    pub fn open(&self, parent: &mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        let mut confirm_prefix: MutableComponent = component::translatable("text.skyblocker.confirm").with_color(mcrust::net::minecraft::util::common_colors::RED).with_style_fn(|style| style.with_bold(Some(true)));
        let mut confirm_component: compat::Shared<CountdownComponent> = CountdownComponent::new(DELAY_S, compat::TimeUnit::Seconds, confirm_prefix.clone(), mcrust::net::minecraft::network::chat::style::Style::EMPTY);
        // TODO(fabric2seer): `addButton` on a void
        //   java: PopupScreen.Builder builder = new PopupScreen.Builder(parent, Component.translatable("skyblocker.disableAll.popup.title"))
        //   java: 				.addMessage(Component.translatable("skyblocker.disableAll.popup.warning", Component.translatable("skyblocker.disableAll.popup.seriousWarning").withColor(CommonColors.SOFT_RED)))
        //   java: 				.addButton(confirmComponent, this::tryReset)
        //   java: 				.addButton(CommonComponents.GUI_CANCEL, PopupScreen::onClose);
        let mut builder: crate::unresolved::PopupScreen_Builder = todo!("fabric2seer: the initialiser of `builder` (TODO above)");
        // TODO(fabric2seer): `Builder.build` has no Rust method in the port (looked for `build` in PopupScreen and its supertypes)
        // TODO(fabric2seer): `set_screen` takes 14 argument(s) in the port (`gfx: &mut Minecraft, screen: &mut GuiScreen, screen_extra: &mut ScreenExtra, screen_stack: &mut Vec<HeldScreen>, screen_kind_before_pause: &mut Option<ScreenKind>, screen_is_open: &mut bool, level_screen_closed: &mut bool, frames_on_this_sc ..`), Java passes 1
        // rust: gui::set_screen(&mut builder.build());
    }

    /// java: private void tryReset(PopupScreen popupScreen)
    pub fn try_reset(&self, popup_screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        if compat::millis() >= self.end {
            let minecraft = compat::minecraft();
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                DisableAll::disable_all_0();
                // TODO(fabric2seer): `on_close_for_interrupt` takes 12 argument(s) in the port (`gfx: &mut crate::net::minecraft::client::minecraft::Minecraft, screen: &mut Screen, screen_extra: &mut crate::net::minecraft::client::gui::gui::ScreenExtra, options_data: &mut crate::net::minecraft::client::options::OptionsData, open_sign_e ..`), Java passes 0
                // rust: screen::on_close_for_interrupt();
                // TODO(fabric2seer): `set_screen` takes 14 argument(s) in the port (`gfx: &mut Minecraft, screen: &mut GuiScreen, screen_extra: &mut ScreenExtra, screen_stack: &mut Vec<HeldScreen>, screen_kind_before_pause: &mut Option<ScreenKind>, screen_is_open: &mut bool, level_screen_closed: &mut bool, frames_on_this_sc ..`), Java passes 1
                // rust: gui::set_screen(&mut SkyblockerConfigManager::create_gui(None));
                // TODO(fabric2seer): `add` takes 5 argument(s) in the port (`toast_manager: &mut ToastManager, font: &FontSet, id: SystemToastId, title: &FormattedText, message: Option<&FormattedText>`), Java passes 4
                // rust: system_toast::add(&mut minecraft.gui.toast_manager(), TOAST_ID.get_mut(), component::translatable("skyblocker.disableAll.toast.title"), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.disableAll.success").with_style_format(ChatFormatting::Red)));
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker DisableAll] Failed to disable all features"));
                // TODO(fabric2seer): `add` takes 5 argument(s) in the port (`toast_manager: &mut ToastManager, font: &FontSet, id: SystemToastId, title: &FormattedText, message: Option<&FormattedText>`), Java passes 4
                // rust: system_toast::add(&mut minecraft.gui.toast_manager(), TOAST_ID.get_mut(), component::translatable("skyblocker.disableAll.toast.title"), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.disableAll.failed").with_style_format(ChatFormatting::Red)));
            }
        }
    }

}
