//! Ported from de.hysky.skyblocker.debug.Debug by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::debug::snapshot_debug::SnapshotDebug;
use crate::de::hysky::skyblocker::utils::item_utils;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::brigadier::builder::literal_argument_builder::LiteralArgumentBuilder;
use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::mcrust::modding::events::client_command_registration_callback::FabricClientCommandSource;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::client::key_mapping;
use mcrust::net::minecraft::core::holder_lookup;
use mcrust::net::minecraft::nbt::nbt_utils;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::world::level::storage::tag_value_output::TagValueOutput;
use mcrust::com::mojang::serialization::encoder::Encoder as _;
use mcrust::net::minecraft::world::entity::entity::Entity as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;
use mcrust::net::minecraft::world::level::entity_getter::EntityGetter as _;

/// java: private static final boolean DEBUG_ENABLED = Boolean.parseBoolean(System.getProperty("skyblocker.debug", "false"));
/// (`System.getProperty(key, "false")` is the process environment here; a missing one is the default "false")
pub static DEBUG_ENABLED: LazyLock<bool> = LazyLock::new(|| std::env::var("skyblocker.debug").is_ok_and(|v| v.eq_ignore_ascii_case("true")));
/// java: public static KeyMapping dumpNearbyEntitiesKey;
pub static DUMP_NEARBY_ENTITIES_KEY: compat::MainThread<Option<mcrust::net::minecraft::client::key_mapping::KeyMapping>> = compat::MainThread::new(|| None);
/// java: public static KeyMapping dumpHoveredItemKey;
pub static DUMP_HOVERED_ITEM_KEY: compat::MainThread<Option<mcrust::net::minecraft::client::key_mapping::KeyMapping>> = compat::MainThread::new(|| None);
/// java: private static boolean keyDown = false;
pub static KEY_DOWN: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// java: public class Debug
#[derive(Clone)]
pub struct Debug {}

impl Debug {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static boolean debugEnabled()
    pub fn debug_enabled() -> bool {
        return (*DEBUG_ENABLED || false || SnapshotDebug::is_in_snapshot());
    }

    /// java: public static boolean isTestEnvironment()
    pub fn is_test_environment() -> bool {
        //   java: return Boolean.getBoolean("IS_TEST_ENV");
        // (a system property is the process environment here, as for `DEBUG_ENABLED`)
        return std::env::var("IS_TEST_ENV").is_ok_and(|v| v.eq_ignore_ascii_case("true"));
    }

    /// java: public static boolean webSocketDebug()
    pub fn web_socket_debug() -> bool {
        return SkyblockerConfigManager::get().debug.web_socket_debug;
    }

    /// java: @Init public static void init()
    pub fn init() {
        if !(Debug::debug_enabled()) {
            return;
        }
        SnapshotDebug::init();
        key_binding_helper::register_key_binding(KeyMapping::new("key.skyblocker.debug.dumpNearbyEntities", 73, Category::Misc));
        key_binding_helper::register_key_binding(KeyMapping::new("key.skyblocker.debug.dumpHoveredItem", 85, Category::Misc));
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE).then(literal("debug")
                //   java: 		.then(dumpPlayersCommand()).then(ItemUtils.dumpHeldItemCommand()).then(ItemUtils.dumpHeldItemNetworthCalculationsCommand())
                //   java: 		.then(toggleShowingInvisibleArmorStands()).then(dumpArmorStandHeadTextures()).then(toggleWebSocketDebug())
                //   java: 		.then(EventNotifications.debugToasts()).then(dumpBiome()).then(dumpActionBar()).then(dumpServerResourcePacks())
                //   java: 		.then(auditMixins()).then(prefixTest()).then(apiId()).then(neuName())));
                let mut debug = client_command_registration_callback::literal("debug");
                debug.then(&Debug::dump_players_command());
                // rust: cross-package - `ItemUtils.dumpHeldItemCommand()` / `dumpHeldItemNetworthCalculationsCommand()`
                // (utils/item_utils.rs) are still the converter's placeholders typed over `crate::unresolved`; they join
                // the tree once they answer the real builder (GAPS_K)
                debug.then(&Debug::toggle_showing_invisible_armor_stands());
                debug.then(&Debug::dump_armor_stand_head_textures());
                debug.then(&Debug::toggle_web_socket_debug());
                debug.then(&crate::de::hysky::skyblocker::skyblock::events::event_notifications::EventNotifications::debug_toasts());
                debug.then(&Debug::dump_biome());
                debug.then(&Debug::dump_action_bar());
                debug.then(&Debug::dump_server_resource_packs());
                debug.then(&Debug::audit_mixins());
                debug.then(&Debug::prefix_test());
                debug.then(&Debug::api_id());
                debug.then(&Debug::neu_name());
                let mut root = client_command_registration_callback::literal(skyblocker_mod::NAMESPACE);
                root.then(&debug);
                dispatcher.dispatcher().register(&root);
            },
        ));
        client_tick_events::END_CLIENT_TICK.register(Arc::new(|minecraft: &mut Minecraft| {
            if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::level().is_none() {
                return;
            }
            // java: dumpNearbyEntitiesKey.consumeClick()
            if key_mapping::consume_click("key.skyblocker.debug.dumpNearbyEntities") && !(compat::get_state(&KEY_DOWN)) {
                //   java: client.level.getEntities(client.player, client.player.getBoundingBox().inflate(SkyblockerConfigManager.get().debug.dumpRange)).stream()
                //   java: 		.map(entity -> { TagValueOutput writeView = TagValueOutput.createWithContext(new ProblemReporter.ScopedCollector(LOGGER), RegistryUtils.getRegistryWrapperLookup());
                //   java: 			entity.saveWithoutId(writeView); return writeView.buildResult(); })
                //   java: 		.map(NbtUtils::toPrettyComponent).forEach(text -> client.player.sendSystemMessage(text));
                let level = mcrust::mcrust::modding::game::level().unwrap();
                let (player_id, search_box) = mcrust::mcrust::modding::game::with_player(|player| (player.base().get_id(), player.base.base.base.base.get_bounding_box().inflate((SkyblockerConfigManager::get().debug.dump_range) as f64))).unwrap();
                let entities = level.get_entities(Some(player_id), &search_box, &|_| true);
                for entity in entities.iter() {
                    let tag = {
                        let entity = mcrust::mcrust::modding::game::lock_entity(&**entity, "debug.rs:Debug.init dumpNearbyEntities");
                        // `new ProblemReporter.ScopedCollector(LOGGER)` that is never closed: only its `Collector` half is read
                        let mut write_view: TagValueOutput = TagValueOutput::create_with_context(mcrust::net::minecraft::util::problem_reporter::Collector::new() as std::rc::Rc<dyn mcrust::net::minecraft::util::problem_reporter::ProblemReporter>, crate::de::hysky::skyblocker::utils::registry_utils::RegistryUtils::get_registry_wrapper_lookup());
                        entity.save_without_id(&mut write_view, &level);
                        write_view.build_result()
                    };
                    let text = nbt_utils::to_pretty_component(&tag);
                    // rust: port gap - `player.sendSystemMessage(Component)` (a styled client message, GAPS_batchD #17): the
                    // text is shown as plain feedback
                    script_host::send_feedback(&text.get_string());
                }
                compat::set_state(&KEY_DOWN, true);
            } else if !(key_mapping::consume_click("key.skyblocker.debug.dumpNearbyEntities")) && compat::get_state(&KEY_DOWN) {
                compat::set_state(&KEY_DOWN, false);
            }
        }));
        // java: ScreenEvents.BEFORE_INIT.register((client, screen, w, h) -> { if (screen instanceof AbstractContainerScreen<?> handledScreen)
        //   ScreenKeyboardEvents.afterKeyPress(screen).register((_, keyInput) -> {..}); });
        // (the port's afterKeyPress is one global event, not a per-screen one: the listener is registered once and asks
        // whether a container screen is open - GAPS_batchB #5)
        screen_keyboard_events::AFTER_KEY_PRESS.register(Arc::new(|action: i32, key_event: &KeyEvent| {
            if !compat::container_screen_open() {
                return;
            }
            {
                // java: Slot focusedSlot = ((AbstractContainerScreenAccessor) handledScreen).getFocusedSlot();
                // rust: port gap (GAPS_batchB #2) - the open container screen's hovered slot is not lent to mod callbacks
                let mut focused_slot: Option<Slot> = None;
                // java: dumpHoveredItemKey.matches(keyInput) && client.player != null && focusedSlot != null && focusedSlot.hasItem()
                if key_mapping::matches("key.skyblocker.debug.dumpHoveredItem", key_event) && mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && focused_slot.as_ref().is_some_and(|slot| compat::slot_has_item(slot)) {
                    // java: !keyInput.hasShiftDown()
                    if !key_event.has_shift_down() {
                        // TODO(fabric2seer): `client`: Fabric passes the Minecraft here; this SeerClient callback does not (take it from ClientTickEvents or a hook)
                        //   java: client.player.sendSystemMessage(Constants.PREFIX.get().append("Hovered Item: ").append(SkyblockerConfigManager.get().debug.dumpFormat.format(focusedSlot.getItem())));
                    } else {
                        // TODO(fabric2seer): `client`: Fabric passes the Minecraft here; this SeerClient callback does not (take it from ClientTickEvents or a hook)
                        //   java: client.player.sendSystemMessage(Constants.PREFIX.get().append("Held Item NW Calcs: ").append(Component.literal(SkyblockerMod.GSON_COMPACT.toJson(Calculation.LIST_CODEC.encodeStart(JsonOps.INSTANCE, NetworthCalculator.getItemNetworth(focusedSlot.getItem()).calculations()).getOrThrow()))));
                    }
                }
            }
        }));
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> dumpPlayersCommand()
    pub fn dump_players_command() -> LiteralArgumentBuilder<FabricClientCommandSource> {
        //   java: return literal("dumpPlayers")
        //   java: 				.executes(context -> {
        //   java: 					context.getSource().getLevel().players().forEach(player -> context.getSource().sendFeedback(Component.nullToEmpty("'" + player.getName().getString() + "'")));
        //   java: 					return Command.SINGLE_SUCCESS;
        //   java: 				});
        let mut node = client_command_registration_callback::literal("dumpPlayers");
        node.executes(Arc::new(|_: &CommandContext<FabricClientCommandSource>| {
            use mcrust::net::minecraft::client::player::abstract_client_player::AbstractClientPlayer as _;
            if let Some(level) = mcrust::mcrust::modding::game::level() {
                let players = level.with_players(|players| players.to_vec());
                for avatar in players.iter() {
                    if mcrust::mcrust::modding::game::is_held_here(&**avatar) {
                        continue;
                    }
                    let name: String = mcrust::mcrust::modding::game::lock_entity(&**avatar, "debug.rs:Debug.dumpPlayersCommand").get_game_profile_name().to_string();
                    // `player.getName().getString()` - a player's name is its profile name
                    client_command_registration_callback::send_feedback(&format!("'{}'", name));
                }
            }
            Ok(1)
        }));
        return node;
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> toggleShowingInvisibleArmorStands()
    pub fn toggle_showing_invisible_armor_stands() -> LiteralArgumentBuilder<FabricClientCommandSource> {
        //   java: return literal("toggleShowingInvisibleArmorStands")
        //   java: 				.executes(context -> {
        //   java: 					SkyblockerConfigManager.update(config -> config.debug.showInvisibleArmorStands = !config.debug.showInvisibleArmorStands);
        //   java: 					context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.debug.toggledShowingInvisibleArmorStands", SkyblockerConfigManager.get().debug.showInvisibleArmorStands)));
        //   java: 					return Command.SINGLE_SUCCESS;
        //   java: 				});
        let mut node = client_command_registration_callback::literal("toggleShowingInvisibleArmorStands");
        node.executes(Arc::new(|_: &CommandContext<FabricClientCommandSource>| {
            SkyblockerConfigManager::update(Arc::new(|config: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig| config.debug.show_invisible_armor_stands = !config.debug.show_invisible_armor_stands));
            client_command_registration_callback::send_feedback(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable_args("skyblocker.debug.toggledShowingInvisibleArmorStands", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(SkyblockerConfigManager::get().debug.show_invisible_armor_stands.to_string())])).get_string());
            Ok(1)
        }));
        return node;
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> toggleWebSocketDebug()
    pub fn toggle_web_socket_debug() -> LiteralArgumentBuilder<FabricClientCommandSource> {
        //   java: return literal("toggleWebSocketDebug")
        //   java: 				.executes(context -> {
        //   java: 					SkyblockerConfigManager.update(config -> config.debug.webSocketDebug = !SkyblockerConfigManager.get().debug.webSocketDebug);
        //   java: 					context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.debug.toggledWebSocketDebug", SkyblockerConfigManager.get().debug.webSocketDebug)));
        //   java: 					return Command.SINGLE_SUCCESS;
        //   java: 				});
        let mut node = client_command_registration_callback::literal("toggleWebSocketDebug");
        node.executes(Arc::new(|_: &CommandContext<FabricClientCommandSource>| {
            let web_socket_debug: bool = SkyblockerConfigManager::get().debug.web_socket_debug;
            SkyblockerConfigManager::update(Arc::new(move |config: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig| config.debug.web_socket_debug = !web_socket_debug));
            client_command_registration_callback::send_feedback(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable_args("skyblocker.debug.toggledWebSocketDebug", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(SkyblockerConfigManager::get().debug.web_socket_debug.to_string())])).get_string());
            Ok(1)
        }));
        return node;
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> dumpArmorStandHeadTextures()
    pub fn dump_armor_stand_head_textures() -> LiteralArgumentBuilder<FabricClientCommandSource> {
        //   java: return literal("dumpArmorStandHeadTextures")
        //   java: 				.executes(context -> {
        //   java: 					List<ArmorStand> armorStands = context.getSource().getLevel().getEntitiesOfClass(ArmorStand.class, context.getSource().getPlayer().getBoundingBox().inflate(8d), EntitySelector.ENTITY_NOT_BEING_RIDDEN);
        //   java:
        //   java: 					for (ArmorStand armorStand : armorStands) {
        //   java: 						Iterable<ItemStack> equippedItems = ItemUtils.getArmor(armorStand);
        //   java:
        //   java: 						for (ItemStack stack : equippedItems) {
        //   java: 							ItemUtils.getHeadTextureOptional(stack).ifPresent(texture -> context.getSource().sendFeedback(Component.nullToEmpty(texture)));
        //   java: 						}
        //   java: 					}
        //   java:
        //   java: 					return Command.SINGLE_SUCCESS;
        //   java: 				});
        let mut node = client_command_registration_callback::literal("dumpArmorStandHeadTextures");
        node.executes(Arc::new(|_: &CommandContext<FabricClientCommandSource>| {
            let (Some(level), Some(search_box)) = (mcrust::mcrust::modding::game::level(), mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_bounding_box().inflate(8.0))) else {
                return Ok(1);
            };
            // getEntitiesOfClass(ArmorStand.class, .., EntitySelector.ENTITY_NOT_BEING_RIDDEN)
            let armor_stands = level.get_entities_of_class(
                mcrust::net::minecraft::world::level::entity::entity_type_test::EntityClass::Entity,
                None,
                &search_box,
                &|entity| entity.as_any().is_some_and(|any| any.is::<mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand>()) && mcrust::net::minecraft::world::entity::entity_selector::entity_not_being_ridden(entity),
            );
            for armor_stand in armor_stands.iter() {
                let equipped_items: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = {
                    let guard = mcrust::mcrust::modding::game::lock_entity(&**armor_stand, "debug.rs:Debug.dumpArmorStandHeadTextures");
                    match guard.as_any().and_then(|any| any.downcast_ref::<mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand>()) {
                        Some(stand) => item_utils::ItemUtils::get_armor(stand),
                        None => Vec::new(),
                    }
                };
                for stack in equipped_items.iter() {
                    if let Some(texture) = item_utils::ItemUtils::get_head_texture_optional(stack) {
                        client_command_registration_callback::send_feedback(&texture);
                    }
                }
            }
            Ok(1)
        }));
        return node;
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> dumpBiome()
    pub fn dump_biome() -> LiteralArgumentBuilder<FabricClientCommandSource> {
        //   java: return literal("dumpBiome")
        //   java: 				.executes(context -> {
        //   java: 					FabricClientCommandSource source = context.getSource();
        //   java: 					Holder<Biome> biome = source.getLevel().getBiome(source.getPlayer().blockPosition());
        //   java:
        //   java: 					if (biome != null && biome.value() != null) {
        //   java: 						String biomeData = Biome.DIRECT_CODEC.encodeStart(JsonOps.INSTANCE, biome.value())
        //   java: 								.map(JsonElement::toString)
        //   java: 								.setPartial("")
        //   java: 								.getPartialOrThrow();
        //   java: 						source.sendFeedback(Constants.PREFIX.get().append(Component.literal(String.format("Biome ID: %s, Data: %s", biome.getRegisteredName(), biomeData))));
        //   java: 					}
        //   java:
        //   java: 					return Command.SINGLE_SUCCESS;
        //   java: 				});
        let mut node = client_command_registration_callback::literal("dumpBiome");
        node.executes(Arc::new(|_: &CommandContext<FabricClientCommandSource>| {
            // rust: port gap - `source.getLevel().getBiome(pos)` answers the biome's `Holder` (`getRegisteredName`) and
            // `Biome.DIRECT_CODEC` encodes it; the port's `ClientLevel::get_biome` answers the bare `Biome` and has no
            // direct codec (GAPS_K): the command prints nothing
            Ok(1)
        }));
        return node;
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> dumpActionBar()
    pub fn dump_action_bar() -> LiteralArgumentBuilder<FabricClientCommandSource> {
        //   java: return literal("dumpActionBar")
        //   java: 				.executes(context -> {
        //   java: 					FabricClientCommandSource source = context.getSource();
        //   java: 					Component actionBar = ((HudAccessor) (source.getClient().gui.hud)).getOverlayMessageString();
        //   java:
        //   java: 					if (actionBar != null) {
        //   java: 						Component pretty = NbtUtils.toPrettyComponent(ComponentSerialization.CODEC.encodeStart(RegistryUtils.getRegistryWrapperLookup().createSerializationContext(NbtOps.INSTANCE), actionBar).getOrThrow());
        //   java: 						source.sendFeedback(Constants.PREFIX.get().append("Action Bar: ").append(pretty));
        //   java: 					}
        //   java:
        //   java: 					return Command.SINGLE_SUCCESS;
        //   java: 				});
        let mut node = client_command_registration_callback::literal("dumpActionBar");
        node.executes(Arc::new(|_: &CommandContext<FabricClientCommandSource>| {
            // rust: port gap - `HudAccessor.getOverlayMessageString` and `ComponentSerialization.CODEC` into NBT for
            // `NbtUtils.toPrettyComponent` (GAPS_K): the command prints nothing
            Ok(1)
        }));
        return node;
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> dumpServerResourcePacks()
    pub fn dump_server_resource_packs() -> LiteralArgumentBuilder<FabricClientCommandSource> {
        //   java: return literal("dumpServerResourcePacks")
        //   java: 				.executes(context -> {
        //   java: 					FabricClientCommandSource source = context.getSource();
        //   java: 					ServerPackManager packManager = source.getClient().getDownloadedPackSource().manager;
        //   java:
        //   java: 					for (ServerPackData packData : packManager.packs) {
        //   java: 						Component packText = ComponentUtils.copyOnClickText(packData.url.toString());
        //   java: 						Component message = Constants.PREFIX.get().append(packText);
        //   java: 						source.sendFeedback(message);
        //   java: 					}
        //   java:
        //   java: 					return Command.SINGLE_SUCCESS;
        //   java: 				});
        let mut node = client_command_registration_callback::literal("dumpServerResourcePacks");
        node.executes(Arc::new(|_: &CommandContext<FabricClientCommandSource>| {
            // rust: port gap - `DownloadedPackSource.manager.packs` (`ServerPackManager.packs`) is not reachable from a mod
            // (GAPS_K): the command prints nothing
            Ok(1)
        }));
        return node;
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> auditMixins()
    pub fn audit_mixins() -> LiteralArgumentBuilder<FabricClientCommandSource> {
        //   java: return literal("auditMixins")
        //   java: 				.executes(_ -> {
        //   java: 					MixinEnvironment.getCurrentEnvironment().audit();
        //   java:
        //   java: 					return Command.SINGLE_SUCCESS;
        //   java: 				});
        let mut node = client_command_registration_callback::literal("auditMixins");
        node.executes(Arc::new(|_: &CommandContext<FabricClientCommandSource>| {
            //   java: MixinEnvironment.getCurrentEnvironment().audit();
            // rust: port gap - there is no Mixin environment: SeerClient's hooks are registered, not applied to classes
            Ok(1)
        }));
        return node;
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> prefixTest()
    pub fn prefix_test() -> LiteralArgumentBuilder<FabricClientCommandSource> {
        //   java: return literal("prefixTest")
        //   java: 				.executes(context -> {
        //   java: 					context.getSource().sendFeedback(Constants.PREFIX.get());
        //   java: 					return Command.SINGLE_SUCCESS;
        //   java: 				});
        let mut node = client_command_registration_callback::literal("prefixTest");
        node.executes(Arc::new(|_: &CommandContext<FabricClientCommandSource>| {
            client_command_registration_callback::send_feedback(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().get_string());
            Ok(1)
        }));
        return node;
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> apiId()
    pub fn api_id() -> LiteralArgumentBuilder<FabricClientCommandSource> {
        //   java: return literal("apiId")
        //   java: 				.executes(context -> {
        //   java: 					context.getSource().sendFeedback(Constants.PREFIX.get().append("Item API ID: " + context.getSource().getPlayer().getMainHandItem().getSkyblockApiId()));
        //   java: 					return Command.SINGLE_SUCCESS;
        //   java: 				});
        let mut node = client_command_registration_callback::literal("apiId");
        node.executes(Arc::new(|_: &CommandContext<FabricClientCommandSource>| {
            use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
            let api_id: String = mcrust::mcrust::modding::game::with_player(|player| player.get_main_hand_item().get_skyblock_api_id()).unwrap_or_default();
            client_command_registration_callback::send_feedback(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append_str(&format!("Item API ID: {}", api_id)).get_string());
            Ok(1)
        }));
        return node;
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> neuName()
    pub fn neu_name() -> LiteralArgumentBuilder<FabricClientCommandSource> {
        //   java: return literal("neuName")
        //   java: 				.executes(context -> {
        //   java: 					context.getSource().sendFeedback(Constants.PREFIX.get().append("Item NEU Name: " + context.getSource().getPlayer().getMainHandItem().getNeuName()));
        //   java: 					return Command.SINGLE_SUCCESS;
        //   java: 				});
        let mut node = client_command_registration_callback::literal("neuName");
        node.executes(Arc::new(|_: &CommandContext<FabricClientCommandSource>| {
            use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
            let neu_name: String = mcrust::mcrust::modding::game::with_player(|player| player.get_main_hand_item().get_neu_name()).unwrap_or_default();
            client_command_registration_callback::send_feedback(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append_str(&format!("Item NEU Name: {}", neu_name)).get_string());
            Ok(1)
        }));
        return node;
    }

}

/// java: public enum DumpFormat
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DumpFormat {
    Json,
    Snbt,
}

impl DumpFormat {
    pub const VALUES: [DumpFormat; 2] = [DumpFormat::Json, DumpFormat::Snbt];

    /// java: values()
    pub fn values() -> Vec<DumpFormat> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            DumpFormat::Json => "JSON",
            DumpFormat::Snbt => "SNBT",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> DumpFormat {
        match name {
            "JSON" => DumpFormat::Json,
            "SNBT" => DumpFormat::Snbt,
            _ => panic!("IllegalArgumentException: No enum constant DumpFormat.{name}"),
        }
    }

    /// java: public abstract Component format(ItemStack stack); (overridden by JSON, SNBT)
    pub fn format(&self, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> MutableComponent {
        match self {
            DumpFormat::Json => {
                // java: return Component.literal(SkyblockerMod.GSON_COMPACT.toJson(ItemUtils.EMPTY_ALLOWING_ITEMSTACK_CODEC.encodeStart(RegistryUtils.getRegistryWrapperLookup().createSerializationContext(JsonOps.INSTANCE), stack).getOrThrow()));
                let ops = holder_lookup::create_serialization_context(crate::de::hysky::skyblocker::utils::registry_utils::RegistryUtils::get_registry_wrapper_lookup(), mcrust::com::mojang::serialization::json_ops::JsonOps::instance());
                let encoded = item_utils::EMPTY_ALLOWING_ITEMSTACK_CODEC.get().encode_start(&ops, stack).get_or_throw().unwrap_or_else(|e| panic!("java.lang.IllegalStateException: {e}"));
                return mcrust::net::minecraft::network::chat::component::literal(&compat::json_from_port(&encoded).to_string());
            }
            DumpFormat::Snbt => {
                // java: return NbtUtils.toPrettyComponent(ItemUtils.EMPTY_ALLOWING_ITEMSTACK_CODEC.encodeStart(CLIENT.player.registryAccess().createSerializationContext(NbtOps.INSTANCE), stack).getOrThrow());
                // (`player.registryAccess()` is the connection's registries, the lookup `RegistryUtils.getRegistryWrapperLookup()` answers)
                let ops = holder_lookup::create_serialization_context(crate::de::hysky::skyblocker::utils::registry_utils::RegistryUtils::get_registry_wrapper_lookup(), mcrust::net::minecraft::nbt::nbt_ops::NbtOps::instance());
                let encoded = item_utils::EMPTY_ALLOWING_ITEMSTACK_CODEC.get().encode_start(&ops, stack).get_or_throw().unwrap_or_else(|e| panic!("java.lang.IllegalStateException: {e}"));
                let tag: &mcrust::net::minecraft::nbt::tag::Tag = mcrust::com::mojang::serialization::cast::<mcrust::net::minecraft::nbt::tag::Tag>(&encoded).expect("NbtOps encodes a Tag");
                return compat::component_of(nbt_utils::to_pretty_component(tag));
            }
            _ => unreachable!("abstract enum method"),
        }
    }

}

impl std::fmt::Display for DumpFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
