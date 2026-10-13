//! Ported from de.hysky.skyblocker.skyblock.PetCache by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::pet_info;
use crate::de::hysky::skyblocker::skyblock::item::pet_info::PetInfo;
use crate::de::hysky::skyblocker::skyblock::item::skyblock_item_rarity::SkyblockItemRarity;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::utils::data::profiled_data::ProfiledData;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::regex_utils::RegexUtils;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::entry::skyblocker_mod;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::client::gui::screens::inventory::container_screen::ContainerScreen;
use mcrust::net::minecraft::nbt::tag::Tag;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::util::formatted_char_sequence::FormattedCharSequence;
use mcrust::net::minecraft::util::formatted_char_sequence::Seq;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::world::item::component::custom_data::CustomData;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Path FILE = SkyblockerMod.CONFIG_DIR.resolve("pet_cache.json");
pub static FILE: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("pet_cache.json"));
/// java: private static final ProfiledData<PetInfo> CACHED_PETS = new ProfiledData<>(FILE, PetInfo.CODEC);
pub static CACHED_PETS: compat::MainThread<compat::Shared<ProfiledData<PetInfo>>> = compat::MainThread::new(|| ProfiledData::new(FILE.get_mut().clone(), pet_info::CODEC.get_mut().clone()));
/// java: private static final Pattern AUTOPET_PATTERN = Pattern.compile("^Autopet equipped your \\[Lvl (?<level>\\d+)] (?<name>[A-Za-z ]+)(?: ✦)?! VIEW RULE$");
pub static AUTOPET_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^Autopet equipped your \\[Lvl (?<level>\\d+)] (?<name>[A-Za-z ]+)(?: ✦)?! VIEW RULE$"));
/// java: private static boolean shouldLook4Pets;
pub static SHOULD_LOOK4_PETS: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// java: public class PetCache
#[derive(Clone)]
pub struct PetCache {}

impl PetCache {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        CACHED_PETS.get_mut().lock().borrow_mut().load();
        screen_events::BEFORE_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            // `screen instanceof ContainerScreen genericContainerScreen`: the screen's kind; the title is the `Screen` half
            if Utils::is_on_skyblock() && matches!(compat::minecraft().gui.screen_kind, mcrust::net::minecraft::client::gui::gui::ScreenKind::Container) {
                if screen.get_title().to_string().starts_with("Pets") {
                    compat::set_state(&SHOULD_LOOK4_PETS, true);
                    // TODO(fabric2seer): ScreenEvents.afterTick(..).register: no SeerClient equivalent yet
                    //   java: ScreenEvents.afterTick(screen).register(_ -> {
                    //   java: 						if (shouldLook4Pets) {
                    //   java: 							for (Slot slot : genericContainerScreen.getMenu().slots) {
                    //   java: 								ItemStack stack = slot.getItem();
                    //   java:
                    //   java: 								if (!stack.isEmpty() && ItemUtils.getLoreLineIf(stack, line -> line.equals("Click to despawn!")) != null) {
                    //   java: 									shouldLook4Pets = false;
                    //   java: 									parsePet(stack, false);
                    //   java:
                    //   java: 									break;
                    //   java: 								}
                    //   java: 							}
                    //   java: 						}
                    //   java: 					});
                }
            }
        }));
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(PetCache::onMessage);
    }

    /// java: public static void handlePetEquip(Slot slot, int slotId)
    pub fn handle_pet_equip(slot: &Slot, mut slot_id: i32) {
        if slot_id >= 0 && slot_id <= 53 {
            //   java: ItemStack stack = slot.getItem();
            let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = compat::slot_item(slot);
            if !stack.is_empty() {
                PetCache::parse_pet(&stack, true);
            }
        }
    }

    /// java: private static void parsePet(ItemStack stack, boolean clicked)
    pub fn parse_pet(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut clicked: bool) {
        let mut profile_id: String = Utils::get_profile_id();
        if stack.get_skyblock_id() == "PET" && !profile_id.is_empty() {
            let mut pet_info: PetInfo = stack.get_pet_info();
            if pet_info.is_empty() {
                return;
            }
            compat::set_state(&SHOULD_LOOK4_PETS, false);
            if clicked && PetCache::get_current_pet().is_some() && PetCache::get_current_pet().unwrap().uuid().unwrap_or("".to_string()) == pet_info.uuid().unwrap_or("".to_string()) {
                ProfiledData::remove(CACHED_PETS.get_mut());
            } else {
                ProfiledData::put(CACHED_PETS.get_mut(), pet_info.clone());
            }
            CACHED_PETS.get_mut().lock().borrow().save();
        }
    }

    /// java: @SuppressWarnings("UnstableApiUsage") private static boolean onMessage(Component text, boolean overlay)
    pub fn on_message(text: &MutableComponent, mut overlay: bool) -> bool {
        if !(Utils::is_on_skyblock()) || overlay {
            return true;
        }
        let mut stringified: String = chat_formatting::strip_formatting(&text.get_string());
        let mut matcher: compat::regex::Matcher = AUTOPET_PATTERN.matcher(&stringified);
        if matcher.matches() {
            let mut level: i32 = RegexUtils::parse_int_from_matcher_matcher_string(&matcher, "level");
            let mut name: String = matcher.group_named("name").unwrap();
            let mut ordered: Seq = text.get_visual_order_text();
            let mut name_index: i32 = compat::index_of(&stringified, &name);
            //   java: MutableInt codePointIndex = new MutableInt(0);
            let mut code_point_index: i32 = 0;
            //   java: MutableInt color = new MutableInt(-1);
            let mut color: i32 = -1;
            //   java: ordered.accept((_, style, _) -> {
            //   java: 	if (codePointIndex.intValue() == nameIndex) { color.setValue(style.getColor().getValue()); return false; }
            //   java: 	codePointIndex.getAndIncrement(); return true; });
            ordered.accept(&mut |_: i32, style: &mcrust::net::minecraft::network::chat::style::Style, _: u32| -> bool {
                if code_point_index == name_index {
                    color = style.get_color().expect("NullPointerException: style.getColor()").get_value();
                    return false;
                }
                code_point_index += 1;
                return true;
            });
            //   java: SkyblockItemRarity rarity = SkyblockItemRarity.fromColor(color.intValue());
            let mut rarity: SkyblockItemRarity = SkyblockItemRarity::from_color(color);
            if rarity != SkyblockItemRarity::Unknown {
                //   java: int petOffset = PetConstants.RARITY_OFFSETS.getOrDefault(rarity.name(), 0);
                let mut pet_offset: i32 = *crate::net::azureaaron::networth::utils::pet_constants::RARITY_OFFSETS.get(rarity.name()).unwrap_or(&0);
                //   java: IntList petLevels = new IntArrayList(PetConstants.PET_LEVELS.subList(petOffset, petOffset + level - 1));
                let mut pet_levels: Vec<i32> = crate::net::azureaaron::networth::utils::pet_constants::PET_LEVELS[pet_offset as usize..(pet_offset + level - 1) as usize].to_vec();
                //   java: double exp = petLevels.intStream().sum();
                let mut exp: f64 = pet_levels.iter().fold(0i32, |sum, l| sum.wrapping_add(*l)) as f64;
                let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = ItemRepository::get_items_stream().filter(|s| compat::component_of(FlexibleItemStack::get_stack_or_throw(s).get_hover_name()).get_string().contains(&format!("] {}", name))).next().map(|__a0| FlexibleItemStack::get_stack_or_throw(&__a0)).unwrap_or(mcrust::net::minecraft::world::item::item_stack::empty());
                if !stack.is_empty() {
                    let mut copied: mcrust::net::minecraft::world::item::item_stack::ItemStack = stack.copy();
                    // java: CompoundTag customData = copied.getOrDefault(DataComponents.CUSTOM_DATA, CustomData.EMPTY).copyTag();
                    let mut custom_data: Tag = crate::de::hysky::skyblocker::utils::item_utils::ItemUtils::get_custom_data(&copied);
                    custom_data.put_string("id", "PET");
                    copied.set(mcrust::net::minecraft::core::component::data_components::custom_data().clone(), Some(mcrust::com::mojang::serialization::obj(CustomData::of(&custom_data))));
                    let mut r#type: String = (if !(copied.get_pet_info().is_empty()) { copied.get_pet_info().r#type() } else { name.to_uppercase().replace(" ", "_") });
                    let mut pet_info: PetInfo = PetInfo::new(Some(name.clone()), r#type.clone(), exp, rarity, None, None, None);
                    ProfiledData::put(CACHED_PETS.get_mut(), pet_info.clone());
                    CACHED_PETS.get_mut().lock().borrow().save();
                }
            }
        }
        return true;
    }

    /// java: public static @Nullable PetInfo getCurrentPet()
    pub fn get_current_pet() -> Option<PetInfo> {
        return ProfiledData::get(CACHED_PETS.get_mut());
    }

}
