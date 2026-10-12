//! Ported from de.hysky.skyblocker.skyblock.item.background.ColoredItemBackground by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::renderer::texture::texture_atlas_sprite::TextureAtlasSprite;

/// java: public abstract class ColoredItemBackground<T>
#[derive(Clone)]
pub struct ColoredItemBackgroundBase<T> {
    /// java: private final Int2ReferenceOpenHashMap<T> cache = new Int2ReferenceOpenHashMap<>();
    /// (the map holds Java's `null` values too: a key cached as "no background")
    pub cache: std::collections::HashMap<i32, Option<T>>,
    /// java: private final Supplier<TextureAtlasSprite> sprite;
    pub sprite: Arc<dyn Fn() -> TextureAtlasSprite>,
}

impl<T: Clone + 'static> ColoredItemBackgroundBase<T> {
    /// java: protected ColoredItemBackground()
    pub fn new() -> Self {
        // java: this.sprite = () -> Minecraft.getInstance().getAtlasManager().getAtlasOrThrow(AtlasIds.GUI)
        //   .getSprite(SkyblockerConfigManager.get().general.itemInfoDisplay.itemBackgroundStyle.tex);
        // (the GUI atlas is `Minecraft.gui_sprites` in the port)
        let mut this = Self {
            cache: std::collections::HashMap::new(),
            sprite: Arc::new(|| {
                compat::minecraft().gui_sprites.get_sprite(&crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.item_info_display.item_background_style.fields().tex).clone()
            }),
        };
        this
    }

}

/// java: the virtual methods of ColoredItemBackground (implemented by ColoredItemBackgroundBase and every subclass)
pub trait ColoredItemBackground<T> {
    fn colored_item_background_base(&self) -> &ColoredItemBackgroundBase<T>;
    fn colored_item_background_base_mut(&mut self) -> &mut ColoredItemBackgroundBase<T>;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: protected void onScreenChange(String title, Screen screen)
    fn on_screen_change(&mut self, title: &str, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen) {
    }

    /// java: protected abstract T getColorKey(ItemStack stack, Int2ReferenceOpenHashMap<T> cache);
    /// `None` is Java's null: no background.
    fn get_color_key(&mut self, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, cache: &mut std::collections::HashMap<i32, Option<T>>) -> Option<T>;

    /// java: protected abstract void extract(GuiGraphicsExtractor graphics, int x, int y, T colorKey);
    fn extract(&self, graphics: &GuiGraphicsExtractor, x: i32, y: i32, color_key: &T);

    /// java: public boolean isEnabled()
    fn is_enabled(&self) -> bool {
        return true;
    }

    /// java: public final void tryExtract(ItemStack stack, GuiGraphicsExtractor graphics, int x, int y)
    fn try_extract(&mut self, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, graphics: &GuiGraphicsExtractor, mut x: i32, mut y: i32) {
        // getColorKey(stack, cache): the cache is lent out of the object for the call
        let mut cache = std::mem::take(&mut self.colored_item_background_base_mut().cache);
        let mut value: Option<T> = self.get_color_key(stack, &mut cache);
        self.colored_item_background_base_mut().cache = cache;
        if let Some(value) = &value {
            self.extract(graphics, x, y, value);
        }
    }

    /// java: protected final void clearCache()
    fn clear_cache(&mut self) {
        self.colored_item_background_base_mut().cache.clear();
    }

    /// java: protected final TextureAtlasSprite getSprite()
    fn get_sprite(&self) -> TextureAtlasSprite {
        return (self.colored_item_background_base().sprite)();
    }

}

/// `ColoredItemBackground<?>` - a background of any key type, as `ItemBackgroundManager.BACKGROUNDS` holds them
/// (Java's wildcard; the methods it calls through it).
pub trait AnyColoredItemBackground {
    fn on_screen_change(&mut self, title: &str, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen);
    fn is_enabled(&self) -> bool;
    fn try_extract(&mut self, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, graphics: &GuiGraphicsExtractor, x: i32, y: i32);
    fn clear_cache(&mut self);
}
