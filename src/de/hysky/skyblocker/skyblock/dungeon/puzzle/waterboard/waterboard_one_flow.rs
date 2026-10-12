//! Ported from de.hysky.skyblocker.skyblock.dungeon.puzzle.waterboard.WaterboardOneFlow by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::world::entity::entity::Entity as _;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::server_tick_callback;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::dungeon_puzzle::DungeonPuzzle;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::dungeon_puzzle::DungeonPuzzleBase;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::waterboard::waterboard;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::waterboard::waterboard::LeverType;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Room;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use crate::de::hysky::skyblocker::utils::tickable::Tickable;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::client::player::local_player::LocalPlayer;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::core::block_pos::MutableBlockPos;
use mcrust::net::minecraft::core::direction::Direction;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::interaction_hand::InteractionHand;
use mcrust::net::minecraft::world::interaction_result::InteractionResult;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use mcrust::net::minecraft::world::level::level::Level;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::world::phys::block_hit_result::BlockHitResult;
use mcrust::net::minecraft::world::phys::hit_result::HitResult;
use mcrust::net::minecraft::world::phys::vec3;
use mcrust::net::minecraft::server::packs::resources::resource_provider::ResourceProvider as _;

/// java: public static final WaterboardOneFlow INSTANCE = new WaterboardOneFlow();
pub static INSTANCE: compat::MainThread<compat::Shared<WaterboardOneFlow>> = compat::MainThread::new(|| WaterboardOneFlow::new());
/// java: private static final Identifier WATER_TIMES = SkyblockerMod.id("dungeons/watertimes.json");
pub static WATER_TIMES: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("dungeons/watertimes.json"));
/// java: private static final Component WAIT_TEXT = Component.literal("WAIT").withStyle(ChatFormatting.RED, ChatFormatting.BOLD);
pub static WAIT_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("WAIT").with_style_formats(&[ChatFormatting::Red, ChatFormatting::Bold]));
/// java: private static final Component CLICK_TEXT = Component.literal("CLICK").withStyle(ChatFormatting.GREEN, ChatFormatting.BOLD);
pub static CLICK_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("CLICK").with_style_formats(&[ChatFormatting::Green, ChatFormatting::Bold]));
/// java: private static JsonObject SOLUTIONS;
pub static SOLUTIONS: LazyLock<Mutex<Option<serde_json::Map<String, serde_json::Value>>>> = LazyLock::new(|| Mutex::new(None));

/// java: public class WaterboardOneFlow extends DungeonPuzzle
pub struct WaterboardOneFlow {
    /// java: extends DungeonPuzzle
    pub base: DungeonPuzzleBase,
    /// java: private boolean timerEnabled = false;
    pub timer_enabled: bool,
    /// java: private final List<Mark> marks = new ArrayList<>();
    pub marks: Vec<Mark>,
    /// java: private @Nullable ClientLevel world;
    pub world: Option<Arc<ClientLevel>>,
    /// java: private @Nullable Room room;
    pub room: Option<compat::Shared<dyn Room>>,
    /// java: private @Nullable LocalPlayer player;
    /// java: private @Nullable LocalPlayer player - the local player's entity id (Java holds the object; read
    /// through `game::with_entity`, the same `Mutex` as `Minecraft.player`)
    pub player: Option<i32>,
    /// java: private int variant;
    pub variant: i32,
    /// java: private @Nullable String doors;
    pub doors: Option<String>,
    /// java: private @Nullable String initialDoors;
    pub initial_doors: Option<String>,
    /// java: private @Nullable EnumMap<LeverType, DoubleList> solution;
    pub solution: Option<std::collections::HashMap<LeverType, Vec<f64>>>,
    /// java: private boolean finished;
    pub finished: bool,
    /// java: private long waterStartMillis;
    pub water_start_millis: i64,
    /// java: private long currentTimeMillis;
    pub current_time_millis: i64,
    /// java: private @Nullable CompletableFuture<Void> solve;
    pub solve: Option<mcrust::java::util::concurrent::completable_future::CompletableFuture<()>>,
}

impl std::ops::Deref for WaterboardOneFlow {
    type Target = DungeonPuzzleBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for WaterboardOneFlow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl WaterboardOneFlow {
    /// java: private WaterboardOneFlow()
    pub fn new() -> compat::Shared<Self> {
        // NOTE(fabric2seer): `currentTimeMillis` is read by `reset`, which DungeonPuzzle's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `doors` is read by `reset`, which DungeonPuzzle's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `finished` is read by `reset`, which DungeonPuzzle's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `initialDoors` is read by `reset`, which DungeonPuzzle's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `marks` is read by `reset`, which DungeonPuzzle's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `solution` is read by `reset`, which DungeonPuzzle's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `solve` is read by `reset`, which DungeonPuzzle's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `variant` is read by `reset`, which DungeonPuzzle's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `waterStartMillis` is read by `reset`, which DungeonPuzzle's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        let (_, __this) = DungeonPuzzleBase::new_in("waterboard".to_string(), vec!["water-puzzle".to_string()], move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                timer_enabled: false,
                marks: Vec::new(),
                world: None,
                room: None,
                player: None,
                variant: 0,
                doors: None,
                initial_doors: None,
                solution: None,
                finished: false,
                water_start_millis: 0,
                current_time_millis: 0,
                solve: None,
            });
            ((__h.clone() as compat::Shared<dyn DungeonPuzzle>), __h)
        });
        __this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // TODO(fabric2seer): no SeerClient event yet: hook RETURN of Minecraft's constructor (`net.minecraft.client.Minecraft#<init>`: exists)
        //   java: ClientLifecycleEvents.CLIENT_STARTED.register(WaterboardOneFlow::loadSolutions);
        // TODO(fabric2seer): UseBlockCallback.EVENT.register: no SeerClient equivalent yet
        //   java: UseBlockCallback.EVENT.register(INSTANCE::onUseBlock);
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(..);
            },
        ));
        server_tick_callback::EVENT.register(Arc::new(move || { WaterboardOneFlow::on_server_tick(INSTANCE.get_mut()); }));
        if crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled() {
            client_command_registration_callback::EVENT.register(Arc::new(
                |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                    // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                    //   java: dispatcher.register(..);
                },
            ));
        }
    }

    /// java: private static void loadSolutions(Minecraft client)
    pub fn load_solutions(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        {
            // java: try (BufferedReader reader = client.getResourceManager().openAsReader(WATER_TIMES)) {
            //   SOLUTIONS = JsonParser.parseReader(reader).getAsJsonObject(); } catch (Exception e) {..}
            // - an exception here (the resource's IOException too) is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut reader: String = client.get_resource_manager().open_as_reader(WATER_TIMES.get()).expect("IOException");
                let parsed: serde_json::Value = serde_json::from_str(&reader).expect("JsonParseException");
                compat::set_state(&SOLUTIONS, Some(compat::json::get_as_json_object(&parsed)));
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker Waterboard] Failed to load solutions json"));
            }
        }
    }

    /// java: public void onServerTick()
    pub fn on_server_tick(__this: &compat::Shared<Self>) {
        if !(SkyblockerConfigManager::get().dungeons.puzzle_solvers.waterboard_one_flow) || !({ let __f = compat::dispatch(__this, |__t| __t.should_solve()); __f(&compat::view(__this, |a| a as compat::Shared<dyn DungeonPuzzle>)) }) {
            return;
        }
        { let __v = (compat::read(__this, |__t| __t.current_time_millis)).wrapping_add(50 as i64); compat::write(__this, |__t| __t.current_time_millis = __v); }
    }

    /// java: private void solvePuzzle()
    pub fn solve_puzzle(__this: &compat::Shared<Self>) {
        { let __v = WaterboardOneFlow::find_variant(__this); compat::write(__this, |__t| __t.variant = __v); }
        if compat::read(__this, |__t| __t.variant) == 0 {
            { let __v = true; compat::write(__this, |__t| __t.finished = __v); }
            return;
        }
        { let __v = Some(WaterboardOneFlow::find_doors(__this)); compat::write(__this, |__t| __t.initial_doors = __v); }
        if compat::read(__this, |__t| __t.doors.clone()).is_none() {
            { let __v = compat::read(__this, |__t| __t.initial_doors.clone()); compat::write(__this, |__t| __t.doors = __v); }
            if __this.lock().borrow().doors.as_ref().unwrap().is_empty() {
                { let __v = Some(WaterboardOneFlow::make_empty_solution(__this)); compat::write(__this, |__t| __t.solution = __v); }
                { let __v = true; compat::write(__this, |__t| __t.finished = __v); }
                return;
            } else if compat::str_len(&__this.lock().borrow().doors.as_ref().unwrap()) != 3 {
                script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.dungeons.puzzle.waterboard.invalidDoors")).to_string());
                { let __v = true; compat::write(__this, |__t| __t.finished = __v); }
                return;
            }
        }
        if !(WaterboardOneFlow::check_water(__this)) {
            script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.dungeons.puzzle.waterboard.waterFound")).to_string());
            { let __v = true; compat::write(__this, |__t| __t.finished = __v); }
            return;
        }
        if !(compat::read(__this, |__t| __t.finished)) {
            let mut data: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&(compat::json::get_as_json_object(&(compat::lock(&SOLUTIONS).as_ref().unwrap().get(&compat::read(__this, |__t| __t.variant).to_string()).cloned().unwrap()))).get(compat::read(__this, |__t| __t.doors.clone()).as_deref().unwrap()).cloned().unwrap());
            { let __v = Some(WaterboardOneFlow::setup_solution(__this, &data)); compat::write(__this, |__t| __t.solution = __v); }
        }
    }

    /// java: private EnumMap<LeverType, DoubleList> makeEmptySolution()
    pub fn make_empty_solution(__this: &compat::Shared<Self>) -> std::collections::HashMap<LeverType, Vec<f64>> {
        let mut solution: std::collections::HashMap<LeverType, Vec<f64>> = std::collections::HashMap::new();
        for lever_type in LeverType::values() {
            solution.insert(lever_type, Vec::new());
        }
        return solution;
    }

    /// java: private int findVariant()
    pub fn find_variant(__this: &compat::Shared<Self>) -> i32 {
        let mut first_switches: std::collections::HashSet<LeverType> = std::collections::HashSet::new();
        let mut first_switch_blocks: AABB = AABB::encapsulating_full_blocks(&({ let __r = &compat::read(__this, |__t| __t.room.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.relative_to_actual()); __f(__r, &waterboard::WATER_ENTRANCE_POSITION.get_mut().offset(-1, -1, 0)) }), &({ let __r = &compat::read(__this, |__t| __t.room.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.relative_to_actual()); __f(__r, &waterboard::WATER_ENTRANCE_POSITION.get_mut().offset(1, 0, 1)) }));
        // world.getBlockStates(box) = BlockPos.betweenClosedStream(box).map(this::getBlockState) (BlockGetter:41-43)
        let world = compat::read(__this, |__t| __t.world.clone()).unwrap();
        for state in BlockPos::between_closed_stream_aabb(&first_switch_blocks).map(|pos| world.get_block_state_typed(&pos)).collect::<Vec<_>>() {
            let mut lever_type: Option<LeverType> = LeverType::from_block(&state.get_block());
            if let Some(lever_type) = &lever_type {
                first_switches.insert(*lever_type);
            }
        }
        if first_switches.contains(&LeverType::Gold) && first_switches.contains(&LeverType::Terracotta) {
            return 1;
        } else if first_switches.contains(&LeverType::Emerald) && first_switches.contains(&LeverType::Quartz) {
            return 2;
        } else if first_switches.contains(&LeverType::Quartz) && first_switches.contains(&LeverType::Diamond) {
            return 3;
        } else if first_switches.contains(&LeverType::Gold) && first_switches.contains(&LeverType::Quartz) {
            return 4;
        }
        logger().error(&format!("[Skyblocker Waterboard] Unknown waterboard layout. Detected switches: [{}]", (first_switches.iter().copied().map(|__a0| __a0.get_serialized_name()).collect::<Vec<_>>()).iter().map(|s| s.to_string()).collect::<Vec<String>>().join(", ")));
        return 0;
    }

    /// java: private String findDoors()
    pub fn find_doors(__this: &compat::Shared<Self>) -> String {
        let mut door_builder: String = String::new();
        let mut door_pos: MutableBlockPos = MutableBlockPos::new(15, 57, 19);
        for i in 0..5 {
            if !(({ let (__q0,) = (&({ let __r = &compat::read(__this, |__t| __t.room.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.relative_to_actual()); __f(__r, &door_pos.immutable()) }),); __this.lock().borrow().world.as_ref().unwrap().get_block_state_typed(__q0) }).is_air()) {
                { door_builder.push_str(&i.to_string()); };
            }
            door_pos.move_direction(Direction::North);
        }
        return door_builder.clone();
    }

    /// java: private boolean checkWater()
    pub fn check_water(__this: &compat::Shared<Self>) -> bool {
        for x in waterboard::BOARD_MIN_X..=waterboard::BOARD_MAX_X {
            for y in waterboard::BOARD_MIN_Y..=waterboard::BOARD_MAX_Y {
                let mut pos: BlockPos = ({ let __r = &compat::read(__this, |__t| __t.room.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.relative_to_actual()); __f(__r, &BlockPos::new(x, y, waterboard::BOARD_Z)) });
                let mut state: Arc<BlockState> = __this.lock().borrow().world.as_ref().unwrap().get_block_state_typed(&pos);
                if state.is_block(&blocks::blocks().get("water")) {
                    return false;
                }
            }
        }
        return true;
    }

    /// java: private EnumMap<LeverType, DoubleList> setupSolution(JsonObject data)
    pub fn setup_solution(__this: &compat::Shared<Self>, data: &serde_json::Map<String, serde_json::Value>) -> std::collections::HashMap<LeverType, Vec<f64>> {
        let mut solution: std::collections::HashMap<LeverType, Vec<f64>> = WaterboardOneFlow::make_empty_solution(__this);
        for entry in data.iter().map(|(k, v)| (k.clone(), v.clone())) {
            let mut lever_type: Option<LeverType> = LeverType::from_name(&entry.0);
            if let Some(lever_type) = &lever_type {
                let mut times: Vec<f64> = Vec::new();
                for element in compat::json::get_as_json_array(&entry.1).iter() {
                    times.push(compat::json::get_as_double(&element));
                }
                solution.insert(*lever_type, times.clone());
            }
        }
        for lever_type in LeverType::values() {
            let mut times: Vec<f64> = solution.get(&lever_type).cloned().unwrap();
            if lever_type != LeverType::Water && WaterboardOneFlow::is_lever_active(__this, lever_type) {
                if times.is_empty() || times[0] != 0.0 {
                    // TODO(fabric2seer): List.addFirst(..)
                    //   java: times.addFirst(0.0);
                } else {
                    times.remove(0);
                }
            }
        }
        return solution;
    }

    /// java: private boolean isLeverActive(LeverType leverType)
    pub fn is_lever_active(__this: &compat::Shared<Self>, mut lever_type: LeverType) -> bool {
        let mut offset: Option<BlockPos> = lever_type.fields().initial_positions.as_ref().expect("NullPointerException: initialPositions")[((compat::read(__this, |__t| __t.variant)).wrapping_sub(1)) as usize].clone();
        let Some(mut offset) = offset.clone() else {
            return false;
        };
        return !({ let (__q0,) = (&lever_type.fields().block,); ({ let (__q0,) = (&({ let __r = &compat::read(__this, |__t| __t.room.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.relative_to_actual()); __f(__r, &waterboard::WATER_ENTRANCE_POSITION.get_mut().offset_vec(offset.as_vec3i())) }),); __this.lock().borrow().world.as_ref().unwrap().get_block_state_typed(__q0) }).is_block(__q0) });
    }

    /// java: private boolean isPuzzleSolved()
    pub fn is_puzzle_solved(__this: &compat::Shared<Self>) -> bool {
        if compat::read(__this, |__t| __t.doors.clone()).is_none() || compat::read(__this, |__t| __t.initial_doors.clone()).is_none() || compat::read(__this, |__t| __t.water_start_millis) == (0 as i64) {
            return false;
        }
        let mut current_doors: String = WaterboardOneFlow::find_doors(__this);
        for i in 0..5 {
            let mut s: String = i.to_string();
            if (__this.lock().borrow().initial_doors.as_ref().unwrap().contains(&s) == current_doors.contains(&s)) == __this.lock().borrow().doors.as_ref().unwrap().contains(&s) {
                return false;
            }
        }
        return true;
    }

    /// java: private void extractLeverText(PrimitiveCollector collector, LeverType nextLever)
    pub fn extract_lever_text(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn PrimitiveCollector>, mut next_lever: LeverType) {
        for lever_data in __this.lock().borrow().solution.as_ref().unwrap().iter().map(|(k, v)| (k.clone(), v.clone())) {
            let mut lever: LeverType = lever_data.0;
            for i in 0..(lever_data.1.len() as i32) {
                let mut next_time: f64 = lever_data.1[(i) as usize];
                let mut remaining_time: i64 = ((compat::read(__this, |__t| __t.water_start_millis)).wrapping_add(((next_time * (1000 as f64))) as i64)).wrapping_sub(compat::read(__this, |__t| __t.current_time_millis));
                let mut text: MutableComponent;
                if lever == LeverType::Water && next_time == 0.0 && next_lever != LeverType::Water {
                    text = WAIT_TEXT.get_mut().clone();
                } else if compat::read(__this, |__t| __t.water_start_millis) == (0 as i64) && next_time == 0.0 || compat::read(__this, |__t| __t.water_start_millis) > (0 as i64) && (remaining_time as f64) <= 0.0 {
                    text = CLICK_TEXT.get_mut().clone();
                } else {
                    let mut time_to_show: f64 = (if compat::read(__this, |__t| __t.water_start_millis) == (0 as i64) { next_time } else { ((remaining_time as f64) / 1000.0) });
                    text = (component::literal(&format!("{:.2}", time_to_show))).with_style_format(ChatFormatting::Yellow);
                }
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_text()); __f(collector, &text, &({ let __r = &compat::read(__this, |__t| __t.room.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.relative_to_actual_vec3()); __f(__r, &vec3::at_center_of(&lever.fields().lever_pos)) }).relative(Direction::Up, (0.5 * (i.wrapping_add(1) as f64))), true) });
            }
        }
    }

    /// java: private InteractionResult onUseBlock(Player player, Level world, InteractionHand hand, BlockHitResult blockHitResult)
    pub fn on_use_block(__this: &compat::Shared<Self>, player: &dyn Player, world: &Level, hand: &InteractionHand, block_hit_result: &BlockHitResult) -> InteractionResult {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): static `HitResult.Type` has no Rust counterpart in the port's HitResult file
            // TODO(fabric2seer): the condition: field `BLOCK` of a value of unknown type
            //   java: SkyblockerConfigManager.get().dungeons.puzzleSolvers.waterboardOneFlow &&
            //   java: 					solution != null && blockHitResult.getType() == HitResult.Type.BLOCK
            if todo!("fabric2seer: the condition (TODO above)") {
                let mut lever_type: Option<LeverType> = LeverType::from_pos(&({ let __r = &compat::read(__this, |__t| __t.room.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.actual_to_relative()); __f(__r, &block_hit_result.get_block_pos()) }));
                if let Some(lever_type) = &lever_type {
                    let mut times: Vec<f64> = __this.lock().borrow().solution.as_ref().unwrap().get(lever_type).cloned().unwrap();
                    if compat::read(__this, |__t| __t.water_start_millis) == (0 as i64) && *lever_type != LeverType::Water && (times.is_empty() || times[0] != 0.0) {
                        // TODO(fabric2seer): List.addFirst(..)
                        //   java: times.addFirst(0.0);
                    } else {
                        if !times.is_empty() {
                            times.remove(0);
                        }
                        if compat::read(__this, |__t| __t.water_start_millis) == (0 as i64) && *lever_type == LeverType::Water {
                            { let __v = compat::read(__this, |__t| __t.current_time_millis); compat::write(__this, |__t| __t.water_start_millis = __v); }
                        }
                    }
                }
            }
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Waterboard] Exception in onUseBlock"));
        }
        return InteractionResult::Pass;
    }

    /// java: private void softReset()
    pub fn soft_reset(__this: &compat::Shared<Self>) {
        { let __v = None; compat::write(__this, |__t| __t.solve = __v); }
        { let __v = 0; compat::write(__this, |__t| __t.variant = __v); }
        { let __v = None; compat::write(__this, |__t| __t.doors = __v); }
        { let __v = None; compat::write(__this, |__t| __t.initial_doors = __v); }
        { let __v = None; compat::write(__this, |__t| __t.solution = __v); }
        { let __v = false; compat::write(__this, |__t| __t.finished = __v); }
        { let __v = ((0) as i64) as i64; compat::write(__this, |__t| __t.water_start_millis = __v); }
        { let __v = ((0) as i64) as i64; compat::write(__this, |__t| __t.current_time_millis = __v); }
        compat::write(__this, |__t| {
            for mark in __t.marks.iter_mut() {
                mark.reached = false;
            }
        });
    }

}

impl DungeonPuzzle for WaterboardOneFlow {
    fn dungeon_puzzle_base(&self) -> &DungeonPuzzleBase {
        &self.base
    }
    fn dungeon_puzzle_base_mut(&mut self) -> &mut DungeonPuzzleBase {
        &mut self.base
    }

}

impl Tickable for WaterboardOneFlow {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void tick(Minecraft client) (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn Tickable>, &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        WaterboardOneFlow::tick_dyn
    }

}

impl Renderable for WaterboardOneFlow {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        WaterboardOneFlow::extract_rendering_dyn
    }

}

impl Resettable for WaterboardOneFlow {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        WaterboardOneFlow::reset_dyn
    }

}

impl WaterboardOneFlow {
    /// java: @Override public void tick(Minecraft client)
    pub fn tick_impl(__this: &compat::Shared<Self>, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        if !(SkyblockerConfigManager::get().dungeons.puzzle_solvers.waterboard_one_flow) || !({ let __f = compat::dispatch(__this, |__t| __t.should_solve()); __f(&compat::view(__this, |a| a as compat::Shared<dyn DungeonPuzzle>)) }) || mcrust::mcrust::modding::game::level().is_none() || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || !(DungeonManager::is_current_room_matched()) {
            return;
        }
        { let __v = mcrust::mcrust::modding::game::level(); compat::write(__this, |__t| __t.world = __v); }
        { let __v = DungeonManager::get_current_room(); compat::write(__this, |__t| __t.room = __v); }
        // java: player = client.player;
        { let __v = mcrust::mcrust::modding::game::with_player(|player| player.base().get_id()); compat::write(__this, |__t| __t.player = __v); }
        if compat::read(__this, |__t| __t.solution.clone()).is_none() && !(compat::read(__this, |__t| __t.finished)) && compat::read(__this, |__t| __t.solve.clone()).is_none() {
            let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            { let __v = Some((CompletableFuture::<()>::run_async(move || (|| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { WaterboardOneFlow::solve_puzzle((&__this_1)); } })(), compat::executor)).exceptionally(|e| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; {
                logger().error(&format!("[Skyblocker Waterboard] Encountered an unknown exception while solving waterboard."));
                { let __v = true; compat::write((&__this_2), |__t| __t.finished = __v); }
                return Default::default(); // java: return null
            } })); compat::write(__this, |__t| __t.solve = __v); }
        }
        if !(compat::read(__this, |__t| __t.finished)) && WaterboardOneFlow::is_puzzle_solved(__this) {
            { let __v = true; compat::write(__this, |__t| __t.finished = __v); }
            if compat::read(__this, |__t| __t.timer_enabled) {
                let mut elapsed: f64 = (((compat::read(__this, |__t| __t.current_time_millis)).wrapping_sub(compat::read(__this, |__t| __t.water_start_millis)) as f64) / 1000.0);
                // TODO(fabric2seer): `fmt` takes 1 argument(s) in the port (`f: &mut std::fmt::Formatter<'_>`), Java passes 0
                // rust: script_host::send_feedback(&(constants::PREFIX.get_mut())().append_str("Puzzle solved in ").append((component::literal(&format!("{:.2}", elapsed))).with_style_format(ChatFormatting::Green)).append_str(&ChatFormatting::Reset.fmt()).append_str(" seconds.").to_string());
            }
        }
        if compat::read(__this, |__t| __t.water_start_millis) > (0 as i64) {
            for mark in compat::read(__this, |__t| __t.marks.clone()) {
                if !mark.reached && { let (__q0,) = (&blocks::blocks().get("water"),); __this.lock().borrow().world.as_ref().unwrap().get_block_state_typed(&mark.pos).is_block(__q0) } {
                    mark.reached = true;
                    let mut elapsed: f64 = (((compat::read(__this, |__t| __t.current_time_millis)).wrapping_sub(compat::read(__this, |__t| __t.water_start_millis)) as f64) / 1000.0);
                    // TODO(fabric2seer): `fmt` takes 1 argument(s) in the port (`f: &mut std::fmt::Formatter<'_>`), Java passes 0
                    // rust: script_host::send_feedback(&(constants::PREFIX.get_mut())().append_str(&format!("Mark {} reached in ", mark.index)).append((component::literal(&format!("{:.2}", elapsed))).with_style_format(ChatFormatting::Green)).append_str(&ChatFormatting::Reset.fmt()).append_str(" seconds.").to_string());
                }
            }
        }
    }

    /// WaterboardOneFlow.tick as the dispatch of Tickable.tick
    pub fn tick_dyn(__this: &compat::Shared<dyn Tickable>, __p0: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        Self::tick_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(SkyblockerConfigManager::get().dungeons.puzzle_solvers.waterboard_one_flow) || compat::read(__this, |__t| __t.world.clone()).is_none() || compat::read(__this, |__t| __t.room.clone()).is_none() || compat::read(__this, |__t| __t.player.clone()).is_none() {
            return;
        }
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            for mark in compat::read(__this, |__t| __t.marks.clone()) {
                let mut components: Vec<f32> = ColorUtils::get_float_components_dye_color(&(if mark.reached { DyeColor::Lime } else { DyeColor::White }));
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box()); __f(collector, &mark.pos, &components, 0.5f32, true) });
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_text()); __f(collector, &component::null_to_empty(Some(&format!("Mark {}", mark.index))), &vec3::at_center_of(&mark.pos).relative(Direction::Up, 0.2), true) });
            }
            if compat::read(__this, |__t| __t.solution.clone()).is_some() {
                // TODO(fabric2seer): List.doubleStream(..)
                //   java: List<ObjectDoublePair<LeverType>> sortedTimes = solution.entrySet().stream()
                //   java: 						.flatMap(entry -> entry.getValue().doubleStream().mapToObj(time -> ObjectDoublePair.of(entry.getKey(), time)))
                //   java: 						// Sort by next use time, then by lever type
                //   java: 						.sorted(Comparator
                //   java: 								.<ObjectDoublePair<LeverType>>comparingDouble(p -> p.rightDouble() + (p.left() == LeverType.WATER ? 0.001 : 0.0))
                //   java: 								.thenComparingInt(p -> p.left().ordinal())
                //   java: 						).toList();
                let mut sorted_times: Vec<(LeverType, f64)> = todo!("fabric2seer: the initialiser of `sortedTimes` (TODO above)");
                let mut next_lever: Option<LeverType> = (if sorted_times.is_empty() { None } else { Some(sorted_times[0].0) });
                let mut next_next_lever: Option<LeverType> = (if (sorted_times.len() as i32) < 2 { None } else { Some(sorted_times[(1) as usize].0) });
                if let Some(next_lever) = &next_lever {
                    ({ let __f = compat::dispatch(collector, |__t| __t.submit_line_from_cursor()); __f(collector, &vec3::at_center_of(&({ let __r = &compat::read(__this, |__t| __t.room.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.relative_to_actual()); __f(__r, &next_lever.fields().lever_pos) })), &ColorUtils::get_float_components_dye_color(&DyeColor::Lime), 1.0f32, 4.0f32) });
                    if let Some(next_next_lever) = &next_next_lever {
                        ({ let __f = compat::dispatch(collector, |__t| __t.submit_lines_from_points()); __f(collector, &vec![vec3::at_center_of(&({ let __r = &compat::read(__this, |__t| __t.room.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.relative_to_actual()); __f(__r, &next_lever.fields().lever_pos) })), vec3::at_center_of(&({ let __r = &compat::read(__this, |__t| __t.room.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.relative_to_actual()); __f(__r, &next_next_lever.fields().lever_pos) }))], &ColorUtils::get_float_components_dye_color(&DyeColor::White), 1.0f32, 2.0f32, true) });
                    }
                }
                WaterboardOneFlow::extract_lever_text(__this, collector, next_lever.clone().unwrap());
            }
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Waterboard] Error while rendering one flow"));
        }
    }

    /// WaterboardOneFlow.extractRendering as the dispatch of Renderable.extractRendering
    pub fn extract_rendering_dyn(__this: &compat::Shared<dyn Renderable>, __p0: &compat::Shared<dyn PrimitiveCollector>) {
        Self::extract_rendering_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void reset()
    pub fn reset_impl(__this: &compat::Shared<Self>) {
        <dyn DungeonPuzzle>::reset_body(&compat::view(__this, |a| a as compat::Shared<dyn DungeonPuzzle>));
        WaterboardOneFlow::soft_reset(__this);
    }

    /// WaterboardOneFlow.reset as the dispatch of Resettable.reset
    pub fn reset_dyn(__this: &compat::Shared<dyn Resettable>) {
        Self::reset_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: private static class Mark
#[derive(Clone)]
pub struct Mark {
    /// java: private final int index;
    pub index: i32,
    /// java: private final BlockPos pos;
    pub pos: BlockPos,
    /// java: private boolean reached;
    pub reached: bool,
}

impl Mark {
    /// java: private Mark(int index, BlockPos pos)
    pub fn new(mut index: i32, mut pos: BlockPos) -> Self {
        let __init_index = index;
        let __init_pos = pos.clone();
        let __init_reached = false;
        let mut this = Self {
            index: __init_index,
            pos: __init_pos,
            reached: __init_reached,
        };
        this
    }

}
