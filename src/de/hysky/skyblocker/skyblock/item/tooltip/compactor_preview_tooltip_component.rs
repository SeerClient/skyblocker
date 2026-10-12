//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.CompactorPreviewTooltipComponent by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: private static final Identifier TEXTURE = Identifier.withDefaultNamespace("textures/gui/container/generic_54.png");
pub static TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("textures/gui/container/generic_54.png"));
/// java: private static final Identifier DISABLED_SLOT = Identifier.withDefaultNamespace("container/crafter/disabled_slot");
pub static DISABLED_SLOT: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("container/crafter/disabled_slot"));

/// java: public class CompactorPreviewTooltipComponent implements ClientTooltipComponent
#[derive(Clone)]
pub struct CompactorPreviewTooltipComponent {
    /// java: private final Iterable<IntObjectPair<@Nullable FlexibleItemStack>> items;
    pub items: Vec<(i32, Option<compat::Shared<FlexibleItemStack>>)>,
    /// java: private final IntIntPair dimensions;
    pub dimensions: (i32, i32),
    /// java: private final int columns;
    pub columns: i32,
}

impl CompactorPreviewTooltipComponent {
    /// java: CompactorPreviewTooltipComponent(Iterable<IntObjectPair<@Nullable FlexibleItemStack>> items, IntIntPair dimensions)
    pub fn new(mut items: Vec<(i32, Option<compat::Shared<FlexibleItemStack>>)>, mut dimensions: (i32, i32)) -> Self {
        let __init_items = items.clone();
        let __init_dimensions = dimensions.clone();
        let __init_columns = dimensions.1.max(3);
        let mut this = Self {
            items: __init_items,
            dimensions: __init_dimensions,
            columns: __init_columns,
        };
        this
    }

    /// java: @Override public int getHeight(Font textRenderer)
    pub fn get_height(&self, text_renderer: &Font) -> i32 {
        return (self.dimensions.0.wrapping_mul(18)).wrapping_add(17);
    }

    /// java: @Override public int getWidth(Font textRenderer)
    pub fn get_width(&self, text_renderer: &Font) -> i32 {
        return (self.columns.wrapping_mul(18)).wrapping_add(14);
    }

    /// java: @Override public void extractImage(Font textRenderer, int x, int y, int width, int height, GuiGraphicsExtractor graphics)
    pub fn extract_image(&self, text_renderer: &Font, mut x: i32, mut y: i32, mut width: i32, mut height: i32, graphics: &GuiGraphicsExtractor) {
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), x, y, ((0) as f32) as f32, ((0) as f32) as f32, (self.columns.wrapping_mul(18)).wrapping_add(7), (self.dimensions.0.wrapping_mul(18)).wrapping_add(17), 256, 256);
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), (x.wrapping_add(self.columns.wrapping_mul(18))).wrapping_add(7), y, ((169) as f32) as f32, ((0) as f32) as f32, 7, (self.dimensions.0.wrapping_mul(18)).wrapping_add(17), 256, 256);
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), x, (y.wrapping_add(self.dimensions.0.wrapping_mul(18))).wrapping_add(17), ((0) as f32) as f32, ((215) as f32) as f32, (self.columns.wrapping_mul(18)).wrapping_add(7), 7, 256, 256);
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), (x.wrapping_add(self.columns.wrapping_mul(18))).wrapping_add(7), (y.wrapping_add(self.dimensions.0.wrapping_mul(18))).wrapping_add(17), ((169) as f32) as f32, ((215) as f32) as f32, 7, 7, 256, 256);
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some("Contents"), x.wrapping_add(8), y.wrapping_add(6), 0xFF404040u32 as i32, false);
        for entry in self.items.iter() {
            let mut item_x: i32 = (x.wrapping_add((entry.0.wrapping_rem(self.dimensions.1)).wrapping_mul(18))).wrapping_add(8);
            let mut item_y: i32 = (y.wrapping_add((entry.0.wrapping_div(self.dimensions.1)).wrapping_mul(18))).wrapping_add(18);
            if self.dimensions.1 == 1 {
                gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), DISABLED_SLOT.get_mut(), item_x.wrapping_sub(1), item_y.wrapping_sub(1), 18, 18);
                item_x = item_x.wrapping_add(18);
            }
            let mut flexible: Option<compat::Shared<FlexibleItemStack>> = entry.1.clone();
            if let Some(flexible) = &flexible {
                let mut item: mcrust::net::minecraft::world::item::item_stack::ItemStack = FlexibleItemStack::get_stack_or_throw(flexible);
                graphics.item(&item, item_x, item_y);
                graphics.item_decorations(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &item, item_x, item_y);
            }
            if self.dimensions.1 == 1 {
                item_x = item_x.wrapping_add(18);
                gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), DISABLED_SLOT.get_mut(), item_x.wrapping_sub(1), item_y.wrapping_sub(1), 18, 18);
            }
        }
    }

}

/// `implements ClientTooltipComponent`: the game draws the component through these (the bodies above take the
/// mod's `Font` / `GuiGraphicsExtractor` handles, which do not depend on the arguments the game lends).
impl mcrust::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::ClientTooltipComponent for CompactorPreviewTooltipComponent {
    fn get_height(&self, _font: &dyn mcrust::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipFont) -> i32 {
        CompactorPreviewTooltipComponent::get_height(self, &compat::font())
    }

    fn get_width(&self, _font: &dyn mcrust::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipFont) -> i32 {
        CompactorPreviewTooltipComponent::get_width(self, &compat::font())
    }

    fn extracts_image(&self) -> bool {
        true
    }

    fn extract_image(&self, _font: &dyn mcrust::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipFont, x: i32, y: i32, w: i32, h: i32, _graphics: &mut mcrust::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipGraphics<'_>) {
        CompactorPreviewTooltipComponent::extract_image(self, &compat::font(), x, y, w, h, &GuiGraphicsExtractor::new());
    }
}
