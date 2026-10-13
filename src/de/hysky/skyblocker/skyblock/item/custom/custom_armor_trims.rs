//! Ported from de.hysky.skyblocker.skyblock.item.custom.CustomArmorTrims by fabric2seer.

#[allow(unused_imports)]
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::utils::registry_utils::RegistryUtils;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::com::mojang::brigadier::command_dispatcher::CommandDispatcher;
use mcrust::com::mojang::brigadier::suggestion::suggestion_provider::SuggestionProvider;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::net::minecraft::commands::command_build_context::CommandBuildContext;
use mcrust::net::minecraft::core::holder_lookup::Provider;
use mcrust::net::minecraft::core::registry::Registry;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::resources::resource_key::ResourceKey;
use mcrust::net::minecraft::world::item::equipment::trim::armor_trim::ArmorTrim;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::core::holder_lookup::HolderLookup as _;

/// java: public static final Object2ObjectOpenHashMap<ArmorTrimId, ArmorTrim> TRIMS_CACHE = new Object2ObjectOpenHashMap<>();
pub static TRIMS_CACHE: compat::MainThread<std::collections::HashMap<ArmorTrimId, ArmorTrim>> = compat::MainThread::new(|| std::collections::HashMap::new());
/// java: private static boolean trimsInitialized = false;
pub static TRIMS_INITIALIZED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: public static final Codec<ArmorTrimId> CODEC = RecordCodecBuilder.create(instance -> instance.group( Identifier.CODEC.fieldOf("material").forGetter(ArmorTrimId::material), Identifier.CODEC.fieldOf("pa
pub static ARMOR_TRIM_ID_CODEC: compat::MainThread<CodecRef<ArmorTrimId>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply2(record_codec_builder::of(|o: &ArmorTrimId| o.material(), codec::field_of(mcrust::net::minecraft::resources::identifier::Identifier::codec(), "material")), record_codec_builder::of(|o: &ArmorTrimId| o.pattern(), codec::field_of(mcrust::net::minecraft::resources::identifier::Identifier::codec(), "pattern")), move |__a0, __a1| ArmorTrimId::new(__a0.clone(), __a1.clone()))));

/// java: public class CustomArmorTrims
#[derive(Clone)]
pub struct CustomArmorTrims {}

impl CustomArmorTrims {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        skyblock_events::JOIN.register(Arc::new(move || { CustomArmorTrims::initialize_trim_cache(); }));
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                dispatcher.register("skyblocker", |ctx| {
                    #[allow(unreachable_code)]
                    Ok(())
                });
            },
        ));
    }

    /// java: private static void initializeTrimCache()
    pub fn initialize_trim_cache() {
        // Minecraft client = Minecraft.getInstance(); (never null once the game runs)
        if compat::get_state(&TRIMS_INITIALIZED) || false && !(crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled()) {
            return;
        }
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            TRIMS_CACHE.get_mut().clear();
            let mut wrapper_lookup: Arc<dyn Provider> = RegistryUtils::get_registry_wrapper_lookup();
            // Provider.lookupOrThrow(key) is the registry's lookup (`lookup_registry_or_throw`)
            for material in wrapper_lookup.lookup_registry_or_throw(&mcrust::net::minecraft::core::registries::registries::trim_material()).unwrap().list_elements() {
                for pattern in wrapper_lookup.lookup_registry_or_throw(&mcrust::net::minecraft::core::registries::registries::trim_pattern()).unwrap().list_elements() {
                    // ArmorTrim trim = new ArmorTrim(material, pattern);
                    let mut trim: ArmorTrim = ArmorTrim::new(mcrust::net::minecraft::core::holder::Holder::Reference(material.clone()), mcrust::net::minecraft::core::holder::Holder::Reference(pattern.clone()));
                    // TRIMS_CACHE.put(new ArmorTrimId(material.key().identifier(), pattern.key().identifier()), trim);
                    TRIMS_CACHE.get_mut().insert(ArmorTrimId::new(material.key().identifier().clone(), pattern.key().identifier().clone()), trim);
                }
            }
            logger().info("[Skyblocker] Successfully cached all armor trims!");
            compat::set_state(&TRIMS_INITIALIZED, true);
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker] Encountered an exception while caching armor trims"));
        }
    }

    /// java: private static void registerCommand(CommandDispatcher<FabricClientCommandSource> dispatcher, CommandBuildContext registryAccess)
    pub fn register_command(dispatcher: &CommandDispatcher<crate::unresolved::FabricClientCommandSource>, registry_access: &CommandBuildContext) {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: dispatcher.register(ClientCommands.literal("skyblocker")
        //   java: 				.then(ClientCommands.literal("custom")
        //   java: 						.then(ClientCommands.literal("armorTrim")
        //   java: 								.executes(context -> customizeTrim(context.getSource(), null, null))
        //   java: 								.then(ClientCommands.argument("material", IdentifierArgument.id())
        //   java: 										.suggests(getIdSuggestionProvider(Registries.TRIM_MATERIAL))
        //   java: 										.executes(context -> customizeTrim(context.getSource(), context.getArgument("material", Identifier.class), null))
        //   java: 										.then(ClientCommands.argument("pattern", IdentifierArgument.id())
        //   java: 												.suggests(getIdSuggestionProvider(Registries.TRIM_PATTERN))
        //   java: 												.executes(context -> customizeTrim(context.getSource(), context.getArgument("material", Identifier.class), context.getArgument("pattern", Identifier.class))))))));
    }

    /// java: private static SuggestionProvider<FabricClientCommandSource> getIdSuggestionProvider(ResourceKey<? extends Registry<?>> registryKey)
    pub fn get_id_suggestion_provider(registry_key: &ResourceKey) -> SuggestionProvider<mcrust::mcrust::modding::events::client_command_registration_callback::FabricClientCommandSource> {
        //   java: return (context, builder) -> context.getSource().suggestRegistryElements(registryKey, SharedSuggestionProvider.ElementSuggestionType.ELEMENTS, builder, context);
        // (the port's `suggestRegistryElements` takes the context's input, which is what it reads of the context)
        use mcrust::net::minecraft::commands::shared_suggestion_provider::SharedSuggestionProvider as _;
        let registry_key = registry_key.clone();
        Arc::new(move |context, builder| Ok(context.get_source().suggest_registry_elements(&registry_key, mcrust::net::minecraft::commands::shared_suggestion_provider::ElementSuggestionType::Elements, builder, context.get_input())))
    }

    /// java: @SuppressWarnings("SameReturnValue") private static int customizeTrim(FabricClientCommandSource source, Identifier material, Identifier pattern)
    pub fn customize_trim(source: &crate::unresolved::FabricClientCommandSource, material: Option<&Identifier>, pattern: Option<&Identifier>) -> i32 {
        // TODO(fabric2seer): `.getPlayer(..)` on a FabricClientCommandSource
        //   java: ItemStack heldItem = source.getPlayer().getMainHandItem();
        let mut held_item: mcrust::net::minecraft::world::item::item_stack::ItemStack = mcrust::mcrust::modding::game::with_player(|player| player.get_main_hand_item().clone()).unwrap_or_else(mcrust::net::minecraft::world::item::item_stack::empty);
        if Utils::is_on_skyblock() && true /* java: held_item != null: never null here */ {
            if held_item.is_in_tag(&mcrust::net::minecraft::tags::item_tags::trimmable_armor()) {
                let mut item_uuid: String = held_item.get_uuid();
                if !item_uuid.is_empty() {
                    let custom_armor_trims = &SkyblockerConfigManager::get().general.custom_armor_trims;
                    // (the command's bare form passes no material and no pattern)
                    if material.is_none() && pattern.is_none() {
                        if custom_armor_trims.contains_key(&item_uuid) {
                            SkyblockerConfigManager::update(Arc::new(move |mut config| { config.general.custom_armor_trims.remove(&item_uuid); }));
                            // TODO(fabric2seer): `.sendFeedback(..)` on a FabricClientCommandSource
                            //   java: source.sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.customArmorTrims.removed")));
                        } else {
                            // TODO(fabric2seer): `.sendFeedback(..)` on a FabricClientCommandSource
                            //   java: source.sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.customArmorTrims.neverHad")));
                        }
                    } else {
                        // (the command's other form passes both)
                        let mut trim_id: ArmorTrimId = ArmorTrimId::new(material.cloned().expect("NullPointerException: material"), pattern.cloned().expect("NullPointerException: pattern"));
                        if TRIMS_CACHE.get_mut().get(&trim_id).cloned().is_none() {
                            // TODO(fabric2seer): `.sendError(..)` on a FabricClientCommandSource
                            //   java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customArmorTrims.invalidMaterialOrPattern").withColor(TextColor.RED)));
                            return 1;
                        }
                        SkyblockerConfigManager::update(Arc::new(move |mut config| { config.general.custom_armor_trims.insert(item_uuid.clone(), trim_id.clone()); }));
                        // TODO(fabric2seer): `.sendFeedback(..)` on a FabricClientCommandSource
                        //   java: source.sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.customArmorTrims.added")));
                    }
                } else {
                    // TODO(fabric2seer): `.sendError(..)` on a FabricClientCommandSource
                    //   java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customArmorTrims.noItemUuid").withColor(TextColor.RED)));
                }
            } else {
                // TODO(fabric2seer): `.sendError(..)` on a FabricClientCommandSource
                //   java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customArmorTrims.notAnArmorPiece").withColor(TextColor.RED)));
                return 1;
            }
        } else {
            // TODO(fabric2seer): `.sendError(..)` on a FabricClientCommandSource
            //   java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customArmorTrims.unableToSetTrim").withColor(TextColor.RED)));
        }
        return 1;
    }

}

/// java: public record ArmorTrimId(Identifier material, Identifier pattern) implements Pair<Identifier, Identifier>
/// (a record: `equals` / `hashCode` over its two components)
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArmorTrimId {
    /// java: Identifier material
    pub material: Identifier,
    /// java: Identifier pattern
    pub pattern: Identifier,
}

impl ArmorTrimId {
    /// java: the canonical record constructor
    pub fn new(mut material: Identifier, mut pattern: Identifier) -> Self {
        Self { material, pattern }
    }

    /// java: the record accessor `material()`
    pub fn material(&self) -> Identifier {
        self.material.clone()
    }

    /// java: the record accessor `pattern()`
    pub fn pattern(&self) -> Identifier {
        self.pattern.clone()
    }

    /// java: @Override public Identifier left()
    pub fn left(&self) -> Identifier {
        return self.material();
    }

    /// java: @Override public Identifier right()
    pub fn right(&self) -> Identifier {
        return self.pattern();
    }

}

// Generated Gson forms for src/de/hysky/skyblocker/skyblock/item/custom/custom_armor_trims.rs (fabric2seer round 24): APPEND to that file.
// Needs `serde_json` in Cargo.toml and the `compat` of this round (dandelion::compat::GsonValue, ClassOf, ...).

/// ArmorTrimId's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl ArmorTrimId {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("material".to_string(), dandelion::compat::GsonValue::Str(((&self.material)).to_string())),
            ("pattern".to_string(), dandelion::compat::GsonValue::Str(((&self.pattern)).to_string())),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<ArmorTrimId> {
        let __o = __v.as_object()?;
        // TODO(fabric2seer): no no-argument constructor - Gson builds the object with every field at zero/null (Unsafe)
        None
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `ArmorTrimId.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<ArmorTrimId> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.skyblock.item.custom.CustomArmorTrims$ArmorTrimId",
            new: || panic!("InstantiationException: no no-argument constructor"),
            to_gson: |__x| __x.__gson_write(),
            read_gson: ArmorTrimId::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}
