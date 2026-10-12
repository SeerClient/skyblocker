//! Ported from de.hysky.skyblocker.skyblock.item.background.adders.JacobMedalBackground by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::background::colored_item_background::ColoredItemBackground;
use crate::de::hysky::skyblocker::skyblock::item::background::colored_item_background::ColoredItemBackgroundBase;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::util::argb;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;

/// java: private static final Pattern PATTERN = Pattern.compile("You placed in the (?<bracket>DIAMOND|PLATINUM|GOLD|SILVER|BRONZE) bracket!");
pub static PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("You placed in the (?<bracket>DIAMOND|PLATINUM|GOLD|SILVER|BRONZE) bracket!"));
/// java: private static final Map<String, Integer> BRACKET_COLORS = Map.of( "DIAMOND", Objects.requireNonNull(TextColor.AQUA.getValue()), "PLATINUM", Objects.requireNonNull(TextColor.DARK_AQUA.getValue()), "GO
pub static BRACKET_COLORS: LazyLock<std::collections::HashMap<String, i32>> = LazyLock::new(|| std::collections::HashMap::from([("DIAMOND".to_string(), mcrust::net::minecraft::network::chat::text_color::TextColor::AQUA.get_value()), ("PLATINUM".to_string(), mcrust::net::minecraft::network::chat::text_color::TextColor::DARK_AQUA.get_value()), ("GOLD".to_string(), mcrust::net::minecraft::network::chat::text_color::TextColor::GOLD.get_value()), ("SILVER".to_string(), mcrust::net::minecraft::network::chat::text_color::TextColor::WHITE.get_value()), ("BRONZE".to_string(), mcrust::net::minecraft::network::chat::text_color::TextColor::RED.get_value())]));

/// java: public class JacobMedalBackground extends ColoredItemBackground<Integer>
#[derive(Clone)]
pub struct JacobMedalBackground {
    /// java: extends ColoredItemBackground
    pub base: ColoredItemBackgroundBase<i32>,
}

impl std::ops::Deref for JacobMedalBackground {
    type Target = ColoredItemBackgroundBase<i32>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for JacobMedalBackground {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl JacobMedalBackground {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            base: ColoredItemBackgroundBase::<i32>::new(),
        };
        this
    }

}

impl ColoredItemBackground<i32> for JacobMedalBackground {
    fn colored_item_background_base(&self) -> &ColoredItemBackgroundBase<i32> {
        &self.base
    }
    fn colored_item_background_base_mut(&mut self) -> &mut ColoredItemBackgroundBase<i32> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled()
    fn is_enabled(&self) -> bool {
        return SkyblockerConfigManager::get().general.item_info_display.jacob_medal_backgrounds;
    }

    /// java: @Override protected Integer getColorKey(ItemStack stack, Int2ReferenceOpenHashMap<Integer> cache)
    fn get_color_key(&mut self, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, cache: &mut std::collections::HashMap<i32, Option<i32>>) -> Option<i32> {
        if false /* java: stack == null: never null here */ || stack.is_empty() {
            return None; // java: return null
        }
        // int hashCode = stack.getUuid().isEmpty() ? System.identityHashCode(stack) : stack.getUuid().hashCode();
        let mut hash_code: i32 = (if stack.get_uuid().is_empty() { compat::identity_hash_code(stack) } else { compat::string_hash_code(&stack.get_uuid()) });
        if cache.contains_key(&hash_code) {
            return cache.get(&hash_code).copied().flatten();
        }
        let mut matcher: Option<compat::regex::Matcher> = ItemUtils::get_lore_line_if_match(stack, &PATTERN);
        if let Some(matcher) = &matcher {
            // Integer color = BRACKET_COLORS.get(matcher.group("bracket")); if (color != null)
            if let Some(color) = BRACKET_COLORS.get(matcher.group_named("bracket").as_ref().unwrap()).cloned() {
                cache.insert(hash_code, Some(color));
                return Some(color);
            }
        }
        cache.insert(hash_code, None);
        return None; // java: return null
    }

    /// java: @Override protected void extract(GuiGraphicsExtractor context, int x, int y, Integer color)
    fn extract(&self, context: &GuiGraphicsExtractor, mut x: i32, mut y: i32, color: &i32) {
        let mut r: f32 = ((((*color).wrapping_shr((16) as u32) & 0xFF) as f32) / 255.0f32);
        let mut g: f32 = ((((*color).wrapping_shr((8) as u32) & 0xFF) as f32) / 255.0f32);
        let mut b: f32 = (((*color & 0xFF) as f32) / 255.0f32);
        gui_graphics_extractor::blit_sprite_stretched(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &self.get_sprite(), x, y, 16, 16, argb::color_from_float(SkyblockerConfigManager::get().general.item_info_display.item_background_opacity, r, g, b));
    }

}

impl crate::de::hysky::skyblocker::skyblock::item::background::colored_item_background::AnyColoredItemBackground for JacobMedalBackground {
    fn on_screen_change(&mut self, title: &str, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        ColoredItemBackground::<i32>::on_screen_change(self, title, screen)
    }
    fn is_enabled(&self) -> bool {
        ColoredItemBackground::<i32>::is_enabled(self)
    }
    fn try_extract(&mut self, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, graphics: &GuiGraphicsExtractor, x: i32, y: i32) {
        ColoredItemBackground::<i32>::try_extract(self, stack, graphics, x, y)
    }
    fn clear_cache(&mut self) {
        ColoredItemBackground::<i32>::clear_cache(self)
    }
}
