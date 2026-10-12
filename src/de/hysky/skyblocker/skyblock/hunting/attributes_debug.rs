//! Ported from de.hysky.skyblocker.skyblock.hunting.AttributesDebug by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type;
use crate::de::hysky::skyblocker::utils::container::container_solver;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::roman_numerals::RomanNumerals;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::data_tooltip_info_type::DataTooltipInfoType as _;
use mcrust::com::mojang::serialization::encoder::Encoder as _;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::ContainerScreenHost as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Path ATTRIBUTE_EXPORT_DEST = SkyblockerMod.CONFIG_DIR.resolve("attribute_export.json");
pub static ATTRIBUTE_EXPORT_DEST: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("attribute_export.json"));
/// java: private static final Pattern SOURCE_PATTERN = Pattern.compile("Source: (?<shardName>[A-za-z ]+) Shard \\((?<id>[CUREL]\\d+)\\)");
pub static SOURCE_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Source: (?<shardName>[A-za-z ]+) Shard \\((?<id>[CUREL]\\d+)\\)"));
/// java: private static final Pattern RARITY_PATTERN = Pattern.compile("Rarity: (?<rarity>\\w+)");
pub static RARITY_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Rarity: (?<rarity>\\w+)"));
/// java: private static final Pattern FAMILY_PATTERN = Pattern.compile("^(?<family>[\\w ]+) Family$");
pub static FAMILY_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^(?<family>[\\w ]+) Family$"));
/// java: private static final Pattern ALIGNMENT_PATTERN = Pattern.compile("^(?:COMMON|UNCOMMON|RARE|EPIC|LEGENDARY) (?<alignment>[\\w ]+) SHARD \\(ID [A-Z0-9]+\\)");
pub static ALIGNMENT_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^(?:COMMON|UNCOMMON|RARE|EPIC|LEGENDARY) (?<alignment>[\\w ]+) SHARD \\(ID [A-Z0-9]+\\)"));
/// java: private static final Map<String, Attribute> DUMPED_ATTRIBUTES = new Object2ObjectOpenHashMap<>();
pub static DUMPED_ATTRIBUTES: LazyLock<Mutex<std::collections::HashMap<String, Attribute>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));
// TODO(fabric2seer): `PrimitiveCodec.listOf` has no Rust method in the port (looked for `list_of` in PrimitiveCodec and its supertypes)
//   java: private static final Codec<Attribute> CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 				Codec.STRING.fieldOf("bazaarName").forGetter(Attribute::bazaarName),
//   java: 				Codec.STRING.fieldOf("displayName").forGetter(Attribute::displayName),
//   java: 				Codec.STRING.fieldOf("rarity").forGetter(Attribute::rarity),
//   java: 				Codec.STRING.fieldOf("internalName").forGetter(Attribute::internalName),
//   java: 				Codec.STRING.fieldOf("abilityName").forGetter(Attribute::abilityName),
//   java: 				Codec.STRING.fieldOf("alignment").forGetter(Attribute::alignment),
//   java: 				Codec.STRING.listOf().fieldOf("family").forGetter(Attribute::family),
//   java: 				Codec.STRING.fieldOf("shardId").forGetter(Attribute::shardId)
//   java: 		).apply(instance, Attribute::new));
/// java: private static final Codec<Attribute> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("bazaarName").forGetter(Attribute::bazaarName), Codec.STRING.fieldOf("displayNa
pub static ATTRIBUTE_CODEC: compat::MainThread<CodecRef<Attribute>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));
/// java: public static final Codec<List<Attribute>> LIST_CODEC = CODEC.listOf();
pub static ATTRIBUTE_LIST_CODEC: compat::MainThread<CodecRef<Vec<Attribute>>> = compat::MainThread::new(|| codec::list(ATTRIBUTE_CODEC.get_mut().clone()));

/// java: public class AttributesDebug
#[derive(Clone)]
pub struct AttributesDebug {}

impl AttributesDebug {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        if !(crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled()) && !(SkyblockerConfigManager::get().debug.enable_repo_dev) {
            return;
        }
        screen_events::AFTER_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            let mut attribute_menu: Option<AbstractContainerScreen> = AttributesDebug::check_for_attribute_menu(&screen);
            if let Some(attribute_menu) = &attribute_menu {
                screen_keyboard_events::AFTER_KEY_PRESS.register(Arc::new(|action: i32, key_event: &KeyEvent| {
                    // TODO(fabric2seer): `.key(..)` on a value of unknown type
                    //   java: if (input.key() == InputConstants.KEY_G) {
                    //   java: 						dumpAttributes(attributeMenu);
                    //   java: 					} else if (input.key() == InputConstants.KEY_J) {
                    //   java: 						exportAttributes(attributeMenu);
                    //   java: 					}
                }));
                return;
            }
            let mut hunting_box: Option<AbstractContainerScreen> = AttributesDebug::check_for_hunting_box(&screen);
            if let Some(hunting_box) = &hunting_box {
                screen_keyboard_events::AFTER_KEY_PRESS.register(Arc::new(|action: i32, key_event: &KeyEvent| {
                    // TODO(fabric2seer): `.key(..)` on a value of unknown type
                    //   java: if (input.key() == InputConstants.KEY_G) {
                    //   java: 						fixAttributes(huntingBox);
                    //   java: 					} else if (input.key() == InputConstants.KEY_J) {
                    //   java: 						exportAttributes(huntingBox);
                    //   java: 					}
                }));
            }
        }));
    }

    /// java: private static Int2ObjectMap<ItemStack> getSlots(AbstractContainerScreen<?> screen)
    pub fn get_slots(screen: &AbstractContainerScreen) -> std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        // TODO(fabric2seer): field `slots` of a Object
        //   java: @SuppressWarnings("unchecked")
        //   java: 		Int2ObjectMap<ItemStack> slots = ContainerSolverManager.slotMap(screen.getMenu().slots.subList(0, ((AbstractContainerScreen<ChestMenu>) screen).getMenu().getRowCount() * 9));
        let mut slots: std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack> = todo!("fabric2seer: the initialiser of `slots` (TODO above)");
        container_solver::trim_edges(&mut slots, 6);
        return slots;
    }

    /// java: private static void dumpAttributes(AbstractContainerScreen<?> screen)
    pub fn dump_attributes(screen: &AbstractContainerScreen) {
        let mut slots: std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack> = AttributesDebug::get_slots(screen);
        for stack in slots.values().cloned() {
            if stack.is_empty() {
                continue;
            }
            let mut ability_name: String = compat::component_of(stack.get_hover_name()).get_string();
            let mut source_matcher: Option<compat::regex::Matcher> = ItemUtils::get_lore_line_if_match(&stack, &SOURCE_PATTERN);
            let mut rarity_matcher: Option<compat::regex::Matcher> = ItemUtils::get_lore_line_if_match(&stack, &RARITY_PATTERN);
            let mut words: Vec<String> = (compat::regex::split(&ability_name, " ")).to_vec().into_iter().collect::<Vec<_>>();
            if RomanNumerals::is_valid_roman_numeral(Some((words[words.len() - 1]).trim().to_string())) {
                words.pop().unwrap();
                ability_name = words.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(" ");
            }
            if source_matcher.is_some() && rarity_matcher.is_some() {
                let mut shard_name: String = source_matcher.as_mut().unwrap().group_named("shardName").unwrap();
                let mut shard_id: String = source_matcher.as_mut().unwrap().group_named("id").unwrap();
                let mut rarity: String = rarity_matcher.as_mut().unwrap().group_named("rarity").unwrap();
                let mut bazaar_name_guess: String = format!("SHARD_{}", (shard_name.replace(' ', &'_'.to_string())).to_uppercase());
                let mut neu_id_guess: String = format!("ATTRIBUTE_SHARD_{};1", (ability_name.replace("'", "")).replace(' ', &'_'.to_string()).to_uppercase());
                // TODO(fabric2seer): `.containsKey(..)` on a T
                //   java: boolean hasDataForId = TooltipInfoType.BAZAAR.getData().containsKey(bazaarNameGuess);
                let mut has_data_for_id: bool = todo!("fabric2seer: the initialiser of `hasDataForId` (TODO above)");
                if !has_data_for_id {
                    logger().warn(&format!("[Skyblocker Attributes Debug] No data found for shard. Shard Name: {}", shard_name));
                }
                let mut attribute: Attribute = Attribute::new(bazaar_name_guess.clone(), shard_name.clone(), rarity.clone(), neu_id_guess.clone(), ability_name.clone(), "".to_string(), Vec::new(), shard_id.clone());
                compat::lock(&DUMPED_ATTRIBUTES).insert(shard_name.clone(), attribute.clone());
            } else {
                logger().warn(&format!("[Skyblocker Attributes Debug] Failed to match shard! Name: {}", ability_name));
            }
        }
    }

    /// java: private static void fixAttributes(AbstractContainerScreen<?> screen)
    pub fn fix_attributes(screen: &AbstractContainerScreen) {
        let mut slots: std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack> = AttributesDebug::get_slots(screen);
        for stack in slots.values().cloned() {
            if stack.is_empty() {
                continue;
            }
            let mut ability_name: String = compat::component_of(stack.get_hover_name()).get_string();
            if !(compat::lock(&DUMPED_ATTRIBUTES).contains_key(&ability_name)) {
                logger().error(&format!("[Skyblocker Attributes Debug] Could not find attribute for {}", ability_name));
                continue;
            }
            let mut family_matcher: Option<compat::regex::Matcher> = ItemUtils::get_lore_line_if_match(&stack, &FAMILY_PATTERN);
            let mut alignment_matcher: Option<compat::regex::Matcher> = ItemUtils::get_lore_line_if_match(&stack, &ALIGNMENT_PATTERN);
            if let Some(alignment_matcher) = &alignment_matcher {
                // TODO(fabric2seer): Map.computeIfPresent(..)
                //   java: DUMPED_ATTRIBUTES.computeIfPresent(abilityName, (_, o) -> {
                //   java: 					String alignment = WordUtils.capitalizeFully(alignmentMatcher.group("alignment"));
                //   java: 					return new Attribute(o.bazaarName, o.displayName, o.rarity, o.internalName,
                //   java: 							o.abilityName, alignment, o.family, o.shardId);
                //   java: 				});
            }
            if let Some(family_matcher) = &family_matcher {
                let mut family: String = family_matcher.group_named("family").unwrap();
                let mut families: Vec<String> = compat::lock(&DUMPED_ATTRIBUTES).get(&ability_name).cloned().unwrap().family();
                for __a0 in (compat::regex::split(&family, ",| and ")).iter().cloned().map(|__a0| compat::trim(&__a0)) {
                    families.push(__a0.clone());
                }
            }
        }
    }

    /// java: private static void exportAttributes(AbstractContainerScreen<?> screen)
    pub fn export_attributes(screen: &AbstractContainerScreen) {
        let mut copy: Vec<Attribute> = compat::sorted_by(compat::distinct(compat::lock(&DUMPED_ATTRIBUTES).values().cloned()), |a, b| { let key = |attribute| {
            if compat::str_len(&attribute.shard_id) == 3 {
                return attribute.shard_id.clone();
            }
            return format!("{}0{}", compat::char_at(&attribute.shard_id, 0), compat::substring(&attribute.shard_id, 1));
        }; key(a).cmp(&key(b)) }).collect::<Vec<_>>();
        CompletableFuture::<()>::run_async(move || (|| {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                std::fs::write(ATTRIBUTE_EXPORT_DEST.get_mut(), &compat::jstring(&(ATTRIBUTE_LIST_CODEC.get_mut().encode_start(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &copy)).get_or_throw().unwrap())).expect("IOException");
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker Attributes Debug] Failed to export attributes!"));
            }
        })(), compat::executor);
    }

    /// java: private static @Nullable AbstractContainerScreen<?> checkForAttributeMenu(Screen screen)
    pub fn check_for_attribute_menu(screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> Option<AbstractContainerScreen> {
        if let Some(container_screen) = screen.as_any().downcast_ref::<AbstractContainerScreen>() && container_screen.screen.get_title().to_string().ends_with("Attribute Menu") {
            // TODO(fabric2seer): the mod keeps the game's `AbstractContainerScreen` (Java holds a reference to the live object); the port's `AbstractContainerScreen` is not a value to copy and the port lends the mod no handle to it
            // rust: return Some(container_screen.clone());
            return todo!("fabric2seer: see the TODO above");
        }
        return None;
    }

    /// java: private static @Nullable AbstractContainerScreen<?> checkForHuntingBox(Screen screen)
    pub fn check_for_hunting_box(screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> Option<AbstractContainerScreen> {
        if let Some(container_screen) = screen.as_any().downcast_ref::<AbstractContainerScreen>() && container_screen.screen.get_title().to_string().ends_with("Hunting Box") {
            // TODO(fabric2seer): the mod keeps the game's `AbstractContainerScreen` (Java holds a reference to the live object); the port's `AbstractContainerScreen` is not a value to copy and the port lends the mod no handle to it
            // rust: return Some(container_screen.clone());
            return todo!("fabric2seer: see the TODO above");
        }
        return None;
    }

}

/// java: public record Attribute(String bazaarName, String displayName, String rarity, String internalName, String abilityName, String alignment, List<String> family, String shardId)
#[derive(Clone, Debug, PartialEq)]
pub struct Attribute {
    /// java: String bazaarName
    pub bazaar_name: String,
    /// java: String displayName
    pub display_name: String,
    /// java: String rarity
    pub rarity: String,
    /// java: String internalName
    pub internal_name: String,
    /// java: String abilityName
    pub ability_name: String,
    /// java: String alignment
    pub alignment: String,
    /// java: List<String> family
    pub family: Vec<String>,
    /// java: String shardId
    pub shard_id: String,
}

impl Attribute {
    /// java: the canonical record constructor
    pub fn new(mut bazaar_name: String, mut display_name: String, mut rarity: String, mut internal_name: String, mut ability_name: String, mut alignment: String, mut family: Vec<String>, mut shard_id: String) -> Self {
        Self { bazaar_name, display_name, rarity, internal_name, ability_name, alignment, family, shard_id }
    }

    /// java: the record accessor `bazaarName()`
    pub fn bazaar_name(&self) -> String {
        self.bazaar_name.clone()
    }

    /// java: the record accessor `displayName()`
    pub fn display_name(&self) -> String {
        self.display_name.clone()
    }

    /// java: the record accessor `rarity()`
    pub fn rarity(&self) -> String {
        self.rarity.clone()
    }

    /// java: the record accessor `internalName()`
    pub fn internal_name(&self) -> String {
        self.internal_name.clone()
    }

    /// java: the record accessor `abilityName()`
    pub fn ability_name(&self) -> String {
        self.ability_name.clone()
    }

    /// java: the record accessor `alignment()`
    pub fn alignment(&self) -> String {
        self.alignment.clone()
    }

    /// java: the record accessor `family()`
    pub fn family(&self) -> Vec<String> {
        self.family.clone()
    }

    /// java: the record accessor `shardId()`
    pub fn shard_id(&self) -> String {
        self.shard_id.clone()
    }

}
