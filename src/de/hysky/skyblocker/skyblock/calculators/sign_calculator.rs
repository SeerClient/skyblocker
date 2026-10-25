//! Ported from de.hysky.skyblocker.skyblock.calculators.SignCalculator by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::calculator::Calculator;
use crate::de::hysky::skyblocker::utils::formatters;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static String lastInput;
pub static LAST_INPUT: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(Default::default()));
/// java: private static double output;
pub static OUTPUT: LazyLock<Mutex<f64>> = LazyLock::new(|| Mutex::new(0.0));
/// java: private static Component error;
pub static ERROR: compat::MainThread<Option<MutableComponent>> = compat::MainThread::new(|| None);

/// java: public class SignCalculator
#[derive(Clone)]
pub struct SignCalculator {}

impl SignCalculator {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void extractCalculator(GuiGraphicsExtractor graphics, String message, int renderX, int renderY)
    pub fn extract_calculator(graphics: &GuiGraphicsExtractor, mut message: String, mut render_x: i32, mut render_y: i32) {
        if SkyblockerConfigManager::get().ui_and_visuals.input_calculator.requires_equals && !(message.starts_with("=")) {
            compat::set_state(&OUTPUT, ((-1) as f64) as f64);
            *ERROR.get_mut() = None;
            compat::set_state(&LAST_INPUT, message.clone());
            return;
        }
        if message.starts_with("=") {
            message = compat::substring(&message, 1);
        }
        SignCalculator::calculate(message.clone());
        SignCalculator::extract(graphics, &message, render_x, render_y);
    }

    /// java: @VisibleForTesting public static void calculate(String message)
    pub fn calculate(mut message: String) {
        if !(message == compat::get_state(&LAST_INPUT)) {
            compat::set_state(&LAST_INPUT, message.clone());
            // java: try {..} catch (Calculator.CalculatorException e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                compat::set_state(&OUTPUT, Calculator::calculate(message.clone()));
                *ERROR.get_mut() = None;
            }));
            if let Err(e) = __result {
                compat::set_state(&OUTPUT, ((-1) as f64) as f64);
                // java: error = Component.translatable(e.getMessage(), e.args).withStyle(ChatFormatting.RED);
                // (only a `CalculatorException` is caught: any other panic propagates)
                let exception = crate::de::hysky::skyblocker::utils::calculator::CalculatorException::of(&*e).cloned();
                let Some(exception) = exception else {
                    std::panic::resume_unwind(e);
                };
                *ERROR.get_mut() = Some(exception.to_component().with_style_format(ChatFormatting::Red));
            }
        }
    }

    /// java: public static String getNewValue(boolean isPrice)
    pub fn get_new_value(mut is_price: bool) -> String {
        if compat::get_state(&OUTPUT) == (-1 as f64) {
            return compat::get_state(&LAST_INPUT);
        }
        if is_price {
            return compat::fstr(((compat::java_round_f64((compat::get_state(&OUTPUT) * 100.0)) as f64) / 100.0) as f64);
        }
        return compat::java_round_f64(compat::get_state(&OUTPUT)).to_string();
    }

    /// java: private static void extract(GuiGraphicsExtractor graphics, String input, int renderX, int renderY)
    pub fn extract(graphics: &GuiGraphicsExtractor, input: &str, mut render_x: i32, mut render_y: i32) {
        let mut text: MutableComponent;
        if compat::get_state(&OUTPUT) == (-1 as f64) {
            text = (if ERROR.get_mut().is_some() { ERROR.get_mut().clone() } else { Some(component::translatable("skyblocker.config.uiAndVisuals.inputCalculator.invalidEquation").with_style_format(ChatFormatting::Red)) }).unwrap();
        } else {
            text = (component::literal(&format!("{} = {}", input, formatters::DOUBLE_NUMBERS.get_mut().format(compat::get_state(&OUTPUT))))).with_style_format(ChatFormatting::Green);
        }
        gui_graphics_extractor::centered_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone()), render_x, render_y, 0xFFFFFFFFu32 as i32);
    }

}
