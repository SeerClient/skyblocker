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
// TODO(fabric2seer): `holder_by_name_codec` takes 1 argument(s) in the port (`registry: Arc<dyn Registry>`), Java passes 0
//   java: private static final Codec<Holder<Item>> EMPTY_ALLOWING_ITEM_CODEC = BuiltInRegistries.ITEM.holderByNameCodec();
/// java: private static final Codec<Holder<Item>> EMPTY_ALLOWING_ITEM_CODEC = BuiltInRegistries.ITEM.holderByNameCodec();
pub static EMPTY_ALLOWING_ITEM_CODEC: compat::MainThread<CodecRef<Holder>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `EMPTY_ALLOWING_ITEM_CODEC` - see the TODO above"));
// TODO(fabric2seer): `PrimitiveCodec.orElse` has no Rust method in the port (looked for `or_else` in PrimitiveCodec and its supertypes)
//   java: public static final Codec<ItemStack> EMPTY_ALLOWING_ITEMSTACK_CODEC = Codec.lazyInitialized(() -> RecordCodecBuilder.create(instance -> instance.group(
//   java: 			EMPTY_ALLOWING_ITEM_CODEC.fieldOf("id").forGetter(ItemStack::typeHolder),
//   java: 			Codec.INT.orElse(1).fieldOf("count").forGetter(ItemStack::getCount),
//   java: 			DataComponentPatch.CODEC.optionalFieldOf("components", DataComponentPatch.EMPTY).forGetter(ItemStack::getComponentsPatch)
//   java: 	).apply(instance, ItemStack::new)));
/// java: public static final Codec<ItemStack> EMPTY_ALLOWING_ITEMSTACK_CODEC = Codec.lazyInitialized(() -> RecordCodecBuilder.create(instance -> instance.group( EMPTY_ALLOWING_ITEM_CODEC.fieldOf("id").forGette
pub static EMPTY_ALLOWING_ITEMSTACK_CODEC: compat::MainThread<CodecRef<mcrust::net::minecraft::world::item::item_stack::ItemStack>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `EMPTY_ALLOWING_ITEMSTACK_CODEC` - see the TODO above"));
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
        // TODO(fabric2seer): a constant `case` among pattern cases
        //   java: switch (id) {
        //   java: 			case "ENCHANTED_BOOK" -> {
        //   java: 				if (customData.contains("enchantments")) {
        //   java: 					CompoundTag enchants = customData.getCompoundOrEmpty("enchantments");
        //   java: 					Optional<String> firstEnchant = enchants.keySet().stream().findFirst();
        //   java: 					String enchant = firstEnchant.orElse("");
        //   java: 					return "ENCHANTMENT_" + enchant.toUpperCase(Locale.ENGLISH) + "_" + enchants.getIntOr(enchant, 0);
        //   java: 				}
        //   java: 			}
        //   java: 			case "PET" -> {
        //   java: 				PetInfo petInfo = getPetInfo(stack);
        //   java: 				return "LVL_1_" + petInfo.tier() + "_" + petInfo.type();
        //   java: 			}
        //   java: 			case "POTION" -> {
        //   java: 				String enhanced = customData.getBooleanOr("enhanced", false) ? "_ENHANCED" : "";
        //   java: 				String extended = customData.getBooleanOr("extended", false) ? "_EXTENDED" : "";
        //   java: 				String splash = customData.getBooleanOr("splash", false) ? "_SPLASH" : "";
        //   java: 				if (customData.contains("potion") && customData.contains("potion_level")) {
        //   java: 					return (customData.getStringOr("potion", "") + "_" + id + "_" + customData.getIntOr("potion_level", 0)
        //   java: 							+ enhanced + extended + splash).toUpperCase(Locale.ENGLISH);
        //   java: 				}
        //   java: 			}
        //   java: 			case "RUNE", "UNIQUE_RUNE" -> {
        //   java: 				if (customData.contains("runes")) {
        //   java: 					CompoundTag runes = customData.getCompoundOrEmpty("runes");
        //   java: 					String rune = runes.keySet().stream().findFirst().orElse("");
        //   java: 					return rune.toUpperCase(Locale.ENGLISH) + "_RUNE_" + runes.getIntOr(rune, 0);
        //   java: 				}
        //   java: 			}
        //   java: 			case String s when s.equals("ATTRIBUTE_SHARD") && stack instanceof DataComponentHolder holder -> {
        //   java: 				Attribute attribute = Attributes.getAttributeFromItemName(holder);
        //   java:
        //   java: 				if (attribute != null) return attribute.apiId();
        //   java: 			}
        //   java: 			case "NEW_YEAR_CAKE" -> {
        //   java: 				return id + "_" + customData.getIntOr("new_years_cake", 0);
        //   java: 			}
        //   java: 			case "PARTY_HAT_CRAB", "PARTY_HAT_CRAB_ANIMATED", "BALLOON_HAT_2024", "BALLOON_HAT_2025", "CAKE_HAT_2026" -> {
        //   java: 				return id + "_" + customData.getStringOr("party_hat_color", "").toUpperCase(Locale.ENGLISH);
        //   java: 			}
        //   java: 			case "PARTY_HAT_SLOTH" -> {
        //   java: 				return id + "_" + customData.getStringOr("party_hat_emoji", "").toUpperCase(Locale.ENGLISH);
        //   java: 			}
        //   java: 			case "MIDAS_SWORD" -> {
        //   java: 				if (customData.getIntOr("winning_bid", 0) >= 50000000) {
        //   java: 					return id + "_50M";
        //   java: 				}
        //   java: 			}
        //   java: 			case "MIDAS_STAFF" -> {
        //   java: 				if (customData.getIntOr("winning_bid", 0) >= 100000000) {
        //   java: 					return id + "_100M";
        //   java: 				}
        //   java: 			}
        //   java: 			case "FACTION_RABBIT" -> {
        //   java: 				return id + "_" + customData.getStringOr("faction_rabbit_id", "").toUpperCase(Locale.ENGLISH);
        //   java: 			}
        //   java: 			case "" -> {
        //   java: 				Screen currentScreen = Minecraft.getInstance().gui.screen();
        //   java: 				if (currentScreen instanceof ContainerScreen container && container.getTitle().getString().startsWith("Superpairs")) {
        //   java: 					ItemLore lore = stack.get(DataComponents.LORE);
        //   java: 					if (lore == null) return id;
        //   java: 					List<Component> lines = lore.lines();
        //   java: 					if (lines.size() < 3) return id;
        //   java: 					return EnchantedBookUtils.getApiIdByName(lines.get(2));
        //   java: 				}
        //   java:
        //   java: 				if (currentScreen instanceof ContainerScreen container && container.getTitle().getString().contains("Experimentation Table RNG")) {
        //   java: 					Component stackName = stack.getOrDefault(DataComponents.CUSTOM_NAME, Component.empty());
        //   java: 					return switch (stackName.getString()) {
        //   java: 							case "Titanic Experience Bottle" -> "TITANIC_EXP_BOTTLE";
        //   java: 							case "Grand Experience Bottle" -> "GRAND_EXP_BOTTLE";
        //   java: 							default -> EnchantedBookUtils.getApiIdByName(stackName);
        //   java: 					};
        //   java: 				}
        //   java:
        //   java: 				if (currentScreen instanceof ContainerScreen container && container.getTitle().getString().contains("Attribute Menu")) {
        //   java: 					Component stackName = stack.getOrDefault(DataComponents.CUSTOM_NAME, Component.empty());
        //   java: 					Attribute attribute = Attributes.getAttributeFromAbilityName(stackName.getString());
        //   java: 					if (attribute != null) return attribute.apiId();
        //   java: 					return id;
        //   java: 				}
        //   java:
        //   java: 				if (stack instanceof ItemStack realStack && realStack.has(DataComponents.CUSTOM_NAME)) {
        //   java: 					Component stackName = stack.getOrDefault(DataComponents.CUSTOM_NAME, Component.empty());
        //   java: 					// Enchanted Books in the Bazaar
        //   java: 					if (realStack.is(Items.ENCHANTED_BOOK)) return EnchantedBookUtils.getApiIdByName(stackName);
        //   java: 					// Essences
        //   java: 					if (realStack.is(Items.PLAYER_HEAD)) {
        //   java: 						Matcher matcher = ChestValue.ESSENCE_PATTERN.matcher(stackName.getString());
        //   java: 						if (matcher.find()) return "ESSENCE_" + matcher.group("type").toUpperCase(Locale.ENGLISH);
        //   java: 					}
        //   java: 				}
        //   java: 			}
        //   java: 			default -> {}
        //   java: 		}
        return id;
    }

    /// java: @Deprecated(since = "5.8.0") public static String getNeuId(DataComponentGetter stack)
    pub fn get_neu_id(stack: &dyn DataComponentGetter) -> String {
        if false /* java: stack == null: never null here */ {
            return "".to_string();
        }
        let mut custom_data: Tag = ItemUtils::get_custom_data(stack);
        let mut id: String = custom_data.get_string_or(ID, "");
        // TODO(fabric2seer): a pattern switch expression
        //   java: return switch (id) {
        //   java: 			case "ENCHANTED_BOOK" -> {
        //   java: 				CompoundTag enchantments = customData.getCompoundOrEmpty("enchantments");
        //   java: 				String enchant = enchantments.keySet().stream().findFirst().orElse("");
        //   java: 				yield enchant.toUpperCase(Locale.ENGLISH) + ";" + enchantments.getIntOr(enchant, 0);
        //   java: 			}
        //   java: 			case "PET" -> {
        //   java: 				PetInfo petInfo = getPetInfo(stack);
        //   java: 				yield petInfo.type() + ';' + petInfo.tierIndex();
        //   java: 			}
        //   java: 			case "RUNE", "UNIQUE_RUNE" -> {
        //   java: 				CompoundTag runes = customData.getCompoundOrEmpty("runes");
        //   java: 				String rune = runes.keySet().stream().findFirst().orElse("");
        //   java: 				yield rune.toUpperCase(Locale.ENGLISH) + "_RUNE;" + runes.getIntOr(rune, 0);
        //   java: 			}
        //   java: 			case "POTION" -> "POTION_" + customData.getStringOr("potion", "").toUpperCase(Locale.ENGLISH) + ";" + customData.getIntOr("potion_level", 0);
        //   java: 			case String s when s.equals("ATTRIBUTE_SHARD") && stack instanceof DataComponentHolder holder -> {
        //   java: 				Attribute attribute = Attributes.getAttributeFromItemName(holder);
        //   java: 				if (attribute == null) yield id;
        //   java: 				yield attribute.neuId();
        //   java: 			}
        //   java: 			case "PARTY_HAT_CRAB", "BALLOON_HAT_2024", "BALLOON_HAT_2025", "CAKE_HAT_2026" -> id + "_" + customData.getStringOr("party_hat_color", "").toUpperCase(Locale.ENGLISH);
        //   java: 			case "PARTY_HAT_CRAB_ANIMATED" -> "PARTY_HAT_CRAB_" + customData.getStringOr("party_hat_color", "").toUpperCase(Locale.ENGLISH) + "_ANIMATED";
        //   java: 			case "PARTY_HAT_SLOTH" -> id + "_" + customData.getStringOr("party_hat_emoji", "").toUpperCase(Locale.ENGLISH);
        //   java: 			case "FACTION_RABBIT" -> id + "_" + customData.getStringOr("faction_rabbit_id", "").toUpperCase(Locale.ENGLISH);
        //   java: 			default -> id.replace(":", "-");
        //   java: 		};
        todo!("fabric2seer: the value of this function is in a TODO above")
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
                // rust: port gap - the port's `DataComponentGetter` trait objects cannot be asked for their concrete class
                // (no `as_any`), so the item name is not added for the wiki lookup (GAPS_batchA.md)
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
        // TODO(fabric2seer): `is_empty` takes 1 argument(s) in the port (`tags: &BTreeMap<String, Tag>`), Java passes 0
        // rust: return (!(compound_tag::is_empty()) && custom_data.contains("drill_fuel") || stack.get_skyblock_id() == "PICKONIMBUS");
        return todo!("fabric2seer: see the TODO above");
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
            return Some(((drill_fuel_strings[(0) as usize]).trim().parse::<i32>().expect("NumberFormatException"), ((drill_fuel_strings[(1) as usize]).trim().parse::<i32>().expect("NumberFormatException")).wrapping_mul(1000)));
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
        if !(stack.is_value(&(items::items().get("player_head") as mcrust::com::mojang::serialization::Obj))) {
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
        // TODO(fabric2seer): `.encodeToString(..)` on a Encoder
        //   java: return Base64.getEncoder().encodeToString(str.getBytes());
        todo!("fabric2seer: the value of this function is in a TODO above")
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
        // TODO(fabric2seer): field `HUMANOID_ARMOR` of a value of unknown type
        //   java: return EquipmentSlotGroup.ARMOR.slots().stream()
        //   java: 				.filter(es -> es.getType() == EquipmentSlot.Type.HUMANOID_ARMOR)
        //   java: 				.map(entity::getItemBySlot)
        //   java: 				.toList();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static OptionalInt getItemCountInSack(ItemStack itemStack, List<String> lines)
    pub fn get_item_count_in_sack(item_stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, lines: &Vec<String>) -> Option<i32> {
        // TODO(fabric2seer): the condition: static field Strings.CS
        //   java: lines.size() < 2 || !Strings.CS.endsWithAny(lines.getFirst(), "Sack", "Gemstones")
        if todo!("fabric2seer: the condition (TODO above)") {
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
        if stack.is_value(&(items::items().get("air") as mcrust::com::mojang::serialization::Obj)) {
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
        // TODO(fabric2seer): `.putString(..)` on a T
        //   java: stack.set(DataComponents.CUSTOM_DATA, CustomData.of(Util.make(new CompoundTag(), c -> c.putString(ID, itemId))));
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
        let mut object_info: Arc<dyn ObjectInfo> = todo!("fabric2seer: the initialiser of `objectInfo` (TODO above)");
        return component::object(object_info);
    }

}
