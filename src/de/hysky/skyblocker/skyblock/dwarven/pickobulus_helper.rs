//! Ported from de.hysky.skyblocker.skyblock.dwarven.PickobulusHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dwarven::pickobulus_hud_widget::PickobulusHudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager::WidgetManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::PlayerListManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::utils::area::DwarvenMines;
use crate::de::hysky::skyblocker::utils::block_pos_set::BlockPosSet;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::core::block_pos::MutableBlockPos;
use mcrust::net::minecraft::core::direction::Direction;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::world::level::block::block::Block;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use mcrust::net::minecraft::world::level::clip_context::ClipContext;
use mcrust::net::minecraft::world::phys::block_hit_result::BlockHitResult;
use mcrust::net::minecraft::world::phys::hit_result::HitResult;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::net::minecraft::world::entity::item_owner::ItemOwner as _;

/// java: private static final Set<Block> CONVERT_INTO_BEDROCK_BLOCKS = Set.of( Blocks.STONE, Blocks.COBBLESTONE, Blocks.POLISHED_DIORITE, Blocks.PRISMARINE, Blocks.PRISMARINE_BRICKS, Blocks.DARK_PRISMARINE, Bl
pub static CONVERT_INTO_BEDROCK_BLOCKS: compat::MainThread<std::collections::HashSet<Arc<Block>>> = compat::MainThread::new(|| std::collections::HashSet::from([blocks::blocks().get("stone"), blocks::blocks().get("cobblestone"), blocks::blocks().get("polished_diorite"), blocks::blocks().get("prismarine"), blocks::blocks().get("prismarine_bricks"), blocks::blocks().get("dark_prismarine"), mcrust::net::minecraft::world::level::block::blocks::dyed_terracotta().cyan().clone(), mcrust::net::minecraft::world::level::block::blocks::wool().light_blue().clone(), mcrust::net::minecraft::world::level::block::blocks::wool().gray().clone(), blocks::blocks().get("lapis_block"), blocks::blocks().get("gold_block"), blocks::blocks().get("iron_block"), blocks::blocks().get("diamond_block"), blocks::blocks().get("emerald_block"), blocks::blocks().get("redstone_block"), blocks::blocks().get("coal_block"), blocks::blocks().get("quartz_block"), blocks::blocks().get("gold_ore"), blocks::blocks().get("iron_ore"), blocks::blocks().get("coal_ore"), blocks::blocks().get("lapis_ore"), blocks::blocks().get("redstone_ore"), blocks::blocks().get("diamond_ore"), blocks::blocks().get("emerald_ore"), blocks::blocks().get("nether_quartz_ore"), blocks::blocks().get("netherrack"), blocks::blocks().get("glowstone"), blocks::blocks().get("obsidian"), blocks::blocks().get("end_stone")]));
/// java: private static final Set<Block> STAINED_GLASS_BLOCKS = Set.of(Stream.concat(Blocks.STAINED_GLASS.asList().stream(), Blocks.STAINED_GLASS_PANE.asList().stream()) .toArray(Block[]::new));
pub static STAINED_GLASS_BLOCKS: compat::MainThread<std::collections::HashSet<Arc<Block>>> = compat::MainThread::new(|| (mcrust::net::minecraft::world::level::block::blocks::stained_glass().as_list().into_iter().chain(mcrust::net::minecraft::world::level::block::blocks::stained_glass_pane().as_list().into_iter()).collect::<Vec<_>>()).to_vec());
/// java: private static final float[] LIGHT_BLUE = ColorUtils.getFloatComponents(DyeColor.LIGHT_BLUE);
pub static LIGHT_BLUE: LazyLock<Vec<f32>> = LazyLock::new(|| ColorUtils::get_float_components_dye_color(&DyeColor::LightBlue));
/// java: private static boolean shouldRender;
pub static SHOULD_RENDER: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static @Nullable Component errorMessage;
pub static ERROR_MESSAGE: compat::MainThread<Option<MutableComponent>> = compat::MainThread::new(|| None);
/// java: private static final BlockState[][][] blocks = new BlockState[8][8][8];
pub static BLOCKS: compat::MainThread<Vec<Vec<Vec<Arc<BlockState>>>>> = compat::MainThread::new(|| vec![vec![(0..(8) as usize).map(|_| Default::default()).collect::<Vec<_>>(); (8) as usize]; (8) as usize]);
/// java: private static final BlockPosSet breakBlocks = new BlockPosSet();
pub static BREAK_BLOCKS: compat::MainThread<compat::Shared<BlockPosSet>> = compat::MainThread::new(|| BlockPosSet::new());
/// java: private static final int[] drops = new int[MiningDrop.values().length];
pub static DROPS: LazyLock<Vec<i32>> = LazyLock::new(|| vec![0; ((MiningDrop::values().len() as i32)) as usize]);

/// java: public class PickobulusHelper
#[derive(Clone)]
pub struct PickobulusHelper {}

impl PickobulusHelper {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static boolean shouldRender()
    pub fn should_render() -> bool {
        return compat::get_state(&SHOULD_RENDER);
    }

    /// java: public static @Nullable Component getErrorMessage()
    pub fn get_error_message() -> Option<MutableComponent> {
        return ERROR_MESSAGE.get_mut().clone();
    }

    /// java: public static int getTotalBlocks()
    pub fn get_total_blocks() -> i32 {
        return BlockPosSet::size(BREAK_BLOCKS.get_mut());
    }

    /// java: public static int[] getDrops()
    pub fn get_drops() -> Vec<i32> {
        return DROPS.clone();
    }

    /// java: @Init public static void init()
    pub fn init() {
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { PickobulusHelper::update(); }), 1) });
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { PickobulusHelper::extract_rendering(&__a0); }));
    }

    /// java: private static void update()
    pub fn update() {
        if !(SkyblockerConfigManager::get().mining.pickobulus_helper.enable_pickobulus_helper) && !(WidgetManager::is_widget_in_current_screen(&(PickobulusHudWidget::get_instance() as compat::Shared<dyn HudWidget>))) {
            return;
        }
        compat::set_state(&SHOULD_RENDER, true);
        *ERROR_MESSAGE.get_mut() = None;
        BlockPosSet::clear(BREAK_BLOCKS.get_mut());
        DROPS.fill(0);
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::level().is_none() {
            *ERROR_MESSAGE.get_mut() = Some((component::literal("Can't find player or world?")).with_style_format(ChatFormatting::Red));
            return;
        }
        let mut pickobulus_cooldown_hud: Option<String> = PlayerListManager::get_player_string_list().into_iter().map(|__a0| compat::trim(&__a0)).filter(|entry| entry.starts_with("Pickobulus: ")).next();
        if pickobulus_cooldown_hud.is_some() && !(pickobulus_cooldown_hud.clone().unwrap() == "Pickobulus: Available") {
            compat::set_state(&SHOULD_RENDER, !(SkyblockerConfigManager::get().mining.pickobulus_helper.hide_hud_on_cooldown));
            *ERROR_MESSAGE.get_mut() = Some((component::literal(&format!("Pickobulus is on cooldown: {}", compat::substring(&pickobulus_cooldown_hud.clone().unwrap(), 12)))).with_style_format(ChatFormatting::Red));
            return;
        }
        // TODO(fabric2seer): the condition: `ItemAbility.hasAbility` is not a static method of ItemAbility
        //   java: !ItemAbility.hasAbility(CLIENT.player.getMainHandItem(), "Pickobulus")
        if todo!("fabric2seer: the condition (TODO above)") {
            compat::set_state(&SHOULD_RENDER, false);
            *ERROR_MESSAGE.get_mut() = Some(component::literal("Not holding a tool with pickobulus").with_style_format(ChatFormatting::Red));
            return;
        }
        let mut start: Vec3 = mcrust::mcrust::modding::game::with_player(|player| player.position().add((0) as f64, ((Utils::get_eye_height(player) as f64) + 0.53625), (0) as f64)).unwrap();
        // TODO(fabric2seer): field `COLLIDER` of a value of unknown type
        //   java: BlockHitResult blockHitResult = CLIENT.level.clip(new ClipContext(start, start.add(CLIENT.player.getForward().scale(20)), ClipContext.Block.COLLIDER, ClipContext.Fluid.NONE, CLIENT.player));
        let mut block_hit_result: BlockHitResult = todo!("fabric2seer: the initialiser of `blockHitResult` (TODO above)");
        // TODO(fabric2seer): static `HitResult.Type` has no Rust counterpart in the port's HitResult file
        // TODO(fabric2seer): the condition: field `BLOCK` of a value of unknown type
        //   java: blockHitResult.getType() != HitResult.Type.BLOCK
        if todo!("fabric2seer: the condition (TODO above)") {
            *ERROR_MESSAGE.get_mut() = Some(component::literal("Not looking at a block").with_style_format(ChatFormatting::Red));
            return;
        }
        PickobulusHelper::calculate_pickobulus(&block_hit_result.get_block_pos());
    }

    /// java: @SuppressWarnings("incomplete-switch") private static void calculatePickobulus(BlockPos pos)
    pub fn calculate_pickobulus(pos: &BlockPos) {
        debug_assert!(mcrust::mcrust::modding::game::level().is_some());
        let pos_mutable = pos.mutable().move_by(-4, -4, -4);
        for i in 0..8 {
            for j in 0..8 {
                for k in 0..8 {
                    ((BLOCKS.get_mut()[(i) as usize])[(j) as usize])[(k) as usize] = mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&pos_mutable.immutable());
                    pos_mutable.move_direction(Direction::South);
                }
                pos_mutable.move_by(0, 1, -8);
            }
            pos_mutable.move_by(1, -8, 0);
        }
        for i in 1..7 {
            for j in 1..7 {
                for k in 1..7 {
                    let mut state: Arc<BlockState> = ((BLOCKS.get_mut()[(i) as usize])[(j) as usize])[(k) as usize].clone();
                    if state.is_air() || state.is_block(&blocks::blocks().get("bedrock")) {
                        continue;
                    }
                    let mut exposed: bool = ((((BLOCKS.get_mut()[(i.wrapping_sub(1)) as usize])[(j) as usize])[(k) as usize]).is_air() || (((BLOCKS.get_mut()[(i.wrapping_add(1)) as usize])[(j) as usize])[(k) as usize]).is_air() || (((BLOCKS.get_mut()[(i) as usize])[(j.wrapping_sub(1)) as usize])[(k) as usize]).is_air() || (((BLOCKS.get_mut()[(i) as usize])[(j.wrapping_add(1)) as usize])[(k) as usize]).is_air() || (((BLOCKS.get_mut()[(i) as usize])[(j) as usize])[(k.wrapping_sub(1)) as usize]).is_air() || (((BLOCKS.get_mut()[(i) as usize])[(j) as usize])[(k.wrapping_add(1)) as usize]).is_air());
                    if !exposed {
                        continue;
                    }
                    if compat::read(&Utils::get_area(), |__t| compat::DynAny::dyn_any(__t).downcast_ref::<DwarvenMines>().is_some_and(|__e| *__e == DwarvenMines::GlaciteTunnels)) || compat::read(&Utils::get_area(), |__t| compat::DynAny::dyn_any(__t).downcast_ref::<DwarvenMines>().is_some_and(|__e| *__e == DwarvenMines::GreatGlaciteLake)) {
                        PickobulusHelper::handle_glacite_tunnels(pos, &state, i, j, k);
                    } else if compat::read(&Utils::get_area(), |__t| compat::DynAny::dyn_any(__t).downcast_ref::<DwarvenMines>().is_some_and(|__e| *__e == DwarvenMines::GlaciteMineshafts)) {
                        PickobulusHelper::handle_glacite_mineshafts(pos, &state, i, j, k);
                    } else {
                        match Utils::get_location() {
                            Location::GoldMine | Location::DeepCaverns | Location::DwarvenMines => {
                                PickobulusHelper::handle_convert_into_bedrock(pos, &state, i, j, k);
                            }
                            Location::CrystalHollows => {
                                PickobulusHelper::handle_crystal_hollows(pos, &state, i, j, k);
                            }
                            Location::GlaciteMineshafts => {
                                PickobulusHelper::handle_glacite_mineshafts(pos, &state, i, j, k);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    /// java: private static void handleBreakable(BlockPos pos, int i, int j, int k)
    pub fn handle_breakable(pos: &BlockPos, mut i: i32, mut j: i32, mut k: i32) {
        ((BLOCKS.get_mut()[(i) as usize])[(j) as usize])[(k) as usize] = blocks::blocks().get("air").default_block_state();
        BlockPosSet::add(BREAK_BLOCKS.get_mut(), &pos.offset(i.wrapping_sub(4), j.wrapping_sub(4), k.wrapping_sub(4)));
    }

    /// java: private static void handleConvertIntoBedrock(BlockPos pos, BlockState state, int i, int j, int k)
    pub fn handle_convert_into_bedrock(pos: &BlockPos, state: &Arc<BlockState>, mut i: i32, mut j: i32, mut k: i32) {
        if CONVERT_INTO_BEDROCK_BLOCKS.get_mut().contains(&state.get_block()) || STAINED_GLASS_BLOCKS.get_mut().contains(&state.get_block()) {
            BlockPosSet::add(BREAK_BLOCKS.get_mut(), &pos.offset(i.wrapping_sub(4), j.wrapping_sub(4), k.wrapping_sub(4)));
        }
    }

    /// java: private static void handleCrystalHollows(BlockPos pos, BlockState state, int i, int j, int k)
    pub fn handle_crystal_hollows(pos: &BlockPos, state: &Arc<BlockState>, mut i: i32, mut j: i32, mut k: i32) {
        if STAINED_GLASS_BLOCKS.get_mut().contains(&state.get_block()) {
            DROPS[(MiningDrop::Gemstones.ordinal()) as usize] += 1;
        } else if state.is_block(&blocks::blocks().get("prismarine")) {
            DROPS[(MiningDrop::MithrilPowder.ordinal()) as usize] = (DROPS[(MiningDrop::MithrilPowder.ordinal()) as usize]).wrapping_add(3);
        } else if state.is_block(&blocks::blocks().get("prismarine_bricks")) {
            DROPS[(MiningDrop::MithrilPowder.ordinal()) as usize] = (DROPS[(MiningDrop::MithrilPowder.ordinal()) as usize]).wrapping_add(3);
        } else if state.is_block(&blocks::blocks().get("dark_prismarine")) {
            DROPS[(MiningDrop::MithrilPowder.ordinal()) as usize] = (DROPS[(MiningDrop::MithrilPowder.ordinal()) as usize]).wrapping_add(3);
        } else if state.is_block(mcrust::net::minecraft::world::level::block::blocks::wool().light_blue()) {
            DROPS[(MiningDrop::MithrilPowder.ordinal()) as usize] = (DROPS[(MiningDrop::MithrilPowder.ordinal()) as usize]).wrapping_add(5);
        } else if state.is_block(mcrust::net::minecraft::world::level::block::blocks::wool().gray()) {
            DROPS[(MiningDrop::MithrilPowder.ordinal()) as usize] += 1;
        } else if state.is_block(mcrust::net::minecraft::world::level::block::blocks::dyed_terracotta().cyan()) {
            DROPS[(MiningDrop::MithrilPowder.ordinal()) as usize] += 1;
        }
        PickobulusHelper::handle_breakable(pos, i, j, k);
    }

    /// java: private static void handleGlaciteTunnels(BlockPos pos, BlockState state, int i, int j, int k)
    pub fn handle_glacite_tunnels(pos: &BlockPos, state: &Arc<BlockState>, mut i: i32, mut j: i32, mut k: i32) {
        if state.is_block(&blocks::blocks().get("packed_ice")) {
            DROPS[(MiningDrop::MineshaftPity.ordinal()) as usize] = (DROPS[(MiningDrop::MineshaftPity.ordinal()) as usize]).wrapping_add(2);
            DROPS[(MiningDrop::Ice.ordinal()) as usize] += 1;
            ((BLOCKS.get_mut()[(i) as usize])[(j) as usize])[(k) as usize] = blocks::blocks().get("air").default_block_state();
        } else if STAINED_GLASS_BLOCKS.get_mut().contains(&state.get_block()) {
            DROPS[(MiningDrop::MineshaftPity.ordinal()) as usize] = (DROPS[(MiningDrop::MineshaftPity.ordinal()) as usize]).wrapping_add(2);
            DROPS[(MiningDrop::Gemstones.ordinal()) as usize] += 1;
            ((BLOCKS.get_mut()[(i) as usize])[(j) as usize])[(k) as usize] = blocks::blocks().get("air").default_block_state();
        } else if state.is_block(&blocks::blocks().get("polished_diorite")) {
            DROPS[(MiningDrop::MineshaftPity.ordinal()) as usize] = (DROPS[(MiningDrop::MineshaftPity.ordinal()) as usize]).wrapping_add(4);
            DROPS[(MiningDrop::Titanium.ordinal()) as usize] += 1;
        } else if state.is_block(&blocks::blocks().get("infested_stone")) {
            DROPS[(MiningDrop::Hardstone.ordinal()) as usize] += 1;
        } else if state.is_block(mcrust::net::minecraft::world::level::block::blocks::carpet().light_gray()) {
            DROPS[(MiningDrop::Hardstone.ordinal()) as usize] += 1;
        } else if state.is_block(&blocks::blocks().get("prismarine")) {
            DROPS[(MiningDrop::MineshaftPity.ordinal()) as usize] += 1;
            DROPS[(MiningDrop::Mithril.ordinal()) as usize] += 1;
            DROPS[(MiningDrop::MithrilPowder.ordinal()) as usize] = (DROPS[(MiningDrop::MithrilPowder.ordinal()) as usize]).wrapping_add(3);
        } else if state.is_block(&blocks::blocks().get("prismarine_bricks")) {
            DROPS[(MiningDrop::MineshaftPity.ordinal()) as usize] += 1;
            DROPS[(MiningDrop::Mithril.ordinal()) as usize] += 1;
            DROPS[(MiningDrop::MithrilPowder.ordinal()) as usize] = (DROPS[(MiningDrop::MithrilPowder.ordinal()) as usize]).wrapping_add(3);
        } else if state.is_block(&blocks::blocks().get("dark_prismarine")) {
            DROPS[(MiningDrop::MineshaftPity.ordinal()) as usize] += 1;
            DROPS[(MiningDrop::Mithril.ordinal()) as usize] += 1;
            DROPS[(MiningDrop::MithrilPowder.ordinal()) as usize] = (DROPS[(MiningDrop::MithrilPowder.ordinal()) as usize]).wrapping_add(3);
        } else if state.is_block(mcrust::net::minecraft::world::level::block::blocks::wool().light_blue()) {
            DROPS[(MiningDrop::MineshaftPity.ordinal()) as usize] += 1;
            DROPS[(MiningDrop::Mithril.ordinal()) as usize] += 1;
            DROPS[(MiningDrop::MithrilPowder.ordinal()) as usize] = (DROPS[(MiningDrop::MithrilPowder.ordinal()) as usize]).wrapping_add(5);
        } else if state.is_block(mcrust::net::minecraft::world::level::block::blocks::wool().gray()) {
            DROPS[(MiningDrop::MineshaftPity.ordinal()) as usize] += 1;
            DROPS[(MiningDrop::Mithril.ordinal()) as usize] += 1;
            DROPS[(MiningDrop::MithrilPowder.ordinal()) as usize] += 1;
        } else if state.is_block(mcrust::net::minecraft::world::level::block::blocks::dyed_terracotta().cyan()) {
            DROPS[(MiningDrop::MineshaftPity.ordinal()) as usize] += 1;
            DROPS[(MiningDrop::Mithril.ordinal()) as usize] += 1;
            DROPS[(MiningDrop::MithrilPowder.ordinal()) as usize] += 1;
        } else if state.is_block(mcrust::net::minecraft::world::level::block::blocks::dyed_terracotta().brown()) {
            DROPS[(MiningDrop::MineshaftPity.ordinal()) as usize] = (DROPS[(MiningDrop::MineshaftPity.ordinal()) as usize]).wrapping_add(2);
            DROPS[(MiningDrop::Umber.ordinal()) as usize] += 1;
        } else if state.is_block(&blocks::blocks().get("terracotta")) {
            DROPS[(MiningDrop::MineshaftPity.ordinal()) as usize] = (DROPS[(MiningDrop::MineshaftPity.ordinal()) as usize]).wrapping_add(2);
            DROPS[(MiningDrop::Umber.ordinal()) as usize] += 1;
        } else if state.is_block(&blocks::blocks().get("smooth_red_sandstone")) {
            DROPS[(MiningDrop::MineshaftPity.ordinal()) as usize] = (DROPS[(MiningDrop::MineshaftPity.ordinal()) as usize]).wrapping_add(2);
            DROPS[(MiningDrop::Umber.ordinal()) as usize] += 1;
        } else if state.is_block(&blocks::blocks().get("infested_cobblestone")) {
            DROPS[(MiningDrop::MineshaftPity.ordinal()) as usize] = (DROPS[(MiningDrop::MineshaftPity.ordinal()) as usize]).wrapping_add(2);
            DROPS[(MiningDrop::Tungsten.ordinal()) as usize] += 1;
        } else if state.is_block(&blocks::blocks().get("clay")) {
            DROPS[(MiningDrop::MineshaftPity.ordinal()) as usize] = (DROPS[(MiningDrop::MineshaftPity.ordinal()) as usize]).wrapping_add(2);
            DROPS[(MiningDrop::Tungsten.ordinal()) as usize] += 1;
        } else {
            return;
        }
        BlockPosSet::add(BREAK_BLOCKS.get_mut(), &pos.offset(i.wrapping_sub(4), j.wrapping_sub(4), k.wrapping_sub(4)));
    }

    /// java: private static void handleGlaciteMineshafts(BlockPos pos, BlockState state, int i, int j, int k)
    pub fn handle_glacite_mineshafts(pos: &BlockPos, state: &Arc<BlockState>, mut i: i32, mut j: i32, mut k: i32) {
        if state.is_block(&blocks::blocks().get("stone")) {
            DROPS[(MiningDrop::Hardstone.ordinal()) as usize] += 1;
        }
        PickobulusHelper::handle_breakable(pos, i, j, k);
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(SkyblockerConfigManager::get().mining.pickobulus_helper.enable_pickobulus_helper) {
            return;
        }
        for break_pos in BREAK_BLOCKS.get_mut().lock().borrow().iter() {
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box()); __f(collector, break_pos, &LIGHT_BLUE, 2.0f32, false) });
        }
    }

}

/// java: enum MiningDrop
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MiningDrop {
    MineshaftPity,
    Hardstone,
    Ice,
    Tungsten,
    Umber,
    Mithril,
    Titanium,
    Gemstones,
    MithrilPowder,
}

impl MiningDrop {
    pub const VALUES: [MiningDrop; 9] = [MiningDrop::MineshaftPity, MiningDrop::Hardstone, MiningDrop::Ice, MiningDrop::Tungsten, MiningDrop::Umber, MiningDrop::Mithril, MiningDrop::Titanium, MiningDrop::Gemstones, MiningDrop::MithrilPowder];

    /// java: values()
    pub fn values() -> Vec<MiningDrop> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            MiningDrop::MineshaftPity => "MINESHAFT_PITY",
            MiningDrop::Hardstone => "HARDSTONE",
            MiningDrop::Ice => "ICE",
            MiningDrop::Tungsten => "TUNGSTEN",
            MiningDrop::Umber => "UMBER",
            MiningDrop::Mithril => "MITHRIL",
            MiningDrop::Titanium => "TITANIUM",
            MiningDrop::Gemstones => "GEMSTONES",
            MiningDrop::MithrilPowder => "MITHRIL_POWDER",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> MiningDrop {
        match name {
            "MINESHAFT_PITY" => MiningDrop::MineshaftPity,
            "HARDSTONE" => MiningDrop::Hardstone,
            "ICE" => MiningDrop::Ice,
            "TUNGSTEN" => MiningDrop::Tungsten,
            "UMBER" => MiningDrop::Umber,
            "MITHRIL" => MiningDrop::Mithril,
            "TITANIUM" => MiningDrop::Titanium,
            "GEMSTONES" => MiningDrop::Gemstones,
            "MITHRIL_POWDER" => MiningDrop::MithrilPowder,
            _ => panic!("IllegalArgumentException: No enum constant MiningDrop.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static MiningDropFields {
        static FIELDS: LazyLock<Vec<MiningDropFields>> = LazyLock::new(|| vec![
            MiningDropFields::new("Mineshaft Pity".to_string()),
            MiningDropFields::new("Hardstone".to_string()),
            MiningDropFields::new("Ice".to_string()),
            MiningDropFields::new("Tungsten".to_string()),
            MiningDropFields::new("Umber".to_string()),
            MiningDropFields::new("Mithril".to_string()),
            MiningDropFields::new("Titanium".to_string()),
            MiningDropFields::new("Gemstones".to_string()),
            MiningDropFields::new("Mithril Powder".to_string()),
        ]);
        &FIELDS[self as usize]
    }

    /// java: public String friendlyName()
    pub fn friendly_name(&self) -> String {
        return self.fields().friendly_name.clone();
    }

}

impl std::fmt::Display for MiningDrop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each MiningDrop constant
pub struct MiningDropFields {
    pub friendly_name: String,
}

impl MiningDropFields {
    fn new(mut friendly_name: String) -> Self {
        let mut this = Self {
            friendly_name: Default::default(),
        };
        this.friendly_name = friendly_name.clone();
        this
    }

}
