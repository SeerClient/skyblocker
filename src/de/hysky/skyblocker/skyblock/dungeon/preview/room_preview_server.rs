//! Ported from de.hysky.skyblocker.skyblock.dungeon.preview.RoomPreviewServer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::preview::room_preview::RoomPreview;
use crate::de::hysky::skyblocker::skyblock::dungeon::preview::room_structure::RoomStructure;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use mcrust::java::util::uuid::Uuid;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::screens::title_screen::TitleScreen;
use mcrust::net::minecraft::client::minecraft;
use mcrust::net::minecraft::nbt::compound_tag;
use mcrust::net::minecraft::nbt::tag::Tag;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::resources::resource_key::ResourceKey;
use mcrust::net::minecraft::world::difficulty::Difficulty;
use mcrust::net::minecraft::world::effect::mob_effect_instance::MobEffectInstance;
use mcrust::net::minecraft::world::level::border::world_border::WorldBorder;
use mcrust::net::minecraft::world::level::game_type::GameType;
use mcrust::net::minecraft::world::level::gamerules::game_rules::GameRules;
use mcrust::net::minecraft::world::level::level_settings::DifficultySettings;
use mcrust::net::minecraft::world::level::level_settings::LevelSettings;
use mcrust::net::minecraft::world::level::levelgen::flat::flat_level_generator_settings::FlatLevelGeneratorSettings;
use mcrust::net::minecraft::world::level::levelgen::flat_level_source::FlatLevelSource;
use mcrust::net::minecraft::world::level::levelgen::presets::world_presets;
use mcrust::net::minecraft::world::level::levelgen::world_options::WorldOptions;
use mcrust::net::minecraft::world::level::world_data_configuration::WorldDataConfiguration;
use mcrust::net::minecraft::core::holder_getter::HolderGetter as _;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::util::thread::blockable_event_loop::BlockableEventLoop as _;
use mcrust::net::minecraft::world::level::collision_getter::CollisionGetter as _;
use mcrust::net::minecraft::world::level::level_accessor::LevelAccessor as _;

/// java: private static final String SAVE_NAME = "skyblocker-room-preview";
pub const SAVE_NAME: &str = "skyblocker-room-preview";
/// java: public static boolean isActive = false;
pub static IS_ACTIVE: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: static String selectedRoom = "";
pub static SELECTED_ROOM: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("".to_string()));
/// java: static List<Component> errorMessages = new ArrayList<>();
pub static ERROR_MESSAGES: compat::MainThread<Vec<MutableComponent>> = compat::MainThread::new(|| Vec::new());

/// java: public class RoomPreviewServer
#[derive(Clone)]
pub struct RoomPreviewServer {}

impl RoomPreviewServer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // GAP (verified, RustSrc NEXT_STEPS 000rr): this port has no integrated server, so there is no
        // ServerPlayerEvents / ServerLifecycleEvents to register against.
        // NEEDS-GAME-API: ServerPlayerEvents.JOIN -> RoomPreviewServer::on_player_join
        // NEEDS-GAME-API: ServerPlayerEvents.AFTER_RESPAWN -> RoomPreviewServer::apply_night_vision(newPlayer)
        // NEEDS-GAME-API: ServerLifecycleEvents.SERVER_STARTED -> RoomPreviewServer::check_server
        // NEEDS-GAME-API: ServerLifecycleEvents.SERVER_STOPPING -> RoomPreviewServer::reset
    }

    /// java: public static void onPlayerJoin(ServerPlayer player)
    pub fn on_player_join(player: &crate::unresolved::ServerPlayer) {
        if !(compat::get_state(&IS_ACTIVE)) {
            return;
        }
        RoomPreviewServer::apply_night_vision(player);
        script_host::send_feedback(&(constants::PREFIX.get_mut())().append_str(&(component::translatable_args("skyblocker.dungeons.roomPreview.joinMessage", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(compat::get_state(&SELECTED_ROOM))])).get_string()).to_string());
        // GAP (verified, RustSrc NEXT_STEPS 000rr): `ServerPlayer` is a server-side class and this port has no
        // integrated server, so no ServerPlayer ever reaches here. The Java tail:
        //   java: if (player.getUUID().equals(CLIENT.getUser().getProfileId())) {
        //   java:     CLIENT.execute(() -> Scheduler.INSTANCE.schedule(RoomPreview::onJoin, 5));
        //   java:     for (Component msg : errorMessages) player.sendSystemMessage(msg);
        //   java:     errorMessages.clear();
        //   java: }
        // NEEDS-GAME-API: ServerPlayer::get_uuid / send_system_message
    }

    /// java: public static void applyNightVision(ServerPlayer player)
    pub fn apply_night_vision(player: &crate::unresolved::ServerPlayer) {
        if !(compat::get_state(&IS_ACTIVE)) {
            return;
        }
        // GAP (verified, RustSrc NEXT_STEPS 000rr): no integrated server.
        //   java: player.addEffect(new MobEffectInstance(MobEffects.NIGHT_VISION, -1, 1, false, false));
        // NEEDS-GAME-API: ServerPlayer::add_effect
        let _ = player;
    }

    /// java: private static void checkServer(MinecraftServer server)
    pub fn check_server(server: &crate::unresolved::MinecraftServer) {
        if compat::get_state(&IS_ACTIVE) {
            return;
        }
        // GAP (verified, RustSrc NEXT_STEPS 000rr): no integrated server, so no MinecraftServer reaches this.
        //   java: if (!server.getWorldData().getLevelName().equals(SAVE_NAME)) return;
        //   java: CompoundTag previewData = server.getCommandStorage().get(SkyblockerMod.id(SAVE_NAME));
        //   java: if (previewData.isEmpty()) return;
        //   java: isActive = previewData.getBooleanOr("isActive", false);
        //   java: selectedRoom = previewData.getStringOr("selectedRoom", "");
        // NEEDS-GAME-API: MinecraftServer::get_world_data / get_command_storage
        let _ = server;
    }

    /// java: private static void reset()
    pub fn reset() {
        compat::set_state(&IS_ACTIVE, false);
        compat::set_state(&SELECTED_ROOM, "".to_string());
        ERROR_MESSAGES.get_mut().clear();
    }

    /// java: public static void createServer()
    pub fn create_server() {
        let previous_save: std::path::PathBuf = minecraft::get_level_source().get_level_path(SAVE_NAME).clone();
        // java: FileUtils.deleteQuietly(previousSave);
        if previous_save.is_dir() {
            let _ = std::fs::remove_dir_all(&previous_save);
        } else {
            let _ = std::fs::remove_file(&previous_save);
        }
        compat::set_state(&IS_ACTIVE, true);
        // GAP (verified, RustSrc NEXT_STEPS 000rr/000ru): `WorldOpenFlows.createFreshLevel` ends at the port's
        // integrated-server boundary (`world_open_flows::do_world_load_boundary`), so no singleplayer world opens.
        //   java: CLIENT.createWorldOpenFlows().createFreshLevel(SAVE_NAME,
        //   java:     new LevelSettings(SAVE_NAME, GameType.SPECTATOR, new LevelSettings.DifficultySettings(Difficulty.PEACEFUL, false, false), true, WorldDataConfiguration.DEFAULT),
        //   java:     new WorldOptions(SAVE_NAME.hashCode(), false, false),
        //   java:     lookup -> { var preset = WorldPresets.createTestWorldDimensions(lookup);
        //   java:         var config = new FlatLevelGeneratorSettings(Optional.empty(), lookup.lookupOrThrow(Registries.BIOME).getOrThrow(ResourceKey.create(Registries.BIOME, Identifier.withDefaultNamespace("the_void"))), List.of());
        //   java:         return preset.replaceOverworldGenerator(lookup, new FlatLevelSource(config)); },
        //   java:     new TitleScreen());
        // NEEDS-GAME-API: Minecraft::create_world_open_flows().create_fresh_level(..)
        // java: IntegratedServer server = CLIENT.getSingleplayerServer();
        // NEEDS-GAME-API: Minecraft::get_singleplayer_server - the port has no IntegratedServer, so it is always null.
        let server: Option<crate::unresolved::IntegratedServer> = None;
        let Some(_server) = server else {
            RoomPreviewServer::reset();
            return;
        };
        // java: server.getGameRules().set(GameRules.ADVANCE_TIME, false, server);
        // java: server.getGameRules().set(GameRules.RANDOM_TICK_SPEED, 0, server);
        // NEEDS-GAME-API: IntegratedServer::get_game_rules
    }

    /// java: public static void addErrorMessage(Component errorText)
    pub fn add_error_message(error_text: &MutableComponent) {
        ERROR_MESSAGES.get_mut().push((constants::PREFIX.get_mut())().append_str(&error_text.get_string()));
    }

    /// java: public static @Nullable StructureTemplate getStructureTemplate(String type, String roomName)
    pub fn get_structure_template(r#type: &str, room_name: &str) -> Option<crate::unresolved::StructureTemplate> {
        let block_data: Option<Vec<i32>> = DungeonManager::get_room_block_data(r#type, room_name);
        // java: return blockData.map(blocks -> { StructureTemplate template = new StructureTemplate();
        // java:     template.load(BuiltInRegistries.BLOCK, RoomStructure.getCompound(blocks)); return template; }).orElse(null);
        // GAP (verified): RustSrc has no `StructureTemplate` (no structure_template.rs) - a server-side
        // worldgen class only the integrated server (NEXT_STEPS 000rr) would use.
        // NEEDS-GAME-API: StructureTemplate::new / load
        let _ = block_data;
        None
    }

    /// java: public static void loadRoom(String type, String roomName)
    pub fn load_room(r#type: &str, room_name: String) {
        // java: IntegratedServer server = CLIENT.getSingleplayerServer();
        // NEEDS-GAME-API: Minecraft::get_singleplayer_server - no IntegratedServer in the port (NEXT_STEPS 000rr).
        let server: Option<crate::unresolved::IntegratedServer> = None;
        let Some(_server) = server else {
            return;
        };
        compat::set_state(&SELECTED_ROOM, room_name.clone());
        let Some(_template) = RoomPreviewServer::get_structure_template(r#type, &room_name) else {
            RoomPreviewServer::add_error_message(&(component::translatable_args("skyblocker.dungeons.roomPreview.failedToLoad", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.dungeons.roomPreview.invalidRoom"))])).with_style_format(ChatFormatting::Red));
            return;
        };
        // java: server.execute(() -> template.placeInWorld(server.overworld(), BlockPos.ZERO, BlockPos.ZERO, new StructurePlaceSettings(), server.overworld().getRandom(), 0));
        // java: server.execute(() -> { WorldBorder border = server.overworld().getWorldBorder();
        // java:     border.setCenter((template.getSize().getX() + 1) / 2d, (template.getSize().getZ() + 1) / 2d);
        // java:     border.setSize(Math.max(template.getSize().getX(), template.getSize().getZ())); });
        // java: server.execute(() -> { CompoundTag previewData = new CompoundTag(); previewData.putBoolean("isActive", true);
        // java:     previewData.putString("selectedRoom", roomName); server.getCommandStorage().set(SkyblockerMod.id(SAVE_NAME), previewData); });
        // NEEDS-GAME-API: IntegratedServer::execute / overworld / get_command_storage, StructureTemplate::place_in_world
    }
}
