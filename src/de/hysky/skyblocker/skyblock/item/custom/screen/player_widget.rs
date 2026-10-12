//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.PlayerWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::player::remote_player::RemotePlayer;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::mth;
use mcrust::org::joml::quaternionf::Quaternionf;
use mcrust::org::joml::vector3f::Vector3f;
use mcrust::net::minecraft::world::entity::entity::Entity as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;

/// java: private static final Identifier INNER_SPACE_TEXTURE = SkyblockerMod.id("menu_inner_space");
pub static INNER_SPACE_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("menu_inner_space"));
/// java: private static final float FLIP_ROTATION = (float) Math.PI;
pub const FLIP_ROTATION: f32 = std::f64::consts::PI as f32;

/// java: public class PlayerWidget extends AbstractWidget
pub struct PlayerWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractContainerWidget,
    /// java: private final AbstractClientPlayer player; - the player the armor tab dresses (the same object)
    pub player: compat::Shared<RemotePlayer>,
    /// java: private float xRotation = -10;
    pub x_rotation: f32,
    /// java: private float yRotation = 225;
    pub y_rotation: f32,
}

impl PlayerWidget {
    /// java: public PlayerWidget(int x, int y, int width, int height, AbstractClientPlayer player)
    pub fn new(x: i32, y: i32, width: i32, height: i32, player: compat::Shared<RemotePlayer>) -> compat::Shared<Self> {
        // java: super(x, y, width, height, Component.literal(""));
        // java: this.player.yHeadRot = this.player.yHeadRotO = 0;
        compat::write(&player, |p| {
            let living = p.living_mut();
            living.y_head_rot_o = 0.0;
            living.y_head_rot = living.y_head_rot_o;
        });
        compat::shared(PlayerWidget {
            base: AbstractContainerWidget::new(x, y, width, height, FormattedText::of("")),
            player,
            x_rotation: -10.0,
            y_rotation: 225.0,
        })
    }
}

impl compat::ObjectWidget for PlayerWidget {
    fn object_box(&mut self) -> &mut AbstractContainerWidget {
        &mut self.base
    }

    fn object_box_ref(&self) -> &AbstractContainerWidget {
        &self.base
    }

    /// java: @Override protected void onDrag(MouseButtonEvent click, double offsetX, double offsetY)
    fn on_drag(&mut self, _widget: &mut AbstractWidget, _event: &MouseButtonEvent, offset_x: f64, offset_y: f64) {
        // java: super.onDrag(click, offsetX, offsetY); - empty
        self.x_rotation = mth::clamp_f32(self.x_rotation - (offset_y as f32) * 2.5f32, -50.0f32, 50.0f32);
        self.y_rotation += (offset_x as f32) * 2.5f32;
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, _widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, _mouse_x: i32, _mouse_y: i32, _a: f32) {
        let (x, y, w, h) = (self.base.get_x(), self.base.get_y(), self.base.get_width(), self.base.get_height());
        gui_graphics_extractor::blit_sprite_identifier(context.gl, context.minecraft, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), INNER_SPACE_TEXTURE.get(), x, y, w, h);

        let size: f32 = 64.0;
        let bb_height = compat::read(&self.player, |p| p.base().get_bb_height());
        let translation = Vector3f::new(0.0, bb_height / 2.0 + 0.0625, 0.0);
        let mut rotation = Quaternionf::new();
        rotation.rotation_xyz(-self.x_rotation * mth::DEG_TO_RAD, -self.y_rotation * mth::DEG_TO_RAD, FLIP_ROTATION);

        // java: EntityRenderState renderState = InventoryScreenInvoker.invokeExtractRenderState(this.player);
        let gfx = context.minecraft;
        let Some(dispatcher) = gfx.game_renderer.get_entity_render_dispatcher() else {
            return;
        };
        use mcrust::net::minecraft::client::renderer::item::item_model_resolver::{ItemModelResolver, ItemModelSource};
        let resolver = mcrust::net::minecraft::client::renderer::game_renderer::baked().as_ref().map(|models| ItemModelResolver::new(models.item_models.clone() as Arc<dyn ItemModelSource + Send + Sync>));
        let level = mcrust::mcrust::modding::game::level();
        let render_state = compat::read(&self.player, |p| {
            mcrust::net::minecraft::client::gui::screens::inventory::inventory_screen::extract_render_state_of_entity(
                dispatcher,
                p,
                &|layer, pos| match level.as_deref() {
                    Some(level) => level.get_brightness(layer, pos),
                    None => 0,
                },
                resolver.as_ref(),
                level.as_deref(),
                gfx.gui.hud.is_hidden,
            )
        });
        let Some(render_state) = render_state else {
            return;
        };
        // java: graphics.entity(renderState, size, translation, rotation, null, getX(), getY(), this.getRight(), this.getBottom());
        gui_graphics_extractor::entity(render_state, size, translation, rotation, None, x, y, self.base.get_right(), self.base.get_bottom());
    }

    /// java: @Override public void playDownSound(SoundManager soundManager) - empty
    fn play_down_sound(&self) -> Option<&'static mcrust::net::minecraft::sounds::sound_event::SoundEvent> {
        None
    }
}
