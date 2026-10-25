//! Ported from de.hysky.skyblocker.utils.render.primitive.PrimitiveCollectorImpl by fabric2seer.

use std::sync::Arc;
#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::frustum_utils::FrustumUtils;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::primitive::vanilla_renderer::VanillaRenderer;
use crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper;
use crate::de::hysky::skyblocker::utils::render::state::block_hologram_render_state::BlockHologramRenderState;
use crate::de::hysky::skyblocker::utils::render::state::cursor_line_render_state::CursorLineRenderState;
use crate::de::hysky::skyblocker::utils::render::state::cylinder_render_state::CylinderRenderState;
use crate::de::hysky::skyblocker::utils::render::state::filled_box_render_state::FilledBoxRenderState;
use crate::de::hysky::skyblocker::utils::render::state::filled_circle_render_state::FilledCircleRenderState;
use crate::de::hysky::skyblocker::utils::render::state::lines_render_state::LinesRenderState;
use crate::de::hysky::skyblocker::utils::render::state::outlined_box_render_state::OutlinedBoxRenderState;
use crate::de::hysky::skyblocker::utils::render::state::outlined_circle_render_state::OutlinedCircleRenderState;
use crate::de::hysky::skyblocker::utils::render::state::quad_render_state::QuadRenderState;
use crate::de::hysky::skyblocker::utils::render::state::sphere_render_state::SphereRenderState;
use crate::de::hysky::skyblocker::utils::render::state::text_render_state::TextRenderState;
use crate::de::hysky::skyblocker::utils::render::state::textured_quad_render_state::TexturedQuadRenderState;
use mcrust::com::mojang::blaze3d::systems::render_system;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::font::PreparedText;
use mcrust::net::minecraft::client::renderer::blockentity::state::beacon_render_state::BeaconRenderState;
use mcrust::net::minecraft::client::renderer::blockentity::state::beacon_render_state::Section;
use mcrust::net::minecraft::client::renderer::culling::frustum::Frustum;
use mcrust::net::minecraft::client::renderer::state::level::camera_render_state::CameraRenderState;
use mcrust::net::minecraft::client::renderer::state::level::level_render_state::LevelRenderState;
use mcrust::net::minecraft::client::renderer::submit_node_collector::SubmitNodeCollector;
use mcrust::net::minecraft::client::renderer::submit_node_storage::SubmitNodeStorage;
use mcrust::net::minecraft::client::renderer::feature::submit::submit_node::SubmitNode;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::util::formatted_char_sequence::FormattedCharSequence;
use mcrust::net::minecraft::util::formatted_char_sequence::Seq;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::world::phys::vec3;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::player::player::Player as _;

/// java: private static final int MAX_OVERWORLD_BUILD_HEIGHT = 319;
pub const MAX_OVERWORLD_BUILD_HEIGHT: i32 = 319;

/// OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): `SKYBLOCKER_VANILLA_COLLECTOR=1` restores the
/// per-primitive frustum copy, the per-frame backend query and the beacon beam's separate player null check.
pub static VANILLA_COLLECTOR: compat::EnvSwitch = compat::EnvSwitch::new("SKYBLOCKER_VANILLA_COLLECTOR");

/// `Frustum`'s hooked methods: while a non-Track injector targets one, the visibility test runs on a copy of the
/// frustum, as before, so no injector code ever runs inside the collector's borrow.
static FRUSTUM_STAND_DOWN: compat::HookStandDown = compat::HookStandDown::new(&[("net.minecraft.client.renderer.culling.Frustum", "*")], false);

/// The hooked methods `isVulkan`'s query calls: while a non-Track injector targets one, the query runs every frame.
static DEVICE_STAND_DOWN: compat::HookStandDown = compat::HookStandDown::new(
    &[
        ("com.mojang.blaze3d.", "getDevice"),
        ("com.mojang.blaze3d.", "tryGetDevice"),
        ("com.mojang.blaze3d.", "getDeviceInfo"),
        ("com.mojang.blaze3d.", "backendName"),
    ],
    false,
);

/// java: `isVulkan = RenderSystem.getDevice().getDeviceInfo().backendName().equalsIgnoreCase("vulkan")` (constructor).
///
/// OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): asked ONCE per process instead of once per frame
/// (a new collector every frame). The device is created once (`RenderSystem.initRenderer`, at startup) and its backend
/// cannot change without a restart, so every frame's query answers the first one's. Only an answer computed while no
/// injector targets the query's methods is kept.
fn backend_is_vulkan() -> bool {
    fn query() -> bool {
        compat::equals_ignore_case(&render_system::get_device().get_device_info().backend_name(), "vulkan")
    }
    static IS_VULKAN: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if VANILLA_COLLECTOR.on() || !DEVICE_STAND_DOWN.clear() {
        return query();
    }
    *IS_VULKAN.get_or_init(query)
}

/// java: public final class PrimitiveCollectorImpl implements PrimitiveCollector
pub struct PrimitiveCollectorImpl {
    /// java: private final boolean isVulkan;
    pub is_vulkan: bool,
    /// java: private final LevelRenderState worldState; - the collector's only use of it is
    /// `worldState.blockEntityRenderStates.add(state)` (`submitBeaconBeam`). The level state is the game's, lent to
    /// `RenderHelper.startExtraction` and not held across the listeners it calls, so the added states wait here and
    /// `RenderHelper` appends them to that same list when the collection ends (`end_collection_into`).
    pub world_state_block_entity_render_states: Vec<mcrust::net::minecraft::client::renderer::blockentity::state::block_entity_render_state::PooledBlockEntityRenderState>,
    /// java: private final Frustum frustum;
    pub frustum: Frustum,
    /// java: private final List<VanillaSubmittable<?>> vanillaSubmittables = new ArrayList<>();
    /// (`List<VanillaSubmittable<?>>`: each record behind its type-erased submit)
    pub vanilla_submittables: Vec<std::rc::Rc<dyn ErasedVanillaSubmittable>>,
    /// java: private final List<FilledBoxRenderState> filledBoxStates = new ArrayList<>();
    pub filled_box_states: Vec<FilledBoxRenderState>,
    /// java: private final List<FilledBoxRenderState> filledBoxThroughWallsStates = new ArrayList<>();
    pub filled_box_through_walls_states: Vec<FilledBoxRenderState>,
    /// java: private final List<OutlinedBoxRenderState> outlinedBoxStates = new ArrayList<>();
    pub outlined_box_states: Vec<OutlinedBoxRenderState>,
    /// java: private final List<OutlinedBoxRenderState> outlinedBoxThroughWallsStates = new ArrayList<>();
    pub outlined_box_through_walls_states: Vec<OutlinedBoxRenderState>,
    /// java: private final List<LinesRenderState> linesStates = new ArrayList<>();
    pub lines_states: Vec<LinesRenderState>,
    /// java: private final List<LinesRenderState> linesThroughWallsStates = new ArrayList<>();
    pub lines_through_walls_states: Vec<LinesRenderState>,
    /// java: private final List<CursorLineRenderState> cursorLineStates = new ArrayList<>();
    pub cursor_line_states: Vec<CursorLineRenderState>,
    /// java: private final List<QuadRenderState> quadStates = new ArrayList<>();
    pub quad_states: Vec<QuadRenderState>,
    /// java: private final List<QuadRenderState> quadThroughWallsStates = new ArrayList<>();
    pub quad_through_walls_states: Vec<QuadRenderState>,
    /// java: private final List<TexturedQuadRenderState> texturedQuadStates = new ArrayList<>();
    pub textured_quad_states: Vec<TexturedQuadRenderState>,
    /// java: private final List<TexturedQuadRenderState> texturedQuadThroughWallsStates = new ArrayList<>();
    pub textured_quad_through_walls_states: Vec<TexturedQuadRenderState>,
    /// java: private final List<BlockHologramRenderState> blockHologramStates = new ArrayList<>();
    pub block_hologram_states: Vec<BlockHologramRenderState>,
    /// java: private final List<TextRenderState> textStates = new ArrayList<>();
    pub text_states: Vec<TextRenderState>,
    /// java: private final List<TextRenderState> textThroughWallsStates = new ArrayList<>();
    pub text_through_walls_states: Vec<TextRenderState>,
    /// java: private final List<CylinderRenderState> cylinderStates = new ArrayList<>();
    pub cylinder_states: Vec<CylinderRenderState>,
    /// java: private final List<SphereRenderState> sphereStates = new ArrayList<>();
    pub sphere_states: Vec<SphereRenderState>,
    /// java: private final List<FilledCircleRenderState> filledCircleStates = new ArrayList<>();
    pub filled_circle_states: Vec<FilledCircleRenderState>,
    /// java: private final List<OutlinedCircleRenderState> outlinedCircleStates = new ArrayList<>();
    pub outlined_circle_states: Vec<OutlinedCircleRenderState>,
    /// java: private boolean frozen = false;
    pub frozen: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl PrimitiveCollectorImpl {
    /// java: public PrimitiveCollectorImpl(LevelRenderState worldState, Frustum frustum)
    pub fn new(frustum: Frustum) -> compat::Shared<Self> {
        // OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): see `backend_is_vulkan`. Was:
        //     let __init_is_vulkan = compat::equals_ignore_case(&render_system::get_device().get_device_info().backend_name(), "vulkan");
        let __init_is_vulkan = backend_is_vulkan();
        let __init_frustum = frustum.clone();
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            is_vulkan: __init_is_vulkan,
            world_state_block_entity_render_states: Vec::new(),
            frustum: __init_frustum,
            vanilla_submittables: Vec::new(),
            filled_box_states: Vec::new(),
            filled_box_through_walls_states: Vec::new(),
            outlined_box_states: Vec::new(),
            outlined_box_through_walls_states: Vec::new(),
            lines_states: Vec::new(),
            lines_through_walls_states: Vec::new(),
            cursor_line_states: Vec::new(),
            quad_states: Vec::new(),
            quad_through_walls_states: Vec::new(),
            textured_quad_states: Vec::new(),
            textured_quad_through_walls_states: Vec::new(),
            block_hologram_states: Vec::new(),
            text_states: Vec::new(),
            text_through_walls_states: Vec::new(),
            cylinder_states: Vec::new(),
            sphere_states: Vec::new(),
            filled_circle_states: Vec::new(),
            outlined_circle_states: Vec::new(),
            frozen: false,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// `FrustumUtils.isVisible(this.frustum, minX, minY, minZ, maxX, maxY, maxZ)`.
    ///
    /// OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): tested under the collector's read borrow on
    /// the frustum in place, instead of on a copy of the ~300-byte frustum per submitted primitive. The test reads the
    /// frustum and calls `Frustum.isVisible`, nothing else: with no injector on `Frustum` (`FRUSTUM_STAND_DOWN`) no
    /// code can reach this collector while the borrow is held. The same frustum value, the same answer.
    fn frustum_visible(__this: &compat::Shared<Self>, min_x: f64, min_y: f64, min_z: f64, max_x: f64, max_y: f64, max_z: f64) -> bool {
        if VANILLA_COLLECTOR.on() || !FRUSTUM_STAND_DOWN.clear() {
            // Was (at every submit): FrustumUtils::is_visible_7(&compat::read(__this, |__t| __t.frustum.clone()), ..)
            return FrustumUtils::is_visible_7(&compat::read(__this, |__t| __t.frustum.clone()), min_x, min_y, min_z, max_x, max_y, max_z);
        }
        compat::read(__this, |__t| FrustumUtils::is_visible_7(&__t.frustum, min_x, min_y, min_z, max_x, max_y, max_z))
    }

    /// java: private void submitFilledBox(double minX, double minY, double minZ, double maxX, double maxY, double maxZ, float[] colourComponents, float alpha, boolean throughWalls)
    pub fn submit_filled_box_9(__this: &compat::Shared<Self>, mut min_x: f64, mut min_y: f64, mut min_z: f64, mut max_x: f64, mut max_y: f64, mut max_z: f64, colour_components: &Vec<f32>, mut alpha: f32, mut through_walls: bool) {
        PrimitiveCollectorImpl::ensure_not_frozen(__this);
        // Was: if !(FrustumUtils::is_visible_7(&compat::read(__this, |__t| __t.frustum.clone()), min_x, min_y, min_z, max_x, max_y, max_z)) {
        if !(PrimitiveCollectorImpl::frustum_visible(__this, min_x, min_y, min_z, max_x, max_y, max_z)) {
            return;
        }
        let mut state: FilledBoxRenderState = FilledBoxRenderState::new(min_x, min_y, min_z, max_x, max_y, max_z, colour_components.clone(), alpha);
        if through_walls {
            let __h0 = state.clone();
            __this.lock().borrow_mut().filled_box_through_walls_states.push(__h0);
        } else {
            let __h0 = state.clone();
            __this.lock().borrow_mut().filled_box_states.push(__h0);
        }
    }

    /// java: private void submitBeaconBeam(BlockPos pos, float[] colourComponents)
    pub fn submit_beacon_beam(__this: &compat::Shared<Self>, mut pos: BlockPos, colour_components: &Vec<f32>) {
        PrimitiveCollectorImpl::ensure_not_frozen(__this);
        // Was: if !(FrustumUtils::is_visible_7(&compat::read(__this, |__t| __t.frustum.clone()), ..)) {
        if !(PrimitiveCollectorImpl::frustum_visible(__this, ((pos.get_x()) as f64) as f64, ((pos.get_y()) as f64) as f64, ((pos.get_z()) as f64) as f64, ((pos.get_x().wrapping_add(1)) as f64) as f64, ((MAX_OVERWORLD_BUILD_HEIGHT) as f64) as f64, ((pos.get_z().wrapping_add(1)) as f64) as f64)) {
            return;
        }
        let mut colour: i32 = argb::color_from_float(1.0f32, colour_components[(0) as usize], colour_components[(1) as usize], colour_components[(2) as usize]);
        let mut length: f32 = ((RenderHelper::get_camera().position().subtract_vec(&vec3::at_center_of(&pos)).horizontal_distance()) as f32);
        // java: BeaconRenderState state = new BeaconRenderState(); - the port's beacon state is its BlockEntityRenderState
        // half (`base`) with the beacon's own fields (`beacon`)
        let mut state = mcrust::net::minecraft::client::renderer::blockentity::beacon_renderer::BeaconRenderStateFull {
            base: mcrust::net::minecraft::client::renderer::blockentity::state::block_entity_render_state::BlockEntityRenderState::new(),
            beacon: BeaconRenderState::new(),
        };
        state.base.block_pos = pos.clone();
        // java: state.blockState = Blocks.BEACON.defaultBlockState();
        state.base.set_block_state(blocks::beacon().default_block_state());
        state.base.block_entity_type = mcrust::net::minecraft::world::level::block::entity::block_entity_types::beacon();
        state.base.light_coords = mcrust::net::minecraft::util::light_coords_util::FULL_BRIGHT;
        state.base.break_progress = None;
        state.beacon.animation_time = (if mcrust::mcrust::modding::game::level().is_some() { ((compat::floor_mod_i64(mcrust::mcrust::modding::game::level().unwrap().get_game_time(), (40 as i64)) as f32) + compat::minecraft().get_delta_tracker().get_game_time_delta_partial_tick(true)) } else { 0.0f32 });
        state.beacon.sections.push(Section { color: colour, height: MAX_OVERWORLD_BUILD_HEIGHT });
        // OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): the null check folded into the read -
        // `with_player` answers `None` for a null player and `isScoping` is then not asked, as the `&&` did. Was:
        //     state.beacon.beam_radius_scale = (if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && mcrust::mcrust::modding::game::with_player(|player| player.is_scoping()).unwrap_or(false) { 1.0f32 } else { (1.0f32).max((length / 96.0f32)) });
        let scoping: bool = if VANILLA_COLLECTOR.on() {
            mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && mcrust::mcrust::modding::game::with_player(|player| player.is_scoping()).unwrap_or(false)
        } else {
            mcrust::mcrust::modding::game::with_player(|player| player.is_scoping()).unwrap_or(false)
        };
        state.beacon.beam_radius_scale = (if scoping { 1.0f32 } else { (1.0f32).max((length / 96.0f32)) });
        // java: this.worldState.blockEntityRenderStates.add(state);
        let __h0 = mcrust::net::minecraft::client::renderer::blockentity::state::block_entity_render_state::PooledBlockEntityRenderState::new(Box::new(state));
        __this.lock().borrow_mut().world_state_block_entity_render_states.push(__h0);
    }

    /// java: private void submitOutlinedBox(double minX, double minY, double minZ, double maxX, double maxY, double maxZ, float[] colourComponents, float alpha, float lineWidth, boolean throughWalls)
    pub fn submit_outlined_box_10(__this: &compat::Shared<Self>, mut min_x: f64, mut min_y: f64, mut min_z: f64, mut max_x: f64, mut max_y: f64, mut max_z: f64, colour_components: &Vec<f32>, mut alpha: f32, mut line_width: f32, mut through_walls: bool) {
        PrimitiveCollectorImpl::ensure_not_frozen(__this);
        // Was: if !(FrustumUtils::is_visible_7(&compat::read(__this, |__t| __t.frustum.clone()), min_x, min_y, min_z, max_x, max_y, max_z)) {
        if !(PrimitiveCollectorImpl::frustum_visible(__this, min_x, min_y, min_z, max_x, max_y, max_z)) {
            return;
        }
        let mut state: OutlinedBoxRenderState = OutlinedBoxRenderState::new(min_x, min_y, min_z, max_x, max_y, max_z, colour_components.clone(), alpha, line_width);
        if through_walls {
            let __h0 = state.clone();
            __this.lock().borrow_mut().outlined_box_through_walls_states.push(__h0);
        } else {
            let __h0 = state.clone();
            __this.lock().borrow_mut().outlined_box_states.push(__h0);
        }
    }

    /// java: private void submitText(FormattedCharSequence text, Vec3 pos, float scale, float yOffset, boolean throughWalls)
    ///
    /// (`text`: the component whose `getVisualOrderText()` Java hands in - the port's `Font.prepareText` /
    /// `Font.width` take the component and lay it out in visual order themselves)
    pub fn submit_text_formatted_char_sequence_vec3_float_float_boolean(__this: &compat::Shared<Self>, text: &mcrust::net::minecraft::network::chat::formatted_text::FormattedText, pos: &Vec3, mut scale: f32, mut y_offset: f32, mut through_walls: bool) {
        PrimitiveCollectorImpl::ensure_not_frozen(__this);
        let text_renderer: Font = compat::font();
        let x_offset: f32 = ((-text_renderer.width_of(text)) as f32) / 2.0f32;
        // java: Font.PreparedText glyphs = textRenderer.prepareText(text, xOffset, yOffset, CommonColors.WHITE, false, false, 0);
        // (`includeEmpty` false: the port's `prepare_text` keeps no empty glyphs)
        let glyphs: PreparedText = text_renderer.prepare_text(text, x_offset, y_offset, mcrust::net::minecraft::util::common_colors::WHITE, false, 0);
        let mut state: TextRenderState = TextRenderState::new(glyphs.clone(), pos.clone(), (scale * 0.025f32), y_offset);
        if through_walls {
            let __h0 = state.clone();
            __this.lock().borrow_mut().text_through_walls_states.push(__h0);
        } else {
            let __h0 = state.clone();
            __this.lock().borrow_mut().text_states.push(__h0);
        }
    }

    /// java: public void endCollection()
    pub fn end_collection(__this: &compat::Shared<Self>) {
        { let __v = true; compat::write(__this, |__t| __t.frozen = __v); }
    }

    /// java: private void ensureNotFrozen()
    pub fn ensure_not_frozen(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.frozen) {
            panic!("IllegalStateException: {}", "Cannot submit primitives once the collection phase has ended!");
        }
    }

    /// RenderHelper.startExtraction's end: `collector.endCollection()`, with the states `submitBeaconBeam` added to
    /// `worldState.blockEntityRenderStates` appended to that list (see `world_state_block_entity_render_states`).
    pub fn end_collection_into(__this: &compat::Shared<Self>, world_state: &mut LevelRenderState) {
        PrimitiveCollectorImpl::end_collection(__this);
        let added = std::mem::take(&mut __this.lock().borrow_mut().world_state_block_entity_render_states);
        world_state.block_entity_render_states.extend(added);
    }

    /// java: @SuppressWarnings("unchecked") public void dispatchSubmits(LevelRenderState levelState, SubmitNodeCollector submitNodeCollector)
    ///
    /// The collector is used once (Java drops it after this call), so each list is MOVED into its submit where Java
    /// hands the list itself.
    pub fn dispatch_submits(__this: &compat::Shared<Self>, level_state: &LevelRenderState, submit_node_collector: &mut SubmitNodeStorage) {
        use mcrust::net::minecraft::client::renderer::submit_node_collection::PhaseId;
        use super::{block_hologram_feature_renderer, cursor_line_feature_renderer, cylinder_feature_renderer, filled_box_feature_renderer, filled_box_instanced_feature_renderer, filled_circle_feature_renderer, lines_feature_renderer, outlined_box_feature_renderer, outlined_box_instanced_feature_renderer, outlined_circle_feature_renderer, quad_feature_renderer, sphere_feature_renderer, text_feature_renderer, textured_quad_feature_renderer};
        if !(compat::read(__this, |__t| __t.frozen)) {
            panic!("IllegalStateException: {}", "Cannot dispatch submits until the collection phase has ended!");
        }
        let camera = &level_state.camera_render_state;
        let is_vulkan = compat::read(__this, |__t| __t.is_vulkan);
        let vanilla_submittables = std::mem::take(&mut __this.lock().borrow_mut().vanilla_submittables);
        if !vanilla_submittables.is_empty() {
            for submittable in vanilla_submittables.iter() {
                // java: ((VanillaRenderer<Object>) submittable.renderer).submitVanilla(submittable.state(), levelState, submitNodeCollector);
                submittable.submit(level_state, submit_node_collector);
            }
        }
        // SubmitRenderPhases.AFTER_TERRAIN / ALWAYS_ON_TOP / TEXTS - the frame's phases of those names
        let mut submit_custom = |phase: PhaseId, node: std::sync::Arc<dyn SubmitNode>| compat::leftovers_submit_custom(submit_node_collector, phase, node);
        let this_guard = __this.lock();
        let mut this = this_guard.borrow_mut();
        if !this.filled_box_states.is_empty() {
            let states = std::mem::take(&mut this.filled_box_states);
            if is_vulkan {
                submit_custom(PhaseId::AfterTerrain, std::sync::Arc::new(filled_box_instanced_feature_renderer::Submit::new(states, camera.clone(), false)));
            } else {
                submit_custom(PhaseId::AfterTerrain, std::sync::Arc::new(filled_box_feature_renderer::Submit::new(states, camera.clone(), false)));
            }
        }
        if !this.filled_box_through_walls_states.is_empty() {
            let states = std::mem::take(&mut this.filled_box_through_walls_states);
            if is_vulkan {
                submit_custom(PhaseId::AlwaysOnTopGizmos, std::sync::Arc::new(filled_box_instanced_feature_renderer::Submit::new(states, camera.clone(), true)));
            } else {
                submit_custom(PhaseId::AlwaysOnTopGizmos, std::sync::Arc::new(filled_box_feature_renderer::Submit::new(states, camera.clone(), true)));
            }
        }
        if !this.outlined_box_states.is_empty() {
            let states = std::mem::take(&mut this.outlined_box_states);
            if is_vulkan {
                submit_custom(PhaseId::AfterTerrain, std::sync::Arc::new(outlined_box_instanced_feature_renderer::Submit::new(states, camera.clone(), false)));
            } else {
                submit_custom(PhaseId::AfterTerrain, std::sync::Arc::new(outlined_box_feature_renderer::Submit::new(states, camera.clone(), false)));
            }
        }
        if !this.outlined_box_through_walls_states.is_empty() {
            let states = std::mem::take(&mut this.outlined_box_through_walls_states);
            if is_vulkan {
                submit_custom(PhaseId::AlwaysOnTopGizmos, std::sync::Arc::new(outlined_box_instanced_feature_renderer::Submit::new(states, camera.clone(), true)));
            } else {
                submit_custom(PhaseId::AlwaysOnTopGizmos, std::sync::Arc::new(outlined_box_feature_renderer::Submit::new(states, camera.clone(), true)));
            }
        }
        if !this.lines_states.is_empty() {
            submit_custom(PhaseId::AfterTerrain, std::sync::Arc::new(lines_feature_renderer::Submit::new(std::mem::take(&mut this.lines_states), camera.clone(), false)));
        }
        if !this.lines_through_walls_states.is_empty() {
            submit_custom(PhaseId::AlwaysOnTopGizmos, std::sync::Arc::new(lines_feature_renderer::Submit::new(std::mem::take(&mut this.lines_through_walls_states), camera.clone(), true)));
        }
        if !this.cursor_line_states.is_empty() {
            submit_custom(PhaseId::AlwaysOnTopGizmos, std::sync::Arc::new(cursor_line_feature_renderer::Submit::new(std::mem::take(&mut this.cursor_line_states), camera.clone())));
        }
        if !this.quad_states.is_empty() {
            submit_custom(PhaseId::AfterTerrain, std::sync::Arc::new(quad_feature_renderer::Submit::new(std::mem::take(&mut this.quad_states), camera.clone(), false)));
        }
        if !this.quad_through_walls_states.is_empty() {
            submit_custom(PhaseId::AlwaysOnTopGizmos, std::sync::Arc::new(quad_feature_renderer::Submit::new(std::mem::take(&mut this.quad_through_walls_states), camera.clone(), true)));
        }
        if !this.textured_quad_states.is_empty() {
            submit_custom(PhaseId::AfterTerrain, std::sync::Arc::new(textured_quad_feature_renderer::Submit::new(std::mem::take(&mut this.textured_quad_states), camera.clone(), false)));
        }
        if !this.textured_quad_through_walls_states.is_empty() {
            submit_custom(PhaseId::AlwaysOnTopGizmos, std::sync::Arc::new(textured_quad_feature_renderer::Submit::new(std::mem::take(&mut this.textured_quad_through_walls_states), camera.clone(), true)));
        }
        if !this.block_hologram_states.is_empty() {
            submit_custom(PhaseId::AfterTerrain, std::sync::Arc::new(block_hologram_feature_renderer::Submit::new(std::mem::take(&mut this.block_hologram_states), camera.clone())));
        }
        if !this.text_states.is_empty() {
            submit_custom(PhaseId::Texts, std::sync::Arc::new(text_feature_renderer::Submit::new(std::mem::take(&mut this.text_states), camera.clone(), false)));
        }
        if !this.text_through_walls_states.is_empty() {
            submit_custom(PhaseId::AlwaysOnTopGizmos, std::sync::Arc::new(text_feature_renderer::Submit::new(std::mem::take(&mut this.text_through_walls_states), camera.clone(), true)));
        }
        if !this.cylinder_states.is_empty() {
            submit_custom(PhaseId::AfterTerrain, std::sync::Arc::new(cylinder_feature_renderer::Submit::new(std::mem::take(&mut this.cylinder_states), camera.clone())));
        }
        if !this.sphere_states.is_empty() {
            submit_custom(PhaseId::AfterTerrain, std::sync::Arc::new(sphere_feature_renderer::Submit::new(std::mem::take(&mut this.sphere_states), camera.clone())));
        }
        if !this.filled_circle_states.is_empty() {
            submit_custom(PhaseId::AfterTerrain, std::sync::Arc::new(filled_circle_feature_renderer::Submit::new(std::mem::take(&mut this.filled_circle_states), camera.clone())));
        }
        if !this.outlined_circle_states.is_empty() {
            submit_custom(PhaseId::AfterTerrain, std::sync::Arc::new(outlined_circle_feature_renderer::Submit::new(std::mem::take(&mut this.outlined_circle_states), camera.clone())));
        }
    }

}

impl PrimitiveCollector for PrimitiveCollectorImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public <S> void submitVanilla(S state, VanillaRenderer<S> renderer)
    fn submit_vanilla<S: Clone + 'static>(&mut self, state: &S, renderer: &compat::Shared<dyn VanillaRenderer<S>>) where Self: Sized {
        PrimitiveCollectorImpl::ensure_not_frozen(&self.self_ref.upgrade().unwrap());
        self.vanilla_submittables.push(std::rc::Rc::new(VanillaSubmittable::new(state.clone(), renderer.clone())) as std::rc::Rc<dyn ErasedVanillaSubmittable>);
    }

    /// java: @Override public void submitFilledBoxWithBeaconBeam(BlockPos pos, float[] colourComponents, float alpha, boolean throughWalls) (dispatch: the body for this class)
    fn submit_filled_box_with_beacon_beam(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &BlockPos, &Vec<f32>, f32, bool) {
        PrimitiveCollectorImpl::submit_filled_box_with_beacon_beam_dyn
    }

    /// java: @Override public void submitFilledBoxWithBeaconBeam(AABB box, float[] colourComponents, float alpha, boolean throughWalls) (dispatch: the body for this class)
    fn submit_filled_box_with_beacon_beam_aabb_float_float_boolean(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &AABB, &Vec<f32>, f32, bool) {
        PrimitiveCollectorImpl::submit_filled_box_with_beacon_beam_aabb_float_float_boolean_dyn
    }

    /// java: @Override public void submitFilledBox(BlockPos pos, float[] colourComponents, float alpha, boolean throughWalls) (dispatch: the body for this class)
    fn submit_filled_box(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &BlockPos, &Vec<f32>, f32, bool) {
        PrimitiveCollectorImpl::submit_filled_box_dyn
    }

    /// java: @Override public void submitFilledBox(Vec3 pos, Vec3 dimensions, float[] colourComponents, float alpha, boolean throughWalls) (dispatch: the body for this class)
    fn submit_filled_box_5(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &Vec3, &Vec3, &Vec<f32>, f32, bool) {
        PrimitiveCollectorImpl::submit_filled_box_5_dyn
    }

    /// java: @Override public void submitFilledBox(AABB box, float[] colourComponents, float alpha, boolean throughWalls) (dispatch: the body for this class)
    fn submit_filled_box_aabb_float_float_boolean(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &AABB, &Vec<f32>, f32, bool) {
        PrimitiveCollectorImpl::submit_filled_box_aabb_float_float_boolean_dyn
    }

    /// java: @Override public void submitOutlinedBox(BlockPos pos, float[] colourComponents, float lineWidth, boolean throughWalls) (dispatch: the body for this class)
    fn submit_outlined_box(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &BlockPos, &Vec<f32>, f32, bool) {
        PrimitiveCollectorImpl::submit_outlined_box_dyn
    }

    /// java: @Override public void submitOutlinedBox(AABB box, float[] colourComponents, float lineWidth, boolean throughWalls) (dispatch: the body for this class)
    fn submit_outlined_box_aabb_float_float_boolean(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &AABB, &Vec<f32>, f32, bool) {
        PrimitiveCollectorImpl::submit_outlined_box_aabb_float_float_boolean_dyn
    }

    /// java: @Override public void submitOutlinedBox(AABB box, float[] colourComponents, float alpha, float lineWidth, boolean throughWalls) (dispatch: the body for this class)
    fn submit_outlined_box_5(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &AABB, &Vec<f32>, f32, f32, bool) {
        PrimitiveCollectorImpl::submit_outlined_box_5_dyn
    }

    /// java: @Override public void submitLinesFromPoints(Vec3[] points, float[] colourComponents, float alpha, float lineWidth, boolean throughWalls) (dispatch: the body for this class)
    fn submit_lines_from_points(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &Vec<Vec3>, &Vec<f32>, f32, f32, bool) {
        PrimitiveCollectorImpl::submit_lines_from_points_dyn
    }

    /// java: @Override public void submitLineFromCursor(Vec3 point, float[] colourComponents, float alpha, float lineWidth) (dispatch: the body for this class)
    fn submit_line_from_cursor(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &Vec3, &Vec<f32>, f32, f32) {
        PrimitiveCollectorImpl::submit_line_from_cursor_dyn
    }

    /// java: @Override public void submitQuad(Vec3[] points, float[] colourComponents, float alpha, boolean throughWalls) (dispatch: the body for this class)
    fn submit_quad(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &Vec<Vec3>, &Vec<f32>, f32, bool) {
        PrimitiveCollectorImpl::submit_quad_dyn
    }

    /// java: @Override public void submitTexturedQuad(Vec3 pos, float width, float height, float textureWidth, float textureHeight, Vec3 renderOffset, Identifier texture, float[] shaderColour, float alpha, boolean (dispatch: the body for this class)
    fn submit_textured_quad(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &Vec3, f32, f32, f32, f32, &Vec3, &Identifier, &Vec<f32>, f32, bool) {
        PrimitiveCollectorImpl::submit_textured_quad_dyn
    }

    /// java: @Override public void submitBlockHologram(BlockPos pos, BlockState state, float alpha) (dispatch: the body for this class)
    fn submit_block_hologram(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &BlockPos, &Arc<BlockState>, f32) {
        PrimitiveCollectorImpl::submit_block_hologram_dyn
    }

    /// java: @Override public void submitText(Component text, Vec3 pos, boolean throughWalls) (dispatch: the body for this class)
    fn submit_text(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &MutableComponent, &Vec3, bool) {
        PrimitiveCollectorImpl::submit_text_dyn
    }

    /// java: @Override public void submitText(Component text, Vec3 pos, float scale, boolean throughWalls) (dispatch: the body for this class)
    fn submit_text_4(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &MutableComponent, &Vec3, f32, bool) {
        PrimitiveCollectorImpl::submit_text_4_dyn
    }

    /// java: @Override public void submitText(Component text, Vec3 pos, float scale, float yOffset, boolean throughWalls) (dispatch: the body for this class)
    fn submit_text_5(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &MutableComponent, &Vec3, f32, f32, bool) {
        PrimitiveCollectorImpl::submit_text_5_dyn
    }

    /// java: @Override public void submitCylinder(Vec3 centre, float radius, float height, int segments, int colour) (dispatch: the body for this class)
    fn submit_cylinder(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &Vec3, f32, f32, i32, i32) {
        PrimitiveCollectorImpl::submit_cylinder_dyn
    }

    /// java: @Override public void submitSphere(Vec3 centre, float radius, int segments, int rings, int colour) (dispatch: the body for this class)
    fn submit_sphere(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &Vec3, f32, i32, i32, i32) {
        PrimitiveCollectorImpl::submit_sphere_dyn
    }

    /// java: @Override public void submitFilledCircle(Vec3 centre, float radius, int segments, int colour) (dispatch: the body for this class)
    fn submit_filled_circle(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &Vec3, f32, i32, i32) {
        PrimitiveCollectorImpl::submit_filled_circle_dyn
    }

    /// java: @Override public void submitOutlinedCircle(Vec3 centre, float radius, float thickness, int segments, int colour) (dispatch: the body for this class)
    fn submit_outlined_circle(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &Vec3, f32, f32, i32, i32) {
        PrimitiveCollectorImpl::submit_outlined_circle_dyn
    }

}

impl PrimitiveCollectorImpl {
    /// java: @Override public void submitFilledBoxWithBeaconBeam(BlockPos pos, float[] colourComponents, float alpha, boolean throughWalls)
    pub fn submit_filled_box_with_beacon_beam_impl(__this: &compat::Shared<Self>, pos: &BlockPos, colour_components: &Vec<f32>, mut alpha: f32, mut through_walls: bool) {
        ({ let __f = compat::dispatch(__this, |__t| __t.submit_filled_box()); __f(&compat::view(__this, |a| a as compat::Shared<dyn PrimitiveCollector>), pos, colour_components, alpha, through_walls) });
        PrimitiveCollectorImpl::submit_beacon_beam(__this, pos.clone(), colour_components);
    }

    /// PrimitiveCollectorImpl.submitFilledBoxWithBeaconBeam as the dispatch of PrimitiveCollector.submitFilledBoxWithBeaconBeam
    pub fn submit_filled_box_with_beacon_beam_dyn(__this: &compat::Shared<dyn PrimitiveCollector>, __p0: &BlockPos, __p1: &Vec<f32>, __p2: f32, __p3: bool) {
        Self::submit_filled_box_with_beacon_beam_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

    /// java: @Override public void submitFilledBoxWithBeaconBeam(AABB box, float[] colourComponents, float alpha, boolean throughWalls)
    pub fn submit_filled_box_with_beacon_beam_aabb_float_float_boolean_impl(__this: &compat::Shared<Self>, r#box: &AABB, colour_components: &Vec<f32>, mut alpha: f32, mut through_walls: bool) {
        ({ let __f = compat::dispatch(__this, |__t| __t.submit_filled_box_aabb_float_float_boolean()); __f(&compat::view(__this, |a| a as compat::Shared<dyn PrimitiveCollector>), r#box, colour_components, alpha, through_walls) });
        PrimitiveCollectorImpl::submit_beacon_beam(__this, BlockPos::new((((r#box).min_x) as i32), (((r#box).min_y) as i32), (((r#box).min_z) as i32)), colour_components);
    }

    /// PrimitiveCollectorImpl.submitFilledBoxWithBeaconBeam as the dispatch of PrimitiveCollector.submitFilledBoxWithBeaconBeam
    pub fn submit_filled_box_with_beacon_beam_aabb_float_float_boolean_dyn(__this: &compat::Shared<dyn PrimitiveCollector>, __p0: &AABB, __p1: &Vec<f32>, __p2: f32, __p3: bool) {
        Self::submit_filled_box_with_beacon_beam_aabb_float_float_boolean_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

    /// java: @Override public void submitFilledBox(BlockPos pos, float[] colourComponents, float alpha, boolean throughWalls)
    pub fn submit_filled_box_impl(__this: &compat::Shared<Self>, pos: &BlockPos, colour_components: &Vec<f32>, mut alpha: f32, mut through_walls: bool) {
        PrimitiveCollectorImpl::submit_filled_box_9(__this, ((pos.get_x()) as f64) as f64, ((pos.get_y()) as f64) as f64, ((pos.get_z()) as f64) as f64, ((pos.get_x().wrapping_add(1)) as f64) as f64, ((pos.get_y().wrapping_add(1)) as f64) as f64, ((pos.get_z().wrapping_add(1)) as f64) as f64, colour_components, alpha, through_walls);
    }

    /// PrimitiveCollectorImpl.submitFilledBox as the dispatch of PrimitiveCollector.submitFilledBox
    pub fn submit_filled_box_dyn(__this: &compat::Shared<dyn PrimitiveCollector>, __p0: &BlockPos, __p1: &Vec<f32>, __p2: f32, __p3: bool) {
        Self::submit_filled_box_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

    /// java: @Override public void submitFilledBox(Vec3 pos, Vec3 dimensions, float[] colourComponents, float alpha, boolean throughWalls)
    pub fn submit_filled_box_5_impl(__this: &compat::Shared<Self>, pos: &Vec3, dimensions: &Vec3, colour_components: &Vec<f32>, mut alpha: f32, mut through_walls: bool) {
        PrimitiveCollectorImpl::submit_filled_box_9(__this, pos.x, pos.y, pos.z, (pos.x + dimensions.x), (pos.y + dimensions.y), (pos.z + dimensions.z), colour_components, alpha, through_walls);
    }

    /// PrimitiveCollectorImpl.submitFilledBox as the dispatch of PrimitiveCollector.submitFilledBox
    pub fn submit_filled_box_5_dyn(__this: &compat::Shared<dyn PrimitiveCollector>, __p0: &Vec3, __p1: &Vec3, __p2: &Vec<f32>, __p3: f32, __p4: bool) {
        Self::submit_filled_box_5_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4)
    }

    /// java: @Override public void submitFilledBox(AABB box, float[] colourComponents, float alpha, boolean throughWalls)
    pub fn submit_filled_box_aabb_float_float_boolean_impl(__this: &compat::Shared<Self>, r#box: &AABB, colour_components: &Vec<f32>, mut alpha: f32, mut through_walls: bool) {
        PrimitiveCollectorImpl::submit_filled_box_9(__this, (r#box).min_x, (r#box).min_y, (r#box).min_z, (r#box).max_x, (r#box).max_y, (r#box).max_z, colour_components, alpha, through_walls);
    }

    /// PrimitiveCollectorImpl.submitFilledBox as the dispatch of PrimitiveCollector.submitFilledBox
    pub fn submit_filled_box_aabb_float_float_boolean_dyn(__this: &compat::Shared<dyn PrimitiveCollector>, __p0: &AABB, __p1: &Vec<f32>, __p2: f32, __p3: bool) {
        Self::submit_filled_box_aabb_float_float_boolean_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

    /// java: @Override public void submitOutlinedBox(BlockPos pos, float[] colourComponents, float lineWidth, boolean throughWalls)
    pub fn submit_outlined_box_impl(__this: &compat::Shared<Self>, pos: &BlockPos, colour_components: &Vec<f32>, mut line_width: f32, mut through_walls: bool) {
        PrimitiveCollectorImpl::submit_outlined_box_10(__this, ((pos.get_x()) as f64) as f64, ((pos.get_y()) as f64) as f64, ((pos.get_z()) as f64) as f64, ((pos.get_x().wrapping_add(1)) as f64) as f64, ((pos.get_y().wrapping_add(1)) as f64) as f64, ((pos.get_z().wrapping_add(1)) as f64) as f64, colour_components, 1.0f32, line_width, through_walls);
    }

    /// PrimitiveCollectorImpl.submitOutlinedBox as the dispatch of PrimitiveCollector.submitOutlinedBox
    pub fn submit_outlined_box_dyn(__this: &compat::Shared<dyn PrimitiveCollector>, __p0: &BlockPos, __p1: &Vec<f32>, __p2: f32, __p3: bool) {
        Self::submit_outlined_box_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

    /// java: @Override public void submitOutlinedBox(AABB box, float[] colourComponents, float lineWidth, boolean throughWalls)
    pub fn submit_outlined_box_aabb_float_float_boolean_impl(__this: &compat::Shared<Self>, r#box: &AABB, colour_components: &Vec<f32>, mut line_width: f32, mut through_walls: bool) {
        ({ let __f = compat::dispatch(__this, |__t| __t.submit_outlined_box_5()); __f(&compat::view(__this, |a| a as compat::Shared<dyn PrimitiveCollector>), r#box, colour_components, 1.0f32, line_width, through_walls) });
    }

    /// PrimitiveCollectorImpl.submitOutlinedBox as the dispatch of PrimitiveCollector.submitOutlinedBox
    pub fn submit_outlined_box_aabb_float_float_boolean_dyn(__this: &compat::Shared<dyn PrimitiveCollector>, __p0: &AABB, __p1: &Vec<f32>, __p2: f32, __p3: bool) {
        Self::submit_outlined_box_aabb_float_float_boolean_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

    /// java: @Override public void submitOutlinedBox(AABB box, float[] colourComponents, float alpha, float lineWidth, boolean throughWalls)
    pub fn submit_outlined_box_5_impl(__this: &compat::Shared<Self>, r#box: &AABB, colour_components: &Vec<f32>, mut alpha: f32, mut line_width: f32, mut through_walls: bool) {
        PrimitiveCollectorImpl::submit_outlined_box_10(__this, (r#box).min_x, (r#box).min_y, (r#box).min_z, (r#box).max_x, (r#box).max_y, (r#box).max_z, colour_components, alpha, line_width, through_walls);
    }

    /// PrimitiveCollectorImpl.submitOutlinedBox as the dispatch of PrimitiveCollector.submitOutlinedBox
    pub fn submit_outlined_box_5_dyn(__this: &compat::Shared<dyn PrimitiveCollector>, __p0: &AABB, __p1: &Vec<f32>, __p2: f32, __p3: f32, __p4: bool) {
        Self::submit_outlined_box_5_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4)
    }

    /// java: @Override public void submitLinesFromPoints(Vec3[] points, float[] colourComponents, float alpha, float lineWidth, boolean throughWalls)
    pub fn submit_lines_from_points_impl(__this: &compat::Shared<Self>, points: &Vec<Vec3>, colour_components: &Vec<f32>, mut alpha: f32, mut line_width: f32, mut through_walls: bool) {
        PrimitiveCollectorImpl::ensure_not_frozen(__this);
        let mut state: LinesRenderState = LinesRenderState::new(points.clone(), colour_components.clone(), alpha, line_width);
        if through_walls {
            let __h0 = state.clone();
            __this.lock().borrow_mut().lines_through_walls_states.push(__h0);
        } else {
            let __h0 = state.clone();
            __this.lock().borrow_mut().lines_states.push(__h0);
        }
    }

    /// PrimitiveCollectorImpl.submitLinesFromPoints as the dispatch of PrimitiveCollector.submitLinesFromPoints
    pub fn submit_lines_from_points_dyn(__this: &compat::Shared<dyn PrimitiveCollector>, __p0: &Vec<Vec3>, __p1: &Vec<f32>, __p2: f32, __p3: f32, __p4: bool) {
        Self::submit_lines_from_points_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4)
    }

    /// java: @Override public void submitLineFromCursor(Vec3 point, float[] colourComponents, float alpha, float lineWidth)
    pub fn submit_line_from_cursor_impl(__this: &compat::Shared<Self>, point: &Vec3, colour_components: &Vec<f32>, mut alpha: f32, mut line_width: f32) {
        PrimitiveCollectorImpl::ensure_not_frozen(__this);
        let mut state: CursorLineRenderState = CursorLineRenderState::new(point.clone(), colour_components.clone(), alpha, line_width);
        let __h0 = state.clone();
        __this.lock().borrow_mut().cursor_line_states.push(__h0);
    }

    /// PrimitiveCollectorImpl.submitLineFromCursor as the dispatch of PrimitiveCollector.submitLineFromCursor
    pub fn submit_line_from_cursor_dyn(__this: &compat::Shared<dyn PrimitiveCollector>, __p0: &Vec3, __p1: &Vec<f32>, __p2: f32, __p3: f32) {
        Self::submit_line_from_cursor_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

    /// java: @Override public void submitQuad(Vec3[] points, float[] colourComponents, float alpha, boolean throughWalls)
    pub fn submit_quad_impl(__this: &compat::Shared<Self>, points: &Vec<Vec3>, colour_components: &Vec<f32>, mut alpha: f32, mut through_walls: bool) {
        PrimitiveCollectorImpl::ensure_not_frozen(__this);
        let mut state: QuadRenderState = QuadRenderState::new(points.clone(), colour_components.clone(), alpha);
        if through_walls {
            let __h0 = state.clone();
            __this.lock().borrow_mut().quad_through_walls_states.push(__h0);
        } else {
            let __h0 = state.clone();
            __this.lock().borrow_mut().quad_states.push(__h0);
        }
    }

    /// PrimitiveCollectorImpl.submitQuad as the dispatch of PrimitiveCollector.submitQuad
    pub fn submit_quad_dyn(__this: &compat::Shared<dyn PrimitiveCollector>, __p0: &Vec<Vec3>, __p1: &Vec<f32>, __p2: f32, __p3: bool) {
        Self::submit_quad_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

    /// java: @Override public void submitTexturedQuad(Vec3 pos, float width, float height, float textureWidth, float textureHeight, Vec3 renderOffset, Identifier texture, float[] shaderColour, float alpha, boolean
    pub fn submit_textured_quad_impl(__this: &compat::Shared<Self>, pos: &Vec3, mut width: f32, mut height: f32, mut texture_width: f32, mut texture_height: f32, render_offset: &Vec3, texture: &Identifier, shader_colour: &Vec<f32>, mut alpha: f32, mut through_walls: bool) {
        PrimitiveCollectorImpl::ensure_not_frozen(__this);
        let mut state: TexturedQuadRenderState = TexturedQuadRenderState::new(pos.clone(), width, height, texture_width, texture_height, render_offset.clone(), texture.clone(), shader_colour.clone(), alpha);
        if through_walls {
            let __h0 = state.clone();
            __this.lock().borrow_mut().textured_quad_through_walls_states.push(__h0);
        } else {
            let __h0 = state.clone();
            __this.lock().borrow_mut().textured_quad_states.push(__h0);
        }
    }

    /// PrimitiveCollectorImpl.submitTexturedQuad as the dispatch of PrimitiveCollector.submitTexturedQuad
    pub fn submit_textured_quad_dyn(__this: &compat::Shared<dyn PrimitiveCollector>, __p0: &Vec3, __p1: f32, __p2: f32, __p3: f32, __p4: f32, __p5: &Vec3, __p6: &Identifier, __p7: &Vec<f32>, __p8: f32, __p9: bool) {
        Self::submit_textured_quad_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4, __p5, __p6, __p7, __p8, __p9)
    }

    /// java: @Override public void submitBlockHologram(BlockPos pos, BlockState state, float alpha)
    pub fn submit_block_hologram_impl(__this: &compat::Shared<Self>, pos: &BlockPos, state: &Arc<BlockState>, mut alpha: f32) {
        PrimitiveCollectorImpl::ensure_not_frozen(__this);
        // Was: if !(FrustumUtils::is_visible_7(&compat::read(__this, |__t| __t.frustum.clone()), ..)) {
        if !(PrimitiveCollectorImpl::frustum_visible(__this, ((pos.get_x()) as f64) as f64, ((pos.get_y()) as f64) as f64, ((pos.get_z()) as f64) as f64, ((pos.get_x().wrapping_add(1)) as f64) as f64, ((pos.get_y().wrapping_add(1)) as f64) as f64, ((pos.get_z().wrapping_add(1)) as f64) as f64)) {
            return;
        }
        let mut render_state: BlockHologramRenderState = BlockHologramRenderState::new(pos.clone(), state.clone(), alpha);
        let __h0 = render_state.clone();
        __this.lock().borrow_mut().block_hologram_states.push(__h0);
    }

    /// PrimitiveCollectorImpl.submitBlockHologram as the dispatch of PrimitiveCollector.submitBlockHologram
    pub fn submit_block_hologram_dyn(__this: &compat::Shared<dyn PrimitiveCollector>, __p0: &BlockPos, __p1: &Arc<BlockState>, __p2: f32) {
        Self::submit_block_hologram_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

    /// java: @Override public void submitText(Component text, Vec3 pos, boolean throughWalls)
    pub fn submit_text_impl(__this: &compat::Shared<Self>, text: &MutableComponent, pos: &Vec3, mut through_walls: bool) {
        ({ let __f = compat::dispatch(__this, |__t| __t.submit_text_4()); __f(&compat::view(__this, |a| a as compat::Shared<dyn PrimitiveCollector>), text, pos, ((1) as f32) as f32, through_walls) });
    }

    /// PrimitiveCollectorImpl.submitText as the dispatch of PrimitiveCollector.submitText
    pub fn submit_text_dyn(__this: &compat::Shared<dyn PrimitiveCollector>, __p0: &MutableComponent, __p1: &Vec3, __p2: bool) {
        Self::submit_text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

    /// java: @Override public void submitText(Component text, Vec3 pos, float scale, boolean throughWalls)
    pub fn submit_text_4_impl(__this: &compat::Shared<Self>, text: &MutableComponent, pos: &Vec3, mut scale: f32, mut through_walls: bool) {
        ({ let __f = compat::dispatch(__this, |__t| __t.submit_text_5()); __f(&compat::view(__this, |a| a as compat::Shared<dyn PrimitiveCollector>), text, pos, scale, ((0) as f32) as f32, through_walls) });
    }

    /// PrimitiveCollectorImpl.submitText as the dispatch of PrimitiveCollector.submitText
    pub fn submit_text_4_dyn(__this: &compat::Shared<dyn PrimitiveCollector>, __p0: &MutableComponent, __p1: &Vec3, __p2: f32, __p3: bool) {
        Self::submit_text_4_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

    /// java: @Override public void submitText(Component text, Vec3 pos, float scale, float yOffset, boolean throughWalls)
    pub fn submit_text_5_impl(__this: &compat::Shared<Self>, text: &MutableComponent, pos: &Vec3, mut scale: f32, mut y_offset: f32, mut through_walls: bool) {
        PrimitiveCollectorImpl::submit_text_formatted_char_sequence_vec3_float_float_boolean(__this, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone()), pos, scale, y_offset, through_walls);
    }

    /// PrimitiveCollectorImpl.submitText as the dispatch of PrimitiveCollector.submitText
    pub fn submit_text_5_dyn(__this: &compat::Shared<dyn PrimitiveCollector>, __p0: &MutableComponent, __p1: &Vec3, __p2: f32, __p3: f32, __p4: bool) {
        Self::submit_text_5_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4)
    }

    /// java: @Override public void submitCylinder(Vec3 centre, float radius, float height, int segments, int colour)
    pub fn submit_cylinder_impl(__this: &compat::Shared<Self>, centre: &Vec3, mut radius: f32, mut height: f32, mut segments: i32, mut colour: i32) {
        PrimitiveCollectorImpl::ensure_not_frozen(__this);
        let mut state: CylinderRenderState = CylinderRenderState::new(centre.clone(), radius, height, segments, colour);
        let __h0 = state.clone();
        __this.lock().borrow_mut().cylinder_states.push(__h0);
    }

    /// PrimitiveCollectorImpl.submitCylinder as the dispatch of PrimitiveCollector.submitCylinder
    pub fn submit_cylinder_dyn(__this: &compat::Shared<dyn PrimitiveCollector>, __p0: &Vec3, __p1: f32, __p2: f32, __p3: i32, __p4: i32) {
        Self::submit_cylinder_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4)
    }

    /// java: @Override public void submitSphere(Vec3 centre, float radius, int segments, int rings, int colour)
    pub fn submit_sphere_impl(__this: &compat::Shared<Self>, centre: &Vec3, mut radius: f32, mut segments: i32, mut rings: i32, mut colour: i32) {
        PrimitiveCollectorImpl::ensure_not_frozen(__this);
        let mut state: SphereRenderState = SphereRenderState::new(centre.clone(), radius, segments, rings, colour);
        let __h0 = state.clone();
        __this.lock().borrow_mut().sphere_states.push(__h0);
    }

    /// PrimitiveCollectorImpl.submitSphere as the dispatch of PrimitiveCollector.submitSphere
    pub fn submit_sphere_dyn(__this: &compat::Shared<dyn PrimitiveCollector>, __p0: &Vec3, __p1: f32, __p2: i32, __p3: i32, __p4: i32) {
        Self::submit_sphere_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4)
    }

    /// java: @Override public void submitFilledCircle(Vec3 centre, float radius, int segments, int colour)
    pub fn submit_filled_circle_impl(__this: &compat::Shared<Self>, centre: &Vec3, mut radius: f32, mut segments: i32, mut colour: i32) {
        PrimitiveCollectorImpl::ensure_not_frozen(__this);
        let mut state: FilledCircleRenderState = FilledCircleRenderState::new(centre.clone(), radius, segments, colour);
        let __h0 = state.clone();
        __this.lock().borrow_mut().filled_circle_states.push(__h0);
    }

    /// PrimitiveCollectorImpl.submitFilledCircle as the dispatch of PrimitiveCollector.submitFilledCircle
    pub fn submit_filled_circle_dyn(__this: &compat::Shared<dyn PrimitiveCollector>, __p0: &Vec3, __p1: f32, __p2: i32, __p3: i32) {
        Self::submit_filled_circle_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

    /// java: @Override public void submitOutlinedCircle(Vec3 centre, float radius, float thickness, int segments, int colour)
    pub fn submit_outlined_circle_impl(__this: &compat::Shared<Self>, centre: &Vec3, mut radius: f32, mut thickness: f32, mut segments: i32, mut colour: i32) {
        PrimitiveCollectorImpl::ensure_not_frozen(__this);
        let mut state: OutlinedCircleRenderState = OutlinedCircleRenderState::new(centre.clone(), radius, thickness, segments, colour);
        let __h0 = state.clone();
        __this.lock().borrow_mut().outlined_circle_states.push(__h0);
    }

    /// PrimitiveCollectorImpl.submitOutlinedCircle as the dispatch of PrimitiveCollector.submitOutlinedCircle
    pub fn submit_outlined_circle_dyn(__this: &compat::Shared<dyn PrimitiveCollector>, __p0: &Vec3, __p1: f32, __p2: f32, __p3: i32, __p4: i32) {
        Self::submit_outlined_circle_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4)
    }

}

/// java: private record VanillaSubmittable<S>(S state, VanillaRenderer<S> renderer)
#[derive(Clone)]
pub struct VanillaSubmittable<S> {
    /// java: S state
    pub state: S,
    /// java: VanillaRenderer<S> renderer
    pub renderer: compat::Shared<dyn VanillaRenderer<S>>,
}

impl<S: Clone + 'static> VanillaSubmittable<S> {
    /// java: the canonical record constructor
    pub fn new(mut state: S, mut renderer: compat::Shared<dyn VanillaRenderer<S>>) -> Self {
        Self { state, renderer }
    }

    /// java: the record accessor `state()`
    pub fn state(&self) -> S {
        self.state.clone()
    }

    /// java: the record accessor `renderer()`
    pub fn renderer(&self) -> compat::Shared<dyn VanillaRenderer<S>> {
        self.renderer.clone()
    }

}

/// A `VanillaSubmittable<?>` of the collector's list: its renderer handed its own state (the record's type parameter
/// erased, as Java's wildcard list holds it).
pub trait ErasedVanillaSubmittable {
    fn submit(&self, level_state: &LevelRenderState, submit_node_collector: &mut dyn SubmitNodeCollector);
}

impl<S: Clone + 'static> ErasedVanillaSubmittable for VanillaSubmittable<S> {
    fn submit(&self, level_state: &LevelRenderState, submit_node_collector: &mut dyn SubmitNodeCollector) {
        compat::read(&self.renderer, |renderer| renderer.submit_vanilla(&self.state, level_state, submit_node_collector));
    }
}
