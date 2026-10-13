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
        // rust: port gap - `PopupScreen.Builder` (a vanilla popup with mod callbacks on its buttons) has no modding
        // API: the game builds its popups itself (`gui::PopupState`), and a mod can only open a `ModScreen` (GAPS_L.md)
        // rust: gui::set_screen(&mut builder.build());
    }

    /// `SystemToast.add(minecraft.gui.toastManager(), TOAST_ID, Component.translatable("skyblocker.disableAll.toast.title"),
    /// Component.translatable(message).withStyle(ChatFormatting.RED))` - the two calls of `tryReset`.
    fn add_toast(message: &str) {
        let minecraft = compat::minecraft();
        let title = mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.disableAll.toast.title"));
        let message = mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable(message).with_style_format(ChatFormatting::Red));
        system_toast::add(&mut minecraft.gui.toast_manager().borrow_mut(), &minecraft.font, *TOAST_ID.get(), &title, Some(&message));
    }

    /// java: private void tryReset(PopupScreen popupScreen)
    pub fn try_reset(&self, popup_screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        if compat::millis() >= self.end {
            let minecraft = compat::minecraft();
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                DisableAll::disable_all_0();
                // java: popupScreen.onClose(); - `PopupScreen.onClose` sets the screen back to its background screen
                mcrust::mcrust::modding::game::set_screen_to_parent();
                // java: minecraft.gui.setScreen(SkyblockerConfigManager.createGUI(null));
                crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::create_gui(None).set();
                // java: SystemToast.add(minecraft.gui.toastManager(), TOAST_ID, Component.translatable("skyblocker.disableAll.toast.title"), Component.translatable("skyblocker.disableAll.success").withStyle(ChatFormatting.RED));
                DisableAllPopup::add_toast("skyblocker.disableAll.success");
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker DisableAll] Failed to disable all features"));
                // java: SystemToast.add(minecraft.gui.toastManager(), TOAST_ID, Component.translatable("skyblocker.disableAll.toast.title"), Component.translatable("skyblocker.disableAll.failed").withStyle(ChatFormatting.RED));
                DisableAllPopup::add_toast("skyblocker.disableAll.failed");
            }
        }
    }

}
