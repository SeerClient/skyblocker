//! Ported from de.hysky.skyblocker.skyblock.profileviewer.utils.SubPageSelectButton by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer::profile_viewer_page::ProfileViewerPage;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::widget_sprites::WidgetSprites;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::world::item::component::item_lore::ItemLore;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final WidgetSprites TEXTURES = new WidgetSprites(SkyblockerMod.id("textures/gui/profile_viewer/button_icon_toggled.png"), SkyblockerMod.id("textures/gui/profile_viewer/button_icon.png")
pub static TEXTURES: compat::MainThread<WidgetSprites> = compat::MainThread::new(|| WidgetSprites::new(crate::entry::skyblocker_mod::id("textures/gui/profile_viewer/button_icon_toggled.png"), crate::entry::skyblocker_mod::id("textures/gui/profile_viewer/button_icon.png"), crate::entry::skyblocker_mod::id("textures/gui/profile_viewer/button_icon_toggled_highlighted.png"), crate::entry::skyblocker_mod::id("textures/gui/profile_viewer/button_icon_highlighted.png")));

/// java: public class SubPageSelectButton extends AbstractWidget
#[derive(Clone)]
pub struct SubPageSelectButton {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final ProfileViewerPage page;
    pub page: compat::Shared<dyn ProfileViewerPage>,
    /// java: private final int index;
    pub index: i32,
    /// java: private boolean toggled;
    pub toggled: bool,
    /// java: private final FlexibleItemStack ICON;
    pub icon: compat::Shared<FlexibleItemStack>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for SubPageSelectButton {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SubPageSelectButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SubPageSelectButton {
    /// java: public SubPageSelectButton(ProfileViewerPage page, int x, int y, int index, FlexibleItemStack item)
    pub fn new(mut page: compat::Shared<dyn ProfileViewerPage>, mut x: i32, mut y: i32, mut index: i32, mut item: compat::Shared<FlexibleItemStack>) -> compat::Shared<Self> {
        let __init_icon = item.clone();
        let __init_toggled = (index == 0);
        let __init_index = index;
        let __init_page = page.clone();
        let __this = compat::shared(Self {
            base: AbstractWidget::new(x, y, 22, 22, (&component::literal("Placeholder").get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            self_ref: std::sync::Weak::new(),
            page: __init_page,
            index: __init_index,
            toggled: __init_toggled,
            icon: __init_icon,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = false; compat::write(&__this, |__t| __t.visible = __v); }
        __this
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURES.get_mut().get(compat::read(__this, |__t| __t.toggled), (mouse_x > __this.lock().borrow().get_x() && mouse_x < (__this.lock().borrow().get_x()).wrapping_add(20) && mouse_y > __this.lock().borrow().get_y() && mouse_y < (__this.lock().borrow().get_y()).wrapping_add(20))), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), ((0) as f32) as f32, ((0) as f32) as f32, 20, 20, 20, 20);
        graphics.item(&FlexibleItemStack::get_stack_or_throw(&compat::read(__this, |__t| __t.icon.clone())), (__this.lock().borrow().get_x()).wrapping_add(2), (__this.lock().borrow().get_y()).wrapping_add(2));
        if mouse_x > __this.lock().borrow().get_x() && mouse_x < (__this.lock().borrow().get_x()).wrapping_add(20) && mouse_y > __this.lock().borrow().get_y() && mouse_y < (__this.lock().borrow().get_y()).wrapping_add(20) {
            let mut lore: Option<ItemLore> = FlexibleItemStack::get::<ItemLore>(&compat::read(__this, |__t| __t.icon.clone()), mcrust::net::minecraft::core::component::data_components::lore());
            if let Some(lore) = &lore {
                // TODO(fabric2seer): the port's `set_component_tooltip_for_next_frame` ports this Java method with other parameters (tooltip_for_next_frame, lines for Java's lines, xo, yo)
                // TODO(fabric2seer): `set_component_tooltip_for_next_frame` takes 2 argument(s) in the port (`tooltip_for_next_frame: &mut Option<DeferredTooltip>, lines: Vec<crate::net::minecraft::network::chat::formatted_text::FormattedText>`), Java passes 4
                // rust: gui_graphics_extractor::set_component_tooltip_for_next_frame(&mut compat::font(), lore.lines().clone(), mouse_x, mouse_y.wrapping_add(10));
            }
        }
        // java: this.handleCursor(graphics); - the game asks every widget for its cursor after the frame: this class
        // requests one through `ModWidget::handle_cursor` below
    }

    /// java: @Override public boolean isMouseOver(double mouseX, double mouseY)
    pub fn is_mouse_over(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64) -> bool {
        return (compat::read(__this, |__t| __t.active) && compat::read(__this, |__t| __t.visible) && mouse_x > (__this.lock().borrow().get_x() as f64) && mouse_x < ((__this.lock().borrow().get_x()).wrapping_add(20) as f64) && mouse_y > (__this.lock().borrow().get_y() as f64) && mouse_y < ((__this.lock().borrow().get_y()).wrapping_add(20) as f64));
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

    /// java: public void setToggled(boolean toggled)
    pub fn set_toggled(__this: &compat::Shared<Self>, mut toggled: bool) {
        { let __v = toggled; compat::write(__this, |__t| __t.toggled = __v); }
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        ({ let __r = &compat::read(__this, |__t| __t.page.clone()); let __f = compat::dispatch(__r, |__t| __t.on_nav_button_click()); __f(__r, __this) });
    }

    /// java: public int getIndex()
    pub fn get_index(__this: &compat::Shared<Self>) -> i32 {
        return compat::read(__this, |__t| __t.index);
    }

}

/// SubPageSelectButton's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for SubPageSelectButton {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// SubPageSelectButton as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<SubPageSelectButton> {
    /// java: `this.handleCursor(graphics)` at the end of extractWidgetRenderState - AbstractWidget's body
    fn handle_cursor(&self, widget: &AbstractWidget, cursors: &mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes) -> Option<mcrust::com::mojang::blaze3d::platform::cursor::cursor_type::CursorType> {
        widget.handle_cursor(cursors)
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| SubPageSelectButton::extract_widget_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| SubPageSelectButton::update_widget_narration(__h, output))
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| SubPageSelectButton::on_click(__h, event, double_click))
    }

    /// java: @Override public boolean isMouseOver(double mouseX, double mouseY)
    fn is_mouse_over(&self, widget: &AbstractWidget, mouse_x: f64, mouse_y: f64) -> bool {
        self.lend_ref(widget, |__h| SubPageSelectButton::is_mouse_over(__h, mouse_x, mouse_y))
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, isMouseOver, onClick, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
