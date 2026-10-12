//! Ported from de.hysky.skyblocker.debug.Debug by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::debug::snapshot_debug::SnapshotDebug;
use crate::de::hysky::skyblocker::utils::item_utils;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::brigadier::builder::literal_argument_builder::LiteralArgumentBuilder;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::client::key_mapping;
use mcrust::net::minecraft::core::holder_lookup;
use mcrust::net::minecraft::nbt::nbt_utils;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::world::level::storage::tag_value_output::TagValueOutput;
use mcrust::com::mojang::serialization::encoder::Encoder as _;
use mcrust::net::minecraft::world::entity::entity::Entity as _;
use mcrust::net::minecraft::world::level::entity_getter::EntityGetter as _;

/// java: private static final boolean DEBUG_ENABLED = Boolean.parseBoolean(System.getProperty("skyblocker.debug", "false"));
pub static DEBUG_ENABLED: LazyLock<bool> = LazyLock::new(|| std::env::var("skyblocker.debug").ok().unwrap().eq_ignore_ascii_case("true"));
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
        // TODO(fabric2seer): Boolean.getBoolean
        //   java: return Boolean.getBoolean("IS_TEST_ENV");
        todo!("fabric2seer: the value of this function is in a TODO above")
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
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(..);
            },
        ));
        client_tick_events::END_CLIENT_TICK.register(Arc::new(|minecraft: &mut Minecraft| {
            if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::level().is_none() {
                return;
            }
            // TODO(fabric2seer): `consume_click` takes 1 argument(s) in the port (`name: &str`), Java passes 0
            if key_mapping::consume_click() && !(compat::get_state(&KEY_DOWN)) {
                // TODO(fabric2seer): `get_entities` takes 3 argument(s) in the port (`except: Option<i32>, bb: &crate::net::minecraft::world::phys::aabb::AABB, selector: &dyn Fn( &(dyn crate::net::minecraft::world::entity::entity::Entity + Send + Sync), ) -> bool`), Java passes 2
                // TODO(fabric2seer): the mod keeps the game's `LocalPlayer` (Java holds a reference to the live object); the port's `LocalPlayer` is not a value to copy and the port lends the mod no handle to it
                // rust: for text in mcrust::mcrust::modding::game::with_player(|player| mcrust::mcrust::modding::game::level().unwrap().get_entities(Some(player.clone()), &player.base.base.base.base.get_bounding_box().inflate((SkyblockerConfigManager::get().debug.dump_range) as f64)).into_iter().map(|entity| {
                // rust: // TODO(fabric2seer): `logger` (a logger) used as a value
                // rust: //   java: TagValueOutput writeView = TagValueOutput.createWithContext(new ProblemReporter.ScopedCollector(LOGGER), RegistryUtils.getRegistryWrapperLookup());
                // rust: let mut write_view: TagValueOutput = todo!("fabric2seer: the initialiser of `writeView` (TODO above)");
                // rust: // TODO(fabric2seer): `save_without_id` takes 2 argument(s) in the port (`output: &mut dyn crate::net::minecraft::world::level::storage::value_output::ValueOutput, level: &crate::net::minecraft::client::multiplayer::client_level::ClientLevel`), Java passes 1
                // rust: // rust: entity.save_without_id(&mut write_view);
                // rust: return write_view.build_result();
                // rust: }).map(|__a0| compat::component_of(nbt_utils::to_pretty_component(&__a0)))) {
                // rust: mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&text.to_string()); });
                // rust: }
                compat::set_state(&KEY_DOWN, true);
            } else if !(key_mapping::consume_click()) && compat::get_state(&KEY_DOWN) {
                compat::set_state(&KEY_DOWN, false);
            }
        }));
        screen_events::BEFORE_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            let Some(handled_screen) = screen.as_any().downcast_ref::<AbstractContainerScreen>() else {
                return;
            };
            screen_keyboard_events::AFTER_KEY_PRESS.register(Arc::new(|action: i32, key_event: &KeyEvent| {
                let mut focused_slot: Option<Slot> = Some(handled_screen.hovered_slot);
                // TODO(fabric2seer): the condition: `client`: Fabric passes the Minecraft here; this SeerClient callback does not (take it from ClientTickEvents or a hook)
                //   java: dumpHoveredItemKey.matches(keyInput) && client.player != null && focusedSlot != null && focusedSlot.hasItem()
                if todo!("fabric2seer: the condition (TODO above)") {
                    // TODO(fabric2seer): the condition: `.hasShiftDown(..)` on a value of unknown type
                    //   java: !keyInput.hasShiftDown()
                    if todo!("fabric2seer: the condition (TODO above)") {
                        // TODO(fabric2seer): `client`: Fabric passes the Minecraft here; this SeerClient callback does not (take it from ClientTickEvents or a hook)
                        //   java: client.player.sendSystemMessage(Constants.PREFIX.get().append("Hovered Item: ").append(SkyblockerConfigManager.get().debug.dumpFormat.format(focusedSlot.getItem())));
                    } else {
                        // TODO(fabric2seer): `client`: Fabric passes the Minecraft here; this SeerClient callback does not (take it from ClientTickEvents or a hook)
                        //   java: client.player.sendSystemMessage(Constants.PREFIX.get().append("Held Item NW Calcs: ").append(Component.literal(SkyblockerMod.GSON_COMPACT.toJson(Calculation.LIST_CODEC.encodeStart(JsonOps.INSTANCE, NetworthCalculator.getItemNetworth(focusedSlot.getItem()).calculations()).getOrThrow()))));
                    }
                }
            }));
        }));
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> dumpPlayersCommand()
    pub fn dump_players_command() -> LiteralArgumentBuilder<crate::unresolved::FabricClientCommandSource> {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: return literal("dumpPlayers")
        //   java: 				.executes(context -> {
        //   java: 					context.getSource().getLevel().players().forEach(player -> context.getSource().sendFeedback(Component.nullToEmpty("'" + player.getName().getString() + "'")));
        //   java: 					return Command.SINGLE_SUCCESS;
        //   java: 				});
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> toggleShowingInvisibleArmorStands()
    pub fn toggle_showing_invisible_armor_stands() -> LiteralArgumentBuilder<crate::unresolved::FabricClientCommandSource> {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: return literal("toggleShowingInvisibleArmorStands")
        //   java: 				.executes(context -> {
        //   java: 					SkyblockerConfigManager.update(config -> config.debug.showInvisibleArmorStands = !config.debug.showInvisibleArmorStands);
        //   java: 					context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.debug.toggledShowingInvisibleArmorStands", SkyblockerConfigManager.get().debug.showInvisibleArmorStands)));
        //   java: 					return Command.SINGLE_SUCCESS;
        //   java: 				});
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> toggleWebSocketDebug()
    pub fn toggle_web_socket_debug() -> LiteralArgumentBuilder<crate::unresolved::FabricClientCommandSource> {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: return literal("toggleWebSocketDebug")
        //   java: 				.executes(context -> {
        //   java: 					SkyblockerConfigManager.update(config -> config.debug.webSocketDebug = !SkyblockerConfigManager.get().debug.webSocketDebug);
        //   java: 					context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.debug.toggledWebSocketDebug", SkyblockerConfigManager.get().debug.webSocketDebug)));
        //   java: 					return Command.SINGLE_SUCCESS;
        //   java: 				});
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> dumpArmorStandHeadTextures()
    pub fn dump_armor_stand_head_textures() -> LiteralArgumentBuilder<crate::unresolved::FabricClientCommandSource> {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
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
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> dumpBiome()
    pub fn dump_biome() -> LiteralArgumentBuilder<crate::unresolved::FabricClientCommandSource> {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
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
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> dumpActionBar()
    pub fn dump_action_bar() -> LiteralArgumentBuilder<crate::unresolved::FabricClientCommandSource> {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
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
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> dumpServerResourcePacks()
    pub fn dump_server_resource_packs() -> LiteralArgumentBuilder<crate::unresolved::FabricClientCommandSource> {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
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
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> auditMixins()
    pub fn audit_mixins() -> LiteralArgumentBuilder<crate::unresolved::FabricClientCommandSource> {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: return literal("auditMixins")
        //   java: 				.executes(_ -> {
        //   java: 					MixinEnvironment.getCurrentEnvironment().audit();
        //   java:
        //   java: 					return Command.SINGLE_SUCCESS;
        //   java: 				});
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> prefixTest()
    pub fn prefix_test() -> LiteralArgumentBuilder<crate::unresolved::FabricClientCommandSource> {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: return literal("prefixTest")
        //   java: 				.executes(context -> {
        //   java: 					context.getSource().sendFeedback(Constants.PREFIX.get());
        //   java: 					return Command.SINGLE_SUCCESS;
        //   java: 				});
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> apiId()
    pub fn api_id() -> LiteralArgumentBuilder<crate::unresolved::FabricClientCommandSource> {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: return literal("apiId")
        //   java: 				.executes(context -> {
        //   java: 					context.getSource().sendFeedback(Constants.PREFIX.get().append("Item API ID: " + context.getSource().getPlayer().getMainHandItem().getSkyblockApiId()));
        //   java: 					return Command.SINGLE_SUCCESS;
        //   java: 				});
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> neuName()
    pub fn neu_name() -> LiteralArgumentBuilder<crate::unresolved::FabricClientCommandSource> {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: return literal("neuName")
        //   java: 				.executes(context -> {
        //   java: 					context.getSource().sendFeedback(Constants.PREFIX.get().append("Item NEU Name: " + context.getSource().getPlayer().getMainHandItem().getNeuName()));
        //   java: 					return Command.SINGLE_SUCCESS;
        //   java: 				});
        todo!("fabric2seer: the value of this function is in a TODO above")
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
                // TODO(fabric2seer): `.toJson(..)` on a Gson
                //   java: return Component.literal(SkyblockerMod.GSON_COMPACT.toJson(ItemUtils.EMPTY_ALLOWING_ITEMSTACK_CODEC.encodeStart(RegistryUtils.getRegistryWrapperLookup().createSerializationContext(JsonOps.INSTANCE), stack).getOrThrow()));
            }
            DumpFormat::Snbt => {
                // TODO(fabric2seer): `LocalPlayer.registryAccess` has no Rust method in the port (looked for `registry_access` in LocalPlayer and its supertypes)
                // TODO(fabric2seer): `create_serialization_context` takes 2 argument(s) in the port (`provider: Arc<dyn Provider>, parent: crate::com::mojang::serialization::dynamic_ops::Ops`), Java passes 1
                // rust: return compat::component_of(nbt_utils::to_pretty_component(&(item_utils::EMPTY_ALLOWING_ITEMSTACK_CODEC.get_mut().encode_start(&mcrust::mcrust::modding::game::with_player(|player| holder_lookup::create_serialization_context(mcrust::net::minecraft::nbt::nbt_ops::NbtOps::instance())).unwrap(), stack)).get_or_throw().unwrap()));
                return todo!("fabric2seer: see the TODO above");
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
