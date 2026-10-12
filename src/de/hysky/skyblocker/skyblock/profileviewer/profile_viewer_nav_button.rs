//! Ported from de.hysky.skyblocker.skyblock.profileviewer.ProfileViewerNavButton by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer::profile_viewer_screen::ProfileViewerScreen;
use crate::de::hysky::skyblocker::skyblock::profileviewer::utils::profile_viewer_utils::ProfileViewerUtils;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier BUTTON_TEXTURES_TOGGLED = Identifier.parse("container/creative_inventory/tab_top_selected_2");
pub static BUTTON_TEXTURES_TOGGLED: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::parse("container/creative_inventory/tab_top_selected_2"));
/// java: private static final Identifier BUTTON_TEXTURES = Identifier.parse("container/creative_inventory/tab_top_unselected_2");
pub static BUTTON_TEXTURES: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::parse("container/creative_inventory/tab_top_unselected_2"));
/// java: private static final Map<String, FlexibleItemStack> HEAD_ICON = Map.ofEntries( Map.entry("Skills", Ico.IRON_SWORD), Map.entry("Slayers", ProfileViewerUtils.createSkull("eyJ0ZXh0dXJlcyI6eyJTS0lOIjp7InV
pub static HEAD_ICON: compat::MainThread<std::collections::HashMap<String, compat::Shared<FlexibleItemStack>>> = compat::MainThread::new(|| std::collections::HashMap::from([("Skills".to_string(), ico::IRON_SWORD.get_mut().clone()), ("Slayers".to_string(), ProfileViewerUtils::create_skull("eyJ0ZXh0dXJlcyI6eyJTS0lOIjp7InVybCI6Imh0dHBzOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlLzkzMzZkN2NjOTVjYmY2Njg5ZjVlOGM5NTQyOTRlYzhkMWVmYzQ5NGE0MDMxMzI1YmI0MjdiYzgxZDU2YTQ4NGQifX19")), ("Pets".to_string(), ico::BONE.get_mut().clone()), ("Dungeons".to_string(), ProfileViewerUtils::create_skull("eyJ0ZXh0dXJlcyI6eyJTS0lOIjp7InVybCI6Imh0dHBzOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlLzliNTY4OTViOTY1OTg5NmFkNjQ3ZjU4NTk5MjM4YWY1MzJkNDZkYjljMWIwMzg5YjhiYmViNzA5OTlkYWIzM2QifX19")), ("Inventories".to_string(), ico::E_CHEST.get_mut().clone()), ("Collections".to_string(), ico::PAINTING.get_mut().clone())]));

/// java: public class ProfileViewerNavButton extends AbstractWidget
#[derive(Clone)]
pub struct ProfileViewerNavButton {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private boolean toggled;
    pub toggled: bool,
    /// java: private final int index;
    pub index: i32,
    /// java: private final ProfileViewerScreen screen;
    pub screen: compat::Shared<ProfileViewerScreen>,
    /// java: private final FlexibleItemStack icon;
    pub icon: compat::Shared<FlexibleItemStack>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for ProfileViewerNavButton {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ProfileViewerNavButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ProfileViewerNavButton {
    /// java: public ProfileViewerNavButton(ProfileViewerScreen screen, String tabName, int index, boolean toggled)
    pub fn new(mut screen: compat::Shared<ProfileViewerScreen>, mut tab_name: String, mut index: i32, mut toggled: bool) -> compat::Shared<Self> {
        let __init_screen = screen.clone();
        let __init_toggled = toggled;
        let __init_index = index;
        let __init_icon = HEAD_ICON.get_mut().get(&tab_name).cloned().unwrap_or_else(|| ico::BARRIER.get_mut().clone());
        let __this = compat::shared(Self {
            base: AbstractWidget::new(-100, -100, 28, 32, (&component::empty().get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            self_ref: std::sync::Weak::new(),
            toggled: __init_toggled,
            index: __init_index,
            screen: __init_screen,
            icon: __init_icon,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &(if compat::read(__this, |__t| __t.toggled) { BUTTON_TEXTURES_TOGGLED.get_mut().clone() } else { BUTTON_TEXTURES.get_mut().clone() }), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), compat::read(__this, |__t| __t.width), (compat::read(__this, |__t| __t.height)).wrapping_sub(if compat::read(__this, |__t| __t.toggled) { 0 } else { 4 }));
        graphics.item(&FlexibleItemStack::get_stack_or_throw(&compat::read(__this, |__t| __t.icon.clone())), (__this.lock().borrow().get_x()).wrapping_add(6), (__this.lock().borrow().get_y()).wrapping_add(if compat::read(__this, |__t| __t.toggled) { 7 } else { 9 }));
        __this.lock().borrow().handle_cursor(graphics);
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        ProfileViewerScreen::on_nav_button_click(&compat::read(__this, |__t| __t.screen.clone()), __this);
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

    /// java: public void setToggled(boolean toggled)
    pub fn set_toggled(__this: &compat::Shared<Self>, mut toggled: bool) {
        { let __v = toggled; compat::write(__this, |__t| __t.toggled = __v); }
    }

    /// java: public int getIndex()
    pub fn get_index(__this: &compat::Shared<Self>) -> i32 {
        return compat::read(__this, |__t| __t.index);
    }

}

/// ProfileViewerNavButton's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for ProfileViewerNavButton {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// ProfileViewerNavButton as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<ProfileViewerNavButton> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| ProfileViewerNavButton::extract_widget_render_state(__h, &context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ProfileViewerNavButton::update_widget_narration(__h, output))
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| ProfileViewerNavButton::on_click(__h, event, double_click))
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, onClick, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
