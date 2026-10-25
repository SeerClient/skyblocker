//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.widgets.InventoryWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::background::item_background_manager::ItemBackgroundManager;
use crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text_manager::SlotTextManager;
use crate::de::hysky::skyblocker::utils::hovered_item::hovered_item_stack_provider::HoveredItemStackProvider;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: private static final Identifier CONTAINER_BACKGROUND = Identifier.withDefaultNamespace("textures/gui/container/generic_54.png");
pub static CONTAINER_BACKGROUND: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("textures/gui/container/generic_54.png"));
/// java: private static final int HOTBAR_SEPARATOR_HEIGHT = 4;
pub const HOTBAR_SEPARATOR_HEIGHT: i32 = 4;

/// java: public final class InventoryWidget extends AbstractWidget implements HoveredItemStackProvider
#[derive(Clone)]
pub struct InventoryWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final int rows;
    pub rows: i32,
    /// java: private final int columns;
    pub columns: i32,
    /// java: private final List<List<ItemStack>> pages;
    pub pages: Vec<Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>>,
    /// java: private final boolean hasHotbar;
    pub has_hotbar: bool,
    /// java: private int index = 0;
    pub index: i32,
}

impl std::ops::Deref for InventoryWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for InventoryWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl InventoryWidget {
    /// java: public InventoryWidget(Component label, int rows, int columns, List<List<ItemStack>> pages, boolean hasHotbar)
    pub fn new(mut label: MutableComponent, mut rows: i32, mut columns: i32, mut pages: Vec<Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>>, mut has_hotbar: bool) -> compat::Shared<Self> {
        let mut width: i32 = (columns.wrapping_mul(18)).wrapping_add(7i32.wrapping_mul(2));
        let mut height: i32 = (((rows.wrapping_mul(18)).wrapping_add(17)).wrapping_add(7)).wrapping_add(if has_hotbar { HOTBAR_SEPARATOR_HEIGHT } else { 0 });
        // java: super(0, 0, width, height, label);
        use mcrust::net::minecraft::network::chat::component::Component as _;
        let mut base = AbstractWidget::new(0, 0, width, height, label.get_string(), mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::PlainText, mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress::Inert);
        base.set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(label.clone()));
        // Make the widget ignore clicks
        base.active = false;
        let __this: compat::Shared<Self> = compat::shared(Self {
            base,
            rows,
            columns: columns.max(3),
            pages: pages.clone(),
            has_hotbar,
            index: 0,
        });
        __this
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        let mut font: Font = compat::font();
        let mut x: i32 = __this.lock().borrow().get_x();
        let mut y: i32 = __this.lock().borrow().get_y();
        if compat::read(__this, |__t| __t.has_hotbar) {
            gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), CONTAINER_BACKGROUND.get_mut(), x, (y.wrapping_add(compat::read(__this, |__t| __t.rows))).wrapping_add(10), ((0) as f32) as f32, ((136) as f32) as f32, ((compat::read(__this, |__t| __t.columns)).wrapping_mul(18)).wrapping_add(7), ((compat::read(__this, |__t| __t.rows)).wrapping_mul(18)).wrapping_add(17), 256, 256);
            gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), CONTAINER_BACKGROUND.get_mut(), (x.wrapping_add((compat::read(__this, |__t| __t.columns)).wrapping_mul(18))).wrapping_add(7), y, ((169) as f32) as f32, ((0) as f32) as f32, 7, ((compat::read(__this, |__t| __t.rows)).wrapping_mul(18)).wrapping_add(21), 256, 256);
            gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), CONTAINER_BACKGROUND.get_mut(), x, y, ((0) as f32) as f32, ((0) as f32) as f32, ((compat::read(__this, |__t| __t.columns)).wrapping_mul(18)).wrapping_add(7), 14, 256, 256);
            gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), CONTAINER_BACKGROUND.get_mut(), (x.wrapping_add((compat::read(__this, |__t| __t.columns)).wrapping_mul(18))).wrapping_add(7), (y.wrapping_add((compat::read(__this, |__t| __t.rows)).wrapping_mul(18))).wrapping_add(21), ((169) as f32) as f32, ((215) as f32) as f32, 7, 7, 256, 256);
        } else {
            gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), CONTAINER_BACKGROUND.get_mut(), x, y, ((0) as f32) as f32, ((0) as f32) as f32, ((compat::read(__this, |__t| __t.columns)).wrapping_mul(18)).wrapping_add(7), ((compat::read(__this, |__t| __t.rows)).wrapping_mul(18)).wrapping_add(17), 256, 256);
            gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), CONTAINER_BACKGROUND.get_mut(), (x.wrapping_add((compat::read(__this, |__t| __t.columns)).wrapping_mul(18))).wrapping_add(7), y, ((169) as f32) as f32, ((0) as f32) as f32, 7, ((compat::read(__this, |__t| __t.rows)).wrapping_mul(18)).wrapping_add(17), 256, 256);
            gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), CONTAINER_BACKGROUND.get_mut(), x, (y.wrapping_add((compat::read(__this, |__t| __t.rows)).wrapping_mul(18))).wrapping_add(17), ((0) as f32) as f32, ((215) as f32) as f32, ((compat::read(__this, |__t| __t.columns)).wrapping_mul(18)).wrapping_add(7), 7, 256, 256);
            gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), CONTAINER_BACKGROUND.get_mut(), (x.wrapping_add((compat::read(__this, |__t| __t.columns)).wrapping_mul(18))).wrapping_add(7), (y.wrapping_add((compat::read(__this, |__t| __t.rows)).wrapping_mul(18))).wrapping_add(17), ((169) as f32) as f32, ((215) as f32) as f32, 7, 7, 256, 256);
        }
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::component_of(__this.lock().borrow().get_message_component())), x.wrapping_add(8), y.wrapping_add(6), 0xFF404040u32 as i32, false);
        if __this.lock().borrow().pages.is_empty() {
            return;
        }
        let __page_index = compat::read(__this, |__t| __t.index) as usize;
        let __this_guard = __this.lock();
        let __this_ref = __this_guard.borrow();
        let stacks = &__this_ref.pages[__page_index];
        let mut hovered_stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = None;
        for i in 0..(stacks.len() as i32) {
            let mut column: i32 = i.wrapping_rem(compat::read(__this, |__t| __t.columns));
            let mut row: i32 = i.wrapping_div(compat::read(__this, |__t| __t.columns));
            let mut hotbar_y_shift: i32 = (if compat::read(__this, |__t| __t.has_hotbar) && compat::read(__this, |__t| __t.rows) > 1 && row.wrapping_add(1) == compat::read(__this, |__t| __t.rows) { HOTBAR_SEPARATOR_HEIGHT } else { 0 });
            let stack = &stacks[(i) as usize];
            let mut item_x: i32 = (x.wrapping_add(column.wrapping_mul(18))).wrapping_add(8);
            let mut item_y: i32 = ((y.wrapping_add(row.wrapping_mul(18))).wrapping_add(18)).wrapping_add(hotbar_y_shift);
            ItemBackgroundManager::draw_backgrounds(stack, graphics, item_x, item_y);
            if ItemProtection::is_item_protected_ref(Some(stack)) {
                ItemProtection::draw_slot_icon(graphics, item_x, item_y);
            }
            graphics.fake_item(stack, item_x, item_y);
            graphics.item_decorations(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), stack, item_x, item_y);
            SlotTextManager::extract_slot_text_7(graphics, &font, None, stack, i, item_x, item_y);
            if !stack.is_empty() && GuiHelper::point_is_in_area((mouse_x) as f64, (mouse_y) as f64, ((item_x.wrapping_sub(1)) as f64) as f64, ((item_y.wrapping_sub(1)) as f64) as f64, (((item_x.wrapping_add(mcrust::net::minecraft::client::gui::render::gui_renderer::DEFAULT_ITEM_SIZE)).wrapping_add(1)) as f64) as f64, (((item_y.wrapping_add(mcrust::net::minecraft::client::gui::render::gui_renderer::DEFAULT_ITEM_SIZE)).wrapping_add(1)) as f64) as f64) {
                hovered_stack = Some(stack.clone());
            }
        }
        if let Some(hovered_stack) = &hovered_stack {
            // java: graphics.setTooltipForNextFrame(font, hoveredStack, mouseX, mouseY);
            compat::batch_w_set_item_tooltip_for_next_frame(hovered_stack);
        }
    }

    /// java: public void forwards()
    pub fn forwards(__this: &compat::Shared<Self>) {
        { let __v = ((compat::read(__this, |__t| __t.index)).wrapping_add(1)).clamp(0, (__this.lock().borrow().pages.len() as i32).wrapping_sub(1)); compat::write(__this, |__t| __t.index = __v); }
    }

    /// java: public void backwards()
    pub fn backwards(__this: &compat::Shared<Self>) {
        { let __v = ((compat::read(__this, |__t| __t.index)).wrapping_sub(1)).clamp(0, (__this.lock().borrow().pages.len() as i32).wrapping_sub(1)); compat::write(__this, |__t| __t.index = __v); }
    }

    /// java: public int getPage()
    pub fn get_page(__this: &compat::Shared<Self>) -> i32 {
        return (compat::read(__this, |__t| __t.index)).wrapping_add(1);
    }

    /// java: public int getMaxPages()
    pub fn get_max_pages(__this: &compat::Shared<Self>) -> i32 {
        return (__this.lock().borrow().pages.len() as i32);
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, output: &dyn NarrationElementOutput) {
    }

    /// java: @Override public boolean shouldTakeFocusAfterInteraction()
    pub fn should_take_focus_after_interaction(__this: &compat::Shared<Self>) -> bool {
        return false;
    }

}

impl HoveredItemStackProvider for InventoryWidget {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public @Nullable ItemStack getFocusedItem() (dispatch: the body for this class)
    fn get_focused_item(&self) -> fn(&compat::Shared<dyn HoveredItemStackProvider>) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        InventoryWidget::get_focused_item_dyn
    }

}

impl InventoryWidget {
    /// java: @Override public @Nullable ItemStack getFocusedItem()
    pub fn get_focused_item_impl(__this: &compat::Shared<Self>) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        return None;
    }

    /// InventoryWidget.getFocusedItem as the dispatch of HoveredItemStackProvider.getFocusedItem
    pub fn get_focused_item_dyn(__this: &compat::Shared<dyn HoveredItemStackProvider>) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        Self::get_focused_item_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// InventoryWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for InventoryWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// InventoryWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<InventoryWidget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| InventoryWidget::extract_widget_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| InventoryWidget::update_widget_narration(__h, output))
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, shouldTakeFocusAfterInteraction, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
