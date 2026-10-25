//! Ported from de.hysky.skyblocker.skyblock.item.background.adders.ItemRarityBackground by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::background::colored_item_background::ColoredItemBackground;
use crate::de::hysky::skyblocker::skyblock::item::background::colored_item_background::ColoredItemBackgroundBase;
use crate::de::hysky::skyblocker::skyblock::item::skyblock_item_rarity::SkyblockItemRarity;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::util::argb;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;

/// java: public class ItemRarityBackground extends ColoredItemBackground<SkyblockItemRarity>
#[derive(Clone)]
pub struct ItemRarityBackground {
    /// java: extends ColoredItemBackground
    pub base: ColoredItemBackgroundBase<SkyblockItemRarity>,
}

impl std::ops::Deref for ItemRarityBackground {
    type Target = ColoredItemBackgroundBase<SkyblockItemRarity>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ItemRarityBackground {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ItemRarityBackground {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            base: ColoredItemBackgroundBase::<SkyblockItemRarity>::new(),
        };
        this
    }

}

impl ColoredItemBackground<crate::de::hysky::skyblocker::skyblock::item::skyblock_item_rarity::SkyblockItemRarity> for ItemRarityBackground {
    fn colored_item_background_base(&self) -> &ColoredItemBackgroundBase<crate::de::hysky::skyblocker::skyblock::item::skyblock_item_rarity::SkyblockItemRarity> {
        &self.base
    }
    fn colored_item_background_base_mut(&mut self) -> &mut ColoredItemBackgroundBase<crate::de::hysky::skyblocker::skyblock::item::skyblock_item_rarity::SkyblockItemRarity> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled()
    fn is_enabled(&self) -> bool {
        return SkyblockerConfigManager::get().general.item_info_display.item_rarity_backgrounds;
    }

    /// java: @Override protected SkyblockItemRarity getColorKey(ItemStack stack, Int2ReferenceOpenHashMap<SkyblockItemRarity> cache)
    fn get_color_key(&mut self, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, cache: &mut crate::de::hysky::skyblocker::skyblock::item::background::colored_item_background::ColorKeyCache<SkyblockItemRarity>) -> Option<SkyblockItemRarity> {
        return Some(stack.get_skyblock_rarity());
    }

    /// java: @Override protected void extract(GuiGraphicsExtractor context, int x, int y, SkyblockItemRarity rarity)
    fn extract(&self, context: &GuiGraphicsExtractor, mut x: i32, mut y: i32, rarity: &SkyblockItemRarity) {
        if *rarity == SkyblockItemRarity::Unknown {
            return;
        }
        let mut use_legacy: bool = SkyblockerConfigManager::get().general.item_info_display.item_rarity_backgrounds_legacy_colors;
        gui_graphics_extractor::blit_sprite_stretched(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &self.get_sprite(), x, y, 16, 16, argb::color_alpha_f32_rgb(SkyblockerConfigManager::get().general.item_info_display.item_background_opacity, (if use_legacy { rarity.fields().legacy_color } else { rarity.fields().color })));
    }

}

impl crate::de::hysky::skyblocker::skyblock::item::background::colored_item_background::AnyColoredItemBackground for ItemRarityBackground {
    fn on_screen_change(&mut self, title: &str, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        ColoredItemBackground::<SkyblockItemRarity>::on_screen_change(self, title, screen)
    }
    fn is_enabled(&self) -> bool {
        ColoredItemBackground::<SkyblockItemRarity>::is_enabled(self)
    }
    fn try_extract(&mut self, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, graphics: &GuiGraphicsExtractor, x: i32, y: i32) {
        ColoredItemBackground::<SkyblockItemRarity>::try_extract(self, stack, graphics, x, y)
    }
    fn clear_cache(&mut self) {
        ColoredItemBackground::<SkyblockItemRarity>::clear_cache(self)
    }
}
