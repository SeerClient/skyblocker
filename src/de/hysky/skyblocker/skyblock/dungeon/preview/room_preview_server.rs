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
        // TODO(fabric2seer): ServerPlayerEvents.JOIN.register: no SeerClient equivalent yet
        //   java: ServerPlayerEvents.JOIN.register(RoomPreviewServer::onPlayerJoin);
        // TODO(fabric2seer): ServerPlayerEvents.AFTER_RESPAWN.register: no SeerClient equivalent yet
        //   java: ServerPlayerEvents.AFTER_RESPAWN.register((_, newP, _) -> applyNightVision(newP));
        // TODO(fabric2seer): ServerLifecycleEvents.SERVER_STARTED.register: no SeerClient equivalent yet
        //   java: ServerLifecycleEvents.SERVER_STARTED.register(RoomPreviewServer::checkServer);
        // TODO(fabric2seer): ServerLifecycleEvents.SERVER_STOPPING.register: no SeerClient equivalent yet
        //   java: ServerLifecycleEvents.SERVER_STOPPING.register(_ -> RoomPreviewServer.reset());
    }

    /// java: public static void onPlayerJoin(ServerPlayer player)
    pub fn on_player_join(player: &crate::unresolved::ServerPlayer) {
        if !(compat::get_state(&IS_ACTIVE)) {
            return;
        }
        RoomPreviewServer::apply_night_vision(player);
        script_host::send_feedback(&(constants::PREFIX.get_mut())().append_str(&(component::translatable_args("skyblocker.dungeons.roomPreview.joinMessage", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(compat::get_state(&SELECTED_ROOM))])).get_string()).to_string());
        // rust: port gap - `ServerPlayer` is a server-side class: this port has no integrated server (MinecraftServer's
        // instance half / IntegratedServer are unported, RustSrc NEXT_STEPS 000rr), so no ServerPlayer ever joins. GAPS_batchB.md.
        // rust: if compat::uuid_of(player.get_uuid()) == compat::minecraft().get_user().get_profile_id() {
        // rust:     crate::compat::execute(move || { ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || { RoomPreview::on_join(); }), 5) }); });
        // rust:     for msg in ERROR_MESSAGES.get_mut().iter() {
        // rust:         script_host::send_feedback(&msg.to_string());
        // rust:     }
        // rust:     ERROR_MESSAGES.get_mut().clear();
        // rust: }
    }

    /// java: public static void applyNightVision(ServerPlayer player)
    pub fn apply_night_vision(player: &crate::unresolved::ServerPlayer) {
        if !(compat::get_state(&IS_ACTIVE)) {
            return;
        }
        // rust: port gap - `ServerPlayer.addEffect`: no integrated server in this port (RustSrc NEXT_STEPS 000rr). GAPS_batchB.md.
        // rust: player.add_effect(MobEffectInstance::with_visibility(*mcrust::net::minecraft::world::effect::mob_effects::night_vision(), -1, 1, false, false));
    }

    /// java: private static void checkServer(MinecraftServer server)
    pub fn check_server(server: &crate::unresolved::MinecraftServer) {
        if compat::get_state(&IS_ACTIVE) {
            return;
        }
        // rust: port gap - `MinecraftServer.getWorldData` / `getCommandStorage`: no integrated server in this port
        // (RustSrc NEXT_STEPS 000rr), so no MinecraftServer reaches this. GAPS_batchB.md. The Java body:
        //   TODO(fabric2seer): `MinecraftServer.getWorldData` has no Rust method in the port (looked for `get_world_data` in MinecraftServer and its supertypes)
        //   TODO(fabric2seer): `WorldData.getLevelName` has no Rust method in the port (looked for `get_level_name` in WorldData and its supertypes)
        // rust: if !(server.get_world_data().get_level_name() == SAVE_NAME) {
        // rust:     return;
        // rust: }
        //   TODO(fabric2seer): `MinecraftServer.getCommandStorage` has no Rust method in the port (looked for `get_command_storage` in MinecraftServer and its supertypes)
        //   TODO(fabric2seer): `CommandStorage.get` has no Rust method in the port (looked for `get` in CommandStorage and its supertypes)
        //   rust: let mut preview_data: Tag = server.get_command_storage().get(&crate::entry::skyblocker_mod::id(SAVE_NAME));
        // rust: let mut preview_data: Tag = todo!("fabric2seer: the initialiser of `previewData` (TODO above)");
        //   TODO(fabric2seer): `is_empty` takes 1 argument(s) in the port (`tags: &BTreeMap<String, Tag>`), Java passes 0
        // rust: if compound_tag::is_empty() {
        // rust:     return;
        // rust: }
        //   TODO(fabric2seer): `MinecraftServer.getCommandStorage` has no Rust method in the port (looked for `get_command_storage` in MinecraftServer and its supertypes)
        //   TODO(fabric2seer): `CommandStorage.get` has no Rust method in the port (looked for `get` in CommandStorage and its supertypes)
        //   rust: compat::set_state(&IS_ACTIVE, server.get_command_storage().get(&crate::entry::skyblocker_mod::id(SAVE_NAME)).get_boolean_or("isActive", false));
        //   TODO(fabric2seer): `MinecraftServer.getCommandStorage` has no Rust method in the port (looked for `get_command_storage` in MinecraftServer and its supertypes)
        //   TODO(fabric2seer): `CommandStorage.get` has no Rust method in the port (looked for `get` in CommandStorage and its supertypes)
        //   rust: compat::set_state(&SELECTED_ROOM, server.get_command_storage().get(&crate::entry::skyblocker_mod::id(SAVE_NAME)).get_string_or("selectedRoom", ""));
    }

    /// java: private static void reset()
    pub fn reset() {
        compat::set_state(&IS_ACTIVE, false);
        compat::set_state(&SELECTED_ROOM, "".to_string());
        ERROR_MESSAGES.get_mut().clear();
    }

    /// java: public static void createServer()
    pub fn create_server() {
        let mut previous_save: std::path::PathBuf = minecraft::get_level_source().get_level_path(SAVE_NAME).clone();
        // TODO(fabric2seer): static call FileUtils.deleteQuietly(..)
        //   java: FileUtils.deleteQuietly(previousSave);
        compat::set_state(&IS_ACTIVE, true);
        // TODO(fabric2seer): `Minecraft.createWorldOpenFlows` has no Rust method in the port (looked for `create_world_open_flows` in Minecraft and its supertypes)
        // TODO(fabric2seer): static `WorldDataConfiguration.DEFAULT` has no Rust counterpart in the port's WorldDataConfiguration file
        // TODO(fabric2seer): `new` takes 2 argument(s) in the port (`fading: bool, logo_renderer: Option< crate::net::minecraft::client::gui::components::logo_renderer::LogoRenderer, >`), Java passes 0
        // TODO(fabric2seer): `WorldOpenFlows.createFreshLevel` has no Rust method in the port (looked for `create_fresh_level` in WorldOpenFlows and its supertypes)
        // rust: compat::minecraft().create_world_open_flows().create_fresh_level(SAVE_NAME, &LevelSettings::new(SAVE_NAME.to_string(), GameType::Spectator, DifficultySettings::new(Difficulty::Peaceful, false, false), true, WorldDataConfiguration::DEFAULT.clone()), &WorldOptions::new((compat::string_hash_code(&SAVE_NAME)) as i64, false, false), &Arc::new(move |lookup| {
        // rust: let mut preset = world_presets::create_test_world_dimensions(&lookup).unwrap();
        // rust: // TODO(fabric2seer): `Provider.lookupOrThrow` has no Rust method in the port (looked for `lookup_or_throw` in HolderLookup and its supertypes)
        // rust: // rust: let mut config = FlatLevelGeneratorSettings::new(None, lookup.lookup_or_throw(&mcrust::net::minecraft::core::registries::registries::biome()).get_or_throw(&ResourceKey::create(&mcrust::net::minecraft::core::registries::registries::biome(), Identifier::with_default_namespace("the_void"))).unwrap(), vec![]);
        // rust: return preset.replace_overworld_generator(&lookup, FlatLevelSource::new(config.clone())).unwrap();
        // rust: }), &TitleScreen::new());
        // TODO(fabric2seer): `Minecraft.getSingleplayerServer` has no Rust method in the port (looked for `get_singleplayer_server` in Minecraft and its supertypes)
        // rust: let mut server: Option<crate::unresolved::IntegratedServer> = Some(compat::minecraft().get_singleplayer_server());
        let mut server: crate::unresolved::IntegratedServer = todo!("fabric2seer: the initialiser of `server` (TODO above)");
        if false /* java: server == null: never null here */ {
            RoomPreviewServer::reset();
            return;
        }
        // TODO(fabric2seer): `IntegratedServer.getGameRules` has no Rust method in the port (looked for `get_game_rules` in IntegratedServer and its supertypes)
        // TODO(fabric2seer): static `GameRules.ADVANCE_TIME` has no Rust counterpart in the port's GameRules file
        // TODO(fabric2seer): `set` takes 2 argument(s) in the port (`game_rule: &'static GameRule, value: GameRuleValue`), Java passes 3
        // rust: server.get_game_rules().set(&GameRules::ADVANCE_TIME, false, server.clone());
        // TODO(fabric2seer): `IntegratedServer.getGameRules` has no Rust method in the port (looked for `get_game_rules` in IntegratedServer and its supertypes)
        // TODO(fabric2seer): static `GameRules.RANDOM_TICK_SPEED` has no Rust counterpart in the port's GameRules file
        // TODO(fabric2seer): `set` takes 2 argument(s) in the port (`game_rule: &'static GameRule, value: GameRuleValue`), Java passes 3
        // rust: server.get_game_rules().set(&GameRules::RANDOM_TICK_SPEED, 0, server.clone());
    }

    /// java: public static void addErrorMessage(Component errorText)
    pub fn add_error_message(error_text: &MutableComponent) {
        ERROR_MESSAGES.get_mut().push((constants::PREFIX.get_mut())().append_str(&error_text.get_string()));
    }

    /// java: public static @Nullable StructureTemplate getStructureTemplate(String type, String roomName)
    pub fn get_structure_template(r#type: &str, room_name: &str) -> Option<crate::unresolved::StructureTemplate> {
        let mut block_data: Option<Vec<i32>> = DungeonManager::get_room_block_data(r#type, room_name);
        return block_data.clone().map(|blocks| {
            // TODO(fabric2seer): `new StructureTemplate(..)`: the port has no `StructureTemplate::new`
            // rust: let mut template: crate::unresolved::StructureTemplate = crate::unresolved::StructureTemplate::new();
            let mut template: crate::unresolved::StructureTemplate = todo!("fabric2seer: the initialiser of `template` (TODO above)");
            // TODO(fabric2seer): `StructureTemplate.load` has no Rust method in the port (looked for `load` in StructureTemplate and its supertypes)
            // rust: template.load(mcrust::net::minecraft::core::registries::built_in_registries::block(), &RoomStructure::get_compound(&blocks));
            return template;
        });
    }

    /// java: public static void loadRoom(String type, String roomName)
    pub fn load_room(r#type: &str, mut room_name: String) {
        // TODO(fabric2seer): `Minecraft.getSingleplayerServer` has no Rust method in the port (looked for `get_singleplayer_server` in Minecraft and its supertypes)
        // rust: let mut server: Option<crate::unresolved::IntegratedServer> = Some(compat::minecraft().get_singleplayer_server());
        let mut server: crate::unresolved::IntegratedServer = todo!("fabric2seer: the initialiser of `server` (TODO above)");
        if false /* java: server == null: never null here */ {
            return;
        }
        compat::set_state(&SELECTED_ROOM, room_name.clone());
        let Some(mut template) = RoomPreviewServer::get_structure_template(r#type, &room_name) else {
            RoomPreviewServer::add_error_message(&(component::translatable_args("skyblocker.dungeons.roomPreview.failedToLoad", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.dungeons.roomPreview.invalidRoom"))])).with_style_format(ChatFormatting::Red));
            return;
        };
        // TODO(fabric2seer): `IntegratedServer.overworld` has no Rust method in the port (looked for `overworld` in IntegratedServer and its supertypes)
        // TODO(fabric2seer): `new StructurePlaceSettings(..)`: the port has no `StructurePlaceSettings::new`
        // TODO(fabric2seer): `StructureTemplate.placeInWorld` has no Rust method in the port (looked for `place_in_world` in StructureTemplate and its supertypes)
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Runnable`)
        // rust: server.execute(|| { template.place_in_world(&server.overworld(), &mcrust::net::minecraft::core::block_pos::ZERO, &mcrust::net::minecraft::core::block_pos::ZERO, &crate::unresolved::StructurePlaceSettings::new(), &server.overworld().get_random(), 0); });
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Runnable`)
        // rust: server.execute(|| {
        // rust: // TODO(fabric2seer): `IntegratedServer.overworld` has no Rust method in the port (looked for `overworld` in IntegratedServer and its supertypes)
        // rust: // rust: let mut border: WorldBorder = server.overworld().get_world_border_level().unwrap();
        // rust: let mut border: WorldBorder = todo!("fabric2seer: the initialiser of `border` (TODO above)");
        // rust: // TODO(fabric2seer): `StructureTemplate.getSize` has no Rust method in the port (looked for `get_size` in StructureTemplate and its supertypes)
        // rust: // rust: border.set_center(((((template.get_size().get_x()) as f64) + (1 as f64)) / (2 as f64)), ((((template.get_size().get_z()) as f64) + (1 as f64)) / (2 as f64)));
        // rust: // TODO(fabric2seer): `StructureTemplate.getSize` has no Rust method in the port (looked for `get_size` in StructureTemplate and its supertypes)
        // rust: // rust: border.set_size(((template.get_size().get_x()).max(template.get_size().get_z())) as f64);
        // rust: });
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Runnable`)
        // rust: server.execute(|| {
        // rust: let mut preview_data: Tag = mcrust::net::minecraft::nbt::compound_tag::Tag::compound();
        // rust: preview_data.put_boolean("isActive", true);
        // rust: preview_data.put_string("selectedRoom", &room_name);
        // rust: // TODO(fabric2seer): `IntegratedServer.getCommandStorage` has no Rust method in the port (looked for `get_command_storage` in IntegratedServer and its supertypes)
        // rust: // TODO(fabric2seer): `CommandStorage.set` has no Rust method in the port (looked for `set` in CommandStorage and its supertypes)
        // rust: // rust: server.get_command_storage().set(&crate::entry::skyblocker_mod::id(SAVE_NAME), &preview_data);
        // rust: });
    }

}
