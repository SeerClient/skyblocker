//! Ported from de.hysky.skyblocker.skyblock.profileviewer.inventory.Inventory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::background::item_background_manager::ItemBackgroundManager;
use crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text_manager::SlotTextManager;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::item_loaders::item_loader::ItemLoader;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::item_loaders::item_loader::ItemLoaderBase;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::pagination_button::PaginationButton;
use crate::de::hysky::skyblocker::skyblock::profileviewer::profile_viewer_page::ProfileViewerPage;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::resources::language::i18n;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;

/// java: private static final Identifier TEXTURE = Identifier.parse("textures/gui/container/generic_54.png");
pub static TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::parse("textures/gui/container/generic_54.png"));
/// java: private static final Font textRenderer = CLIENT.font;
pub static TEXT_RENDERER: compat::LiveFont = compat::LiveFont;

/// java: public class Inventory implements ProfileViewerPage
#[derive(Clone)]
pub struct Inventory {
    /// java: private final IntIntPair dimensions;
    pub dimensions: (i32, i32),
    /// java: private final int itemsPerPage;
    pub items_per_page: i32,
    /// java: private final List<ItemStack> containerList;
    pub container_list: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: private final String containerName;
    pub container_name: String,
    /// java: private int activePage = 0;
    pub active_page: i32,
    /// java: private int totalPages = 1;
    pub total_pages: i32,
    /// java: private final PaginationButton previousPage = new PaginationButton(this, -1000, 0, false);
    pub previous_page: Option<compat::Shared<PaginationButton>>,
    /// java: private final PaginationButton nextPage = new PaginationButton(this, -1000, 0, true);
    pub next_page: Option<compat::Shared<PaginationButton>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl Inventory {
    /// java: public Inventory(String name, IntIntPair dimensions, JsonObject inventory)
    pub fn new(mut name: String, mut dimensions: (i32, i32), mut inventory: serde_json::Map<String, serde_json::Value>) -> compat::Shared<Self> {
        let __this = Self::new_4(name.clone(), dimensions.clone(), inventory.clone(), (ItemLoaderBase::new() as compat::Shared<dyn ItemLoader>));
        __this
    }

    /// java: public Inventory(String name, IntIntPair dimensions, JsonObject inventory, ItemLoader itemLoader)
    pub fn new_4(mut name: String, mut dimensions: (i32, i32), mut inventory: serde_json::Map<String, serde_json::Value>, mut item_loader: compat::Shared<dyn ItemLoader>) -> compat::Shared<Self> {
        let __init_container_name = name.clone();
        let __init_dimensions = dimensions;
        let __init_items_per_page = dimensions.1.wrapping_mul(dimensions.0);
        let __init_container_list = ({ let __f = compat::dispatch(&item_loader, |__t| __t.load_items()); __f(&item_loader, &inventory) });
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            dimensions: __init_dimensions,
            items_per_page: __init_items_per_page,
            container_list: __init_container_list,
            container_name: __init_container_name,
            active_page: 0,
            total_pages: 1,
            previous_page: None,
            next_page: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = Some(PaginationButton::new((__this.clone() as compat::Shared<dyn ProfileViewerPage>), -1000, 0, false)); compat::write(&__this, |__t| __t.previous_page = __v); }
        { let __v = Some(PaginationButton::new((__this.clone() as compat::Shared<dyn ProfileViewerPage>), -1000, 0, true)); compat::write(&__this, |__t| __t.next_page = __v); }
        { let __v = ((((((__this.lock().borrow().container_list.len() as i32)) as f64) / (compat::read(&__this, |__t| __t.items_per_page) as f64)).ceil()) as i32); compat::write(&__this, |__t| __t.total_pages = __v); }
        __this
    }

}

impl ProfileViewerPage for Inventory {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta, int rootX, int rootY) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>, &mut GuiGraphicsExtractor, i32, i32, f32, i32, i32) {
        Inventory::extract_render_state_dyn
    }

    /// java: public void nextPage() (dispatch: the body for this class)
    fn next_page(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>) {
        Inventory::next_page_dyn
    }

    /// java: public void previousPage() (dispatch: the body for this class)
    fn previous_page(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>) {
        Inventory::previous_page_dyn
    }

    /// java: @Override public void markWidgetsAsVisible() (dispatch: the body for this class)
    fn mark_widgets_as_visible(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>) {
        Inventory::mark_widgets_as_visible_dyn
    }

    /// java: @Override public void markWidgetsAsInvisible() (dispatch: the body for this class)
    fn mark_widgets_as_invisible(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>) {
        Inventory::mark_widgets_as_invisible_dyn
    }

    /// java: @Override public List<AbstractWidget> getButtons() (dispatch: the body for this class)
    fn get_buttons(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>) -> Option<Vec<AbstractWidget>> {
        Inventory::get_buttons_dyn
    }

}

impl Inventory {
    /// java: public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta, int rootX, int rootY)
    pub fn extract_render_state_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32, mut root_x: i32, mut root_y: i32) {
        let mut root_y_adjusted: i32 = root_y.wrapping_add(26i32.wrapping_sub((compat::read(__this, |__t| __t.dimensions.clone()).0).wrapping_mul(3)));
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), root_x, root_y_adjusted, ((0) as f32) as f32, ((0) as f32) as f32, ((compat::read(__this, |__t| __t.dimensions.clone()).1).wrapping_mul(18)).wrapping_add(7), ((compat::read(__this, |__t| __t.dimensions.clone()).0).wrapping_mul(18)).wrapping_add(17), 256, 256);
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), (root_x.wrapping_add((compat::read(__this, |__t| __t.dimensions.clone()).1).wrapping_mul(18))).wrapping_add(7), root_y_adjusted, ((169) as f32) as f32, ((0) as f32) as f32, 7, ((compat::read(__this, |__t| __t.dimensions.clone()).0).wrapping_mul(18)).wrapping_add(17), 256, 256);
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), root_x, (root_y_adjusted.wrapping_add((compat::read(__this, |__t| __t.dimensions.clone()).0).wrapping_mul(18))).wrapping_add(17), ((0) as f32) as f32, ((215) as f32) as f32, ((compat::read(__this, |__t| __t.dimensions.clone()).1).wrapping_mul(18)).wrapping_add(7), 7, 256, 256);
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), (root_x.wrapping_add((compat::read(__this, |__t| __t.dimensions.clone()).1).wrapping_mul(18))).wrapping_add(7), (root_y_adjusted.wrapping_add((compat::read(__this, |__t| __t.dimensions.clone()).0).wrapping_mul(18))).wrapping_add(17), ((169) as f32) as f32, ((215) as f32) as f32, 7, 7, 256, 256);
        // java: graphics.text(textRenderer,  I18n.get("skyblocker.profileviewer.inventory." + containerName), rootX + 7, rootYAdjusted + 7, Color.DARK_GRAY.getRGB(), false);
        let __name = mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(mcrust::net::minecraft::network::chat::component::literal(&i18n::get(&format!("skyblocker.profileviewer.inventory.{}", compat::read(__this, |__t| __t.container_name.clone())), &[])));
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &__name, root_x.wrapping_add(7), root_y_adjusted.wrapping_add(7), mcrust::java::awt::color::dark_gray().get_rgb(), false);
        if (__this.lock().borrow().container_list.len() as i32) > compat::read(__this, |__t| __t.items_per_page) {
            let __wa1 = root_x.wrapping_add(44);
            let __o1 = compat::read(__this, |__t| __t.previous_page.clone()).unwrap();
            compat::write(&__o1, |__t| __t.set_x(__wa1));
            let __wa2 = root_y.wrapping_add(136);
            let __o2 = compat::read(__this, |__t| __t.previous_page.clone()).unwrap();
            compat::write(&__o2, |__t| __t.set_y(__wa2));
            // java: button.extractRenderState(graphics, mouseX, mouseY, a); - `AbstractWidget.extractRenderState`: when visible, the
            // hover flag and then the class's `extractWidgetRenderState` (the screen draws the same widget again, as in Java)
            {
                let __button = compat::read(__this, |__t| __t.previous_page.clone()).unwrap();
                if compat::read(&__button, |__t| __t.base.visible) {
                    compat::write(&__button, |__t| __t.base.set_hovered_from_extract(mouse_x, mouse_y));
                    PaginationButton::extract_widget_render_state(&__button, graphics, mouse_x, mouse_y, delta);
                }
            }
            // java: graphics.centeredText(textRenderer, "Page: " + (activePage + 1) + "/" + totalPages, rootX + 88, rootY + 140, Color.WHITE.getRGB());
            let __page_text = format!("Page: {}/{}", (compat::read(__this, |__t| __t.active_page)).wrapping_add(1), compat::read(__this, |__t| __t.total_pages));
            gui_graphics_extractor::centered_text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &__page_text, root_x.wrapping_add(88), root_y.wrapping_add(140), mcrust::java::awt::color::white().get_rgb());
            let __wa7 = root_x.wrapping_add(121);
            let __o4 = compat::read(__this, |__t| __t.next_page.clone()).unwrap();
            compat::write(&__o4, |__t| __t.set_x(__wa7));
            let __wa8 = root_y.wrapping_add(136);
            let __o5 = compat::read(__this, |__t| __t.next_page.clone()).unwrap();
            compat::write(&__o5, |__t| __t.set_y(__wa8));
            // java: button.extractRenderState(graphics, mouseX, mouseY, a); - `AbstractWidget.extractRenderState`: when visible, the
            // hover flag and then the class's `extractWidgetRenderState` (the screen draws the same widget again, as in Java)
            {
                let __button = compat::read(__this, |__t| __t.next_page.clone()).unwrap();
                if compat::read(&__button, |__t| __t.base.visible) {
                    compat::write(&__button, |__t| __t.base.set_hovered_from_extract(mouse_x, mouse_y));
                    PaginationButton::extract_widget_render_state(&__button, graphics, mouse_x, mouse_y, delta);
                }
            }
        }
        let mut start_index: i32 = (compat::read(__this, |__t| __t.active_page)).wrapping_mul(compat::read(__this, |__t| __t.items_per_page));
        let mut end_index: i32 = (start_index.wrapping_add(compat::read(__this, |__t| __t.items_per_page))).min((__this.lock().borrow().container_list.len() as i32));
        let mut hovered_stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = None;
        // (the page's own list, read in place: nothing in the loop reaches this page)
        let __this_guard = __this.lock();
        let __this_ref = __this_guard.borrow();
        for i in 0..end_index.wrapping_sub(start_index) {
            let stack = &__this_ref.container_list[(start_index.wrapping_add(i)) as usize];
            if stack.is_empty() {
                continue;
            }
            let mut column: i32 = i.wrapping_rem(compat::read(__this, |__t| __t.dimensions.clone()).1);
            let mut row: i32 = i.wrapping_div(compat::read(__this, |__t| __t.dimensions.clone()).1);
            let mut x: i32 = (root_x.wrapping_add(8)).wrapping_add(column.wrapping_mul(18));
            let mut y: i32 = (root_y_adjusted.wrapping_add(18)).wrapping_add(row.wrapping_mul(18));
            ItemBackgroundManager::draw_backgrounds(stack, graphics, x, y);
            if ItemProtection::is_item_protected_ref(Some(stack)) {
                ItemProtection::draw_slot_icon(graphics, x, y);
            }
            graphics.item(stack, x, y);
            graphics.item_decorations(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), stack, x, y);
            SlotTextManager::extract_slot_text_7(graphics, &TEXT_RENDERER.get_mut(), None, stack, i, x, y);
            if mouse_x > x.wrapping_sub(2) && mouse_x < (x.wrapping_add(16)).wrapping_add(1) && mouse_y > y.wrapping_sub(2) && mouse_y < (y.wrapping_add(16)).wrapping_add(1) {
                hovered_stack = Some(stack.clone());
            }
        }
        if let Some(hovered_stack) = &hovered_stack {
            // java: graphics.setTooltipForNextFrame(textRenderer, hoveredStack, mouseX, mouseY);
            compat::batch_w_set_item_tooltip_for_next_frame(hovered_stack);
        }
    }

    /// Inventory.extractRenderState as the dispatch of ProfileViewerPage.extractRenderState
    pub fn extract_render_state_dyn(__this: &compat::Shared<dyn ProfileViewerPage>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32, __p4: i32, __p5: i32) {
        Self::extract_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4, __p5)
    }

    /// java: public void nextPage()
    pub fn next_page_impl(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.active_page) < (compat::read(__this, |__t| __t.total_pages)).wrapping_sub(1) {
            { let __v = compat::read(__this, |__t| __t.active_page) + 1; compat::write(__this, |__t| __t.active_page = __v); }
        }
    }

    /// Inventory.nextPage as the dispatch of ProfileViewerPage.nextPage
    pub fn next_page_dyn(__this: &compat::Shared<dyn ProfileViewerPage>) {
        Self::next_page_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: public void previousPage()
    pub fn previous_page_impl(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.active_page) > 0 {
            { let __v = compat::read(__this, |__t| __t.active_page) - 1; compat::write(__this, |__t| __t.active_page = __v); }
        }
    }

    /// Inventory.previousPage as the dispatch of ProfileViewerPage.previousPage
    pub fn previous_page_dyn(__this: &compat::Shared<dyn ProfileViewerPage>) {
        Self::previous_page_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void markWidgetsAsVisible()
    pub fn mark_widgets_as_visible_impl(__this: &compat::Shared<Self>) {
        { let __v = true; compat::write(&compat::read(__this, |__t| __t.next_page.clone()).unwrap(), |__t| __t.visible = __v); }
        { let __v = true; compat::write(&compat::read(__this, |__t| __t.previous_page.clone()).unwrap(), |__t| __t.visible = __v); }
        { let __v = true; compat::write(&compat::read(__this, |__t| __t.next_page.clone()).unwrap(), |__t| __t.active = __v); }
        { let __v = true; compat::write(&compat::read(__this, |__t| __t.previous_page.clone()).unwrap(), |__t| __t.active = __v); }
    }

    /// Inventory.markWidgetsAsVisible as the dispatch of ProfileViewerPage.markWidgetsAsVisible
    pub fn mark_widgets_as_visible_dyn(__this: &compat::Shared<dyn ProfileViewerPage>) {
        Self::mark_widgets_as_visible_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void markWidgetsAsInvisible()
    pub fn mark_widgets_as_invisible_impl(__this: &compat::Shared<Self>) {
        { let __v = false; compat::write(&compat::read(__this, |__t| __t.next_page.clone()).unwrap(), |__t| __t.visible = __v); }
        { let __v = false; compat::write(&compat::read(__this, |__t| __t.previous_page.clone()).unwrap(), |__t| __t.visible = __v); }
        { let __v = false; compat::write(&compat::read(__this, |__t| __t.next_page.clone()).unwrap(), |__t| __t.active = __v); }
        { let __v = false; compat::write(&compat::read(__this, |__t| __t.previous_page.clone()).unwrap(), |__t| __t.active = __v); }
    }

    /// Inventory.markWidgetsAsInvisible as the dispatch of ProfileViewerPage.markWidgetsAsInvisible
    pub fn mark_widgets_as_invisible_dyn(__this: &compat::Shared<dyn ProfileViewerPage>) {
        Self::mark_widgets_as_invisible_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<AbstractWidget> getButtons()
    pub fn get_buttons_impl(__this: &compat::Shared<Self>) -> Option<Vec<AbstractWidget>> {
        let mut buttons: Vec<AbstractWidget> = Vec::new();
        // (the widget objects, as the game's widgets - each attached to its object)
        buttons.push(compat::WidgetAdapter::widget(compat::read(__this, |__t| __t.next_page.clone()).expect("NullPointerException")));
        buttons.push(compat::WidgetAdapter::widget(compat::read(__this, |__t| __t.previous_page.clone()).expect("NullPointerException")));
        return Some(buttons);
    }

    /// Inventory.getButtons as the dispatch of ProfileViewerPage.getButtons
    pub fn get_buttons_dyn(__this: &compat::Shared<dyn ProfileViewerPage>) -> Option<Vec<AbstractWidget>> {
        Self::get_buttons_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
