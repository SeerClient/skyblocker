//! Ported from de.hysky.skyblocker.skyblock.crimson.dojo.MasteryTestHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::crimson::dojo::dojo_manager;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use mcrust::java::text::decimal_format::DecimalFormat;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use mcrust::net::minecraft::world::phys::vec3;
use mcrust::net::minecraft::world::entity::item_owner::ItemOwner as _;

// TODO(fabric2seer): `new` takes 2 argument(s) in the port (`pattern: &str, symbols: DecimalFormatSymbols`), Java passes 1
//   java: private static final DecimalFormat FORMATTER = new DecimalFormat("0.00");
/// java: private static final DecimalFormat FORMATTER = new DecimalFormat("0.00");
pub static FORMATTER: compat::MainThread<DecimalFormat> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `FORMATTER` - see the TODO above"));
/// java: private static final int BLOCK_LIFE_TIME = 6550;
pub const BLOCK_LIFE_TIME: i32 = 6550;
/// java: private static final List<BlockPos> blockOrder = new ArrayList<>();
pub static BLOCK_ORDER: compat::MainThread<Vec<BlockPos>> = compat::MainThread::new(|| Vec::new());
/// java: private static final Object2LongOpenHashMap<BlockPos> endTimes = new Object2LongOpenHashMap<>();
pub static END_TIMES: compat::MainThread<std::collections::HashMap<BlockPos, i64>> = compat::MainThread::new(|| std::collections::HashMap::new());

/// java: public class MasteryTestHelper
#[derive(Clone)]
pub struct MasteryTestHelper {}

impl MasteryTestHelper {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: protected static void reset()
    pub fn reset() {
        BLOCK_ORDER.get_mut().clear();
        END_TIMES.get_mut().clear();
    }

    /// java: protected static void onBlockUpdate(BlockPos pos, BlockState state)
    pub fn on_block_update(mut pos: BlockPos, state: &Arc<BlockState>) {
        if false /* java: compat::minecraft() == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        if state.is_block(mcrust::net::minecraft::world::level::block::blocks::wool().lime()) {
            BLOCK_ORDER.get_mut().push(pos.clone());
            let mut travel_time: i64 = ((((mcrust::mcrust::modding::game::with_player(|player| player.position().distance_to(&vec3::at_center_of(&pos))).unwrap() * (1000 as f64)) / (60 as f64))) as i64);
            END_TIMES.get_mut().insert(pos.clone(), (((compat::millis()).wrapping_add(BLOCK_LIFE_TIME as i64)).wrapping_sub(compat::get_state(&dojo_manager::PING))).wrapping_sub(travel_time));
        }
        if state.is_air() {
            compat::remove_item(&mut *BLOCK_ORDER.get_mut(), &pos.clone());
            // TODO(fabric2seer): Map.removeLong(..)
            //   java: endTimes.removeLong(pos);
        }
    }

    /// java: protected static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(BLOCK_ORDER.get_mut().is_empty()) {
            // TODO(fabric2seer): `.getColorComponents(..)` on a value of unknown type
            //   java: collector.submitLineFromCursor(Vec3.atCenterOf(blockOrder.getFirst()), Color.LIGHT_GRAY.getColorComponents(new float[]{0, 0, 0}), 1f, 2);
        }
        if (BLOCK_ORDER.get_mut().len() as i32) >= 2 {
            // TODO(fabric2seer): `.getColorComponents(..)` on a value of unknown type
            //   java: collector.submitLinesFromPoints(new Vec3[]{Vec3.atCenterOf(blockOrder.get(0)), Vec3.atCenterOf(blockOrder.get(1))}, Color.LIGHT_GRAY.getColorComponents(new float[]{0, 0, 0}), 1, 2, false);
        }
        let mut current_time: i64 = compat::millis();
        for pos in BLOCK_ORDER.get_mut().iter() {
            let mut block_end_time: i64 = END_TIMES.get_mut().get(pos).copied().unwrap_or_default();
            let mut seconds_time: f32 = ((block_end_time.wrapping_sub(current_time) as f32) / 1000.0f32).max((0 as f32));
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_text_4()); __f(collector, &component::literal(&FORMATTER.get_mut().format(seconds_time as f64)), &vec3::at_center_of(&pos.offset(0, 1, 0)), ((3) as f32) as f32, true) });
        }
    }

}
