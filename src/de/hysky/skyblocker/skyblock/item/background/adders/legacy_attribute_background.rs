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
    fn get_color_key(&mut self, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, cache: &mut std::collections::HashMap<i32, i32>) -> i32 {
        if false /* java: stack == null: never null here */ || stack.is_empty() || stack.get_skyblock_id() == "ATTRIBUTE_SHARD" {
            return Default::default(); // java: return null
        }
        // TODO(fabric2seer): static `System.identityHashCode` has no Rust counterpart in the port
        // rust: let mut hash_code: i32 = System::identity_hash_code(stack.clone());
        let mut hash_code: i32 = todo!("fabric2seer: the initialiser of `hashCode` (TODO above)");
        if cache.contains_key(&hash_code) {
            return cache.get(&hash_code).copied().unwrap_or_default();
        }
        let mut has_attributes: bool = ItemUtils::get_custom_data(stack).contains("attributes");
        cache.insert(hash_code, (if has_attributes { Some(COLOR) } else { None }).unwrap());
        return (if has_attributes { Some(COLOR) } else { None }).unwrap();
    }

    /// java: @Override protected void extract(GuiGraphicsExtractor context, int x, int y, Integer color)
    fn extract(&self, context: &GuiGraphicsExtractor, mut x: i32, mut y: i32, color: &i32) {
        gui_graphics_extractor::blit_sprite_stretched(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &self.get_sprite(), x, y, 16, 16, *color);
    }

}
