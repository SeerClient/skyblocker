//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.widgets.PageTabWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::profile_viewer_widget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::profile_viewer_widget::ProfileViewerWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::profile_viewer_widget::ProfileViewerWidgetBase;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: private static final Identifier TEXTURE_1_UNSELECTED = Identifier.withDefaultNamespace("container/creative_inventory/tab_top_unselected_1");
pub static TEXTURE_1_UNSELECTED: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("container/creative_inventory/tab_top_unselected_1"));
/// java: private static final Identifier TEXTURE_1_SELECTED = Identifier.withDefaultNamespace("container/creative_inventory/tab_top_selected_1");
pub static TEXTURE_1_SELECTED: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("container/creative_inventory/tab_top_selected_1"));
/// java: private static final Identifier TEXTURE_2_UNSELECTED = Identifier.withDefaultNamespace("container/creative_inventory/tab_top_unselected_2");
pub static TEXTURE_2_UNSELECTED: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("container/creative_inventory/tab_top_unselected_2"));
/// java: private static final Identifier TEXTURE_2_SELECTED = Identifier.withDefaultNamespace("container/creative_inventory/tab_top_selected_2");
pub static TEXTURE_2_SELECTED: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("container/creative_inventory/tab_top_selected_2"));
/// java: private static final int WIDTH = 26;
pub const WIDTH: i32 = 26;
/// java: private static final int HEIGHT = 32;
pub const HEIGHT: i32 = 32;
/// java: private static final int HEIGHT_OFFSET = 4;
pub const HEIGHT_OFFSET: i32 = 4;

/// java: public final class PageTabWidget extends ProfileViewerWidget
#[derive(Clone)]
pub struct PageTabWidget {
    /// java: extends ProfileViewerWidget
    pub base: ProfileViewerWidgetBase,
    /// java: private final FlexibleItemStack icon;
    pub icon: compat::Shared<FlexibleItemStack>,
    /// java: private final int index;
    pub index: i32,
    /// java: private final IntConsumer pageSwitcher;
    pub page_switcher: Arc<dyn Fn(i32)>,
    /// java: private boolean selected = false;
    pub selected: bool,
}

impl std::ops::Deref for PageTabWidget {
    type Target = ProfileViewerWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PageTabWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PageTabWidget {
    /// java: public PageTabWidget(FlexibleItemStack icon, int index, IntConsumer pageSwitcher)
    pub fn new(mut icon: compat::Shared<FlexibleItemStack>, mut index: i32, page_switcher: Arc<dyn Fn(i32)>) -> compat::Shared<Self> {
        let __init_icon = icon.clone();
        let __init_index = index;
        let __init_page_switcher = page_switcher.clone();
        let (_, __this) = ProfileViewerWidgetBase::new_6_in(0i32.wrapping_add(WIDTH.wrapping_mul(index)), (-HEIGHT).wrapping_add(HEIGHT_OFFSET), WIDTH, HEIGHT, false, component::empty(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                icon: __init_icon,
                index: __init_index,
                page_switcher: __init_page_switcher,
                selected: false,
            });
            ((__h.clone() as compat::Shared<dyn ProfileViewerWidget>), __h)
        });
        { let __v = true; compat::write(&__this, |__t| __t.active = __v); }
        __this
    }

    /// java: @Override public void onClick(MouseButtonEvent event, boolean doubleClick)
    pub fn on_click(__this: &compat::Shared<Self>, event: &MouseButtonEvent, mut double_click: bool) {
        ((compat::read(__this, |__t| __t.page_switcher.clone())))(compat::read(__this, |__t| __t.index));
    }

    /// java: public void setSelected(boolean selected)
    pub fn set_selected(__this: &compat::Shared<Self>, mut selected: bool) {
        { let __v = selected; compat::write(__this, |__t| __t.selected = __v); }
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        let mut texture_unselected: Identifier = (if compat::read(__this, |__t| __t.index) == 0 { TEXTURE_1_UNSELECTED.get_mut().clone() } else { TEXTURE_2_UNSELECTED.get_mut().clone() });
        let mut texture_selected: Identifier = (if compat::read(__this, |__t| __t.index) == 0 { TEXTURE_1_SELECTED.get_mut().clone() } else { TEXTURE_2_SELECTED.get_mut().clone() });
        let mut texture: Identifier = (if compat::read(__this, |__t| __t.selected) { texture_selected.clone() } else { texture_unselected.clone() });
        let mut icon_y_offset: i32 = (if compat::read(__this, |__t| __t.selected) { -2 } else { 0 });
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &texture, __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), __this.lock().borrow().get_width(), __this.lock().borrow().get_height());
        graphics.fake_item(&FlexibleItemStack::get_stack_or_throw(&compat::read(__this, |__t| __t.icon.clone())), (__this.lock().borrow().get_x()).wrapping_add(((__this.lock().borrow().get_width()).wrapping_sub(*profile_viewer_widget::ITEM_SIZE)).wrapping_div(2)), ((__this.lock().borrow().get_y()).wrapping_add(icon_y_offset)).wrapping_add(((__this.lock().borrow().get_height()).wrapping_sub(*profile_viewer_widget::ITEM_SIZE)).wrapping_div(2)));
        // java: this.handleCursor(graphics); - the game asks every widget for its cursor after the frame: this class
        // requests one through `ModWidget::handle_cursor` below
    }

}

impl ProfileViewerWidget for PageTabWidget {
    fn profile_viewer_widget_base(&self) -> &ProfileViewerWidgetBase {
        &self.base
    }
    fn profile_viewer_widget_base_mut(&mut self) -> &mut ProfileViewerWidgetBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// PageTabWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for PageTabWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base.base
    }
}

/// PageTabWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<PageTabWidget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| PageTabWidget::extract_widget_render_state(__h, &context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.update_widget_narration()); __f(&compat::view(__h, |a| a as compat::Shared<dyn ProfileViewerWidget>), output) }))
    }

    /// java: @Override public void onClick(MouseButtonEvent event, boolean doubleClick)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| PageTabWidget::on_click(__h, event, double_click))
    }

    /// java: `this.handleCursor(graphics)` at the end of extractWidgetRenderState - AbstractWidget's body
    fn handle_cursor(&self, widget: &AbstractWidget, cursors: &mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes) -> Option<mcrust::com::mojang::blaze3d::platform::cursor::cursor_type::CursorType> {
        widget.handle_cursor(cursors)
    }

}
