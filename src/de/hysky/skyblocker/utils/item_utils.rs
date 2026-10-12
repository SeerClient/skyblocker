//! Ported from de.hysky.skyblocker.utils.ItemUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::debug::debug::DumpFormat;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack;
use crate::de::hysky::skyblocker::skyblock::item::pet_info;
use crate::de::hysky::skyblocker::skyblock::item::pet_info::PetInfo;
use crate::de::hysky::skyblocker::skyblock::item::skyblock_item_rarity::SkyblockItemRarity;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::craft_price_tooltip::CraftPriceTooltip;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::obtained_date_tooltip::ObtainedDateTooltip;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::bazaar_product::BazaarProduct;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use crate::de::hysky::skyblocker::utils::regex_list_utils::RegexListUtils;
use crate::de::hysky::skyblocker::utils::regex_utils::RegexUtils;
use crate::de::hysky::skyblocker::utils::utils;
use mcrust::com::mojang::authlib::game_profile::GameProfile;
use mcrust::com::mojang::authlib::properties::property_map::PropertyMap;
use mcrust::com::mojang::brigadier::builder::literal_argument_builder::LiteralArgumentBuilder;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::java::util::base64;
use mcrust::java::util::uuid::Uuid;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::client::gui::screens::inventory::container_screen::ContainerScreen;
use mcrust::net::minecraft::core::component::data_component_getter::DataComponentGetter;
use mcrust::net::minecraft::core::holder::Holder;
use mcrust::net::minecraft::nbt::compound_tag;
use mcrust::net::minecraft::nbt::tag::Tag;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::contents::objects::object_info::ObjectInfo;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::util;
use mcrust::net::minecraft::world::entity::equipment_slot::EquipmentSlot;
use mcrust::net::minecraft::world::entity::equipment_slot_group::EquipmentSlotGroup;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity;
use mcrust::net::minecraft::world::item::component::resolvable_profile::ResolvableProfile;
use mcrust::net::minecraft::world::item::item::Item;
use mcrust::net::minecraft::world::item::item_instance::ItemInstance;
use mcrust::net::minecraft::world::item::items;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::data_tooltip_info_type::DataTooltipInfoType as _;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::net::minecraft::core::typed_instance::TypedInstance;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public static final String ID = "id";
pub const ID: &str = "id";
/// java: public static final String UUID = "uuid";
pub const UUID: &str = "uuid";
/// java: public static final Pattern NOT_DURABILITY = Pattern.compile("[^0-9 /]");
pub static NOT_DURABILITY: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("[^0-9 /]"));
/// java: public static final Predicate<String> FUEL_PREDICATE = line -> line.contains("Fuel: ");
pub static FUEL_PREDICATE: compat::MainThread<Arc<dyn Fn(String) -> bool>> = compat::MainThread::new(|| Arc::new(move |line| line.contains("Fuel: ")));
/// java: private static final Codec<Holder<Item>> EMPTY_ALLOWING_ITEM_CODEC = BuiltInRegistries.ITEM.holderByNameCodec();
pub static EMPTY_ALLOWING_ITEM_CODEC: compat::MainThread<CodecRef<Holder>> = compat::MainThread::new(|| {
    let registry: Arc<dyn mcrust::net::minecraft::core::registry::Registry> = mcrust::net::minecraft::core::registries::built_in_registries::item().clone();
    mcrust::net::minecraft::core::registry::holder_by_name_codec(registry)
});
/// java: public static final Codec<ItemStack> EMPTY_ALLOWING_ITEMSTACK_CODEC = Codec.lazyInitialized(() -> RecordCodecBuilder.create(instance -> instance.group( EMPTY_ALLOWING_ITEM_CODEC.fieldOf("id").forGette
pub static EMPTY_ALLOWING_ITEMSTACK_CODEC: compat::MainThread<CodecRef<mcrust::net::minecraft::world::item::item_stack::ItemStack>> = compat::MainThread::new(|| {
    use mcrust::net::minecraft::world::item::item_stack::ItemStack;
    // `Codec.lazyInitialized`: the body is built on first use (it reads EMPTY_ALLOWING_ITEM_CODEC, captured here
    // because the render-thread static cannot be read from the codec's own call)
    let item_codec: CodecRef<Holder> = EMPTY_ALLOWING_ITEM_CODEC.get().clone();
    codec::lazy_initialized(move || {
        record_codec_builder::create(record_codec_builder::apply3(
            // EMPTY_ALLOWING_ITEM_CODEC.fieldOf("id").forGetter(ItemStack::typeHolder)
            record_codec_builder::of(|o: &ItemStack| TypedInstance::type_holder(o), codec::field_of(item_codec.clone(), "id")),
            // Codec.INT.orElse(1).fieldOf("count").forGetter(ItemStack::getCount)
            record_codec_builder::of(|o: &ItemStack| o.get_count(), codec::field_of(codec::map_result(codec::int_codec(), Arc::new(OrElse { value: 1 })), "count")),
            // DataComponentPatch.CODEC.optionalFieldOf("components", DataComponentPatch.EMPTY).forGetter(ItemStack::getComponentsPatch)
            record_codec_builder::of(|o: &ItemStack| o.get_components_patch(), codec::optional_field_of(mcrust::net::minecraft::core::component::data_component_patch::codec(), "components", mcrust::net::minecraft::core::component::data_component_patch::empty())),
            // ItemStack::new (Holder<Item>, int, DataComponentPatch)
            |item: Holder, count: i32, components: mcrust::net::minecraft::core::component::data_component_patch::DataComponentPatch| {
                let item: Arc<Item> = mcrust::net::minecraft::world::item::item::from_obj(&item.value()).expect("a Holder<Item>");
                ItemStack::with_patch(item, count, &components)
            },
        ))
    })
});

/// DFU `Codec.orElse(A value)` = `orElseGet(noop, () -> value)`'s `ResultFunction`: a decode that has no result
/// answers `success((value, input))`; the encode is unchanged.
struct OrElse<A> {
    value: A,
}

impl<A: Clone + Send + Sync + 'static> mcrust::com::mojang::serialization::codec::ResultFunction<A> for OrElse<A> {
    fn apply(&self, _ops: &mcrust::com::mojang::serialization::dynamic_ops::Ops, input: &mcrust::com::mojang::serialization::Obj, a: mcrust::com::mojang::serialization::data_result::DataResult<(A, mcrust::com::mojang::serialization::Obj)>) -> mcrust::com::mojang::serialization::data_result::DataResult<(A, mcrust::com::mojang::serialization::Obj)> {
        mcrust::com::mojang::serialization::data_result::DataResult::success(a.result().unwrap_or_else(|| (self.value.clone(), input.clone())))
    }

    fn co_apply(&self, _ops: &mcrust::com::mojang::serialization::dynamic_ops::Ops, _input: &A, t: mcrust::com::mojang::serialization::data_result::DataResult<mcrust::com::mojang::serialization::Obj>) -> mcrust::com::mojang::serialization::data_result::DataResult<mcrust::com::mojang::serialization::Obj> {
        t
    }
}
/// java: private static final Pattern STORED_PATTERN = Pattern.compile("Stored: ([\\d,]+)/\\S+");
pub static STORED_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Stored: ([\\d,]+)/\\S+"));
/// java: private static final Pattern GEMSTONES_SACK_AMOUNT_PATTERN = Pattern.compile(" Amount: ([\\d,]+)");
pub static GEMSTONES_SACK_AMOUNT_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(" Amount: ([\\d,]+)"));
/// java: private static final Pattern STASH_COUNT_PATTERN = Pattern.compile("x([\\d,]+)$");
pub static STASH_COUNT_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("x([\\d,]+)$"));
/// java: private static final Pattern COMPOSTER_COUNT_PATTERN = Pattern.compile("Compost Available: ([\\d,]+)");
pub static COMPOSTER_COUNT_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Compost Available: ([\\d,]+)"));
/// java: private static final Pattern HUNTING_BOX_COUNT_PATTERN = Pattern.compile("Owned: (?<shards>[\\d,]+) Shards?");
pub static HUNTING_BOX_COUNT_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Owned: (?<shards>[\\d,]+) Shards?"));
/// java: private static final short LOG_INTERVAL = 1000;
pub const LOG_INTERVAL: i16 = (1000) as i16;
/// java: private static long lastLog = Util.getMillis();
pub static LAST_LOG: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(util::get_millis()));

/// java: public final class ItemUtils
#[derive(Clone)]
pub struct ItemUtils {}

impl ItemUtils {
    /// java: private ItemUtils()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static LiteralArgumentBuilder<FabricClientCommandSource> dumpHeldItemCommand()
    pub fn dump_held_item_command() -> LiteralArgumentBuilder<crate::unresolved::FabricClientCommandSource> {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: return literal("dumpHeldItem").executes(context -> {
        //   java: 			context.getSource().sendFeedback(Component.literal("[Skyblocker Debug] Held Item: " + SkyblockerMod.GSON_COMPACT.toJson(ItemStack.CODEC.encodeStart(RegistryUtils.getRegistryWrapperLookup().createSerializationContext(JsonOps.INSTANCE), context.getSource().getPlayer().getMainHandItem()).getOrThrow())));
        //   java: 			return Command.SINGLE_SUCCESS;
        //   java: 		});
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static LiteralArgumentBuilder<FabricClientCommandSource> dumpHeldItemNetworthCalculationsCommand()
    pub fn dump_held_item_networth_calculations_command() -> LiteralArgumentBuilder<crate::unresolved::FabricClientCommandSource> {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: return literal("dumpHeldItemNetworthCalcs").executes(context -> {
        //   java: 			context.getSource().sendFeedback(Component.literal("[Skyblocker Debug] Held Item NW Calcs: " + SkyblockerMod.GSON_COMPACT.toJson(Calculation.LIST_CODEC.encodeStart(JsonOps.INSTANCE, NetworthCalculator.getItemNetworth(context.getSource().getPlayer().getMainHandItem()).calculations()).getOrThrow())));
        //   java: 			return Command.SINGLE_SUCCESS;
        //   java: 		});
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static CompoundTag getCustomData(DataComponentGetter stack)
    pub fn get_custom_data(stack: &dyn DataComponentGetter) -> Tag {
        // java: ((CustomDataAccessor) (Object) stack.getOrDefault(DataComponents.CUSTOM_DATA, CustomData.EMPTY)).getTag()
        let custom_data: mcrust::com::mojang::serialization::Obj = stack.get_or_default(mcrust::net::minecraft::core::component::data_components::custom_data(), mcrust::com::mojang::serialization::obj(mcrust::net::minecraft::world::item::component::custom_data::empty()));
        return mcrust::com::mojang::serialization::cast::<mcrust::net::minecraft::world::item::component::custom_data::CustomData>(&custom_data).expect("ClassCastException: CustomData").copy_tag();
    }

    /// java: public static Optional<String> getItemIdOptional(DataComponentGetter stack)
    pub fn get_item_id_optional(stack: &dyn DataComponentGetter) -> Option<String> {
        let mut custom_data: Tag = ItemUtils::get_custom_data(stack);
        // java: customData.getString(ID) - Optional<String>: present for a string tag only
        return match custom_data.get(ID) {
            Some(Tag::String(value)) => Some(value.clone()),
            _ => None,
        };
    }

    /// java: @Deprecated(since = "5.8.0") public static String getItemId(DataComponentGetter stack)
    pub fn get_item_id(stack: &dyn DataComponentGetter) -> String {
        return ItemUtils::get_custom_data(stack).get_string_or(ID, "");
    }

    /// java: public static Optional<String> getItemUuidOptional(DataComponentGetter stack)
    pub fn get_item_uuid_optional(stack: &dyn DataComponentGetter) -> Option<String> {
        let mut custom_data: Tag = ItemUtils::get_custom_data(stack);
        // java: customData.getString(UUID) - Optional<String>: present for a string tag only
        return match custom_data.get(UUID) {
            Some(Tag::String(value)) => Some(value.clone()),
            _ => None,
        };
    }

    /// java: @Deprecated(since = "5.8.0") public static String getItemUuid(DataComponentGetter stack)
    pub fn get_item_uuid(stack: &dyn DataComponentGetter) -> String {
        return ItemUtils::get_custom_data(stack).get_string_or(UUID, "");
    }

    /// java: @Deprecated(since = "5.8.0") public static String getSkyblockApiId(DataComponentGetter stack)
    pub fn get_skyblock_api_id(stack: &dyn DataComponentGetter) -> String {
        let mut custom_data: Tag = ItemUtils::get_custom_data(stack);
        let mut id: String = custom_data.get_string_or(ID, "");
        if custom_data.contains("is_shiny") {
            return format!("SHINY_{}", id);
        }
        if id.contains(";") && !(NEURepoManager::is_loading()) {
            // TODO(fabric2seer): `.getBazaarStocks(..)` on a NEUConstants
            //   java: return NEURepoManager.getConstants().getBazaarStocks().getBazaarStockOrDefault(id);
        }
        // java: switch (id) { .. } - arrow cases, no fall-through: a case whose test fails leaves the switch
        match id.as_str() {
            "ENCHANTED_BOOK" => {
                if custom_data.contains("enchantments") {
                    let enchants: Tag = custom_data.get_compound_or_empty("enchantments");
                    // java: enchants.keySet().stream().findFirst() - the port's compound keeps its keys sorted
                    let enchant: String = enchants.key_set().first().map(|k| k.to_string()).unwrap_or_default();
                    return format!("ENCHANTMENT_{}_{}", enchant.to_uppercase(), enchants.get_int_or(&enchant, 0));
                }
            }
            "PET" => {
                let pet_info: PetInfo = ItemUtils::get_pet_info(stack);
                return format!("LVL_1_{}_{}", pet_info.tier().to_string(), pet_info.r#type());
            }
            "POTION" => {
                let enhanced: &str = if custom_data.get_boolean_or("enhanced", false) { "_ENHANCED" } else { "" };
                let extended: &str = if custom_data.get_boolean_or("extended", false) { "_EXTENDED" } else { "" };
                let splash: &str = if custom_data.get_boolean_or("splash", false) { "_SPLASH" } else { "" };
                if custom_data.contains("potion") && custom_data.contains("potion_level") {
                    return format!("{}_{}_{}{}{}{}", custom_data.get_string_or("potion", ""), id, custom_data.get_int_or("potion_level", 0), enhanced, extended, splash).to_uppercase();
                }
            }
            "RUNE" | "UNIQUE_RUNE" => {
                if custom_data.contains("runes") {
                    let runes: Tag = custom_data.get_compound_or_empty("runes");
                    let rune: String = runes.key_set().first().map(|k| k.to_string()).unwrap_or_default();
                    return format!("{}_RUNE_{}", rune.to_uppercase(), runes.get_int_or(&rune, 0));
                }
            }
            // java: case String s when s.equals("ATTRIBUTE_SHARD") && stack instanceof DataComponentHolder holder
            "ATTRIBUTE_SHARD" if stack.as_any().is_some_and(|a| a.is::<mcrust::net::minecraft::world::item::item_stack::ItemStack>()) => {
                let holder = stack.as_any().and_then(|a| a.downcast_ref::<mcrust::net::minecraft::world::item::item_stack::ItemStack>()).unwrap();
                let attribute = crate::de::hysky::skyblocker::skyblock::hunting::attributes::Attributes::get_attribute_from_item_name(holder);
                if let Some(attribute) = attribute {
                    return attribute.api_id();
                }
            }
            "NEW_YEAR_CAKE" => {
                return format!("{}_{}", id, custom_data.get_int_or("new_years_cake", 0));
            }
            "PARTY_HAT_CRAB" | "PARTY_HAT_CRAB_ANIMATED" | "BALLOON_HAT_2024" | "BALLOON_HAT_2025" | "CAKE_HAT_2026" => {
                return format!("{}_{}", id, custom_data.get_string_or("party_hat_color", "").to_uppercase());
            }
            "PARTY_HAT_SLOTH" => {
                return format!("{}_{}", id, custom_data.get_string_or("party_hat_emoji", "").to_uppercase());
            }
            "MIDAS_SWORD" => {
                if custom_data.get_int_or("winning_bid", 0) >= 50000000 {
                    return format!("{}_50M", id);
                }
            }
            "MIDAS_STAFF" => {
                if custom_data.get_int_or("winning_bid", 0) >= 100000000 {
                    return format!("{}_100M", id);
                }
            }
            "FACTION_RABBIT" => {
                return format!("{}_{}", id, custom_data.get_string_or("faction_rabbit_id", "").to_uppercase());
            }
            "" => {
                // java: Screen currentScreen = Minecraft.getInstance().gui.screen(); `instanceof ContainerScreen` is the
                // screen's kind here, and its title the screen's title
                let container_title: Option<String> = container_screen_title();
                if container_title.as_ref().is_some_and(|t| t.starts_with("Superpairs")) {
                    let Some(lore) = stack.get(mcrust::net::minecraft::core::component::data_components::lore()) else {
                        return id;
                    };
                    let lore = mcrust::com::mojang::serialization::cast::<mcrust::net::minecraft::world::item::component::item_lore::ItemLore>(&lore).expect("ClassCastException: ItemLore");
                    let lines = lore.lines();
                    if lines.len() < 3 {
                        return id;
                    }
                    return crate::de::hysky::skyblocker::utils::enchanted_book_utils::EnchantedBookUtils::get_api_id_by_name(&compat::component_of(lines[2].clone()));
                }
                if container_title.as_ref().is_some_and(|t| t.contains("Experimentation Table RNG")) {
                    let stack_name: MutableComponent = custom_name_or_empty(stack);
                    return match stack_name.get_string().as_str() {
                        "Titanic Experience Bottle" => "TITANIC_EXP_BOTTLE".to_string(),
                        "Grand Experience Bottle" => "GRAND_EXP_BOTTLE".to_string(),
                        _ => crate::de::hysky::skyblocker::utils::enchanted_book_utils::EnchantedBookUtils::get_api_id_by_name(&stack_name),
                    };
                }
                if container_title.as_ref().is_some_and(|t| t.contains("Attribute Menu")) {
                    let stack_name: MutableComponent = custom_name_or_empty(stack);
                    let attribute = crate::de::hysky::skyblocker::skyblock::hunting::attributes::Attributes::get_attribute_from_ability_name(&stack_name.get_string());
                    if let Some(attribute) = attribute {
                        return attribute.api_id();
                    }
                    return id;
                }
                // java: if (stack instanceof ItemStack realStack && realStack.has(DataComponents.CUSTOM_NAME))
                if let Some(real_stack) = stack.as_any().and_then(|a| a.downcast_ref::<mcrust::net::minecraft::world::item::item_stack::ItemStack>())
                    && stack.get(mcrust::net::minecraft::core::component::data_components::custom_name()).is_some()
                {
                    let stack_name: MutableComponent = custom_name_or_empty(stack);
                    // Enchanted Books in the Bazaar
                    if real_stack.is_value(&items::items().get("enchanted_book").erased_obj()) {
                        return crate::de::hysky::skyblocker::utils::enchanted_book_utils::EnchantedBookUtils::get_api_id_by_name(&stack_name);
                    }
                    // Essences
                    if real_stack.is_value(&items::items().get("player_head").erased_obj()) {
                        let mut matcher: compat::regex::Matcher = crate::de::hysky::skyblocker::skyblock::chest_value::ESSENCE_PATTERN.matcher(&stack_name.get_string());
                        if matcher.find() {
                            return format!("ESSENCE_{}", matcher.group_named("type").unwrap_or_default().to_uppercase());
                        }
                    }
                }
            }
            _ => {}
        }
        return id;
    }

    /// java: @Deprecated(since = "5.8.0") public static String getNeuId(DataComponentGetter stack)
    pub fn get_neu_id(stack: &dyn DataComponentGetter) -> String {
        if false /* java: stack == null: never null here */ {
            return "".to_string();
        }
        let mut custom_data: Tag = ItemUtils::get_custom_data(stack);
        let mut id: String = custom_data.get_string_or(ID, "");
        // java: return switch (id) { .. };
        return match id.as_str() {
            "ENCHANTED_BOOK" => {
                let enchantments: Tag = custom_data.get_compound_or_empty("enchantments");
                // java: enchantments.keySet().stream().findFirst() - the port's compound keeps its keys sorted
                let enchant: String = enchantments.key_set().first().map(|k| k.to_string()).unwrap_or_default();
                format!("{};{}", enchant.to_uppercase(), enchantments.get_int_or(&enchant, 0))
            }
            "PET" => {
                let pet_info: PetInfo = ItemUtils::get_pet_info(stack);
                format!("{};{}", pet_info.r#type(), pet_info.tier_index())
            }
            "RUNE" | "UNIQUE_RUNE" => {
                let runes: Tag = custom_data.get_compound_or_empty("runes");
                let rune: String = runes.key_set().first().map(|k| k.to_string()).unwrap_or_default();
                format!("{}_RUNE;{}", rune.to_uppercase(), runes.get_int_or(&rune, 0))
            }
            "POTION" => format!("POTION_{};{}", custom_data.get_string_or("potion", "").to_uppercase(), custom_data.get_int_or("potion_level", 0)),
            // java: case String s when s.equals("ATTRIBUTE_SHARD") && stack instanceof DataComponentHolder holder
            "ATTRIBUTE_SHARD" if stack.as_any().is_some_and(|a| a.is::<mcrust::net::minecraft::world::item::item_stack::ItemStack>()) => {
                let holder = stack.as_any().and_then(|a| a.downcast_ref::<mcrust::net::minecraft::world::item::item_stack::ItemStack>()).unwrap();
                match crate::de::hysky::skyblocker::skyblock::hunting::attributes::Attributes::get_attribute_from_item_name(holder) {
                    None => id,
                    Some(attribute) => attribute.neu_id(),
                }
            }
            "PARTY_HAT_CRAB" | "BALLOON_HAT_2024" | "BALLOON_HAT_2025" | "CAKE_HAT_2026" => format!("{}_{}", id, custom_data.get_string_or("party_hat_color", "").to_uppercase()),
            "PARTY_HAT_CRAB_ANIMATED" => format!("PARTY_HAT_CRAB_{}_ANIMATED", custom_data.get_string_or("party_hat_color", "").to_uppercase()),
            "PARTY_HAT_SLOTH" => format!("{}_{}", id, custom_data.get_string_or("party_hat_emoji", "").to_uppercase()),
            "FACTION_RABBIT" => format!("{}_{}", id, custom_data.get_string_or("faction_rabbit_id", "").to_uppercase()),
            _ => id.replace(":", "-"),
        };
    }

    /// java: public static @NEUId String getNeuIdFromApiId(String apiId)
    pub fn get_neu_id_from_api_id(api_id: &str) -> String {
        if api_id.starts_with("LVL_") {
            let mut parts: Vec<String> = compat::regex::split_limit(&api_id, "_", 4);
            if (parts.len() as i32) != 4 {
                return api_id.to_string();
            }
            let mut rarity: Option<SkyblockItemRarity> = SkyblockItemRarity::contains_name(&parts[(2) as usize]);
            if rarity.is_none() {
                return api_id.to_string();
            }
            return format!("{};{}+{}", parts[(3) as usize], rarity.clone().unwrap().ordinal(), parts[(1) as usize]);
        }
        if api_id.contains("_POTION_") {
            let mut parts: Vec<String> = compat::regex::split_limit(&api_id, "_POTION_", 2);
            if (parts.len() as i32) != 2 {
                return api_id.to_string();
            }
            let mut potion_name: String = parts[(0) as usize].clone();
            parts = compat::regex::split_limit(&(parts[(1) as usize]), "_", 2);
            let mut potion_level: String = parts[(0) as usize].clone();
            return format!("POTION_{};{}", potion_name, potion_level);
        }
        return api_id.to_string();
    }

    /// java: @Deprecated(since = "5.8.0") public static PetInfo getPetInfo(DataComponentGetter stack)
    pub fn get_pet_info(stack: &dyn DataComponentGetter) -> PetInfo {
        if !(ItemUtils::get_custom_data(stack).get_string_or(ID, "") == "PET") {
            return pet_info::EMPTY.clone();
        }
        let mut pet_info: String = ItemUtils::get_custom_data(stack).get_string_or("petInfo", "");
        if !pet_info.is_empty() {
            // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut json_element: serde_json::Value = serde_json::from_str::<serde_json::Value>(&pet_info).expect("JsonSyntaxException");
                // java: if (stack instanceof ItemStack itemStack) jsonElement.getAsJsonObject().addProperty("name", itemStack.getHoverName().getString());
                if let Some(item_stack) = stack.as_any().and_then(|a| a.downcast_ref::<mcrust::net::minecraft::world::item::item_stack::ItemStack>()) {
                    let name: String = compat::component_of(item_stack.get_hover_name()).get_string();
                    json_element.as_object_mut().expect("IllegalStateException: Not a JSON Object").insert("name".to_string(), serde_json::Value::String(name));
                }
                return (pet_info::CODEC.get_mut().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&json_element))).set_partial(pet_info::EMPTY.clone()).get_partial_or_throw().unwrap();
            })) {
                Ok(__v) => return __v,
                Err(_) => {
                }
            }
        }
        return pet_info::EMPTY.clone();
    }

    /// java: public static OptionalDouble getItemPrice(SkyblockerStack stack)
    pub fn get_item_price(stack: &dyn SkyblockerStack) -> Option<f64> {
        return ItemUtils::get_item_price_2(Some(stack.get_skyblock_api_id()), false);
    }

    /// java: public static OptionalDouble getItemPrice(@Nullable String skyblockApiId)
    pub fn get_item_price_string(mut skyblock_api_id: Option<String>) -> Option<f64> {
        return ItemUtils::get_item_price_2(skyblock_api_id.clone(), false);
    }

    /// java: public static OptionalDouble getItemPrice(@Nullable String skyblockApiId, boolean useBazaarBuyPrice)
    pub fn get_item_price_2(mut skyblock_api_id: Option<String>, mut use_bazaar_buy_price: bool) -> Option<f64> {
        return ItemUtils::get_item_price_3(skyblock_api_id.clone(), use_bazaar_buy_price, false);
    }

    /// java: public static OptionalDouble getItemPrice(@Nullable String skyblockApiId, boolean useBazaarBuyPrice, boolean useAuctionAverage)
    pub fn get_item_price_3(mut skyblock_api_id: Option<String>, mut use_bazaar_buy_price: bool, mut use_auction_average: bool) -> Option<f64> {
        let mut bazaar_prices: Option<std::collections::HashMap<String, BazaarProduct>> = tooltip_info_type::BAZAAR.get_mut().lock().borrow().get_data();
        let mut three_day_average_prices: Option<std::collections::HashMap<String, f64>> = tooltip_info_type::THREE_DAY_AVERAGE.get_mut().lock().borrow().get_data();
        let mut lowest_bin_prices: Option<std::collections::HashMap<String, f64>> = tooltip_info_type::LOWEST_BINS.get_mut().lock().borrow().get_data();
        if skyblock_api_id.is_none() || skyblock_api_id.as_ref().unwrap().is_empty() {
            return None;
        }
        if bazaar_prices.is_some() && bazaar_prices.as_ref().unwrap().contains_key(skyblock_api_id.as_ref().unwrap()) {
            let mut product: BazaarProduct = bazaar_prices.as_ref().unwrap().get(skyblock_api_id.as_ref().unwrap()).cloned().unwrap();
            return (if use_bazaar_buy_price { product.buy_price() } else { product.sell_price() });
        }
        if use_auction_average && three_day_average_prices.is_some() && three_day_average_prices.as_ref().unwrap().contains_key(skyblock_api_id.as_ref().unwrap()) {
            return Some(three_day_average_prices.as_ref().unwrap().get(skyblock_api_id.as_ref().unwrap()).copied().unwrap_or_default());
        }
        if lowest_bin_prices.is_some() && lowest_bin_prices.as_ref().unwrap().contains_key(skyblock_api_id.as_ref().unwrap()) {
            return Some(lowest_bin_prices.as_ref().unwrap().get(skyblock_api_id.as_ref().unwrap()).copied().unwrap_or_default());
        }
        return None;
    }

    /// java: public static double getCraftCost(String neuId)
    pub fn get_craft_cost(neu_id: &str) -> f64 {
        return CraftPriceTooltip::get_item_cost(neu_id);
    }

    /// java: @Deprecated(since = "1.21.0") public static String getTimestamp(ItemStack stack)
    pub fn get_timestamp(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> String {
        return ObtainedDateTooltip::get_timestamp(stack);
    }

    /// java: public static boolean hasCustomDurability(ItemStack stack)
    pub fn has_custom_durability(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        let mut custom_data: Tag = ItemUtils::get_custom_data(stack);
        // java: return !customData.isEmpty() && customData.contains("drill_fuel") || stack.getSkyblockId().equals("PICKONIMBUS");
        return (!custom_data.compound_is_empty() && custom_data.contains("drill_fuel")) || stack.get_skyblock_id() == "PICKONIMBUS";
    }

    /// java: public static @Nullable IntIntPair getDurability(ItemStack stack)
    pub fn get_durability(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> Option<(i32, i32)> {
        let mut custom_data: Tag = ItemUtils::get_custom_data(stack);
        // java: if (customData.isEmpty())
        if custom_data.compound_size() == 0 {
            return None;
        }
        if stack.get_skyblock_id() == "PICKONIMBUS" {
            let mut pickonimbus_durability: i32 = custom_data.get_int_or("pickonimbus_durability", 0);
            return Some(((if custom_data.contains("pickonimbus_durability") { pickonimbus_durability } else { 2000 }), 2000));
        }
        let mut drill_fuel: Option<String> = Some(chat_formatting::strip_formatting(ItemUtils::get_lore_line_if(stack, FUEL_PREDICATE.get_mut().clone()).as_deref().unwrap()));
        if let Some(drill_fuel) = &drill_fuel {
            let mut drill_fuel_strings: Vec<String> = compat::regex::split(&compat::trim(&NOT_DURABILITY.matcher(drill_fuel).replace_all("")), "/");
            return Some(((drill_fuel_strings[(0) as usize]).parse::<i32>().expect("NumberFormatException"), ((drill_fuel_strings[(1) as usize]).parse::<i32>().expect("NumberFormatException")).wrapping_mul(1000)));
        }
        return None;
    }

    /// java: public static @Nullable String getLoreLineContains(ItemStack stack, String substring)
    pub fn get_lore_line_contains(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, substring: &str) -> Option<String> {
        for line in stack.skyblocker_get_lore_strings() {
            if line.contains(substring) {
                return Some(line.clone());
            }
        }
        return None;
    }

    /// java: public static @Nullable String getLoreLineIf(ItemStack stack, Predicate<String> predicate)
    pub fn get_lore_line_if(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, predicate: Arc<dyn Fn(String) -> bool>) -> Option<String> {
        for line in stack.skyblocker_get_lore_strings() {
            if (predicate)(line.clone()) {
                return Some(line.clone());
            }
        }
        return None;
    }

    /// java: public static @Nullable Matcher getLoreLineIfMatch(ItemStack stack, Pattern pattern)
    pub fn get_lore_line_if_match(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, pattern: &compat::regex::Pattern) -> Option<compat::regex::Matcher> {
        return RegexListUtils::match_in_list(&stack.skyblocker_get_lore_strings(), pattern);
    }

    /// java: public static List<Matcher> getLoreLineIfMatch(ItemStack stack, Pattern... patterns)
    pub fn get_lore_line_if_match_item_stack_pattern(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, patterns: &Vec<compat::regex::Pattern>) -> Vec<compat::regex::Matcher> {
        return RegexListUtils::match_in_list_list_pattern(&stack.skyblocker_get_lore_strings(), patterns);
    }

    /// java: public static @Nullable Matcher getLoreLineIfContainsMatch(ItemStack stack, Pattern pattern)
    pub fn get_lore_line_if_contains_match(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, pattern: &compat::regex::Pattern) -> Option<compat::regex::Matcher> {
        return RegexListUtils::find_in_list(&stack.skyblocker_get_lore_strings(), pattern);
    }

    /// java: @Deprecated public static List<Component> getLore(DataComponentGetter stack)
    pub fn get_lore(stack: &dyn DataComponentGetter) -> Vec<MutableComponent> {
        // java: return stack.getOrDefault(DataComponents.LORE, ItemLore.EMPTY).styledLines();
        let lore: mcrust::com::mojang::serialization::Obj = stack.get_or_default(mcrust::net::minecraft::core::component::data_components::lore(), mcrust::com::mojang::serialization::obj(mcrust::net::minecraft::world::item::component::item_lore::empty()));
        let lore = mcrust::com::mojang::serialization::cast::<mcrust::net::minecraft::world::item::component::item_lore::ItemLore>(&lore).expect("ClassCastException: ItemLore");
        return lore.styled_lines().iter().map(|line| compat::component_of(line.clone())).collect();
    }

    /// java: public static PropertyMap propertyMapWithTexture(String textureValue)
    pub fn property_map_with_texture(texture_value: &str) -> PropertyMap {
        return (mcrust::net::minecraft::util::extra_codecs::property_map().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&serde_json::from_str::<serde_json::Value>(&format!("[{{\"name\":\"textures\",\"value\":\"{}\"}}]", texture_value)).expect("JsonSyntaxException")))).get_or_throw().unwrap();
    }

    /// java: public static String getHeadTexture(ItemInstance stack)
    pub fn get_head_texture<S: ItemInstance + TypedInstance + ?Sized>(stack: &S) -> String {
        // java: if (!stack.is(Items.PLAYER_HEAD)) return "";
        if !(stack.is_value(&items::items().get("player_head").erased_obj())) {
            return "".to_string();
        }
        let Some(mut profile) = DataComponentGetter::get(stack, mcrust::net::minecraft::core::component::data_components::profile()) else {
            return "".to_string();
        };
        let profile: &ResolvableProfile = mcrust::com::mojang::serialization::cast::<ResolvableProfile>(&profile).expect("ClassCastException: ResolvableProfile");
        // java: return profile.partialProfile().properties().get("textures").stream().filter(Objects::nonNull)
        //           .map(Property::value).findFirst().orElse("");
        return profile.partial_profile().properties().get("textures").into_iter().map(|property| property.value().to_string()).next().unwrap_or_default();
    }

    /// java: public static Optional<String> getHeadTextureOptional(ItemStack stack)
    pub fn get_head_texture_optional(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> Option<String> {
        let mut texture: String = ItemUtils::get_head_texture(stack);
        if compat::is_blank(&texture) {
            return None;
        }
        return Some(texture.clone());
    }

    /// java: public static String toTextureBase64(String textureUUID)
    pub fn to_texture_base64(texture_uuid: &str) -> String {
        let mut str: String = format!("{{textures:{{SKIN:{{url:\"http://textures.minecraft.net/texture/{}\"}}}}}}", texture_uuid);
        // java: return Base64.getEncoder().encodeToString(str.getBytes()); - `getBytes()` in the default charset (UTF-8)
        return base64::get_encoder().encode_to_string(str.as_bytes());
    }

    /// java: public static FlexibleItemStack createSkull(String textureBase64)
    pub fn create_skull(texture_base64: &str) -> compat::Shared<FlexibleItemStack> {
        let mut profile: GameProfile = GameProfile::new(Uuid::random_uuid(), "a", ItemUtils::property_map_with_texture(texture_base64));
        return ItemUtils::create_skull_game_profile(&profile);
    }

    /// java: public static FlexibleItemStack createSkull(GameProfile profile)
    pub fn create_skull_game_profile(profile: &GameProfile) -> compat::Shared<FlexibleItemStack> {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stack: compat::Shared<FlexibleItemStack> = FlexibleItemStack::new2(items::items().get("player_head"));
            FlexibleItemStack::set(&stack, mcrust::net::minecraft::core::component::data_components::profile().clone(), ResolvableProfile::create_resolved(profile.clone()).unwrap());
            return stack;
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                panic!("RuntimeException: {}", compat::panic_message(&e));
            }
        }
    }

    /// java: public static FlexibleItemStack getSkyblockerStack()
    pub fn get_skyblocker_stack() -> compat::Shared<FlexibleItemStack> {
        return ItemUtils::create_skull("e3RleHR1cmVzOntTS0lOOnt1cmw6Imh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvZDdjYzY2ODc0MjNkMDU3MGQ1NTZhYzUzZTA2NzZjYjU2M2JiZGQ5NzE3Y2Q4MjY5YmRlYmVkNmY2ZDRlN2JmOCJ9fX0=");
    }

    /// java: public static FlexibleItemStack getSkyblockerForgeStack()
    pub fn get_skyblocker_forge_stack() -> compat::Shared<FlexibleItemStack> {
        return ItemUtils::create_skull("eyJ0ZXh0dXJlcyI6eyJTS0lOIjp7InVybCI6Imh0dHBzOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlLzJkZGQ4OWE2YWU5NTdmNzY2ZDMwMDAxMWZmNDQ3MTQ4MWMzYmI2MWI2NzYwNzhhOGM2YzNjNDA4MzIwMWI1YzIifX19");
    }

    /// java: public static FlexibleItemStack getSkyblockerKatStack()
    pub fn get_skyblocker_kat_stack() -> compat::Shared<FlexibleItemStack> {
        return ItemUtils::create_skull("eyJ0ZXh0dXJlcyI6eyJTS0lOIjp7InVybCI6Imh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvZWFhZTgxMjc3NTcwNmI1NjU5NjY4MzQ4NmFiZWFmODU3ZWExYzA2OGNiMzRhOGJjMWRlOWE1N2M2MzhjZjQxMSIsIm1ldGFkYXRhIjp7fX19LCJwcm9maWxlSWQiOiIzNjYwYWEzNzBiYjAyZjk1YTQwNTRmNTVmODlhYTI5ZCIsInByb2ZpbGVOYW1lIjoibmVhODkiLCJpc1B1YmxpYyI6dHJ1ZSwidGltZXN0YW1wIjoxNzcxOTU3MjE2NDE4fQ==");
    }

    /// java: public static String getConcatenatedLore(ItemStack item)
    pub fn get_concatenated_lore(item: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> String {
        return ItemUtils::concatenate_lore(&ItemUtils::get_lore(item));
    }

    /// java: public static String concatenateLore(List<Component> lore)
    pub fn concatenate_lore(lore: &Vec<MutableComponent>) -> String {
        let mut string_builder: String = String::new();
        for i in 0..(lore.len() as i32) {
            { string_builder.push_str(&(lore[(i) as usize]).get_string()); };
            if i != (lore.len() as i32).wrapping_sub(1) {
                { string_builder.push_str(" "); };
            }
        }
        return string_builder.clone();
    }

    /// java: public static boolean isSoulbound(ItemStack stack)
    pub fn is_soulbound(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        return ItemUtils::get_lore(stack).into_iter().any(|lore| lore.get_string().to_lowercase().contains("soulbound"));
    }

    /// java: public static List<ItemStack> getArmor(LivingEntity entity)
    pub fn get_armor(entity: &dyn LivingEntity) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        // java: return EquipmentSlotGroup.ARMOR.slots().stream()
        //   .filter(es -> es.getType() == EquipmentSlot.Type.HUMANOID_ARMOR).map(entity::getItemBySlot).toList();
        return mcrust::net::minecraft::world::entity::equipment_slot_group::EquipmentSlotGroup::Armor
            .slots()
            .iter()
            .filter(|es| es.get_type() == mcrust::net::minecraft::world::entity::equipment_slot::Type::HumanoidArmor)
            .map(|es| entity.living().get_item_by_slot(*es).clone())
            .collect();
    }

    /// java: public static OptionalInt getItemCountInSack(ItemStack itemStack, List<String> lines)
    pub fn get_item_count_in_sack(item_stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, lines: &Vec<String>) -> Option<i32> {
        // java: lines.size() < 2 || !Strings.CS.endsWithAny(lines.getFirst(), "Sack", "Gemstones") - case-sensitive
        if (lines.len() as i32) < 2 || !["Sack", "Gemstones"].iter().any(|suffix| lines[0].ends_with(suffix)) {
            return None;
        }
        if compat::component_of(item_stack.get_hover_name()).get_string().ends_with("Gemstone") {
            let mut matcher: Option<compat::regex::Matcher> = RegexListUtils::match_in_list(lines, &GEMSTONES_SACK_AMOUNT_PATTERN);
            if let Some(matcher) = &matcher {
                return RegexUtils::parse_optional_int_from_matcher(matcher, 1);
            }
        }
        let mut matcher: Option<compat::regex::Matcher> = RegexListUtils::match_in_list(lines, &STORED_PATTERN);
        if let Some(matcher) = &matcher {
            return RegexUtils::parse_optional_int_from_matcher(matcher, 1);
        }
        if (util::get_millis()).wrapping_sub(compat::get_state(&LAST_LOG)) > (LOG_INTERVAL as i64) {
            logger().warn(&format!("Failed to find stored amount in sack tooltip for item `{}`", DumpFormat::Json.format(item_stack).get_string()));
            compat::set_state(&LAST_LOG, util::get_millis());
        }
        return None;
    }

    /// java: public static OptionalInt getItemCountInStash(ItemStack itemStack)
    pub fn get_item_count_in_stash(item_stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> Option<i32> {
        return ItemUtils::get_item_count_in_stash_component(&compat::component_of(item_stack.get_hover_name()));
    }

    /// java: public static OptionalInt getItemCountInStash(Component itemName)
    pub fn get_item_count_in_stash_component(item_name: &MutableComponent) -> Option<i32> {
        return RegexUtils::find_int_from_matcher(&mut STASH_COUNT_PATTERN.matcher(&item_name.get_string()));
    }

    /// java: public static OptionalInt getCompostCountInComposter(List<String> lines)
    pub fn get_compost_count_in_composter(lines: &Vec<String>) -> Option<i32> {
        let mut matcher: Option<compat::regex::Matcher> = RegexListUtils::match_in_list(lines, &COMPOSTER_COUNT_PATTERN);
        return (if matcher.is_some() { RegexUtils::parse_optional_int_from_matcher(matcher.as_ref().unwrap(), 1) } else { None });
    }

    /// java: public static OptionalInt getItemCountInHuntingBox(ItemStack stack)
    pub fn get_item_count_in_hunting_box(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> Option<i32> {
        let mut matcher: Option<compat::regex::Matcher> = ItemUtils::get_lore_line_if_contains_match(stack, &HUNTING_BOX_COUNT_PATTERN);
        return (if matcher.is_some() { RegexUtils::parse_optional_int_from_matcher_match_result_string(matcher.as_ref().unwrap(), "shards") } else { None });
    }

    /// java: public static OptionalInt getItemCountInSuperpairs(ItemStack stack)
    pub fn get_item_count_in_superpairs(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> Option<i32> {
        let mut current_screen: Option<&mcrust::net::minecraft::client::gui::screens::screen::Screen> = compat::minecraft().gui.screen();
        // java: currentScreen instanceof ContainerScreen container - the port's screen is the husk; a ContainerScreen is
        // the screen over the player's open ChestMenu
        if let Some(container) = current_screen.filter(|_| mcrust::mcrust::modding::game::with_chest_menu(|_| ()).is_some()) {
            if container.title.starts_with("Superpairs") {
                if compat::component_of(stack.get_hover_name()).get_string().contains("Enchanted Book") {
                    return Some(1);
                }
            } else if container.title.contains("Experimentation Table RNG") {
                return Some(1);
            }
        }
        return None;
    }

    /// java: @Deprecated(since = "5.8.0") public static <T extends ItemInstance & SkyblockerStack> SkyblockItemRarity getItemRarity(T stack)
    pub fn get_item_rarity<T: ItemInstance + TypedInstance + SkyblockerStack + 'static>(stack: &T) -> SkyblockItemRarity {
        // java: if (stack.is(Items.AIR)) return SkyblockItemRarity.UNKNOWN;
        if stack.is_value(&items::items().get("air").erased_obj()) {
            return SkyblockItemRarity::Unknown;
        }
        if ItemUtils::get_custom_data(stack).get_string_or(ID, "") == "PET" {
            // java: PetInfo info = stack.getPetInfo();
            let mut info: PetInfo = stack.get_pet_info();
            if info.is_empty() {
                return SkyblockItemRarity::Unknown;
            }
            return (if info.item().is_some() && info.item().unwrap() == "PET_ITEM_TIER_BOOST" { info.rarity().next() } else { info.rarity() });
        }
        // java: Optional<SkyblockItemRarity> rarity = ItemUtils.getLore(stack).reversed().stream().map(Component::getString)
        //           .map(SkyblockItemRarity::containsName).flatMap(Optional::stream).findFirst();
        let mut rarity: Option<SkyblockItemRarity> = ItemUtils::get_lore(stack).iter().rev().map(|line| line.get_string()).filter_map(|line| SkyblockItemRarity::contains_name(&line)).next();
        if rarity.is_some() {
            return rarity.clone().unwrap();
        }
        // java: Identifier tooltipStyle = stack.get(DataComponents.TOOLTIP_STYLE);
        let mut tooltip_style: Option<Identifier> = DataComponentGetter::get(stack, mcrust::net::minecraft::core::component::data_components::tooltip_style()).and_then(|value| mcrust::com::mojang::serialization::cast::<Identifier>(&value).cloned());
        if let Some(tooltip_style) = tooltip_style.as_ref().filter(|tooltip_style| tooltip_style.namespace() == utils::HYPIXEL_SKYBLOCK_NAMESPACE) {
            return SkyblockItemRarity::contains_name(&tooltip_style.path().to_uppercase()).unwrap_or(SkyblockItemRarity::Unknown);
        }
        return SkyblockItemRarity::Unknown;
    }

    /// java: public static FlexibleItemStack getNamedPlaceholder(String itemName)
    pub fn get_named_placeholder(item_name: &str) -> compat::Shared<FlexibleItemStack> {
        let mut stack: compat::Shared<FlexibleItemStack> = FlexibleItemStack::copy(ico::BARRIER.get_mut());
        FlexibleItemStack::set(&stack, mcrust::net::minecraft::core::component::data_components::custom_name().clone(), component::literal(item_name));
        return stack;
    }

    /// java: public static FlexibleItemStack getItemIdPlaceholder(String itemId)
    pub fn get_item_id_placeholder(item_id: &str) -> compat::Shared<FlexibleItemStack> {
        let mut stack: compat::Shared<FlexibleItemStack> = FlexibleItemStack::copy(ico::POISONOUS_POTATO.get_mut());
        FlexibleItemStack::set(&stack, mcrust::net::minecraft::core::component::data_components::item_name().clone(), component::literal(item_id));
        // java: stack.set(DataComponents.CUSTOM_DATA, CustomData.of(Util.make(new CompoundTag(), c -> c.putString(ID, itemId))));
        let mut tag: Tag = compound_tag::compound();
        tag.put_string(ID, item_id);
        FlexibleItemStack::set(&stack, mcrust::net::minecraft::core::component::data_components::custom_data().clone(), mcrust::net::minecraft::world::item::component::custom_data::CustomData::of(&tag));
        return stack;
    }

    /// java: public static MutableComponent getIcon(ItemInstance itemInstance)
    pub fn get_icon<S: ItemInstance + TypedInstance + ?Sized>(item_instance: &S) -> MutableComponent {
        let mut item: Arc<Item> = mcrust::net::minecraft::world::item::item::from_obj(&item_instance.type_holder().value()).expect("a Holder<Item>");
        // TODO(fabric2seer): a pattern switch expression
        //   java: ObjectInfo objectInfo = switch (item) {
        //   java: 			case BlockItem _ when itemInstance.is(Items.PLAYER_HEAD) && itemInstance.get(DataComponents.PROFILE) != null -> new PlayerSprite(itemInstance.get(DataComponents.PROFILE), true);
        //   java: 			case BlockItem _ -> new AtlasSprite(AtlasIds.BLOCKS, BuiltInRegistries.ITEM.getKey(item).withPath(itemId -> "block/" + itemId));
        //   java: 			default -> new AtlasSprite(AtlasIds.ITEMS, BuiltInRegistries.ITEM.getKey(item).withPath(itemId -> "item/" + itemId));
        //   java: 		};
        use mcrust::net::minecraft::core::registry::Registry as _;
        // `case BlockItem _` is `item.block().is_some()` in the port (`Item.block` is BlockItem's field)
        let item_key: Identifier = mcrust::net::minecraft::core::registries::built_in_registries::item().get_key(&item.erased_obj()).expect("a registered item");
        let profile: Option<mcrust::net::minecraft::world::item::component::resolvable_profile::ResolvableProfile> = item_instance
            .get(mcrust::net::minecraft::core::component::data_components::profile())
            .and_then(|o| mcrust::com::mojang::serialization::cast::<mcrust::net::minecraft::world::item::component::resolvable_profile::ResolvableProfile>(&o).cloned());
        let mut object_info: Arc<dyn ObjectInfo> = if item.block().is_some() && item_instance.is_value(&items::items().get("player_head").erased_obj()) && profile.is_some() {
            Arc::new(mcrust::net::minecraft::network::chat::contents::objects::player_sprite::PlayerSprite::new(profile.unwrap(), true))
        } else if item.block().is_some() {
            Arc::new(mcrust::net::minecraft::network::chat::contents::objects::atlas_sprite::AtlasSprite::new(mcrust::net::minecraft::data::atlas_ids::blocks(), item_key.with_path_op(|item_id| format!("block/{}", item_id))))
        } else {
            Arc::new(mcrust::net::minecraft::network::chat::contents::objects::atlas_sprite::AtlasSprite::new(mcrust::net::minecraft::data::atlas_ids::items(), item_key.with_path_op(|item_id| format!("item/{}", item_id))))
        };
        return component::object(object_info);
    }

}

/// `Minecraft.getInstance().gui.screen()` when it is a `ContainerScreen`: its title (the port names the open
/// screen by its kind).
fn container_screen_title() -> Option<String> {
    let client = compat::minecraft();
    if !matches!(client.gui.screen_kind, mcrust::net::minecraft::client::gui::gui::ScreenKind::Container) {
        return None;
    }
    client.gui.screen().map(|screen| screen.get_title().to_string())
}

/// `stack.getOrDefault(DataComponents.CUSTOM_NAME, Component.empty())`.
fn custom_name_or_empty(stack: &dyn DataComponentGetter) -> MutableComponent {
    match stack.get(mcrust::net::minecraft::core::component::data_components::custom_name()) {
        Some(name) => match mcrust::com::mojang::serialization::cast::<mcrust::net::minecraft::network::chat::formatted_text::FormattedText>(&name) {
            Some(text) => compat::component_of(text.clone()),
            None => component::empty(),
        },
        None => component::empty(),
    }
}
