//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.element.Elements by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::ui_and_visuals_config::TabHudStyle;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::ico_fat_text_element::IcoFatTextElement;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::ico_text_element::IcoTextElement;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::plain_text_element::PlainTextElement;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::progress_element::ProgressElement;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class Elements
#[derive(Clone)]
pub struct Elements {}

impl Elements {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static Element iconTextComponent()
    pub fn icon_text_component() -> compat::Shared<dyn Element> {
        return Elements::icon_text_component_2(None, None);
    }

    /// java: public static Element iconTextComponent(@Nullable FlexibleItemStack icon, @Nullable Component text)
    pub fn icon_text_component_2(mut icon: Option<compat::Shared<FlexibleItemStack>>, mut text: Option<MutableComponent>) -> compat::Shared<dyn Element> {
        if SkyblockerConfigManager::get().ui_and_visuals.tab_hud.display_icons {
            return (IcoTextElement::new(icon.clone(), text.clone()) as compat::Shared<dyn Element>);
        } else {
            return (PlainTextElement::new(text.clone()) as compat::Shared<dyn Element>);
        }
    }

    /// java: public static Element iconFatTextComponent()
    pub fn icon_fat_text_component() -> compat::Shared<dyn Element> {
        return Elements::icon_fat_text_component_3(None, None, None);
    }

    /// java: public static Element iconFatTextComponent(@Nullable FlexibleItemStack icon, @Nullable Component line1, @Nullable Component line2)
    pub fn icon_fat_text_component_3(mut icon: Option<compat::Shared<FlexibleItemStack>>, mut line1: Option<MutableComponent>, mut line2: Option<MutableComponent>) -> compat::Shared<dyn Element> {
        if SkyblockerConfigManager::get().ui_and_visuals.tab_hud.display_icons {
            return (IcoFatTextElement::new(icon.clone(), line1.clone(), line2.clone()) as compat::Shared<dyn Element>);
        } else {
            return (PlainTextElement::new_2(line1.clone(), line2.clone()) as compat::Shared<dyn Element>);
        }
    }

    /// java: public static Element progressComponent()
    pub fn progress_component() -> compat::Shared<dyn Element> {
        return match SkyblockerConfigManager::get().ui_and_visuals.tab_hud.style {
            TabHudStyle::Fancy => (ProgressElement::new_0() as compat::Shared<dyn Element>),
            _ => Elements::icon_text_component(),
        };
    }

    /// java: public static Element progressComponent(@Nullable FlexibleItemStack icon, Component description, @Range(from = 0, to = 100) float percent)
    pub fn progress_component_3(mut icon: Option<compat::Shared<FlexibleItemStack>>, description: &MutableComponent, mut percent: f32) -> compat::Shared<dyn Element> {
        return match SkyblockerConfigManager::get().ui_and_visuals.tab_hud.style {
            TabHudStyle::Fancy => (ProgressElement::new_3(icon.clone(), Some(description.clone()), percent) as compat::Shared<dyn Element>),
            _ => Elements::icon_text_component_2(icon.clone(), Some(Elements::append_colon(description).append((component::literal(&format!("{}%", compat::fstr(percent as f64)))).with_color(ColorUtils::percent_to_color(percent))))),
        };
    }

    /// java: public static Element progressComponent(@Nullable FlexibleItemStack icon, Component description, @Range(from = 0, to = 100) float percent, int color)
    pub fn progress_component_flexible_item_stack_component_float_int(mut icon: Option<compat::Shared<FlexibleItemStack>>, description: &MutableComponent, mut percent: f32, mut color: i32) -> compat::Shared<dyn Element> {
        return match SkyblockerConfigManager::get().ui_and_visuals.tab_hud.style {
            TabHudStyle::Fancy => (ProgressElement::new3(icon.clone(), Some(description.clone()), percent, color) as compat::Shared<dyn Element>),
            _ => Elements::icon_text_component_2(icon.clone(), Some(Elements::append_colon(description).append((component::literal(&format!("{}%", compat::fstr(percent as f64)))).with_color(color)))),
        };
    }

    /// java: public static Element progressComponent(@Nullable FlexibleItemStack icon, Component description, Component bar, @Range(from = 0, to = 100) float percent)
    pub fn progress_component_flexible_item_stack_component_component_float(mut icon: Option<compat::Shared<FlexibleItemStack>>, description: &MutableComponent, bar: &MutableComponent, mut percent: f32) -> compat::Shared<dyn Element> {
        return match SkyblockerConfigManager::get().ui_and_visuals.tab_hud.style {
            TabHudStyle::Fancy => (ProgressElement::new2(icon.clone(), Some(description.clone()), Some(bar.clone()), percent) as compat::Shared<dyn Element>),
            _ => Elements::icon_text_component_2(icon.clone(), Some(Elements::append_colon(description).append(bar.copy().with_color(ColorUtils::percent_to_color(percent))))),
        };
    }

    /// java: public static Element progressComponent(@Nullable FlexibleItemStack icon, Component description, Component bar, @Range(from = 0, to = 100) float percent, int color)
    pub fn progress_component_5(mut icon: Option<compat::Shared<FlexibleItemStack>>, description: &MutableComponent, bar: &MutableComponent, mut percent: f32, mut color: i32) -> compat::Shared<dyn Element> {
        return match SkyblockerConfigManager::get().ui_and_visuals.tab_hud.style {
            TabHudStyle::Fancy => (ProgressElement::new(icon.clone(), Some(description.clone()), Some(bar.clone()), percent, color) as compat::Shared<dyn Element>),
            _ => Elements::icon_text_component_2(icon.clone(), Some(Elements::append_colon(description).append(bar.copy().with_color(color)))),
        };
    }

    /// java: private static MutableComponent appendColon(Component text)
    pub fn append_colon(text: &MutableComponent) -> MutableComponent {
        let mut string: String = text.get_string();
        if string.ends_with(": ") {
            return text.copy();
        } else if string.ends_with(":") {
            return text.copy().append_str(" ");
        } else {
            return text.copy().append_str(": ");
        }
    }

}
