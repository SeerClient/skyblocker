//! Ported from de.hysky.skyblocker.skyblock.dungeon.puzzle.TeleportMaze by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::dungeon_puzzle::DungeonPuzzle;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::dungeon_puzzle::DungeonPuzzleBase;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Room;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use crate::de::hysky::skyblocker::utils::tickable::Tickable;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::world::level::block::block::Block;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::level::level::Level;
use mcrust::net::minecraft::world::phys::vec3;
use mcrust::net::minecraft::world::level::block_getter::BlockGetter as _;

/// java: private static final float[] LIME = ColorUtils.getFloatComponents(DyeColor.LIME);
pub static LIME: LazyLock<Vec<f32>> = LazyLock::new(|| ColorUtils::get_float_components_dye_color(&DyeColor::Lime));
/// java: private static final float[] RED = ColorUtils.getFloatComponents(DyeColor.RED);
pub static RED: LazyLock<Vec<f32>> = LazyLock::new(|| ColorUtils::get_float_components_dye_color(&DyeColor::Red));
/// java: private static final Set<BlockPos> ROOM_CENTERS = Set.of( new BlockPos(7, 68, 9), new BlockPos(23, 68, 9), new BlockPos(7, 68, 17), new BlockPos(23, 68, 17), new BlockPos(7, 68, 25), new BlockPos(15, 
pub static ROOM_CENTERS: compat::MainThread<std::collections::HashSet<BlockPos>> = compat::MainThread::new(|| std::collections::HashSet::from([BlockPos::new(7, 68, 9), BlockPos::new(23, 68, 9), BlockPos::new(7, 68, 17), BlockPos::new(23, 68, 17), BlockPos::new(7, 68, 25), BlockPos::new(15, 68, 25), BlockPos::new(23, 68, 25)]));
/// java: private static final Set<BlockPos> TELEPORT_PADS = Set.of( new BlockPos(15, 69, 12), // Start new BlockPos(4, 69, 6), new BlockPos(10, 69, 6), new BlockPos(20, 69, 6), new BlockPos(26, 69, 6), new Blo
pub static TELEPORT_PADS: compat::MainThread<std::collections::HashSet<BlockPos>> = compat::MainThread::new(|| std::collections::HashSet::from([BlockPos::new(15, 69, 12), BlockPos::new(4, 69, 6), BlockPos::new(10, 69, 6), BlockPos::new(20, 69, 6), BlockPos::new(26, 69, 6), BlockPos::new(4, 69, 12), BlockPos::new(10, 69, 12), BlockPos::new(20, 69, 12), BlockPos::new(26, 69, 12), BlockPos::new(4, 69, 14), BlockPos::new(10, 69, 14), BlockPos::new(20, 69, 14), BlockPos::new(26, 69, 14), BlockPos::new(4, 69, 20), BlockPos::new(10, 69, 20), BlockPos::new(20, 69, 20), BlockPos::new(26, 69, 20), BlockPos::new(4, 69, 22), BlockPos::new(10, 69, 22), BlockPos::new(12, 69, 22), BlockPos::new(18, 69, 22), BlockPos::new(20, 69, 22), BlockPos::new(26, 69, 22), BlockPos::new(4, 69, 28), BlockPos::new(10, 69, 28), BlockPos::new(12, 69, 28), BlockPos::new(18, 69, 28), BlockPos::new(20, 69, 28), BlockPos::new(26, 69, 28), BlockPos::new(15, 69, 14)]));
/// java: public static final TeleportMaze INSTANCE = new TeleportMaze();
pub static INSTANCE: compat::MainThread<compat::Shared<TeleportMaze>> = compat::MainThread::new(|| TeleportMaze::new());

/// java: public class TeleportMaze extends DungeonPuzzle
#[derive(Clone)]
pub struct TeleportMaze {
    /// java: extends DungeonPuzzle
    pub base: DungeonPuzzleBase,
    /// java: private final Map<BlockPos, RoomType> pads = new HashMap<>();
    pub pads: std::collections::HashMap<BlockPos, RoomType>,
    /// java: private @Nullable BlockPos finalPad;
    pub final_pad: Option<BlockPos>,
}

impl std::ops::Deref for TeleportMaze {
    type Target = DungeonPuzzleBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TeleportMaze {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TeleportMaze {
    /// java: private TeleportMaze()
    pub fn new() -> compat::Shared<Self> {
        // NOTE(fabric2seer): `finalPad` is read by `reset`, which DungeonPuzzle's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `pads` is read by `reset`, which DungeonPuzzle's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        let (_, __this) = DungeonPuzzleBase::new_in("teleport-maze".to_string(), vec!["teleport-pad-room".to_string()], move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                pads: std::collections::HashMap::new(),
                final_pad: None,
            });
            ((__h.clone() as compat::Shared<dyn DungeonPuzzle>), __h)
        });
        __this
    }

    /// java: public void onTeleport(Minecraft client, BlockPos from, BlockPos to)
    pub fn on_teleport(__this: &compat::Shared<Self>, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, from: &BlockPos, to: &BlockPos) {
        if !({ let __f = compat::dispatch(__this, |__t| __t.should_solve()); __f(&compat::view(__this, |a| a as compat::Shared<dyn DungeonPuzzle>)) }) || !(DungeonManager::is_current_room_matched()) || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::level().is_none() {
            return;
        }
        let mut prev_player: BlockPos = ({ let __r = &DungeonManager::get_current_room().unwrap(); let __f = compat::dispatch(__r, |__t| __t.actual_to_relative()); __f(__r, &from.at_y(69)) });
        let mut player: BlockPos = ({ let __r = &DungeonManager::get_current_room().unwrap(); let __f = compat::dispatch(__r, |__t| __t.actual_to_relative()); __f(__r, &to.at_y(69)) });
        if prev_player == player || !(TELEPORT_PADS.get_mut().contains(&prev_player)) {
            return;
        }
        TeleportMaze::process_teleport(__this, &mcrust::mcrust::modding::game::level().unwrap(), &prev_player, &player);
        let mut nearest_pad: BlockPos = TELEPORT_PADS.get_mut().iter().cloned().min_by(|a, b| { let key = |pad: &BlockPos| pad.dist_sqr(&player); key(a).total_cmp(&key(b)) }).unwrap();
        TeleportMaze::process_teleport(__this, &mcrust::mcrust::modding::game::level().unwrap(), &nearest_pad, &prev_player);
    }

    /// java: private void processTeleport(Level world, BlockPos from, BlockPos to)
    pub fn process_teleport(__this: &compat::Shared<Self>, world: &Level, from: &BlockPos, to: &BlockPos) {
        if let Some(r#type) = TeleportMaze::get_room_type(__this, world, to) {
            let __h0 = ({ let __r = &DungeonManager::get_current_room().unwrap(); let __f = compat::dispatch(__r, |__t| __t.relative_to_actual()); __f(__r, from) });
            __this.lock().borrow_mut().pads.insert(__h0, r#type);
        }
    }

    /// java: private Optional<RoomType> getRoomType(Level world, BlockPos pos)
    pub fn get_room_type(__this: &compat::Shared<Self>, world: &Level, pos: &BlockPos) -> Option<RoomType> {
        if pos.get_x() == 15 && pos.get_z() == 12 {
            return Some(RoomType::Entrance);
        }
        return ROOM_CENTERS.get_mut().iter().cloned().filter(|center| (center.get_x().wrapping_sub(3) <= pos.get_x() && pos.get_x() <= center.get_x().wrapping_add(3) && center.get_z().wrapping_sub(3) <= pos.get_z() && pos.get_z() <= center.get_z().wrapping_add(3))).next().map(|__a0| ({ let __r = &DungeonManager::get_current_room().unwrap(); let __f = compat::dispatch(__r, |__t| __t.relative_to_actual()); __f(__r, &__a0) })).map(|__a0| world.get_block_state_typed(&__a0)).map(|__a0| __a0.get_block()).and_then(|__a0| RoomType::from_block(&__a0));
    }

}

impl DungeonPuzzle for TeleportMaze {
    fn dungeon_puzzle_base(&self) -> &DungeonPuzzleBase {
        &self.base
    }
    fn dungeon_puzzle_base_mut(&mut self) -> &mut DungeonPuzzleBase {
        &mut self.base
    }

}

impl Tickable for TeleportMaze {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void tick(Minecraft client) (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn Tickable>, &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        TeleportMaze::tick_dyn
    }

}

impl Renderable for TeleportMaze {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        TeleportMaze::extract_rendering_dyn
    }

}

impl Resettable for TeleportMaze {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        TeleportMaze::reset_dyn
    }

}

impl TeleportMaze {
    /// java: @Override public void tick(Minecraft client)
    pub fn tick_impl(__this: &compat::Shared<Self>, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        if !(SkyblockerConfigManager::get().dungeons.puzzle_solvers.solve_teleport_maze) || !({ let __f = compat::dispatch(__this, |__t| __t.should_solve()); __f(&compat::view(__this, |a| a as compat::Shared<dyn DungeonPuzzle>)) }) || compat::read(__this, |__t| __t.final_pad.clone()).is_some() {
            return;
        }
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let mut final_pads: Vec<BlockPos> = TELEPORT_PADS.get_mut().iter().cloned().filter(|pad| (pad.get_x() != 15)).map(|__a0| ({ let __r = &DungeonManager::get_current_room().unwrap(); let __f = compat::dispatch(__r, |__t| __t.relative_to_actual()); __f(__r, &__a0) })).filter(|pad| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; !((&__this_1).lock().borrow().pads.contains_key(pad)) }).collect::<Vec<_>>();
        if (final_pads.len() as i32) == 1 {
            { let __v = Some(final_pads[0].clone()); compat::write(__this, |__t| __t.final_pad = __v); }
        }
    }

    /// TeleportMaze.tick as the dispatch of Tickable.tick
    pub fn tick_dyn(__this: &compat::Shared<dyn Tickable>, __p0: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        Self::tick_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(SkyblockerConfigManager::get().dungeons.puzzle_solvers.solve_teleport_maze) || !({ let __f = compat::dispatch(__this, |__t| __t.should_solve()); __f(&compat::view(__this, |a| a as compat::Shared<dyn DungeonPuzzle>)) }) {
            return;
        }
        let mut debug: bool = crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled();
        for entry in { __this.lock().borrow().pads.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>() } {
            let mut color: Vec<f32> = (if debug { entry.1.fields().color_components.clone() } else { RED.clone() });
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box()); __f(collector, &entry.0, &color, 0.5f32, debug) });
        }
        if compat::read(__this, |__t| __t.final_pad.clone()).is_some() {
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box()); __f(collector, &compat::read(__this, |__t| __t.final_pad.clone()).unwrap(), &LIME, 1.0f32, true) });
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_line_from_cursor()); __f(collector, &vec3::at_center_of(&compat::read(__this, |__t| __t.final_pad.clone()).unwrap()), &LIME, 1.0f32, 2.0f32) });
        }
    }

    /// TeleportMaze.extractRendering as the dispatch of Renderable.extractRendering
    pub fn extract_rendering_dyn(__this: &compat::Shared<dyn Renderable>, __p0: &compat::Shared<dyn PrimitiveCollector>) {
        Self::extract_rendering_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void reset()
    pub fn reset_impl(__this: &compat::Shared<Self>) {
        <dyn DungeonPuzzle>::reset_body(&compat::view(__this, |a| a as compat::Shared<dyn DungeonPuzzle>));
        __this.lock().borrow_mut().pads.clear();
        { let __v = None; compat::write(__this, |__t| __t.final_pad = __v); }
    }

    /// TeleportMaze.reset as the dispatch of Resettable.reset
    pub fn reset_dyn(__this: &compat::Shared<dyn Resettable>) {
        Self::reset_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: private enum RoomType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RoomType {
    Entrance,
    Coal,
    Iron,
    Redstone,
    Lapis,
    Gold,
    Diamond,
    Emerald,
}

impl RoomType {
    pub const VALUES: [RoomType; 8] = [RoomType::Entrance, RoomType::Coal, RoomType::Iron, RoomType::Redstone, RoomType::Lapis, RoomType::Gold, RoomType::Diamond, RoomType::Emerald];

    /// java: values()
    pub fn values() -> Vec<RoomType> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            RoomType::Entrance => "ENTRANCE",
            RoomType::Coal => "COAL",
            RoomType::Iron => "IRON",
            RoomType::Redstone => "REDSTONE",
            RoomType::Lapis => "LAPIS",
            RoomType::Gold => "GOLD",
            RoomType::Diamond => "DIAMOND",
            RoomType::Emerald => "EMERALD",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> RoomType {
        match name {
            "ENTRANCE" => RoomType::Entrance,
            "COAL" => RoomType::Coal,
            "IRON" => RoomType::Iron,
            "REDSTONE" => RoomType::Redstone,
            "LAPIS" => RoomType::Lapis,
            "GOLD" => RoomType::Gold,
            "DIAMOND" => RoomType::Diamond,
            "EMERALD" => RoomType::Emerald,
            _ => panic!("IllegalArgumentException: No enum constant RoomType.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static RoomTypeFields {
        static FIELDS: LazyLock<Vec<RoomTypeFields>> = LazyLock::new(|| vec![
            RoomTypeFields::new(blocks::blocks().get("BARRIER"), DyeColor::Gray),
            RoomTypeFields::new(blocks::blocks().get("COAL_ORE"), DyeColor::Black),
            RoomTypeFields::new(blocks::blocks().get("IRON_ORE"), DyeColor::LightGray),
            RoomTypeFields::new(blocks::blocks().get("REDSTONE_ORE"), DyeColor::Red),
            RoomTypeFields::new(blocks::blocks().get("LAPIS_ORE"), DyeColor::Blue),
            RoomTypeFields::new(blocks::blocks().get("GOLD_ORE"), DyeColor::Yellow),
            RoomTypeFields::new(blocks::blocks().get("DIAMOND_ORE"), DyeColor::Cyan),
            RoomTypeFields::new(blocks::blocks().get("EMERALD_ORE"), DyeColor::Lime),
        ]);
        &FIELDS[self as usize]
    }

    /// java: private static Optional<RoomType> fromBlock(Block block)
    pub fn from_block(block: &Arc<Block>) -> Option<RoomType> {
        RoomType::values().into_iter().find(|r#type| Arc::ptr_eq(&r#type.fields().block, block))
    }

}

impl std::fmt::Display for RoomType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each RoomType constant
pub struct RoomTypeFields {
    pub block: Arc<Block>,
    pub color_components: Vec<f32>,
}

impl RoomTypeFields {
    fn new(mut block: Arc<Block>, mut dye_color: DyeColor) -> Self {
        let mut this = Self {
            block: block.clone(),
            color_components: ColorUtils::get_float_components_dye_color(&dye_color),
        };
        this
    }

}
