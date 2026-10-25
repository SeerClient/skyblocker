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
/// java: private static final Codec<Attribute> CODEC = RecordCodecBuilder.create(instance -> instance.group(
///   Codec.STRING.fieldOf("bazaarName"), .. Codec.STRING.listOf().fieldOf("family"), Codec.STRING.fieldOf("shardId")).apply(instance, Attribute::new));
/// (`LazyLock`: `exportAttributes` encodes on a worker thread)
pub static ATTRIBUTE_CODEC: LazyLock<CodecRef<Attribute>> = LazyLock::new(|| record_codec_builder::create(record_codec_builder::apply8(record_codec_builder::of(|o: &Attribute| o.bazaar_name(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "bazaarName")), record_codec_builder::of(|o: &Attribute| o.display_name(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "displayName")), record_codec_builder::of(|o: &Attribute| o.rarity(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "rarity")), record_codec_builder::of(|o: &Attribute| o.internal_name(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "internalName")), record_codec_builder::of(|o: &Attribute| o.ability_name(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "abilityName")), record_codec_builder::of(|o: &Attribute| o.alignment(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "alignment")), record_codec_builder::of(|o: &Attribute| o.family(), codec::field_of(codec::list(mcrust::com::mojang::serialization::codec::string_codec()), "family")), record_codec_builder::of(|o: &Attribute| o.shard_id(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "shardId")), move |__a0, __a1, __a2, __a3, __a4, __a5, __a6, __a7| Attribute::new(__a0.clone(), __a1.clone(), __a2.clone(), __a3.clone(), __a4.clone(), __a5.clone(), __a6.clone(), __a7.clone()))));
/// java: public static final Codec<List<Attribute>> LIST_CODEC = CODEC.listOf();
pub static ATTRIBUTE_LIST_CODEC: LazyLock<CodecRef<Vec<Attribute>>> = LazyLock::new(|| codec::list(ATTRIBUTE_CODEC.clone()));
/// java: private static final Path ATTRIBUTE_EXPORT_DEST (read on the export worker; built from `CONFIG_DIR` on the render thread)
static ATTRIBUTE_EXPORT_DEST_PATH: std::sync::OnceLock<std::path::PathBuf> = std::sync::OnceLock::new();
/// Which of the two menus the open container screen is (`ScreenEvents.AFTER_INIT` records it; see `init`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpenMenu {
    AttributeMenu,
    HuntingBox,
}
/// The open container screen's menu as `AFTER_INIT` found it (`None` for any other screen).
pub static OPEN_MENU: compat::MainThread<Option<OpenMenu>> = compat::MainThread::new(|| None);

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
    ///
    /// Java registers a per-screen `ScreenKeyboardEvents.afterKeyPress(screen)` listener from `AFTER_INIT` over the
    /// attribute menu or the hunting box it found. The port's `AFTER_KEY_PRESS` is one global event, so the screen
    /// `AFTER_INIT` found is recorded (`OPEN_MENU`, rewritten for every screen) and ONE listener asks it.
    pub fn init() {
        if !(crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled()) && !(SkyblockerConfigManager::get().debug.enable_repo_dev) {
            return;
        }
        let _ = ATTRIBUTE_EXPORT_DEST_PATH.set(ATTRIBUTE_EXPORT_DEST.get().clone());
        screen_events::AFTER_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            *OPEN_MENU.get_mut() = if AttributesDebug::check_for_attribute_menu(&screen) {
                Some(OpenMenu::AttributeMenu)
            } else if AttributesDebug::check_for_hunting_box(&screen) {
                Some(OpenMenu::HuntingBox)
            } else {
                None
            };
        }));
        screen_keyboard_events::AFTER_KEY_PRESS.register(Arc::new(|action: i32, input: &KeyEvent| {
            if !compat::container_screen_open() {
                return;
            }
            match *OPEN_MENU.get() {
                Some(OpenMenu::AttributeMenu) => {
                    if input.key() == mcrust::com::mojang::blaze3d::platform::input_constants::KEY_G {
                        AttributesDebug::dump_attributes();
                    } else if input.key() == mcrust::com::mojang::blaze3d::platform::input_constants::KEY_J {
                        AttributesDebug::export_attributes();
                    }
                }
                Some(OpenMenu::HuntingBox) => {
                    if input.key() == mcrust::com::mojang::blaze3d::platform::input_constants::KEY_G {
                        AttributesDebug::fix_attributes();
                    } else if input.key() == mcrust::com::mojang::blaze3d::platform::input_constants::KEY_J {
                        AttributesDebug::export_attributes();
                    }
                }
                None => {}
            }
        }));
    }

    /// java: private static Int2ObjectMap<ItemStack> getSlots(AbstractContainerScreen<?> screen)
    /// (`screen.getMenu()` is the player's open menu, read live)
    pub fn get_slots() -> std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        // java: ContainerSolverManager.slotMap(screen.getMenu().slots.subList(0, ((AbstractContainerScreen<ChestMenu>) screen).getMenu().getRowCount() * 9));
        let rows: i32 = mcrust::mcrust::modding::game::with_chest_menu(|chest| chest.get_row_count()).expect("ClassCastException: the menu is not a ChestMenu");
        let mut slots: std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack> = compat::with_open_menu(|menu, containers| {
            let mut slot_map = std::collections::BTreeMap::new();
            for i in 0..rows.wrapping_mul(9) {
                slot_map.insert(i, menu.slots[(i) as usize].get_item(containers).duplicate());
            }
            slot_map
        }).unwrap_or_default();
        container_solver::trim_edges(&mut slots, 6);
        return slots;
    }

    /// java: private static void dumpAttributes(AbstractContainerScreen<?> screen)
    pub fn dump_attributes() {
        let mut slots: std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack> = AttributesDebug::get_slots();
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
                // java: boolean hasDataForId = TooltipInfoType.BAZAAR.getData().containsKey(bazaarNameGuess);
                let mut has_data_for_id: bool = compat::read(tooltip_info_type::BAZAAR.get(), |__t| __t.get_data()).expect("NullPointerException: BAZAAR data").contains_key(&bazaar_name_guess);
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
    pub fn fix_attributes() {
        let mut slots: std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack> = AttributesDebug::get_slots();
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
                // java: DUMPED_ATTRIBUTES.computeIfPresent(abilityName, (_, o) -> new Attribute(.., WordUtils.capitalizeFully(alignmentMatcher.group("alignment")), ..));
                if let Some(o) = compat::lock(&DUMPED_ATTRIBUTES).get_mut(&ability_name) {
                    let mut alignment: String = compat::capitalize_fully(&alignment_matcher.group_named("alignment").unwrap());
                    *o = Attribute::new(o.bazaar_name.clone(), o.display_name.clone(), o.rarity.clone(), o.internal_name.clone(), o.ability_name.clone(), alignment, o.family.clone(), o.shard_id.clone());
                }
            }
            if let Some(family_matcher) = &family_matcher {
                let mut family: String = family_matcher.group_named("family").unwrap();
                // java: List<String> families = DUMPED_ATTRIBUTES.get(abilityName).family(); ...forEach(families::add) - the record's own list
                let mut __dumped = compat::lock(&DUMPED_ATTRIBUTES);
                let families: &mut Vec<String> = &mut __dumped.get_mut(&ability_name).expect("NullPointerException").family;
                for __a0 in (compat::regex::split(&family, ",| and ")).iter().cloned().map(|__a0| compat::trim(&__a0)) {
                    families.push(__a0.clone());
                }
            }
        }
    }

    /// java: private static void exportAttributes(AbstractContainerScreen<?> screen)
    pub fn export_attributes() {
        let mut copy: Vec<Attribute> = compat::sorted_by(compat::distinct(compat::lock(&DUMPED_ATTRIBUTES).values().cloned()), |a, b| { let key = |attribute: &Attribute| {
            if compat::str_len(&attribute.shard_id) == 3 {
                return attribute.shard_id.clone();
            }
            return format!("{}0{}", compat::char_at(&attribute.shard_id, 0), compat::substring(&attribute.shard_id, 1));
        }; key(a).cmp(&key(b)) }).collect::<Vec<_>>();
        CompletableFuture::<()>::run_async(move || (|| {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                std::fs::write(ATTRIBUTE_EXPORT_DEST_PATH.get().expect("ATTRIBUTE_EXPORT_DEST"), compat::json::to_string(&compat::json_from_port(&(ATTRIBUTE_LIST_CODEC.encode_start(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &copy)).get_or_throw().unwrap()))).expect("IOException");
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker Attributes Debug] Failed to export attributes!"));
            }
        })(), compat::executor);
    }

    /// java: private static @Nullable AbstractContainerScreen<?> checkForAttributeMenu(Screen screen)
    /// (whether `screen` - the `Screen` half `AFTER_INIT` hands over - is that container screen: the screen object
    /// itself is the game's, and the menu it shows is the player's open one, which `get_slots` reads)
    pub fn check_for_attribute_menu(screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        return compat::container_screen_open() && screen.title.ends_with("Attribute Menu");
    }

    /// java: private static @Nullable AbstractContainerScreen<?> checkForHuntingBox(Screen screen)
    pub fn check_for_hunting_box(screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        return compat::container_screen_open() && screen.title.ends_with("Hunting Box");
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
