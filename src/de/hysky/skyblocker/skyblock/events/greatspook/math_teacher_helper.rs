//! Ported from de.hysky.skyblocker.skyblock.events.greatspook.MathTeacherHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::calculator::Calculator;
use crate::de::hysky::skyblocker::utils::constants;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::click_event::ClickEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::contents::plain_text_contents::PlainTextContents;
use mcrust::net::minecraft::network::chat::hover_event::HoverEvent;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public final class MathTeacherHelper
#[derive(Clone)]
pub struct MathTeacherHelper {}

impl MathTeacherHelper {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // java: ClientReceiveMessageEvents.MODIFY_GAME.register(MathTeacherHelper::onMessage);
        compat::register_modify_game(|message, overlay| MathTeacherHelper::on_message(message, overlay));
    }

    /// java: public static Component onMessage(Component message, boolean overlay)
    pub fn on_message(message: &MutableComponent, mut overlay: bool) -> MutableComponent {
        if overlay || !(SkyblockerConfigManager::get().helpers.great_spook_event.enable_math_teacher_helper) {
            return message.clone();
        }
        let siblings = message.get_siblings();
        // `message.getContents() != PlainTextContents.EMPTY`: the EMPTY singleton
        if !matches!(&**message.get_contents(), mcrust::net::minecraft::network::chat::component_contents::ComponentContents::Text(PlainTextContents::Empty)) || (siblings.len() as i32) != 3 {
            return message.clone();
        }
        if !(siblings[0].get_string() == "QUICK MATHS! ") {
            return message.clone();
        }
        let mut expression: String = ((siblings[(2) as usize]).get_string()).replace('x', &'*'.to_string());
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut result: String = compat::string_format("%.0f", &[Calculator::calculate(expression.clone()).into()]);
            return (message.copy().append_str(" = ")).append(component::literal(&result).with_style_format(ChatFormatting::Aqua)).append(compat::component_of(mcrust::net::minecraft::network::chat::common_components::space())).append(component::translatable("text.skyblocker.clickToSend").with_style_format(ChatFormatting::Green).with_style_fn(|style| (style.with_click_event(Some(ClickEvent::new_run_command(format!("/ac {}", result))))).with_hover_event(Some(HoverEvent::new_show_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from((constants::PREFIX.get_mut())().append(component::translatable("text.skyblocker.clickToSend.@Tooltip"))))))));
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                logger().error(&format!("[Skyblocker Math Teacher Helper] Failed to calculate math expression: {}", expression));
                return message.clone();
            }
        }
    }

}
