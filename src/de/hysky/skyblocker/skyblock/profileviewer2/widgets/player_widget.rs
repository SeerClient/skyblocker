//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.widgets.PlayerWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::profile_viewer_player::ProfileViewerPlayer;
use mcrust::com::mojang::authlib::game_profile::GameProfile;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::active_text_collector;
use mcrust::net::minecraft::client::gui::active_text_collector::ActiveTextCollector;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::screens::inventory::inventory_screen;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::entity::Entity as _;

/// java: private static final Identifier BACKGROUND = SkyblockerMod.id("profile_viewer2/player_background");
pub static BACKGROUND: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("profile_viewer2/player_background"));
/// java: public static final int WIDTH = 82;
pub const WIDTH: i32 = 82;
/// java: public static final int HEIGHT = 110;
pub const HEIGHT: i32 = 110;
/// java: private static final int NAME_TAG_X_OFFSET = 1;
pub const NAME_TAG_X_OFFSET: i32 = 1;
/// java: private static final int NAME_TAG_Y_OFFSET = 2;
pub const NAME_TAG_Y_OFFSET: i32 = 2;

/// java: public final class PlayerWidget extends AbstractWidget
pub struct PlayerWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final ProfileViewerPlayer entity;
    pub entity: ProfileViewerPlayer,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for PlayerWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PlayerWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PlayerWidget {
    /// java: public PlayerWidget(GameProfile playerProfile)
    pub fn new(mut player_profile: GameProfile) -> compat::Shared<Self> {
        let __init_entity = ProfileViewerPlayer::new(player_profile.clone());
        let __this = compat::shared(Self {
            base: AbstractWidget::new(0, 0, WIDTH, HEIGHT, (&component::empty().get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            self_ref: std::sync::Weak::new(),
            entity: __init_entity,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = false; compat::write(&__this, |__t| __t.active = __v); }
        __this
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        let mut font: Font = compat::font();
        // TODO(fabric2seer): field `NONE` of a value of unknown type
        //   java: ActiveTextCollector textCollector = graphics.textRendererForWidget(this, GuiGraphicsExtractor.HoveredTextEffects.NONE);
        let mut text_collector: Box<dyn ActiveTextCollector> = todo!("fabric2seer: the initialiser of `textCollector` (TODO above)");
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), BACKGROUND.get_mut(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), __this.lock().borrow().get_width(), __this.lock().borrow().get_height());
        // TODO(fabric2seer): `accept_scrolling_with_default_center` takes 6 argument(s) in the port (`left: i32, right: i32, top: i32, bottom: i32, line_width: i32, time_millis: i64`), Java passes 5
        // rust: active_text_collector::accept_scrolling_with_default_center(compat::component_of(__this.lock().borrow().entity.get_name()), (__this.lock().borrow().get_x()).wrapping_add(NAME_TAG_X_OFFSET), (__this.lock().borrow().get_right()).wrapping_sub(NAME_TAG_X_OFFSET), (__this.lock().borrow().get_y()).wrapping_add(NAME_TAG_Y_OFFSET), ((__this.lock().borrow().get_y()).wrapping_add(NAME_TAG_Y_OFFSET)).wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT));
        // TODO(fabric2seer): `extract_entity_in_inventory_follows_mouse` takes 14 argument(s) in the port (`x0: i32, y0: i32, x1: i32, y1: i32, size: i32, offset_y: f32, mouse_x: f32, mouse_y: f32, dispatcher: &EntityRenderDispatcher, entity: &dyn crate::net::minecraft::world::entity::entity::Entity, get_brightness: &dyn Fn( crate::net::minecraft ..`), Java passes 10
        // rust: inventory_screen::extract_entity_in_inventory_follows_mouse(graphics.clone(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), __this.lock().borrow().get_right(), __this.lock().borrow().get_bottom(), (42) as f32, 0.0625f32, (mouse_x) as f32, &mouse_y, &compat::read(__this, |__t| __t.entity.clone()));
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, output: &dyn NarrationElementOutput) {
    }

    /// java: @Override public boolean shouldTakeFocusAfterInteraction()
    pub fn should_take_focus_after_interaction(__this: &compat::Shared<Self>) -> bool {
        return false;
    }

}

/// PlayerWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for PlayerWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// PlayerWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<PlayerWidget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| PlayerWidget::extract_widget_render_state(__h, &context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| PlayerWidget::update_widget_narration(__h, output))
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, shouldTakeFocusAfterInteraction, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
