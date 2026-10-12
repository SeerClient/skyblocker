//! Ported from de.hysky.skyblocker.skyblock.garden.GreenhousePaste by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::world_events;
use crate::de::hysky::skyblocker::skyblock::garden::greenhouse_crops;
use crate::de::hysky::skyblocker::skyblock::item::head_textures;
use crate::de::hysky::skyblocker::utils::codec_utils::CodecUtils;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonDataBase;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::lz_string::LZString;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::blaze3d::vertex::pose_stack::PoseStack;
use mcrust::com::mojang::brigadier::builder::literal_argument_builder::LiteralArgumentBuilder;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::model::object::skull::skull_model_base::SkullModelBase;
use mcrust::net::minecraft::client::model::object::skull::skull_model_base::State;
use mcrust::net::minecraft::client::renderer::rendertype::render_type::RenderType;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::entity::equipment_slot::EquipmentSlot;
use mcrust::net::minecraft::world::level::block::block::Block;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use mcrust::net::minecraft::world::level::level::Level;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::core::component::data_component_map::DataComponentGetter as _;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;
use mcrust::net::minecraft::world::level::block_getter::BlockGetter as _;
use mcrust::net::minecraft::world::level::entity_getter::EntityGetter as _;

/// java: private static final float PREVIEW_ALPHA = 0.6f;
pub const PREVIEW_ALPHA: f32 = 0.6f32;
/// java: private static final float PREVIEW_TINT = 0.4f;
pub const PREVIEW_TINT: f32 = 0.4f32;
/// java: private static final long BLOCK_CHANGE_RATE_LIMIT_MS = 150;
pub const BLOCK_CHANGE_RATE_LIMIT_MS: i64 = ((150) as i64) as i64;
/// java: private static int[][] greenhouse = new int[10][10];
pub static GREENHOUSE: LazyLock<Mutex<Vec<Vec<i32>>>> = LazyLock::new(|| Mutex::new(vec![vec![0; (10) as usize]; (10) as usize]));
/// java: private static int[][] targetGreenhouse = new int[10][10];
pub static TARGET_GREENHOUSE: LazyLock<Mutex<Vec<Vec<i32>>>> = LazyLock::new(|| Mutex::new(vec![vec![0; (10) as usize]; (10) as usize]));
/// java: private static @Nullable BlockPos greenhouseCorner = null;
pub static GREENHOUSE_CORNER: compat::MainThread<Option<BlockPos>> = compat::MainThread::new(|| None);
/// java: private static long lastBlockChangeTimeMs;
pub static LAST_BLOCK_CHANGE_TIME_MS: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(0));
/// java: public static final JsonData<Object2ObjectMap<String, String>> PRESETS_DATA = new JsonData<>(SkyblockerMod.CONFIG_DIR.resolve("greenhouse_presets.json"), CodecUtils.object2ObjectMapCodec(Codec.STRING,
pub static PRESETS_DATA: compat::MainThread<compat::Shared<dyn JsonData<std::collections::HashMap<String, String>>>> = compat::MainThread::new(|| (compat::shared(JsonDataBase::new(skyblocker_mod::CONFIG_DIR.get_mut().join("greenhouse_presets.json"), CodecUtils::object2_object_map_codec(&mcrust::com::mojang::serialization::codec::string_codec(), &mcrust::com::mojang::serialization::codec::string_codec()), std::collections::HashMap::new())) as compat::Shared<dyn JsonData<compat::Object>>));
/// java: private static final Set<String> IGNORE_NAMES = Set.of( "PlantboyRoots", "godseedPillar" );
pub static IGNORE_NAMES: LazyLock<std::collections::HashSet<String>> = LazyLock::new(|| std::collections::HashSet::from(["PlantboyRoots".to_string(), "godseedPillar".to_string()]));

/// java: @SuppressWarnings("SameReturnValue") public class GreenhousePaste
#[derive(Clone)]
pub struct GreenhousePaste {}

impl GreenhousePaste {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // TODO(fabric2seer): no SeerClient event yet: hook RETURN of Minecraft's constructor (`net.minecraft.client.Minecraft#<init>`: exists)
        //   java: ClientLifecycleEvents.CLIENT_STARTED.register(_ -> PRESETS_DATA.init());
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command builder outside dispatcher.register(..)
                //   java: LiteralArgumentBuilder<FabricClientCommandSource> greenhouseCommands = literal("greenhouse")
                //   java: 					.then(literal("paste").executes(_ -> runGreenhousePaste()).then(
                //   java: 							argument("preset", StringArgumentType.greedyString())
                //   java: 									.suggests((_, builder) -> SharedSuggestionProvider.suggest(PRESETS_DATA.getData().keySet(), builder))
                //   java: 									.executes(context -> runGreenhousePaste(StringArgumentType.getString(context, "preset")))
                //   java: 					))
                //   java: 					.then(literal("endPaste").executes(_ -> runGreenhousePasteRemove()))
                //   java: 					.then(literal("presets").executes(Scheduler.queueOpenScreenCommand(GreenhousePresetsScreen::new)))
                //   java: 					.then(literal("rotate")
                //   java: 							.then(literal("right").executes(_ -> runRotateRight()))
                //   java: 							.then(literal("left").executes(_ -> runRotateLeft())))
                //   java: 					.then(literal("mirror").executes(_ -> runMirror()));
                let mut greenhouse_commands: LiteralArgumentBuilder<crate::unresolved::FabricClientCommandSource> = todo!("fabric2seer: the initialiser of `greenhouseCommands` (TODO above)");
                if crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled() {
                    // TODO(fabric2seer): a command builder outside dispatcher.register(..)
                    //   java: greenhouseCommands.then(literal("debug").executes(_ -> {
                    //   java: 					debugPrintGreenhouses();
                    //   java: 					return Command.SINGLE_SUCCESS;
                    //   java: 				}));
                }
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE)
                //   java: 					.then(literal("garden")
                //   java: 							.then(greenhouseCommands)));
            },
        ));
        level_render_extraction_callback::EVENT.register(Arc::new(move |collector| {
            if !(Utils::is_in_garden()) {
                return;
            }
            if !(SkyblockerConfigManager::get().farming.greenhouse.enabled) {
                return;
            }
            GreenhousePaste::render_preview(&collector);
        }));
        // TODO(fabric2seer): ClientPlayerBlockBreakEvents.AFTER.register: no SeerClient equivalent yet
        //   java: ClientPlayerBlockBreakEvents.AFTER.register((_, _, pos, _) -> onBlockChange(pos));
        world_events::BLOCK_STATE_UPDATE.register(Arc::new(move |pos, _, _| { GreenhousePaste::on_block_change(&pos); }));
        GreenhousePaste::remove_preview();
    }

    /// java: public static void onEntityChange(Entity e)
    pub fn on_entity_change(e: &dyn Entity) {
        if !(Utils::is_in_garden()) {
            return;
        }
        if !(SkyblockerConfigManager::get().farming.greenhouse.enabled) {
            return;
        }
        if GREENHOUSE_CORNER.get_mut().is_none() {
            return;
        }
        if ((GREENHOUSE_CORNER.get_mut().as_ref().unwrap().get_x()).wrapping_sub(1) as f64) > e.get_x() || ((GREENHOUSE_CORNER.get_mut().as_ref().unwrap().get_x()).wrapping_add(11) as f64) < e.get_x() || ((GREENHOUSE_CORNER.get_mut().as_ref().unwrap().get_z()).wrapping_sub(1) as f64) > e.get_z() || ((GREENHOUSE_CORNER.get_mut().as_ref().unwrap().get_z()).wrapping_add(11) as f64) < e.get_z() {
            return;
        }
        GreenhousePaste::locate_greenhouse();
    }

    /// java: private static void onBlockChange(BlockPos pos)
    pub fn on_block_change(pos: &BlockPos) {
        if !(Utils::is_in_garden()) {
            return;
        }
        if !(SkyblockerConfigManager::get().farming.greenhouse.enabled) {
            return;
        }
        if GREENHOUSE_CORNER.get_mut().is_none() {
            return;
        }
        if (GREENHOUSE_CORNER.get_mut().as_ref().unwrap().get_x()).wrapping_sub(1) >= pos.get_x() || (GREENHOUSE_CORNER.get_mut().as_ref().unwrap().get_x()).wrapping_add(11) <= pos.get_x() || (GREENHOUSE_CORNER.get_mut().as_ref().unwrap().get_z()).wrapping_sub(1) >= pos.get_z() || (GREENHOUSE_CORNER.get_mut().as_ref().unwrap().get_z()).wrapping_add(11) <= pos.get_z() {
            return;
        }
        let mut current_time_ms: i64 = compat::millis();
        if current_time_ms.wrapping_sub(compat::get_state(&LAST_BLOCK_CHANGE_TIME_MS)) < BLOCK_CHANGE_RATE_LIMIT_MS {
            return;
        }
        compat::set_state(&LAST_BLOCK_CHANGE_TIME_MS, current_time_ms);
        GreenhousePaste::locate_greenhouse();
    }

    /// java: private static int runGreenhousePaste()
    pub fn run_greenhouse_paste() -> i32 {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return 1;
        }
        GreenhousePaste::load_from_clipboard();
        return 1;
    }

    /// java: private static int runGreenhousePaste(String preset)
    pub fn run_greenhouse_paste_1(preset: &str) -> i32 {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return 1;
        }
        // TODO(fabric2seer): `.getOrDefault(..)` on a T
        //   java: loadFromLink(PRESETS_DATA.getData().getOrDefault(preset, ""));
        return 1;
    }

    /// java: private static int runGreenhousePasteRemove()
    pub fn run_greenhouse_paste_remove() -> i32 {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return 1;
        }
        GreenhousePaste::remove_preview();
        mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.farming.greenhouse.greenhousePaste.unload").with_style_format(ChatFormatting::Green)).to_string()); });
        return 1;
    }

    /// java: private static int runRotateRight()
    pub fn run_rotate_right() -> i32 {
        GreenhousePaste::rotate_preview(false);
        return 1;
    }

    /// java: private static int runRotateLeft()
    pub fn run_rotate_left() -> i32 {
        GreenhousePaste::rotate_preview(true);
        return 1;
    }

    /// java: private static int runMirror()
    pub fn run_mirror() -> i32 {
        GreenhousePaste::mirror_preview();
        return 1;
    }

    /// java: public static void removePreview()
    pub fn remove_preview() {
        *GREENHOUSE_CORNER.get_mut() = None;
        for x in 0..10 {
            for z in 0..10 {
                (compat::lock(&GREENHOUSE)[(x) as usize])[(z) as usize] = 0;
                (compat::lock(&TARGET_GREENHOUSE)[(x) as usize])[(z) as usize] = -1;
            }
        }
    }

    /// java: public static void loadFromClipboard()
    pub fn load_from_clipboard() {
        // TODO(fabric2seer): `get_clipboard` takes 1 argument(s) in the port (`clipboard: &mut dyn ClipboardAccess`), Java passes 0
        // rust: let mut clipboard: String = compat::minecraft().keyboard_handler.get_clipboard();
        let mut clipboard: String = todo!("fabric2seer: the initialiser of `clipboard` (TODO above)");
        GreenhousePaste::load_from_link(&clipboard);
    }

    /// java: private static void loadFromLink(String clipboard)
    pub fn load_from_link(clipboard: &str) {
        if !(SkyblockerConfigManager::get().farming.greenhouse.enabled) {
            return;
        }
        if !(GreenhousePaste::is_in_greenhouse()) {
            return;
        }
        mcrust::mcrust::modding::game::with_player(|player| { player; });
        let mut encoded: String = GreenhousePaste::extract_layout_code(clipboard);
        if encoded.is_empty() {
            return;
        }
        let mut success: bool = match GreenhousePaste::site_of(clipboard) {
            LayoutSite::SkyShards => GreenhousePaste::import_sky_shards_greenhouse(&encoded),
            LayoutSite::SkyMutations => GreenhousePaste::import_greenhouse(&encoded),
            LayoutSite::Unknown => (GreenhousePaste::import_sky_shards_greenhouse(&encoded) || GreenhousePaste::import_greenhouse(&encoded)),
            _ => unreachable!("switch expression: no case matched"),
        };
        if !success {
            mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.farming.greenhouse.greenhousePaste.loadFail").with_style_format(ChatFormatting::Red)).append(component::literal(&encoded).with_style_format(ChatFormatting::Gray)).to_string()); });
            return;
        }
        mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.farming.greenhouse.greenhousePaste.loadSuccess").with_style_format(ChatFormatting::Green)).to_string()); });
        GreenhousePaste::locate_greenhouse();
    }

    /// java: static String extractLayoutCode(String clipboard)
    pub fn extract_layout_code(clipboard: &str) -> String {
        let mut trimmed: String = clipboard.trim().to_string();
        let mut layout_index: i32 = compat::index_of(&trimmed, "layout=");
        if layout_index >= 0 {
            return GreenhousePaste::end_of_code(&compat::substring(&trimmed, layout_index.wrapping_add(compat::str_len("layout="))));
        }
        let mut share_index: i32 = compat::index_of(&trimmed, "/share/");
        if share_index >= 0 {
            return GreenhousePaste::end_of_code(&compat::substring(&trimmed, share_index.wrapping_add(compat::str_len("/share/"))));
        }
        return trimmed;
    }

    /// java: private static String endOfCode(String code)
    pub fn end_of_code(code: &str) -> String {
        for i in 0..compat::str_len(&code) {
            let mut c: char = compat::char_at(&code, i);
            let mut valid: bool = (c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c >= '0' && c <= '9' || c == '-' || c == '_' || c == '=' || c == '+');
            if !valid {
                return compat::substring2(&code, 0, i);
            }
        }
        return code.to_string();
    }

    /// java: private static LayoutSite siteOf(String clipboard)
    pub fn site_of(clipboard: &str) -> LayoutSite {
        let mut host: String = clipboard.trim().to_string().to_lowercase();
        let mut scheme_end: i32 = compat::index_of(&host, "://");
        if scheme_end >= 0 {
            host = compat::substring(&host, scheme_end.wrapping_add(compat::str_len("://")));
        }
        for i in 0..compat::str_len(&host) {
            let mut c: char = compat::char_at(&host, i);
            if c == '/' || c == '?' || c == '#' {
                host = compat::substring2(&host, 0, i);
                break;
            }
        }
        if host.contains("skyshards") {
            return LayoutSite::SkyShards;
        }
        if host.contains("skymutations") {
            return LayoutSite::SkyMutations;
        }
        return LayoutSite::Unknown;
    }

    /// java: private static boolean importSkyShardsGreenhouse(String encoded)
    pub fn import_sky_shards_greenhouse(encoded: &str) -> bool {
        // TODO(fabric2seer): `SkyShardsLayout.decode` is not a static method of SkyShardsLayout
        //   java: int[][] layout = SkyShardsLayout.decode(encoded);
        let mut layout: Vec<Vec<i32>> = todo!("fabric2seer: the initialiser of `layout` (TODO above)");
        if false /* java: layout == null: never null here */ {
            return false;
        }
        compat::set_state(&TARGET_GREENHOUSE, layout.clone());
        return true;
    }

    /// java: public static boolean isInGreenhouse()
    pub fn is_in_greenhouse() -> bool {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::level().is_none() {
            return false;
        }
        let mut player_pos: BlockPos = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.block_position()).unwrap();
        let mut plot_pos: BlockPos = player_pos.offset(240, 0, 240);
        plot_pos = BlockPos::new((((((plot_pos.get_x() as f64) / 96.0)) as i32).wrapping_mul(96)).wrapping_sub(240), 60, (((((plot_pos.get_z() as f64) / 96.0)) as i32).wrapping_mul(96)).wrapping_sub(240));
        let mut plot_other_corner: BlockPos = BlockPos::new(plot_pos.get_x().wrapping_add(96), 90, plot_pos.get_z().wrapping_add(96));
        let mut detection_box: AABB = AABB::new((plot_pos.get_x()) as f64, (plot_pos.get_y()) as f64, (plot_pos.get_z()) as f64, (plot_other_corner.get_x()) as f64, (plot_other_corner.get_y()) as f64, (plot_other_corner.get_z()) as f64);
        // TODO(fabric2seer): `get_entities` takes 3 argument(s) in the port (`except: Option<i32>, bb: &crate::net::minecraft::world::phys::aabb::AABB, selector: &dyn Fn( &(dyn crate::net::minecraft::world::entity::entity::Entity + Send + Sync), ) -> bool`), Java passes 2
        for entity in mcrust::mcrust::modding::game::level().unwrap().get_entities(None, &detection_box) {
            let mut custom: Option<MutableComponent> = entity.get_custom_name();
            if custom.is_some() && custom.as_ref().unwrap().get_string().contains("Carpenter") {
                return true;
            }
        }
        mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.farming.greenhouse.greenhousePaste.notInGreenhouse").with_style_format(ChatFormatting::Red)).to_string()); });
        return false;
    }

    /// java: public static void locateGreenhouse()
    pub fn locate_greenhouse() {
        mcrust::mcrust::modding::game::with_player(|player| { player; });
        mcrust::mcrust::modding::game::level();
        let mut player_pos: BlockPos = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.block_position()).unwrap();
        let mut plot_pos: BlockPos = player_pos.offset(240, 0, 240);
        plot_pos = BlockPos::new((((((plot_pos.get_x() as f64) / 96.0)) as i32).wrapping_mul(96)).wrapping_sub(240), 73, (((((plot_pos.get_z() as f64) / 96.0)) as i32).wrapping_mul(96)).wrapping_sub(240));
        *GREENHOUSE_CORNER.get_mut() = Some(BlockPos::new(plot_pos.get_x().wrapping_add(43), 73, plot_pos.get_z().wrapping_add(43)));
        for x in 0..10 {
            for z in 0..10 {
                let mut pos: BlockPos = BlockPos::new((GREENHOUSE_CORNER.get_mut().as_ref().unwrap().get_x()).wrapping_add(x), 73, (GREENHOUSE_CORNER.get_mut().as_ref().unwrap().get_z()).wrapping_add(z));
                let mut crop_id: i32 = GreenhousePaste::get_crop_id_at_position(&mcrust::mcrust::modding::game::level().unwrap(), &pos);
                (compat::lock(&GREENHOUSE)[(x) as usize])[(z) as usize] = crop_id;
            }
        }
        GreenhousePaste::adjust_for_special_crops();
    }

    /// java: private static int getCropIdAtPosition(Level level, BlockPos pos)
    pub fn get_crop_id_at_position(level: &Level, pos: &BlockPos) -> i32 {
        let mut detection_box: AABB = AABB::new((pos.get_x()) as f64, (pos.get_y()) as f64, (pos.get_z()) as f64, (pos.get_x().wrapping_add(1)) as f64, (pos.get_y().wrapping_add(6)) as f64, (pos.get_z().wrapping_add(1)) as f64);
        for entity in level.get_entities_no_spectators(None, &detection_box) {
            let Some(armor_stand) = entity.as_any().unwrap_or(&() as &dyn std::any::Any).downcast_ref::<ArmorStand>() else {
                continue;
            };
            let head = armor_stand.get_item_by_slot(EquipmentSlot::Head);
            if head.is_empty() {
                continue;
            }
            let mut name_component: MutableComponent = compat::component_of(head.get_hover_name());
            let mut name: String = name_component.get_string();
            if IGNORE_NAMES.contains(&name) {
                return 0;
            }
            for crop in greenhouse_crops::CROP_ID_MAP.get_mut().values().cloned() {
                if name.contains(&crop.armor_stand_name()) {
                    return crop.id();
                }
            }
            let mut texture: Option<String> = ItemUtils::get_head_texture_optional(head);
            if texture.is_none() {
                continue;
            }
            let Some(mut crop_by_texture) = greenhouse_crops::CROP_BY_HEAD_TEXTURE_HASH.get_mut().get(&texture.clone().unwrap()).cloned() else {
                continue;
            };
            if !(head_textures::SPECIAL_CROPS.contains(crop_by_texture.head_skin().as_ref().unwrap())) {
                continue;
            }
            return crop_by_texture.id();
        }
        let mut state: Arc<BlockState> = level.get_block_state(&pos.above());
        let mut block: Arc<Block> = state.get_block();
        for crop in greenhouse_crops::CROP_ID_MAP.get_mut().values().cloned() {
            if !crop.is_head() && std::ptr::addr_eq(std::sync::Arc::as_ptr(&crop.crop_block()), std::sync::Arc::as_ptr(&block)) {
                return crop.id();
            }
        }
        return 0;
    }

    /// java: private static void adjustForSpecialCrops()
    pub fn adjust_for_special_crops() {
        for x in 0..9 {
            for z in 0..9 {
                GreenhousePaste::adjust_for_plant_boy(x, z);
                GreenhousePaste::adjust_for_snoozling(x, z);
                GreenhousePaste::adjust_for_godseed(x, z);
            }
        }
    }

    /// java: private static void adjustForPlantBoy(int x, int z)
    pub fn adjust_for_plant_boy(mut x: i32, mut z: i32) {
        GREENHOUSE_CORNER.get_mut();
        mcrust::mcrust::modding::game::level();
        if (compat::lock(&GREENHOUSE)[(x) as usize])[(z) as usize] != 26 {
            return;
        }
        let mut pos: BlockPos = GREENHOUSE_CORNER.get_mut().as_ref().unwrap().offset(x, 0, z);
        let mut detection_box: AABB = AABB::new((pos.get_x()) as f64, (pos.get_y()) as f64, (pos.get_z()) as f64, (pos.get_x().wrapping_add(1)) as f64, (pos.get_y().wrapping_add(6)) as f64, (pos.get_z().wrapping_add(1)) as f64);
        let mut found_armor_stand: Option<ArmorStand> = None;
        // TODO(fabric2seer): `get_entities` takes 3 argument(s) in the port (`except: Option<i32>, bb: &crate::net::minecraft::world::phys::aabb::AABB, selector: &dyn Fn( &(dyn crate::net::minecraft::world::entity::entity::Entity + Send + Sync), ) -> bool`), Java passes 2
        for entity in mcrust::mcrust::modding::game::level().unwrap().get_entities(None, &detection_box) {
            let Some(armor_stand) = entity.as_any().unwrap_or(&() as &dyn std::any::Any).downcast_ref::<ArmorStand>() else {
                continue;
            };
            let head = armor_stand.get_item_by_slot(EquipmentSlot::Head);
            if head.is_empty() {
                continue;
            }
            let mut name_component: MutableComponent = compat::component_of(head.get_hover_name());
            let mut name: String = name_component.get_string();
            if name.contains("Plantboy") && !(name.contains("Advance")) {
                // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
                // rust: found_armor_stand = Some(armor_stand.clone());
                break;
            }
        }
        if let Some(found_armor_stand) = &found_armor_stand {
        } else {
            return;
        }
        found_armor_stand.as_ref().unwrap().base.base.position().add(-0.2, (0) as f64, -0.2);
        let mut bottom_left: BlockPos = BlockPos::new((((found_armor_stand.as_ref().unwrap().base.base.get_x()).floor()) as i32), (((found_armor_stand.as_ref().unwrap().base.base.get_y()).floor()) as i32), (((found_armor_stand.as_ref().unwrap().base.base.get_z()).floor()) as i32));
        (compat::lock(&GREENHOUSE)[(bottom_left.get_x().wrapping_sub(GREENHOUSE_CORNER.get_mut().as_ref().unwrap().get_x())) as usize])[(bottom_left.get_z().wrapping_sub(GREENHOUSE_CORNER.get_mut().as_ref().unwrap().get_z())) as usize] = 26;
        (compat::lock(&GREENHOUSE)[((bottom_left.get_x().wrapping_sub(GREENHOUSE_CORNER.get_mut().as_ref().unwrap().get_x())).wrapping_add(1)) as usize])[(bottom_left.get_z().wrapping_sub(GREENHOUSE_CORNER.get_mut().as_ref().unwrap().get_z())) as usize] = 26;
        (compat::lock(&GREENHOUSE)[(bottom_left.get_x().wrapping_sub(GREENHOUSE_CORNER.get_mut().as_ref().unwrap().get_x())) as usize])[((bottom_left.get_z().wrapping_sub(GREENHOUSE_CORNER.get_mut().as_ref().unwrap().get_z())).wrapping_add(1)) as usize] = 26;
        (compat::lock(&GREENHOUSE)[((bottom_left.get_x().wrapping_sub(GREENHOUSE_CORNER.get_mut().as_ref().unwrap().get_x())).wrapping_add(1)) as usize])[((bottom_left.get_z().wrapping_sub(GREENHOUSE_CORNER.get_mut().as_ref().unwrap().get_z())).wrapping_add(1)) as usize] = 26;
    }

    /// java: private static void adjustForSnoozling(int x, int z)
    pub fn adjust_for_snoozling(mut x: i32, mut z: i32) {
        if (compat::lock(&GREENHOUSE)[(x) as usize])[(z) as usize] != 23 {
            return;
        }
        // java: try {..} catch (ArrayIndexOutOfBoundsException _) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            (compat::lock(&GREENHOUSE)[(x.wrapping_sub(1)) as usize])[(z) as usize] = 23;
            (compat::lock(&GREENHOUSE)[(x.wrapping_add(1)) as usize])[(z) as usize] = 23;
            (compat::lock(&GREENHOUSE)[(x.wrapping_sub(1)) as usize])[(z.wrapping_sub(1)) as usize] = 23;
            (compat::lock(&GREENHOUSE)[(x) as usize])[(z.wrapping_sub(1)) as usize] = 23;
            (compat::lock(&GREENHOUSE)[(x.wrapping_add(1)) as usize])[(z.wrapping_sub(1)) as usize] = 23;
            (compat::lock(&GREENHOUSE)[(x.wrapping_sub(1)) as usize])[(z.wrapping_sub(2)) as usize] = 23;
            (compat::lock(&GREENHOUSE)[(x) as usize])[(z.wrapping_sub(2)) as usize] = 23;
            (compat::lock(&GREENHOUSE)[(x.wrapping_add(1)) as usize])[(z.wrapping_sub(2)) as usize] = 23;
        }));
        if let Err(_) = __result {
        }
    }

    /// java: private static void adjustForGodseed(int x, int z)
    pub fn adjust_for_godseed(mut x: i32, mut z: i32) {
        if (compat::lock(&GREENHOUSE)[(x) as usize])[(z) as usize] != 37 {
            return;
        }
        // java: try {..} catch (ArrayIndexOutOfBoundsException _) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            (compat::lock(&GREENHOUSE)[(x.wrapping_sub(1)) as usize])[(z) as usize] = 37;
            (compat::lock(&GREENHOUSE)[(x.wrapping_add(1)) as usize])[(z) as usize] = 37;
            (compat::lock(&GREENHOUSE)[(x.wrapping_sub(1)) as usize])[(z.wrapping_sub(1)) as usize] = 37;
            (compat::lock(&GREENHOUSE)[(x) as usize])[(z.wrapping_sub(1)) as usize] = 37;
            (compat::lock(&GREENHOUSE)[(x.wrapping_add(1)) as usize])[(z.wrapping_sub(1)) as usize] = 37;
            (compat::lock(&GREENHOUSE)[(x.wrapping_sub(1)) as usize])[(z.wrapping_add(1)) as usize] = 37;
            (compat::lock(&GREENHOUSE)[(x) as usize])[(z.wrapping_add(1)) as usize] = 37;
            (compat::lock(&GREENHOUSE)[(x.wrapping_add(1)) as usize])[(z.wrapping_add(1)) as usize] = 37;
        }));
        if let Err(_) = __result {
        }
    }

    /// java: public static boolean importGreenhouse(String encoded)
    pub fn import_greenhouse(encoded: &str) -> bool {
        // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut json_string: String = LZString::decompress_from_encoded_uri_component(encoded).unwrap();
            let mut json_array: Vec<serde_json::Value> = compat::json::get_as_json_array(&(serde_json::from_str::<serde_json::Value>(&json_string).expect("JsonSyntaxException")));
            for x in 0..10 {
                for z in 0..10 {
                    (compat::lock(&TARGET_GREENHOUSE)[(x) as usize])[(z) as usize] = -1;
                }
            }
            for element in json_array.iter() {
                let mut entry: Vec<serde_json::Value> = compat::json::get_as_json_array(&element);
                let mut x: i32 = compat::json::get_as_int(&(entry[(0) as usize]));
                let mut z: i32 = compat::json::get_as_int(&(entry[(1) as usize]));
                let mut crop_name: String = compat::json::get_as_string(&(entry[(2) as usize]));
                let mut value: i32 = compat::json::get_as_int(&(entry[(3) as usize]));
                if value == 0 {
                    (compat::lock(&TARGET_GREENHOUSE)[(9i32.wrapping_sub(x)) as usize])[(z) as usize] = 0;
                    continue;
                }
                let Some(mut crop) = greenhouse_crops::CROP_ID_MAP.get_mut().get(&crop_name).cloned() else {
                    continue;
                };
                if x >= 0 && x < 10 && z >= 0 && z < 10 {
                    (compat::lock(&TARGET_GREENHOUSE)[(9i32.wrapping_sub(x)) as usize])[(z) as usize] = crop.id();
                }
            }
        }));
        if let Err(_) = __result {
            return false;
        }
        return true;
    }

    /// java: public static void renderPreview(PrimitiveCollector collector)
    pub fn render_preview(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(SkyblockerConfigManager::get().farming.greenhouse.enabled) {
            return;
        }
        if GREENHOUSE_CORNER.get_mut().is_none() || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        if mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_x()).unwrap() < ((GREENHOUSE_CORNER.get_mut().as_ref().unwrap().get_x()).wrapping_sub(43) as f64) || mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_x()).unwrap() > ((GREENHOUSE_CORNER.get_mut().as_ref().unwrap().get_x()).wrapping_add(53) as f64) || mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_z()).unwrap() < ((GREENHOUSE_CORNER.get_mut().as_ref().unwrap().get_z()).wrapping_sub(43) as f64) || mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_z()).unwrap() > ((GREENHOUSE_CORNER.get_mut().as_ref().unwrap().get_z()).wrapping_add(53) as f64) {
            return;
        }
        for x in 0..10 {
            for z in 0..10 {
                let mut target_crop_id: i32 = (compat::lock(&TARGET_GREENHOUSE)[(x) as usize])[(z) as usize];
                if target_crop_id == -1 {
                    continue;
                }
                let mut current_crop_id: i32 = (compat::lock(&GREENHOUSE)[(x) as usize])[(z) as usize];
                let mut pos: BlockPos = BlockPos::new((GREENHOUSE_CORNER.get_mut().as_ref().unwrap().get_x()).wrapping_add(x), (GREENHOUSE_CORNER.get_mut().as_ref().unwrap().get_y()).wrapping_add(1), (GREENHOUSE_CORNER.get_mut().as_ref().unwrap().get_z()).wrapping_add(z));
                if target_crop_id == 0 {
                    if current_crop_id == 0 {
                        if SkyblockerConfigManager::get().farming.greenhouse.show_mutation_slot {
                            ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box_5()); __f(collector, &AABB::new((pos.get_x()) as f64, ((pos.get_y() as f64) - 0.05), (pos.get_z()) as f64, (pos.get_x().wrapping_add(1)) as f64, ((pos.get_y() as f64) + 0.1), (pos.get_z().wrapping_add(1)) as f64), &vec![0.0f32, 1.0f32, 0.0f32], 0.2f32, 3.0f32, false) });
                        }
                    } else {
                        ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box_5()); __f(collector, &AABB::of_block_pos(&pos), &vec![1.0f32, 0.0f32, 0.0f32], 0.5f32, 4.0f32, true) });
                    }
                    continue;
                }
                if current_crop_id == target_crop_id {
                    continue;
                }
                if current_crop_id == 49 || current_crop_id == 48 {
                    if target_crop_id == 48 || target_crop_id == 49 {
                        continue;
                    }
                }
                let Some(mut target_crop) = greenhouse_crops::CROP_BY_INT.get_mut().get(&target_crop_id).cloned() else {
                    continue;
                };
                // TODO(fabric2seer): `Minecraft.getEntityModels` has no Rust method in the port (looked for `get_entity_models` in Minecraft and its supertypes)
                if current_crop_id != 0 {
                    ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box_5()); __f(collector, &AABB::of_block_pos(&pos), &vec![1.0f32, 0.0f32, 0.0f32], 0.5f32, 4.0f32, true) });
                } else if target_crop.is_head() {
                    let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = FlexibleItemStack::get_stack_or_empty(&target_crop.display_stack());
                    let Some(mut profile) = stack.get(mcrust::net::minecraft::core::component::data_components::profile()) else {
                        continue;
                    };
                    // TODO(fabric2seer): field `PLAYER` of a value of unknown type
                    //   java: SkullModelBase model = SkullBlockRenderer.createModel(
                    //   java: 							CLIENT.getEntityModels(),
                    //   java: 							SkullBlock.Types.PLAYER
                    //   java: 					);
                    let mut model: SkullModelBase = todo!("fabric2seer: the initialiser of `model` (TODO above)");
                    if false /* java: model == null: never null here */ {
                        continue;
                    }
                    // TODO(fabric2seer): `Minecraft.playerSkinRenderCache` has no Rust method in the port (looked for `player_skin_render_cache` in Minecraft and its supertypes)
                    // rust: let mut render_type: RenderType = compat::minecraft().player_skin_render_cache().get_or_default(&profile).render_type();
                    let mut render_type: RenderType = todo!("fabric2seer: the initialiser of `renderType` (TODO above)");
                    { let (__a0, __a1,) = (None /* java: null */, &Arc::new(move |_, world_state, submit_node_collector| {
                        let mut matrices: PoseStack = PoseStack::new();
                        // TODO(fabric2seer): field `cameraRenderState` of a value of unknown type
                        //   java: matrices.translate(
                        //   java: 										pos.getX() - worldState.cameraRenderState.pos.x,
                        //   java: 										pos.getY() - worldState.cameraRenderState.pos.y,
                        //   java: 										pos.getZ() - worldState.cameraRenderState.pos.z
                        //   java: 								);
                        matrices.mul_pose_transformation(mcrust::net::minecraft::client::renderer::blockentity::skull_block_renderer::transformations().free_transformations(0));
                        let mut skull_state: State = State::new();
                        skull_state.animation_pos = 0.0f32;
                        model.setup_anim(skull_state.clone());
                        // TODO(fabric2seer): `.submitModel(..)` on a value of unknown type
                        //   java: submitNodeCollector.submitModel(
                        //   java: 										model,
                        //   java: 										skullState,
                        //   java: 										matrices,
                        //   java: 										renderType,
                        //   java: 										LightCoordsUtil.FULL_BRIGHT,
                        //   java: 										OverlayTexture.pack(PREVIEW_TINT, false),
                        //   java: 										0,
                        //   java: 										null
                        //   java: 								);
                    }),); collector.lock().borrow_mut().submit_vanilla(__a0, __a1) };
                } else {
                    let mut block_state: Arc<BlockState> = target_crop.crop_block().default_block_state();
                    if target_crop.crop_block().as_any().is::<Block>() {
                        // TODO(fabric2seer): static `CropBlock.AGE` has no Rust counterpart in the port's CropBlock file
                        // TODO(fabric2seer): `BlockState.setValue` has no Rust method in the port (looked for `set_value` in BlockState and its supertypes)
                        // rust: block_state = target_crop.crop_block().default_block_state().set_value(&Block::AGE, 7);
                    }
                    ({ let __f = compat::dispatch(collector, |__t| __t.submit_block_hologram()); __f(collector, &pos, &block_state, PREVIEW_ALPHA) });
                }
            }
        }
    }

    /// java: public static void rotatePreview(boolean left)
    pub fn rotate_preview(mut left: bool) {
        if left {
            let mut new_target: Vec<Vec<i32>> = vec![vec![0; (10) as usize]; (10) as usize];
            for x in 0..10 {
                for z in 0..10 {
                    (new_target[(z) as usize])[(9i32.wrapping_sub(x)) as usize] = (compat::lock(&TARGET_GREENHOUSE)[(x) as usize])[(z) as usize];
                }
            }
            compat::set_state(&TARGET_GREENHOUSE, new_target.clone());
            return;
        }
        let mut new_target: Vec<Vec<i32>> = vec![vec![0; (10) as usize]; (10) as usize];
        for x in 0..10 {
            for z in 0..10 {
                (new_target[(9i32.wrapping_sub(z)) as usize])[(x) as usize] = (compat::lock(&TARGET_GREENHOUSE)[(x) as usize])[(z) as usize];
            }
        }
        compat::set_state(&TARGET_GREENHOUSE, new_target.clone());
    }

    /// java: public static void mirrorPreview()
    pub fn mirror_preview() {
        let mut new_target: Vec<Vec<i32>> = vec![vec![0; (10) as usize]; (10) as usize];
        for x in 0..10 {
            for z in 0..10 {
                (new_target[(9i32.wrapping_sub(x)) as usize])[(z) as usize] = (compat::lock(&TARGET_GREENHOUSE)[(x) as usize])[(z) as usize];
            }
        }
        compat::set_state(&TARGET_GREENHOUSE, new_target.clone());
    }

    /// java: private static void debugPrintGreenhouses()
    pub fn debug_print_greenhouses() {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(component::literal("=== CURRENT GREENHOUSE ===")).with_style_format(ChatFormatting::Yellow).to_string()); });
        GreenhousePaste::print_grid(&compat::get_state(&GREENHOUSE));
        mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(component::literal("=== TARGET GREENHOUSE ===")).with_style_format(ChatFormatting::Aqua).to_string()); });
        GreenhousePaste::print_grid(&compat::get_state(&TARGET_GREENHOUSE));
    }

    /// java: private static void printGrid(int[][] grid)
    pub fn print_grid(grid: &Vec<Vec<i32>>) {
        mcrust::mcrust::modding::game::with_player(|player| { player; });
        let mut all: String = String::new();
        for z in 0..10 {
            let mut row: String = String::new();
            for x in 0..10 {
                let mut val: i32 = (grid[(x) as usize])[(z) as usize];
                if val == -1 {
                    { row.push_str(" - "); };
                } else {
                    { row.push_str(&format!("{:2} ", val)); };
                }
            }
            { all.push_str(&row); all.push_str("\n"); };
        }
        mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&all.clone()); });
    }

}

/// java: private enum LayoutSite
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LayoutSite {
    SkyShards,
    SkyMutations,
    Unknown,
}

impl LayoutSite {
    pub const VALUES: [LayoutSite; 3] = [LayoutSite::SkyShards, LayoutSite::SkyMutations, LayoutSite::Unknown];

    /// java: values()
    pub fn values() -> Vec<LayoutSite> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            LayoutSite::SkyShards => "SKY_SHARDS",
            LayoutSite::SkyMutations => "SKY_MUTATIONS",
            LayoutSite::Unknown => "UNKNOWN",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> LayoutSite {
        match name {
            "SKY_SHARDS" => LayoutSite::SkyShards,
            "SKY_MUTATIONS" => LayoutSite::SkyMutations,
            "UNKNOWN" => LayoutSite::Unknown,
            _ => panic!("IllegalArgumentException: No enum constant LayoutSite.{name}"),
        }
    }

}

impl std::fmt::Display for LayoutSite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
