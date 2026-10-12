//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.PlayerWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
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
use mcrust::net::minecraft::client::player::abstract_client_player::AbstractClientPlayer;
use mcrust::net::minecraft::client::renderer::entity::state::entity_render_state::EntityRenderState;
use mcrust::net::minecraft::client::sounds::sound_manager::SoundManager;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::mth;
use mcrust::org::joml::quaternionf::Quaternionf;
use mcrust::org::joml::vector3f::Vector3f;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier INNER_SPACE_TEXTURE = SkyblockerMod.id("menu_inner_space");
pub static INNER_SPACE_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("menu_inner_space"));
/// java: private static final float FLIP_ROTATION = (float) Math.PI;
pub static FLIP_ROTATION: LazyLock<f32> = LazyLock::new(|| ((std::f64::consts::PI) as f32));

/// java: public class PlayerWidget extends AbstractWidget
pub struct PlayerWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final AbstractClientPlayer player;
    pub player: Box<dyn AbstractClientPlayer>,
    /// java: private float xRotation = -10;
    pub x_rotation: f32,
    /// java: private float yRotation = 225;
    pub y_rotation: f32,
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
    /// java: public PlayerWidget(int x, int y, int width, int height, AbstractClientPlayer player)
    pub fn new(mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut player: Box<dyn AbstractClientPlayer>) -> compat::Shared<Self> {
        let __init_player = player.clone();
        let __this = compat::shared(Self {
            base: AbstractWidget::new(x, y, width, height, (&component::literal("").get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            player: __init_player,
            x_rotation: ((-10) as f32) as f32,
            y_rotation: ((225) as f32) as f32,
        });
        // TODO(fabric2seer): an assignment used as a value
        //   java: this.player.yHeadRot = this.player.yHeadRotO = 0;
        __this
    }

    /// java: @Override protected void onDrag(MouseButtonEvent click, double offsetX, double offsetY)
    pub fn on_drag(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut offset_x: f64, mut offset_y: f64) {
        // TODO(fabric2seer): `AbstractWidget.onDrag` has no Rust method in the port (looked for `on_drag` in AbstractWidget and its supertypes)
        // rust: __this.lock().borrow_mut().base.on_drag(click, offset_x, offset_y);
        { let __v = mth::clamp_f32((compat::read(__this, |__t| __t.x_rotation) - (((offset_y) as f32) * 2.5f32)), -50.0f32, 50.0f32); compat::write(__this, |__t| __t.x_rotation = __v); }
        { let __v = (compat::read(__this, |__t| __t.y_rotation) + (((offset_x) as f32) * 2.5f32)); compat::write(__this, |__t| __t.y_rotation = __v); }
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), INNER_SPACE_TEXTURE.get_mut(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), __this.lock().borrow().get_width(), __this.lock().borrow().get_height());
        let mut size: f32 = 64.0f32;
        let mut translation: Vector3f = Vector3f::new((0) as f32, ((__this.lock().borrow().player.get_bb_height() / 2.0f32) + 0.0625f32), (0) as f32);
        let rotation = Quaternionf::new().rotation_xyz(((-compat::read(__this, |__t| __t.x_rotation)) * mcrust::net::minecraft::util::mth::DEG_TO_RAD), ((-compat::read(__this, |__t| __t.y_rotation)) * mcrust::net::minecraft::util::mth::DEG_TO_RAD), *FLIP_ROTATION);
        // TODO(fabric2seer): static call InventoryScreenInvoker.invokeExtractRenderState(..)
        //   java: EntityRenderState renderState = InventoryScreenInvoker.invokeExtractRenderState(this.player);
        let mut render_state: EntityRenderState = todo!("fabric2seer: the initialiser of `renderState` (TODO above)");
        // TODO(fabric2seer): the mod keeps the game's `EntityRenderState` (Java holds a reference to the live object); the port's `EntityRenderState` is not a value to copy and the port lends the mod no handle to it
        // rust: gui_graphics_extractor::entity(render_state.clone(), size, translation.clone(), rotation.clone(), None, __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), __this.lock().borrow().get_right(), __this.lock().borrow().get_bottom());
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

    /// java: @Override public void playDownSound(SoundManager soundManager)
    pub fn play_down_sound(__this: &compat::Shared<Self>, sound_manager: &SoundManager) {
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

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| PlayerWidget::update_widget_narration(__h, output))
    }

    /// java: @Override protected void onDrag(MouseButtonEvent click, double offsetX, double offsetY)
    fn on_drag(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, dx: f64, dy: f64) {
        self.lend(widget, |__h| PlayerWidget::on_drag(__h, event, dx, dy))
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, onDrag, playDownSound, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
