//! Ported from de.hysky.skyblocker.skyblock.item.background.ColoredItemBackground by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::renderer::texture::texture_atlas_sprite::TextureAtlasSprite;

/// java: public abstract class ColoredItemBackground<T>
#[derive(Clone)]
pub struct ColoredItemBackgroundBase<T> {
    /// java: private final Int2ReferenceOpenHashMap<T> cache = new Int2ReferenceOpenHashMap<>();
    pub cache: std::collections::HashMap<i32, T>,
    /// java: private final Supplier<TextureAtlasSprite> sprite;
    pub sprite: Arc<dyn Fn() -> TextureAtlasSprite>,
}

impl<T: Clone + 'static> ColoredItemBackgroundBase<T> {
    /// java: protected ColoredItemBackground()
    pub fn new() -> Self {
        // TODO(fabric2seer): `Minecraft.getAtlasManager` has no Rust method in the port (looked for `get_atlas_manager` in Minecraft and its supertypes)
        //   java: this.sprite = () -> Minecraft.getInstance()
        //   java: 				.getAtlasManager()
        //   java: 				.getAtlasOrThrow(AtlasIds.GUI)
        //   java: 				.getSprite(SkyblockerConfigManager.get().general.itemInfoDisplay.itemBackgroundStyle.tex);
        let mut this = Self {
            cache: std::collections::HashMap::new(),
            sprite: todo!("fabric2seer: the field value (TODO above)"),
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
    fn on_screen_change(&self, title: &str, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen) {
    }

    /// java: protected abstract T getColorKey(ItemStack stack, Int2ReferenceOpenHashMap<T> cache);
    fn get_color_key(&mut self, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, cache: &std::collections::HashMap<i32, T>) -> T;

    /// java: protected abstract void extract(GuiGraphicsExtractor graphics, int x, int y, T colorKey);
    fn extract(&self, graphics: &GuiGraphicsExtractor, x: i32, y: i32, color_key: &T);

    /// java: public boolean isEnabled()
    fn is_enabled(&self) -> bool {
        return true;
    }

    /// java: public final void tryExtract(ItemStack stack, GuiGraphicsExtractor graphics, int x, int y)
    fn try_extract(&self, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, graphics: &GuiGraphicsExtractor, mut x: i32, mut y: i32) {
        let mut value: Option<T> = Some(self.get_color_key(stack, &mut self.colored_item_background_base_mut().cache));
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
