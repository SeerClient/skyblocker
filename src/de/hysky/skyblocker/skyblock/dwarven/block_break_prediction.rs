//! Ported from de.hysky.skyblocker.skyblock.dwarven.BlockBreakPrediction by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::PlayerListManager;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::location;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use crate::de::hysky::skyblocker::utils::sky_block_icons;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::core::direction::Direction;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::interaction_hand::InteractionHand;
use mcrust::net::minecraft::world::interaction_result::InteractionResult;
use mcrust::net::minecraft::world::level::block::block::Block;
use mcrust::net::minecraft::world::level::level::Level;
use mcrust::net::minecraft::world::phys::block_hit_result::BlockHitResult;
use mcrust::net::minecraft::core::registry::Registry as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;

/// java: private static final EnumMap<Location, Map<Block, IntIntPair>> blockStrengths = new EnumMap<>(Location.class);
/// The inner `Map<Block, IntIntPair>` is keyed by the `Block` OBJECT (`Object.hashCode`): an identity map, kept as
/// pairs compared with `Arc::ptr_eq`.
pub static BLOCK_STRENGTHS: compat::MainThread<std::collections::HashMap<Location, Vec<(Arc<Block>, (i32, i32))>>> = compat::MainThread::new(|| std::collections::HashMap::new());

/// `map.get(block)` on one of `BLOCK_STRENGTHS`' identity maps.
fn strength_of(strengths: &[(Arc<Block>, (i32, i32))], block: &Arc<Block>) -> Option<(i32, i32)> {
    strengths.iter().find(|(key, _)| Arc::ptr_eq(key, block)).map(|(_, value)| *value)
}
/// java: private static final Pattern MINING_SPEED_PATTERN = Pattern.compile(String.format("Mining Speed: %s(\\d+)", SkyBlockIcons.MINING_SPEED));
pub static MINING_SPEED_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(&format!("Mining Speed: {}(\\d+)", sky_block_icons::MINING_SPEED)));
/// java: private static final Pattern BREAKING_POWER_PATTERN = Pattern.compile("Breaking Power (\\d+)");
pub static BREAKING_POWER_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Breaking Power (\\d+)"));
/// java: private static boolean newBlock = false;
pub static NEW_BLOCK: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static boolean soundPlayed = false;
pub static SOUND_PLAYED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static long currentBlockBreakTime;
pub static CURRENT_BLOCK_BREAK_TIME: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(0));
/// java: private static long startAttackingTime;
pub static START_ATTACKING_TIME: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(0));
/// java: private static boolean sentWarningMessage = false;
pub static SENT_WARNING_MESSAGE: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: public static final Codec<BlockFile> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.INT.fieldOf("blockStrength").forGetter(BlockFile::blockStrength), Codec.INT.fieldOf("breakingPo
pub static BLOCK_FILE_CODEC: compat::MainThread<CodecRef<BlockFile>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply4(record_codec_builder::of(|o: &BlockFile| o.block_strength(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "blockStrength")), record_codec_builder::of(|o: &BlockFile| o.breaking_power(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "breakingPower")), record_codec_builder::of(|o: &BlockFile| o.name(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "name")), record_codec_builder::of(|o: &BlockFile| o.skyblock_blocks(), codec::field_of(SKYBLOCK_BLOCK_LIST_CODEC.get_mut().clone(), "blocks189")), move |__a0, __a1, __a2, __a3| BlockFile::new(__a0.clone(), __a1.clone(), __a2.clone(), __a3.clone()))));
/// java: private static final Codec<SkyblockBlock> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("itemId").forGetter(SkyblockBlock::itemId), Location.CODEC.listOf().fieldOf
pub static SKYBLOCK_BLOCK_CODEC: compat::MainThread<CodecRef<SkyblockBlock>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply2(record_codec_builder::of(|o: &SkyblockBlock| o.item_id(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "itemId")), record_codec_builder::of(|o: &SkyblockBlock| o.only_in(), codec::field_of(codec::list(location::CODEC.get_mut().clone()), "onlyIn")), move |__a0, __a1| SkyblockBlock::new(__a0.clone(), __a1.clone()))));
/// java: public static final Codec<List<SkyblockBlock>> LIST_CODEC = CODEC.listOf();
pub static SKYBLOCK_BLOCK_LIST_CODEC: compat::MainThread<CodecRef<Vec<SkyblockBlock>>> = compat::MainThread::new(|| codec::list(SKYBLOCK_BLOCK_CODEC.get_mut().clone()));

/// java: public class BlockBreakPrediction
#[derive(Clone)]
pub struct BlockBreakPrediction {}

impl BlockBreakPrediction {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // TODO(fabric2seer): AttackBlockCallback.EVENT.register: no SeerClient equivalent yet
        //   java: AttackBlockCallback.EVENT.register(BlockBreakPrediction::onBlockInteract);
        NEURepoManager::run_async_after_load(Arc::new(move || { BlockBreakPrediction::load_block_strength(); }));
    }

    /// java: private static InteractionResult onBlockInteract(Player player, Level level, InteractionHand interactionHand, BlockPos blockPos, Direction direction)
    pub fn on_block_interact(player: &dyn Player, level: &Level, interaction_hand: &InteractionHand, block_pos: &BlockPos, direction: &Direction) -> InteractionResult {
        compat::set_state(&NEW_BLOCK, true);
        compat::set_state(&START_ATTACKING_TIME, compat::millis());
        compat::set_state(&SOUND_PLAYED, false);
        return InteractionResult::Pass;
    }

    /// java: public static int getBlockBreakPrediction(BlockPos pos, int progression)
    pub fn get_block_break_prediction(pos: &BlockPos, mut progression: i32) -> i32 {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || false /* java: compat::minecraft().game_mode == null: never null here */ || !(SkyblockerConfigManager::get().mining.block_break_prediction.enabled) {
            return progression;
        }
        if let Some(mcrust::net::minecraft::world::phys::hit_result::AnyHitResult::Block(hit_result)) = compat::minecraft().hit_result.as_ref() {
            if !(hit_result.get_block_pos() == *pos) {
                return progression;
            }
        } else {
            return progression;
        }
        if compat::get_state(&NEW_BLOCK) {
            compat::set_state(&NEW_BLOCK, false);
            let mut lore_match: Option<compat::regex::Matcher> = ItemUtils::get_lore_line_if_match(&mcrust::mcrust::modding::game::with_player(|player| player.get_main_hand_item().clone()).unwrap(), &BREAKING_POWER_PATTERN);
            if let Some(lore_match) = &lore_match {
                // `NumberUtils.toInt(str)`: `Integer.parseInt` (no trimming), 0 when it throws or the string is null.
                let mut tool_breaking_power: i32 = lore_match.group(1).and_then(|group| group.parse::<i32>().ok()).unwrap_or(0);
                compat::set_state(&CURRENT_BLOCK_BREAK_TIME, BlockBreakPrediction::get_break_time(pos, tool_breaking_power));
            } else {
                compat::set_state(&CURRENT_BLOCK_BREAK_TIME, ((-1) as i64) as i64);
            }
        }
        if !(compat::minecraft().game_mode.is_destroying()) {
            compat::set_state(&CURRENT_BLOCK_BREAK_TIME, ((-1) as i64) as i64);
        }
        if compat::get_state(&CURRENT_BLOCK_BREAK_TIME) > (0 as i64) {
            let mut time_elapsed: i64 = (compat::millis()).wrapping_sub(compat::get_state(&START_ATTACKING_TIME));
            if SkyblockerConfigManager::get().mining.block_break_prediction.play_sound && !(compat::get_state(&SOUND_PLAYED)) && (((time_elapsed.wrapping_mul(10 as i64)).wrapping_div(compat::get_state(&CURRENT_BLOCK_BREAK_TIME))) as i32) == 10 {
                compat::set_state(&SOUND_PLAYED, true);
                mcrust::mcrust::modding::game::with_player(|player| { <_ as mcrust::net::minecraft::world::entity::entity::Entity>::play_sound(player, mcrust::net::minecraft::sounds::sound_events::blaze_hurt().clone(), 100.0f32, 1.0f32); });
            }
            return (((time_elapsed.wrapping_mul(10 as i64)).wrapping_div(compat::get_state(&CURRENT_BLOCK_BREAK_TIME))) as i32).min(9);
        }
        return progression;
    }

    /// java: private static int getCurrentMiningSpeed()
    pub fn get_current_mining_speed() -> i32 {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return -1;
        }
        let mut speed: Option<compat::regex::Matcher> = PlayerListManager::get_player_string_list().into_iter().map(|__a0| MINING_SPEED_PATTERN.matcher(&__a0)).filter_map(|mut __a0| __a0.matches().then_some(__a0)).next();
        if speed.is_none() {
            if !(compat::get_state(&SENT_WARNING_MESSAGE)) {
                mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.blockBreakPrediction.enableStatsMessage")).with_style_format(ChatFormatting::Red).to_string()); });
                compat::set_state(&SENT_WARNING_MESSAGE, true);
            }
            return -1;
        }
        // `NumberUtils.toInt(str)`: `Integer.parseInt` (no trimming), 0 when it throws or the string is null.
        return speed.unwrap().group(1).and_then(|group| group.parse::<i32>().ok()).unwrap_or(0);
    }

    /// java: private static long getBreakTime(BlockPos pos, int toolBreakingPower)
    pub fn get_break_time(pos: &BlockPos, mut tool_breaking_power: i32) -> i64 {
        if mcrust::mcrust::modding::game::level().is_none() {
            return ((-1) as i64) as i64;
        }
        let mut target_block: Arc<Block> = mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(pos).get_block();
        if !(BLOCK_STRENGTHS.get().contains_key(&Utils::get_location())) || strength_of(BLOCK_STRENGTHS.get().get(&Utils::get_location()).unwrap(), &target_block).is_none() {
            return ((-1) as i64) as i64;
        }
        let mut block_strength_and_breaking_power: (i32, i32) = strength_of(BLOCK_STRENGTHS.get().get(&Utils::get_location()).unwrap(), &target_block).unwrap();
        if block_strength_and_breaking_power.1 > tool_breaking_power {
            return ((-1) as i64) as i64;
        }
        let mut block_strength: i32 = block_strength_and_breaking_power.0;
        let mut mining_speed: i32 = BlockBreakPrediction::get_current_mining_speed();
        return ((((50 as f32) * (((block_strength as f32) * 30.0f32) / (mining_speed as f32)).min(((20.0f32 / (3 as f32)) * (block_strength as f32))))) as i64);
    }

    /// java: public static void addStrength(Location location, Block blockId, int strength, int breakingPower)
    pub fn add_strength(mut location: Location, mut block_id: Arc<Block>, mut strength: i32, mut breaking_power: i32) {
        // `computeIfAbsent(location, _ -> new HashMap<>()).put(blockId, IntIntPair.of(strength, breakingPower))`
        let strengths = BLOCK_STRENGTHS.get_mut().entry(location).or_insert_with(Vec::new);
        match strengths.iter_mut().find(|(key, _)| Arc::ptr_eq(key, &block_id)) {
            Some((_, value)) => *value = (strength, breaking_power),
            None => strengths.push((block_id.clone(), (strength, breaking_power))),
        }
    }

    /// java: private static void loadBlockStrength()
    pub fn load_block_strength() {
        // java: try {..} catch (NEURepositoryException exception) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut blocks: Vec<crate::unresolved::NEURepoFile> = NEURepoManager::tree("mining/blocks").collect::<Vec<_>>();
            for file in blocks.iter() {
                //   java: !file.isFile()
                if !compat::neu_repo_file_is_file(file) {
                    continue;
                }
                {
                    //   java: InputStream stream = file.stream(); (try-with-resources)
                    // java: try {..} catch (Exception ex) - an exception here is a panic, caught by `catch_unwind`
                    let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        //   java: BlockFile data = BlockFile.CODEC.parse(JsonOps.INSTANCE, JsonParser.parseString(new String(stream.readAllBytes()))).getOrThrow();
                        use mcrust::com::mojang::serialization::decoder::Decoder as _;
                        let content: String = compat::neu_repo_file_read_all(file);
                        let data: BlockFile = BLOCK_FILE_CODEC.get().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&serde_json::from_str::<serde_json::Value>(&content).expect("JsonSyntaxException"))).get_or_throw().unwrap_or_else(|e| panic!("{e}"));
                        for skyblock_block_type in data.skyblock_blocks.iter() {
                            for location in skyblock_block_type.only_in.iter().copied() {
                                if data.name == "Mithril Ore" {
                                    let mut block: Arc<Block> = BlockBreakPrediction::get_block_from_repo(&skyblock_block_type.item_id);
                                    if std::ptr::addr_eq(std::sync::Arc::as_ptr(&block), std::sync::Arc::as_ptr(mcrust::net::minecraft::world::level::block::blocks::wool().gray())) || std::ptr::addr_eq(std::sync::Arc::as_ptr(&block), std::sync::Arc::as_ptr(mcrust::net::minecraft::world::level::block::blocks::dyed_terracotta().cyan())) {
                                        BlockBreakPrediction::add_strength(location, block.clone(), 500, data.breaking_power);
                                    } else if std::ptr::addr_eq(std::sync::Arc::as_ptr(&block), std::sync::Arc::as_ptr(mcrust::net::minecraft::world::level::block::blocks::wool().light_blue())) {
                                        BlockBreakPrediction::add_strength(location, block.clone(), 1500, data.breaking_power);
                                    } else {
                                        BlockBreakPrediction::add_strength(location, block.clone(), 800, data.breaking_power);
                                    }
                                    continue;
                                }
                                BlockBreakPrediction::add_strength(location, BlockBreakPrediction::get_block_from_repo(&skyblock_block_type.item_id), data.block_strength, data.breaking_power);
                            }
                        }
                    }));
                    if let Err(ex) = __result {
                        logger().error(&format!("[Skyblocker BlockBreakPredictions] Failed to load mining blocks!"));
                    }
                }
            }
        }));
        if let Err(exception) = __result {
            logger().error(&format!("[Skyblocker BlockBreakPredictions] Failed to load mining blocks!"));
        }
    }

    /// java: public static Block getBlockFromRepo(String id)
    pub fn get_block_from_repo(id: &str) -> Arc<Block> {
        // `BuiltInRegistries.BLOCK.getValue(Identifier.parse(id))`: a defaulted registry (air for an unknown id); the
        // value is the erased block.
        let value = mcrust::net::minecraft::core::registries::built_in_registries::block().get_value_by_identifier(&Identifier::parse(id));
        return value
            .as_ref()
            .and_then(|value| mcrust::com::mojang::serialization::cast::<mcrust::net::minecraft::world::level::block::block::ErasedBlock>(value))
            .map(|block| Arc::clone(&block.0))
            .unwrap_or_else(|| mcrust::net::minecraft::world::level::block::blocks::blocks().get("AIR"));
    }

}

/// java: public record BlockFile(int blockStrength, int breakingPower, String name, List<SkyblockBlock> skyblockBlocks)
#[derive(Clone, Debug, PartialEq)]
pub struct BlockFile {
    /// java: int blockStrength
    pub block_strength: i32,
    /// java: int breakingPower
    pub breaking_power: i32,
    /// java: String name
    pub name: String,
    /// java: List<SkyblockBlock> skyblockBlocks
    pub skyblock_blocks: Vec<SkyblockBlock>,
}

impl BlockFile {
    /// java: the canonical record constructor
    pub fn new(mut block_strength: i32, mut breaking_power: i32, mut name: String, mut skyblock_blocks: Vec<SkyblockBlock>) -> Self {
        Self { block_strength, breaking_power, name, skyblock_blocks }
    }

    /// java: the record accessor `blockStrength()`
    pub fn block_strength(&self) -> i32 {
        self.block_strength
    }

    /// java: the record accessor `breakingPower()`
    pub fn breaking_power(&self) -> i32 {
        self.breaking_power
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// java: the record accessor `skyblockBlocks()`
    pub fn skyblock_blocks(&self) -> Vec<SkyblockBlock> {
        self.skyblock_blocks.clone()
    }

}

/// java: public record SkyblockBlock(String itemId, List<Location> onlyIn)
#[derive(Clone, Debug, PartialEq)]
pub struct SkyblockBlock {
    /// java: String itemId
    pub item_id: String,
    /// java: List<Location> onlyIn
    pub only_in: Vec<Location>,
}

impl SkyblockBlock {
    /// java: the canonical record constructor
    pub fn new(mut item_id: String, mut only_in: Vec<Location>) -> Self {
        Self { item_id, only_in }
    }

    /// java: the record accessor `itemId()`
    pub fn item_id(&self) -> String {
        self.item_id.clone()
    }

    /// java: the record accessor `onlyIn()`
    pub fn only_in(&self) -> Vec<Location> {
        self.only_in.clone()
    }

}

