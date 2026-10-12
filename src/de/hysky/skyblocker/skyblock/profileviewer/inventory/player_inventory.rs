//! Ported from de.hysky.skyblocker.skyblock.profileviewer.inventory.PlayerInventory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::background::item_background_manager::ItemBackgroundManager;
use crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text_manager::SlotTextManager;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::item_loaders::inventory_item_loader::InventoryItemLoader;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::item_loaders::item_loader::ItemLoader;
use crate::de::hysky::skyblocker::skyblock::profileviewer::profile_viewer_page::ProfileViewerPage;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::resources::language::i18n;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: private static final Identifier TEXTURE = Identifier.parse("textures/gui/container/generic_54.png");
pub static TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::parse("textures/gui/container/generic_54.png"));
/// java: private static final Font textRenderer = CLIENT.font;
pub static TEXT_RENDERER: compat::MainThread<Font> = compat::MainThread::new(|| compat::font());

/// java: public class PlayerInventory implements ProfileViewerPage
#[derive(Clone)]
pub struct PlayerInventory {
    /// java: private final List<ItemStack> containerList;
    pub container_list: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: private @Nullable ItemStack hoveredItem = null;
    pub hovered_item: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
}

impl PlayerInventory {
    /// java: public PlayerInventory(JsonObject inventory)
    pub fn new(mut inventory: serde_json::Map<String, serde_json::Value>) -> compat::Shared<Self> {
        let __init_container_list = ({ let __r = &InventoryItemLoader::new(); let __f = compat::dispatch(__r, |__t| __t.load_items()); __f(&compat::view(__r, |a| a as compat::Shared<dyn ItemLoader>), &inventory) });
        let __this = compat::shared(Self {
            container_list: __init_container_list,
            hovered_item: None,
        });
        __this
    }

    /// java: private void extractContainerTextures(GuiGraphicsExtractor graphics, String containerName, int rootX, int rootY, IntIntPair dimensions)
    pub fn extract_container_textures(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, container_name: &str, mut root_x: i32, mut root_y: i32, dimensions: &(i32, i32)) {
        if container_name == "inventory" {
            gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), root_x, (root_y.wrapping_add(dimensions.0)).wrapping_add(10), ((0) as f32) as f32, ((136) as f32) as f32, (dimensions.1.wrapping_mul(18)).wrapping_add(7), (dimensions.0.wrapping_mul(18)).wrapping_add(17), 256, 256);
            gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), (root_x.wrapping_add(dimensions.1.wrapping_mul(18))).wrapping_add(7), root_y, ((169) as f32) as f32, ((0) as f32) as f32, 7, (dimensions.0.wrapping_mul(18)).wrapping_add(21), 256, 256);
            gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), root_x, root_y, ((0) as f32) as f32, ((0) as f32) as f32, (dimensions.1.wrapping_mul(18)).wrapping_add(7), 14, 256, 256);
            gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), (root_x.wrapping_add(dimensions.1.wrapping_mul(18))).wrapping_add(7), (root_y.wrapping_add(dimensions.0.wrapping_mul(18))).wrapping_add(21), ((169) as f32) as f32, ((215) as f32) as f32, 7, 7, 256, 256);
        } else {
            gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), root_x, root_y, ((0) as f32) as f32, ((0) as f32) as f32, (dimensions.1.wrapping_mul(18)).wrapping_add(7), (dimensions.0.wrapping_mul(18)).wrapping_add(17), 256, 256);
            gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), (root_x.wrapping_add(dimensions.1.wrapping_mul(18))).wrapping_add(7), root_y, ((169) as f32) as f32, ((0) as f32) as f32, 7, (dimensions.0.wrapping_mul(18)).wrapping_add(17), 256, 256);
            gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), root_x, (root_y.wrapping_add(dimensions.0.wrapping_mul(18))).wrapping_add(17), ((0) as f32) as f32, ((215) as f32) as f32, (dimensions.1.wrapping_mul(18)).wrapping_add(7), 7, 256, 256);
            gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), (root_x.wrapping_add(dimensions.1.wrapping_mul(18))).wrapping_add(7), (root_y.wrapping_add(dimensions.0.wrapping_mul(18))).wrapping_add(17), ((169) as f32) as f32, ((215) as f32) as f32, 7, 7, 256, 256);
        }
        // TODO(fabric2seer): `.getRGB(..)` on a value of unknown type
        //   java: graphics.text(textRenderer,  I18n.get("skyblocker.profileviewer.inventory." + containerName), rootX + 7, rootY + 7, Color.DARK_GRAY.getRGB(), false);
    }

    /// java: private void extractContainerItems(GuiGraphicsExtractor graphics, int rootX, int rootY, IntIntPair dimensions, int startIndex, int endIndex, int mouseX, int mouseY)
    pub fn extract_container_items(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut root_x: i32, mut root_y: i32, dimensions: &(i32, i32), mut start_index: i32, mut end_index: i32, mut mouse_x: i32, mut mouse_y: i32) {
        for i in 0..end_index.wrapping_sub(start_index) {
            let stack = &__this.lock().borrow().container_list[(start_index.wrapping_add(i)) as usize];
            if stack.is_empty() {
                continue;
            }
            let mut column: i32 = i.wrapping_rem(dimensions.1);
            let mut row: i32 = i.wrapping_div(dimensions.1);
            let mut x: i32 = (root_x.wrapping_add(8)).wrapping_add(column.wrapping_mul(18));
            let mut y: i32 = ((root_y.wrapping_add(18)).wrapping_add(row.wrapping_mul(18))).wrapping_add(if dimensions.0 > 1 && row.wrapping_add(1) == dimensions.0 { 4 } else { 0 });
            ItemBackgroundManager::draw_backgrounds(stack, graphics, x, y);
            if ItemProtection::is_item_protected(Some(stack.clone())) {
                ItemProtection::draw_slot_icon(graphics, x, y);
            }
            graphics.item(stack, x, y);
            graphics.item_decorations(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), stack, x, y);
            SlotTextManager::extract_slot_text_7(graphics, TEXT_RENDERER.get_mut(), None, stack, i, x, y);
            if mouse_x > x.wrapping_sub(2) && mouse_x < (x.wrapping_add(16)).wrapping_add(1) && mouse_y > y.wrapping_sub(2) && mouse_y < (y.wrapping_add(16)).wrapping_add(1) {
                { let __v = Some(stack.clone()); compat::write(__this, |__t| __t.hovered_item = __v); }
            }
        }
    }

}

impl ProfileViewerPage for PlayerInventory {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta, int rootX, int rootY) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>, &mut GuiGraphicsExtractor, i32, i32, f32, i32, i32) {
        PlayerInventory::extract_render_state_dyn
    }

}

impl PlayerInventory {
    /// java: public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta, int rootX, int rootY)
    pub fn extract_render_state_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32, mut root_x: i32, mut root_y: i32) {
        PlayerInventory::extract_container_textures(__this, graphics, "armor", root_x, root_y.wrapping_add(108), &(1, 4));
        PlayerInventory::extract_container_textures(__this, graphics, "inventory", root_x, root_y.wrapping_add(2), &(4, 9));
        PlayerInventory::extract_container_textures(__this, graphics, "equipment", root_x.wrapping_add(90), root_y.wrapping_add(108), &(1, 4));
        { let __v = None; compat::write(__this, |__t| __t.hovered_item = __v); }
        PlayerInventory::extract_container_items(__this, graphics, root_x, root_y.wrapping_add(108), &(1, 4), 36, 40, mouse_x, mouse_y);
        PlayerInventory::extract_container_items(__this, graphics, root_x, root_y.wrapping_add(2), &(4, 9), 0, 36, mouse_x, mouse_y);
        PlayerInventory::extract_container_items(__this, graphics, root_x.wrapping_add(90), root_y.wrapping_add(108), &(1, 4), 40, (__this.lock().borrow().container_list.len() as i32), mouse_x, mouse_y);
        if compat::read(__this, |__t| __t.hovered_item.clone()).is_some() {
            // TODO(fabric2seer): `GuiGraphicsExtractor.setTooltipForNextFrame` has no Rust method in the port (looked for `set_tooltip_for_next_frame` in GuiGraphicsExtractor and its supertypes)
            // rust: graphics.set_tooltip_for_next_frame(TEXT_RENDERER.get_mut(), &compat::read(__this, |__t| __t.hovered_item.clone()).unwrap(), mouse_x, mouse_y);
        }
    }

    /// PlayerInventory.extractRenderState as the dispatch of ProfileViewerPage.extractRenderState
    pub fn extract_render_state_dyn(__this: &compat::Shared<dyn ProfileViewerPage>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32, __p4: i32, __p5: i32) {
        Self::extract_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4, __p5)
    }

}
