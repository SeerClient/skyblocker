//! Ported from de.hysky.skyblocker.skyblock.dwarven.CrystalsHudWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dwarven::crystals_locations_manager;
use crate::de::hysky::skyblocker::skyblock::dwarven::mining_location_label::MiningLocationLabel;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::Information;
use crate::de::hysky::skyblocker::utils::location::Location;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::minecraft;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::org::joml::matrix3x2f_stack::Matrix3x2fStack;
use mcrust::org::joml::vector2i::Vector2i;
use crate::de::hysky::skyblocker::skyblock::dwarven::mining_location_label::Category as _;

/// java: protected static final Identifier MAP_TEXTURE = SkyblockerMod.id("textures/gui/crystals_map.png");
pub static MAP_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("textures/gui/crystals_map.png"));
/// java: private static final Identifier MAP_ICON = Identifier.withDefaultNamespace("textures/map/decorations/player.png");
pub static MAP_ICON: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("textures/map/decorations/player.png"));
/// java: private static final List<String> SMALL_LOCATIONS = List.of("Fairy Grotto", "King Yolkar", "Corleone", "Odawa", "Key Guardian", "Xalx", "Unknown");
pub static SMALL_LOCATIONS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["Fairy Grotto".to_string(), "King Yolkar".to_string(), "Corleone".to_string(), "Odawa".to_string(), "Key Guardian".to_string(), "Xalx".to_string(), "Unknown".to_string()]);
/// java: private static @Nullable CrystalsHudWidget instance = null;
pub static INSTANCE: compat::MainThread<Option<compat::Shared<CrystalsHudWidget>>> = compat::MainThread::new(|| None);

/// java: @RegisterWidget public class CrystalsHudWidget extends HudWidget
#[derive(Clone)]
pub struct CrystalsHudWidget {
    /// java: extends HudWidget
    pub base: HudWidgetBase,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for CrystalsHudWidget {
    type Target = HudWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CrystalsHudWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CrystalsHudWidget {
    /// java: public CrystalsHudWidget()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = HudWidgetBase::new_in(Information::new3("hud_crystals".to_string(), component::literal("Crystals HUD"), Location::CrystalHollows), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                self_ref: std::sync::Weak::new(),
            });
            compat::write(&__h, |__t| __t.self_ref = std::sync::Arc::downgrade(&__h));
            ((__h.clone() as compat::Shared<dyn HudWidget>), __h)
        });
        *INSTANCE.get_mut() = Some(__this.clone());
        __this
    }

    /// java: @SuppressWarnings("unused") public static CrystalsHudWidget getInstance()
    pub fn get_instance() -> compat::Shared<CrystalsHudWidget> {
        if INSTANCE.get_mut().is_none() {
            CrystalsHudWidget::new();
        }
        return INSTANCE.get_mut().clone().unwrap();
    }

    /// java: protected static Vector2ic transformLocation(double x, double z)
    pub fn transform_location(mut x: f64, mut z: f64) -> Vector2i {
        let mut transformed_x: i32 = (((((x - (202 as f64)) / (621 as f64)) * (62 as f64))) as i32);
        let mut transformed_y: i32 = (((((z - (202 as f64)) / (621 as f64)) * (62 as f64))) as i32);
        transformed_x = transformed_x.clamp(0, 62);
        transformed_y = transformed_y.clamp(0, 62);
        return Vector2i::new(transformed_x, transformed_y);
    }

    /// java: private static float yaw2Cardinal(float yaw)
    pub fn yaw2_cardinal(mut yaw: f32) -> f32 {
        yaw = (yaw + (180 as f32));
        let mut clipped: i8 = ((((((yaw as f64) + (if (yaw as f64) < 0.0 { -8.0 } else { 8.0 })) * 16.0) / 360.0)) as i8);
        return (((clipped as f32) * 360.0f32) / 16.0f32);
    }

    /// java: public void update()
    pub fn update(__this: &compat::Shared<Self>) {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || minecraft::get_connection(compat::minecraft()).is_none() || !(SkyblockerConfigManager::get().mining.crystals_hud.enabled) {
            return;
        }
        let mut scale: f32 = SkyblockerConfigManager::get().mining.crystals_hud.map_scaling;
        // TODO(fabric2seer): an assignment used as a value
        //   java: w = h = (int) (62 * scale);
    }

}

impl HudWidget for CrystalsHudWidget {
    fn hud_widget_base(&self) -> &HudWidgetBase {
        &self.base
    }
    fn hud_widget_base_mut(&mut self) -> &mut HudWidgetBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        CrystalsHudWidget::extract_widget_render_state_dyn
    }

    /// java: @Override protected void extractWidgetRenderStateForConfig(GuiGraphicsExtractor graphics, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state_for_config(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        CrystalsHudWidget::extract_widget_render_state_for_config_dyn
    }

}

impl CrystalsHudWidget {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, float delta)
    pub fn extract_widget_render_state_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut delta: f32) {
        let mut scale: f32 = SkyblockerConfigManager::get().mining.crystals_hud.map_scaling;
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_scale((scale) as f32, (scale) as f32);
        // TODO(fabric2seer): an assignment used as a value
        //   java: w = h = (int) (62 * scale);
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), MAP_TEXTURE.get_mut(), 0, 0, ((0) as f32) as f32, ((0) as f32) as f32, 62, 62, 62, 62);
        if SkyblockerConfigManager::get().mining.crystals_hud.show_locations {
            for waypoint in crystals_locations_manager::ACTIVE_WAYPOINTS.get_mut().values().cloned() {
                let mut category: compat::Shared<dyn crate::de::hysky::skyblocker::skyblock::dwarven::mining_location_label::Category> = MiningLocationLabel::category(&waypoint);
                let mut render_pos: Vector2i = CrystalsHudWidget::transform_location(waypoint.lock().borrow().center_pos.as_ref().unwrap().x(), waypoint.lock().borrow().center_pos.as_ref().unwrap().z());
                let mut location_size: i32 = SkyblockerConfigManager::get().mining.crystals_hud.location_size;
                if SMALL_LOCATIONS.iter().any(|e| *e == ({ let __f = compat::dispatch(&category, |__t| __t.get_name()); __f(&category) })) {
                    location_size = location_size.wrapping_div(2);
                }
                graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), render_pos.x().wrapping_sub(location_size.wrapping_div(2)), render_pos.y().wrapping_sub(location_size.wrapping_div(2)), render_pos.x().wrapping_add(location_size.wrapping_div(2)), render_pos.y().wrapping_add(location_size.wrapping_div(2)), ({ let __f = compat::dispatch(&category, |__t| __t.get_color()); __f(&category) }));
            }
        }
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || minecraft::get_connection(compat::minecraft()).is_none() {
            gui_graphics_extractor::pop_pose(__pose_1);
            return;
        }
        let mut player_x: f64 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_x()).unwrap();
        let mut player_z: f64 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_z()).unwrap();
        let mut player_rotation: f32 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_y_rot()).unwrap();
        let mut render_pos: Vector2i = CrystalsHudWidget::transform_location(player_x, player_z);
        let mut render_x: i32 = render_pos.x().wrapping_sub(2);
        let mut render_y: i32 = render_pos.y().wrapping_sub(3);
        gui_graphics_extractor::pose_translate((render_x) as f32, (render_y) as f32);
        gui_graphics_extractor::pose_scale((0.75f32) as f32, (0.75f32) as f32);
        gui_graphics_extractor::pose_rotate(((mcrust::net::minecraft::util::mth::DEG_TO_RAD * CrystalsHudWidget::yaw2_cardinal(player_rotation))) as f32);
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), MAP_ICON.get_mut(), 0, 0, ((2) as f32) as f32, ((0) as f32) as f32, 5, 7, 8, 8);
        // TODO(fabric2seer): popMatrix() without a pushMatrix() in this method
        //   java: matrices.popMatrix();
    }

    /// CrystalsHudWidget.extractWidgetRenderState as the dispatch of HudWidget.extractWidgetRenderState
    pub fn extract_widget_render_state_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &mut GuiGraphicsExtractor, __p1: f32) {
        Self::extract_widget_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override protected void extractWidgetRenderStateForConfig(GuiGraphicsExtractor graphics, float delta)
    pub fn extract_widget_render_state_for_config_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut delta: f32) {
        ({ let __f = compat::dispatch(__this, |__t| __t.extract_widget_render_state()); __f(&compat::view(__this, |a| a as compat::Shared<dyn HudWidget>), graphics, delta) });
    }

    /// CrystalsHudWidget.extractWidgetRenderStateForConfig as the dispatch of HudWidget.extractWidgetRenderStateForConfig
    pub fn extract_widget_render_state_for_config_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &mut GuiGraphicsExtractor, __p1: f32) {
        Self::extract_widget_render_state_for_config_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}
