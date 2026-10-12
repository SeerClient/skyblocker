//! Ported from de.hysky.skyblocker.skyblock.entity.MobBoundingBoxes by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow::MobGlow;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final ObjectOpenHashSet<RenderableBox> BOXES_2_RENDER = new ObjectOpenHashSet<>();
///
/// A list: a `RenderableBox`'s record equality reaches its `float[]`, which compares by identity, and every box is
/// submitted with its own array - so the set never holds two equal boxes.
pub static BOXES_2_RENDER: compat::MainThread<Vec<RenderableBox>> = compat::MainThread::new(|| Vec::new());

/// java: public class MobBoundingBoxes
#[derive(Clone)]
pub struct MobBoundingBoxes {}

impl MobBoundingBoxes {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { MobBoundingBoxes::extract_rendering(&__a0); }));
    }

    /// java: public static boolean shouldDrawMobBoundingBox(Entity entity)
    pub fn should_draw_mob_bounding_box(entity: &dyn Entity) -> bool {
        if Utils::is_in_dungeons() && !entity.base().is_invisible() {
            let mut name: String = entity.get_name().get_string();
            let is_player = entity.as_living_entity().is_some_and(|living| living.as_player().is_some());
            let is_armor_stand = entity.as_any().is_some_and(|a| a.is::<mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand>());
            // java: return switch (entity) { case Player _ when .. -> ..; case Player _ when .. -> ..; case ArmorStand _ -> false; default -> .. };
            if is_player && (name == "Lost Adventurer" || name == "Shadow Assassin" || name == "Diamond Guy") {
                return SkyblockerConfigManager::get().dungeons.starred_mob_bounding_boxes;
            }
            if is_player && entity.base().get_id() == crate::de::hysky::skyblocker::skyblock::dungeon::livid_color::LividColor::get_correct_livid_id() {
                return crate::de::hysky::skyblocker::skyblock::dungeon::livid_color::LividColor::should_draw_bounding_box(&name);
            }
            if is_armor_stand {
                return false;
            }
            // Regular Mobs
            return SkyblockerConfigManager::get().dungeons.starred_mob_bounding_boxes && crate::de::hysky::skyblocker::skyblock::entity::glow::adder::dungeon_glow_adder::DungeonGlowAdder::is_starred(entity);
        }
        return false;
    }

    /// java: public static float[] getBoxColor(Entity entity)
    pub fn get_box_color(entity: &dyn Entity) -> Vec<f32> {
        let mut color: i32 = MobGlow::get_mob_glow(entity);
        return ColorUtils::get_float_components(color);
    }

    /// java: public static void submitBox2BeRendered(AABB box, float[] colorComponents)
    pub fn submit_box2_be_rendered(r#box: &AABB, color_components: &Vec<f32>) {
        BOXES_2_RENDER.get_mut().push(RenderableBox::new(r#box.clone(), color_components.clone()));
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        for r#box in BOXES_2_RENDER.get_mut().iter_mut() {
            // `box.extractRendering(collector)`: the record is final, so its own body
            r#box.extract_rendering_impl(collector);
        }
        BOXES_2_RENDER.get_mut().clear();
    }

}

/// java: private record RenderableBox(AABB box, float[] colorComponents) implements Renderable
#[derive(Clone, Debug, PartialEq)]
pub struct RenderableBox {
    /// java: AABB box
    pub r#box: AABB,
    /// java: float[] colorComponents
    pub color_components: Vec<f32>,
}

impl RenderableBox {
    /// java: the canonical record constructor
    pub fn new(mut r#box: AABB, mut color_components: Vec<f32>) -> Self {
        Self { r#box, color_components }
    }

    /// java: the record accessor `box()`
    pub fn r#box(&self) -> AABB {
        self.r#box.clone()
    }

    /// java: the record accessor `colorComponents()`
    pub fn color_components(&self) -> Vec<f32> {
        self.color_components.clone()
    }

}

impl Renderable for RenderableBox {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        RenderableBox::extract_rendering_dyn
    }

}

impl RenderableBox {
    /// java: @Override public void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering_impl(&mut self, collector: &compat::Shared<dyn PrimitiveCollector>) {
        ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box_aabb_float_float_boolean()); __f(collector, &self.r#box, &self.color_components, ((6) as f32) as f32, false) });
    }

    /// RenderableBox.extractRendering as the dispatch of Renderable.extractRendering
    pub fn extract_rendering_dyn(__this: &compat::Shared<dyn Renderable>, __p0: &compat::Shared<dyn PrimitiveCollector>) {
        compat::write_as::<Self, _, _>(__this, |__s| __s.extract_rendering_impl(__p0))
    }

}
