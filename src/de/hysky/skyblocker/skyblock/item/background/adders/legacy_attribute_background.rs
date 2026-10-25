//! Ported from de.hysky.skyblocker.skyblock.item.background.adders.LegacyAttributeBackground by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::background::colored_item_background::ColoredItemBackground;
use crate::de::hysky::skyblocker::skyblock::item::background::colored_item_background::ColoredItemBackgroundBase;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;

/// java: private static final int COLOR = 0xFFFF0000;
pub const COLOR: i32 = 0xFFFF0000u32 as i32;

/// java: public class LegacyAttributeBackground extends ColoredItemBackground<Integer>
#[derive(Clone)]
pub struct LegacyAttributeBackground {
    /// java: extends ColoredItemBackground
    pub base: ColoredItemBackgroundBase<i32>,
}

impl std::ops::Deref for LegacyAttributeBackground {
    type Target = ColoredItemBackgroundBase<i32>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for LegacyAttributeBackground {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl LegacyAttributeBackground {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            base: ColoredItemBackgroundBase::<i32>::new(),
        };
        this
    }

}

impl ColoredItemBackground<i32> for LegacyAttributeBackground {
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
        return SkyblockerConfigManager::get().general.item_info_display.legacy_attribute_backgrounds;
    }

    /// java: @Override protected Integer getColorKey(ItemStack stack, Int2ReferenceOpenHashMap<Integer> cache)
    fn get_color_key(&mut self, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, cache: &mut crate::de::hysky::skyblocker::skyblock::item::background::colored_item_background::ColorKeyCache<i32>) -> Option<i32> {
        // OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): the skyblock id compared BORROWED
        // (`with_skyblock_id`), not cloned per slot per frame - the same id, the same cache write on a miss. Was:
        //     if false /* java: stack == null: never null here */ || stack.is_empty() || stack.get_skyblock_id() == "ATTRIBUTE_SHARD" {
        if false /* java: stack == null: never null here */ || stack.is_empty() || crate::mixins::item_stack_mixin::with_skyblock_id(stack, |skyblock_id: &str| skyblock_id == "ATTRIBUTE_SHARD") {
            return None; // java: return null
        }
        // int hashCode = System.identityHashCode(stack);
        let mut hash_code: i32 = compat::identity_hash_code(stack);
        if cache.contains_key(&hash_code) {
            return cache.get(&hash_code).copied().flatten();
        }
        // PERF (port-only, same result): the custom data read in place, not deep-copied.
        let mut has_attributes: bool = ItemUtils::with_custom_data(stack, |custom_data| custom_data.contains("attributes"));
        cache.insert(hash_code, (if has_attributes { Some(COLOR) } else { None }));
        return (if has_attributes { Some(COLOR) } else { None });
    }

    /// java: @Override protected void extract(GuiGraphicsExtractor context, int x, int y, Integer color)
    fn extract(&self, context: &GuiGraphicsExtractor, mut x: i32, mut y: i32, color: &i32) {
        gui_graphics_extractor::blit_sprite_stretched(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &self.get_sprite(), x, y, 16, 16, *color);
    }

}

impl crate::de::hysky::skyblocker::skyblock::item::background::colored_item_background::AnyColoredItemBackground for LegacyAttributeBackground {
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
