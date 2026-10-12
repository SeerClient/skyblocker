//! Ported from de.hysky.skyblocker.debug.SnapshotDebug by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::shared_constants;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::phys::vec3;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::net::minecraft::world::level::world_gen_level::WorldGenLevel as _;

/// java: private static final float[] RED = { 1.0f, 0.0f, 0.0f };
pub static RED: LazyLock<Vec<f32>> = LazyLock::new(|| vec![1.0f32, 0.0f32, 0.0f32]);
/// java: private static final float ALPHA = 0.5f;
pub const ALPHA: f32 = 0.5f32;
/// java: private static final int CYAN = ARGB.color(ARGB.as8BitChannel(ALPHA), CommonColors.HIGH_CONTRAST_DIAMOND);
pub static CYAN: LazyLock<i32> = LazyLock::new(|| argb::color_alpha_rgb(argb::as_8bit_channel(ALPHA), mcrust::net::minecraft::util::common_colors::HIGH_CONTRAST_DIAMOND));
/// java: private static final int BLUE = ARGB.color(ARGB.as8BitChannel(ALPHA), CommonColors.BLUE);
pub static BLUE: LazyLock<i32> = LazyLock::new(|| argb::color_alpha_rgb(argb::as_8bit_channel(ALPHA), mcrust::net::minecraft::util::common_colors::BLUE));
/// java: private static final float LINE_WIDTH = 8f;
pub const LINE_WIDTH: f32 = 8.0f32;
/// java: public static final long AARON_WORLD_SEED = 5629719634239627355L;
pub const AARON_WORLD_SEED: i64 = 5629719634239627355;

/// java: public class SnapshotDebug
#[derive(Clone)]
pub struct SnapshotDebug {}

impl SnapshotDebug {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static boolean isInSnapshot()
    pub fn is_in_snapshot() -> bool {
        return !(shared_constants::get_current_version().stable());
    }

    /// java: static void init()
    pub fn init() {
        if crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled() {
            level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { SnapshotDebug::extract_rendering(&__a0); }));
        }
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if SnapshotDebug::get_seed() == AARON_WORLD_SEED {
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box_with_beacon_beam()); __f(collector, &BlockPos::new(175, 63, -14), &RED, ALPHA, true) });
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_lines_from_points()); __f(collector, &vec![Vec3::new((173) as f64, (66) as f64, -7.5), Vec3::new((178) as f64, (66) as f64, -7.5)], &RED, ALPHA, LINE_WIDTH, false) });
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_quad()); __f(collector, &vec![Vec3::new((183) as f64, (66) as f64, (-16) as f64), Vec3::new((183) as f64, (63) as f64, (-16) as f64), Vec3::new((183) as f64, (63) as f64, (-14) as f64), Vec3::new((183) as f64, (66) as f64, (-14) as f64)], &RED, ALPHA, false) });
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_text()); __f(collector, &component::null_to_empty(Some(&format!("Skyblocker on {}!", shared_constants::get_current_version().name()))), &Vec3::new(175.5, 67.5, -7.5), false) });
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_cylinder()); __f(collector, &vec3::at_center_of(&BlockPos::new(172, 78, 44)), ((12) as f32) as f32, ((12) as f32) as f32, 32, *CYAN) });
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_cylinder()); __f(collector, &vec3::at_center_of(&BlockPos::new(144, 78, 44)), ((12) as f32) as f32, ((12) as f32) as f32, 32, *BLUE) });
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_block_hologram()); __f(collector, &BlockPos::new(183, 65, 9), &blocks::blocks().get("DIAMOND_BLOCK").default_block_state(), 0.5f32) });
        } else if SnapshotDebug::is_in_snapshot() {
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box_with_beacon_beam()); __f(collector, &BlockPos::new(-3, 63, 5), &RED, ALPHA, true) });
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box()); __f(collector, &BlockPos::new(-3, 63, 5), &RED, ((5) as f32) as f32, true) });
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_lines_from_points()); __f(collector, &vec![Vec3::new((-2) as f64, (65) as f64, 6.5), Vec3::new((3) as f64, (65) as f64, 6.5)], &RED, ALPHA, LINE_WIDTH, false) });
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_line_from_cursor()); __f(collector, &Vec3::new(-2.5, 63.5, 5.5), &RED, ALPHA, LINE_WIDTH) });
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_quad()); __f(collector, &vec![Vec3::new((3) as f64, (66) as f64, (3) as f64), Vec3::new((3) as f64, (63) as f64, (3) as f64), Vec3::new((3) as f64, (63) as f64, (5) as f64), Vec3::new((3) as f64, (66) as f64, (5) as f64)], &RED, ALPHA, false) });
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_text()); __f(collector, &component::null_to_empty(Some(&format!("Skyblocker on {}!", shared_constants::get_current_version().name()))), &Vec3::new(0.5, 66.5, 6.5), false) });
        }
    }

    /// java: private static long getSeed()
    pub fn get_seed() -> i64 {
        // TODO(fabric2seer): `Minecraft.getSingleplayerServer` has no Rust method in the port (looked for `get_singleplayer_server` in Minecraft and its supertypes)
        // TODO(fabric2seer): `IntegratedServer.overworld` has no Rust method in the port (looked for `overworld` in IntegratedServer and its supertypes)
        // rust: return (if compat::minecraft().has_singleplayer_server() { compat::minecraft().get_singleplayer_server().overworld().get_seed() } else { 0 });
        return todo!("fabric2seer: see the TODO above");
    }

}
