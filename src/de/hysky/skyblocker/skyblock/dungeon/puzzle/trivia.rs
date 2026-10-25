//! Ported from de.hysky.skyblocker.skyblock.dungeon.puzzle.Trivia by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::dungeon_puzzle::DungeonPuzzle;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::dungeon_puzzle::DungeonPuzzleBase;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Room;
use crate::de::hysky::skyblocker::skyblock::waypoint::fairy_souls::FairySouls;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use crate::de::hysky::skyblocker::utils::tickable::Tickable;
use crate::de::hysky::skyblocker::utils::time::skyblock_time;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::player::local_player::LocalPlayer;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::core::direction::Direction;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: @SuppressWarnings("unused") private static final Trivia INSTANCE = new Trivia();
pub static INSTANCE: compat::MainThread<compat::Shared<Trivia>> = compat::MainThread::new(|| Trivia::new());
/// java: @VisibleForTesting public static final Pattern PATTERN = Pattern.compile("^ +(?:([A-Za-z,' ]*\\?)| ([ⓐⓑⓒ]) ([a-zA-Z0-9 ]+))|(\\[STATUE] Oruo the Omniscient: (\\w+ answered Question #\\d correctly!|I b
pub static PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^ +(?:([A-Za-z,' ]*\\?)| ([ⓐⓑⓒ]) ([a-zA-Z0-9 ]+))|(\\[STATUE] Oruo the Omniscient: (\\w+ answered Question #\\d correctly!|I bestow upon you all the power of a hundred years!|Yikes))$"));
/// java: private static final BlockPos CHOICE_A = new BlockPos(20, 70, 6);
pub static CHOICE_A: compat::MainThread<BlockPos> = compat::MainThread::new(|| BlockPos::new(20, 70, 6));
/// java: private static final BlockPos CHOICE_B = new BlockPos(15, 70, 9);
pub static CHOICE_B: compat::MainThread<BlockPos> = compat::MainThread::new(|| BlockPos::new(15, 70, 9));
/// java: private static final BlockPos CHOICE_C = new BlockPos(10, 70, 6);
pub static CHOICE_C: compat::MainThread<BlockPos> = compat::MainThread::new(|| BlockPos::new(10, 70, 6));
/// java: private static final float[] ANSWER_COLOR = new float[]{0, 1f, 0};
pub static ANSWER_COLOR: LazyLock<Vec<f32>> = LazyLock::new(|| vec![((0) as f32) as f32, 1.0f32, ((0) as f32) as f32]);
/// java: private static final Direction[] DIRECTIONS = new Direction[]{Direction.NORTH, Direction.SOUTH, Direction.WEST, Direction.EAST};
pub static DIRECTIONS: compat::MainThread<Vec<Direction>> = compat::MainThread::new(|| vec![Direction::North, Direction::South, Direction::West, Direction::East]);
/// java: private static final ArrayList<AABB> BOXES_TO_HIGHLIGHT = new ArrayList<>();
pub static BOXES_TO_HIGHLIGHT: compat::MainThread<Vec<AABB>> = compat::MainThread::new(|| Vec::new());
/// java: private static final Map<String, List<String>> answers = new Object2ObjectOpenHashMap<>();
pub static ANSWERS: LazyLock<Mutex<std::collections::HashMap<String, Vec<String>>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));
/// java: private static String currentSolution = "";
pub static CURRENT_SOLUTION: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("".to_string()));

/// java: public class Trivia extends DungeonPuzzle
#[derive(Clone)]
pub struct Trivia {
    /// java: extends DungeonPuzzle
    pub base: DungeonPuzzleBase,
    /// java: private List<String> solutions = Collections.emptyList();
    pub solutions: Vec<String>,
}

impl std::ops::Deref for Trivia {
    type Target = DungeonPuzzleBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for Trivia {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Trivia {
    /// java: public Trivia()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = DungeonPuzzleBase::new_in("trivia".to_string(), vec!["trivia-room".to_string()], move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                solutions: Default::default(),
            });
            ((__h.clone() as compat::Shared<dyn DungeonPuzzle>), __h)
        });
        // java: ClientReceiveMessageEvents.ALLOW_GAME.register(this::onMessage); - the listener holds this object
        let __this_allow_game = compat::RenderThread::new(__this.clone());
        compat::register_allow_game(move |text, overlay| Trivia::on_message(__this_allow_game.get(), text, overlay));
        __this
    }

    /// java: private static @Nullable BlockPos updateCorrectBlockPos()
    pub fn update_correct_block_pos() -> Option<BlockPos> {
        return match compat::lock(&CURRENT_SOLUTION).as_str() {
            "ⓐ" => Some(CHOICE_A.get_mut().clone()),
            "ⓑ" => Some(CHOICE_B.get_mut().clone()),
            "ⓒ" => Some(CHOICE_C.get_mut().clone()),
            _ => None,
        };
    }

    /// java: @Init(priority = 100) // Load after FairySouls public static void init()
    pub fn init() {
        compat::lock(&ANSWERS).insert("What is the status of The Watcher?".to_string(), vec!["Stalker".to_string()]);
        compat::lock(&ANSWERS).insert("What is the status of Bonzo?".to_string(), vec!["New Necromancer".to_string()]);
        compat::lock(&ANSWERS).insert("What is the status of Scarf?".to_string(), vec!["Apprentice Necromancer".to_string()]);
        compat::lock(&ANSWERS).insert("What is the status of The Professor?".to_string(), vec!["Professor".to_string()]);
        compat::lock(&ANSWERS).insert("What is the status of Thorn?".to_string(), vec!["Shaman Necromancer".to_string()]);
        compat::lock(&ANSWERS).insert("What is the status of Livid?".to_string(), vec!["Master Necromancer".to_string()]);
        compat::lock(&ANSWERS).insert("What is the status of Sadan?".to_string(), vec!["Necromancer Lord".to_string()]);
        compat::lock(&ANSWERS).insert("What is the status of Maxor?".to_string(), vec!["The Wither Lords".to_string()]);
        compat::lock(&ANSWERS).insert("What is the status of Goldor?".to_string(), vec!["The Wither Lords".to_string()]);
        compat::lock(&ANSWERS).insert("What is the status of Storm?".to_string(), vec!["The Wither Lords".to_string()]);
        compat::lock(&ANSWERS).insert("What is the status of Necron?".to_string(), vec!["The Wither Lords".to_string()]);
        compat::lock(&ANSWERS).insert("What is the status of Maxor, Storm, Goldor, and Necron?".to_string(), vec!["The Wither Lords".to_string()]);
        compat::lock(&ANSWERS).insert("Which brother is on the Spider's Den?".to_string(), vec!["Rick".to_string()]);
        compat::lock(&ANSWERS).insert("What is the name of Rick's brother?".to_string(), vec!["Pat".to_string()]);
        compat::lock(&ANSWERS).insert("glass?".to_string(), vec!["Wool Weaver".to_string()]);
        compat::lock(&ANSWERS).insert("What is the name of the person that upgrades pets?".to_string(), vec!["Kat".to_string()]);
        compat::lock(&ANSWERS).insert("What is the name of the lady of the Nether?".to_string(), vec!["Elle".to_string()]);
        compat::lock(&ANSWERS).insert("Which villager in the Village gives you a Rogue Sword?".to_string(), vec!["Jamie".to_string()]);
        compat::lock(&ANSWERS).insert("How many unique minions are there?".to_string(), vec!["61 Minions".to_string()]);
        compat::lock(&ANSWERS).insert("Which of these enemies does not spawn in the Spider's Den?".to_string(), vec!["Zombie Spider".to_string(), "Cave Spider".to_string(), "Wither Skeleton".to_string(), "Dashing Spooder".to_string(), "Broodfather".to_string(), "Night Spider".to_string()]);
        compat::lock(&ANSWERS).insert("Which of these monsters only spawns at night?".to_string(), vec!["Zombie Villager".to_string(), "Ghast".to_string()]);
        compat::lock(&ANSWERS).insert("Which of these is not a dragon in The End?".to_string(), vec!["Zoomer Dragon".to_string(), "Weak Dragon".to_string(), "Stonk Dragon".to_string(), "Holy Dragon".to_string(), "Boomer Dragon".to_string(), "Booger Dragon".to_string(), "Older Dragon".to_string(), "Elder Dragon".to_string(), "Stable Dragon".to_string(), "Professor Dragon".to_string()]);
        FairySouls::run_async_after_fairy_souls_load(Arc::new(move || {
            compat::lock(&ANSWERS).insert("How many total Fairy Souls are there?".to_string(), Trivia::get_fairy_souls_size_string(None));
            compat::lock(&ANSWERS).insert("How many Fairy Souls are there in Spider's Den?".to_string(), Trivia::get_fairy_souls_size_string(Some("combat_1".to_string())));
            compat::lock(&ANSWERS).insert("How many Fairy Souls are there in The End?".to_string(), Trivia::get_fairy_souls_size_string(Some("combat_3".to_string())));
            compat::lock(&ANSWERS).insert("How many Fairy Souls are there in The Farming Islands?".to_string(), Trivia::get_fairy_souls_size_string(Some("farming_1".to_string())));
            compat::lock(&ANSWERS).insert("How many Fairy Souls are there in Crimson Isle?".to_string(), Trivia::get_fairy_souls_size_string(Some("crimson_isle".to_string())));
            compat::lock(&ANSWERS).insert("How many Fairy Souls are there in The Park?".to_string(), Trivia::get_fairy_souls_size_string(Some("foraging_1".to_string())));
            compat::lock(&ANSWERS).insert("How many Fairy Souls are there in Jerry's Workshop?".to_string(), Trivia::get_fairy_souls_size_string(Some("winter".to_string())));
            compat::lock(&ANSWERS).insert("How many Fairy Souls are there in Hub?".to_string(), Trivia::get_fairy_souls_size_string(Some("hub".to_string())));
            compat::lock(&ANSWERS).insert("How many Fairy Souls are there in The Hub?".to_string(), Trivia::get_fairy_souls_size_string(Some("hub".to_string())));
            compat::lock(&ANSWERS).insert("How many Fairy Souls are there in Deep Caverns?".to_string(), Trivia::get_fairy_souls_size_string(Some("mining_2".to_string())));
            compat::lock(&ANSWERS).insert("How many Fairy Souls are there in Gold Mine?".to_string(), Trivia::get_fairy_souls_size_string(Some("mining_1".to_string())));
            compat::lock(&ANSWERS).insert("How many Fairy Souls are there in Dungeon Hub?".to_string(), Trivia::get_fairy_souls_size_string(Some("dungeon_hub".to_string())));
        }));
    }

    /// java: private static List<String> getFairySoulsSizeString(@Nullable String location)
    pub fn get_fairy_souls_size_string(mut location: Option<String>) -> Vec<String> {
        return vec![format!("{} Fairy Souls", FairySouls::get_fairy_souls_size(location.clone()))];
    }

    /// java: @SuppressWarnings("BooleanMethodIsAlwaysInverted") public boolean shouldRun()
    pub fn should_run(__this: &compat::Shared<Self>) -> bool {
        return (Utils::is_in_dungeons() && SkyblockerConfigManager::get().dungeons.puzzle_solvers.solve_trivia);
    }

    /// java: public boolean onMessage(Component message, boolean overlay)
    pub fn on_message(__this: &compat::Shared<Self>, message: &MutableComponent, mut overlay: bool) -> bool {
        if !(Trivia::should_run(__this)) || overlay {
            return true;
        }
        let mut matcher: compat::regex::Matcher = PATTERN.matcher(&chat_formatting::strip_formatting(&message.get_string()));
        if !matcher.matches() {
            return true;
        }
        if matcher.group(4).is_some() {
            ({ let __f = compat::dispatch(__this, |__t| __t.reset()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Resettable>)) });
            return true;
        }
        let mut answer_choice: Option<String> = matcher.group(3);
        if let Some(answer_choice) = &answer_choice {
            if __this.lock().borrow().solutions.is_empty() {
                return true;
            }
            if !(__this.lock().borrow().solutions.iter().any(|e| *e == answer_choice.to_string())) {
                if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
                    return true;
                }
                Utils::send_message_to_bypass_events(&component::null_to_empty(Some(&format!("    {} {} {}{}", compat::jstring(&ChatFormatting::Gold), compat::opt_str(&matcher.group(2)), compat::jstring(&ChatFormatting::Red), answer_choice))));
                return false;
            }
            compat::set_state(&CURRENT_SOLUTION, matcher.group(2).unwrap());
        } else {
            Trivia::update_solutions(__this, matcher.group(0).as_deref().unwrap());
            ({ let __f = compat::dispatch(__this, |__t| __t.reset()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Resettable>)) });
        }
        return true;
    }

    /// java: private void updateSolutions(String question)
    pub fn update_solutions(__this: &compat::Shared<Self>, question: &str) {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut trimmed_question: String = compat::trim(&question);
            if trimmed_question == "What SkyBlock year is it?" {
                let mut year: i32 = compat::lock(&skyblock_time::SKYBLOCK_YEAR).load(std::sync::atomic::Ordering::SeqCst);
                { let __v = vec![format!("Year {}", year)]; compat::write(__this, |__t| __t.solutions = __v); }
            } else {
                { let __v = compat::lock(&ANSWERS).get(&trimmed_question).cloned().unwrap_or_else(|| Default::default()); compat::write(__this, |__t| __t.solutions = __v); }
            }
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker] Failed to update the Trivia puzzle answers!"));
        }
    }

}

impl DungeonPuzzle for Trivia {
    fn dungeon_puzzle_base(&self) -> &DungeonPuzzleBase {
        &self.base
    }
    fn dungeon_puzzle_base_mut(&mut self) -> &mut DungeonPuzzleBase {
        &mut self.base
    }

}

impl Tickable for Trivia {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void tick(Minecraft client) (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn Tickable>, &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        Trivia::tick_dyn
    }

}

impl Renderable for Trivia {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        Trivia::extract_rendering_dyn
    }

}

impl Resettable for Trivia {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        Trivia::reset_dyn
    }

}

impl Trivia {
    /// java: @Override public void tick(Minecraft client)
    pub fn tick_impl(__this: &compat::Shared<Self>, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        if !(Trivia::should_run(__this)) || compat::lock(&CURRENT_SOLUTION).is_empty() || !(BOXES_TO_HIGHLIGHT.get_mut().is_empty()) || mcrust::mcrust::modding::game::level().is_none() {
            return;
        }
        let Some(mut correct_block_pos) = Trivia::update_correct_block_pos() else {
            return;
        };
        let mut room: compat::Shared<dyn Room> = DungeonManager::get_current_room().unwrap();
        for direction in DIRECTIONS.get_mut().iter().copied() {
            let mut button_box: Option<AABB> = RenderHelper::get_block_bounding_box(&mcrust::mcrust::modding::game::level().unwrap(), &({ let __f = compat::dispatch(&room, |__t| __t.relative_to_actual()); __f(&room, &correct_block_pos) }).relative(direction));
            if let Some(button_box) = &button_box {
                BOXES_TO_HIGHLIGHT.get_mut().push(button_box.clone());
            }
        }
    }

    /// Trivia.tick as the dispatch of Tickable.tick
    pub fn tick_dyn(__this: &compat::Shared<dyn Tickable>, __p0: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        Self::tick_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(Trivia::should_run(__this)) || BOXES_TO_HIGHLIGHT.get_mut().is_empty() {
            return;
        }
        for r#box in BOXES_TO_HIGHLIGHT.get_mut().iter() {
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box_aabb_float_float_boolean()); __f(collector, r#box, &ANSWER_COLOR, 0.5f32, false) });
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box_aabb_float_float_boolean()); __f(collector, r#box, &ANSWER_COLOR, 5.0f32, false) });
        }
    }

    /// Trivia.extractRendering as the dispatch of Renderable.extractRendering
    pub fn extract_rendering_dyn(__this: &compat::Shared<dyn Renderable>, __p0: &compat::Shared<dyn PrimitiveCollector>) {
        Self::extract_rendering_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void reset()
    pub fn reset_impl(__this: &compat::Shared<Self>) {
        compat::set_state(&CURRENT_SOLUTION, "".to_string());
        BOXES_TO_HIGHLIGHT.get_mut().clear();
    }

    /// Trivia.reset as the dispatch of Resettable.reset
    pub fn reset_dyn(__this: &compat::Shared<dyn Resettable>) {
        Self::reset_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
