//! Ported from de.hysky.skyblocker.skyblock.crimson.kuudra.Kuudra by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::chest_value;
use crate::de::hysky::skyblocker::skyblock::crimson::crimson_faction::CrimsonFaction;
use crate::de::hysky::skyblocker::skyblock::crimson::kuudra::kuudra_phase::KuudraPhase;
use crate::de::hysky::skyblocker::skyblock::crimson::kuudra::kuudra_profile_data;
use crate::de::hysky::skyblocker::skyblock::crimson::kuudra::kuudra_profile_data::KuudraProfileData;
use crate::de::hysky::skyblocker::utils::data::profiled_data::ProfiledData;
use crate::de::hysky::skyblocker::utils::regex_list_utils::RegexListUtils;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::entry::skyblocker_mod;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::client::gui::screens::inventory::container_screen::ContainerScreen;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData as _;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::ContainerScreenHost as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public static final int KUUDRA_MAGMA_CUBE_SIZE = 30;
pub const KUUDRA_MAGMA_CUBE_SIZE: i32 = 30;
/// java: private static final Path FILE = SkyblockerMod.CONFIG_DIR.resolve("kuudra.json");
pub static FILE: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("kuudra.json"));
/// java: private static final ProfiledData<KuudraProfileData> DATA = new ProfiledData<>(FILE, KuudraProfileData.CODEC);
pub static DATA: compat::MainThread<compat::Shared<ProfiledData<KuudraProfileData>>> = compat::MainThread::new(|| ProfiledData::new(FILE.get_mut().clone(), kuudra_profile_data::CODEC.get_mut().clone()));
/// java: private static final Pattern FACTION_SHOP_PATTERN = Pattern.compile("^(?<faction>Mage|Barbarian) Shop$");
pub static FACTION_SHOP_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^(?<faction>Mage|Barbarian) Shop$"));
/// java: protected static KuudraPhase phase = KuudraPhase.OTHER;
pub static PHASE: LazyLock<Mutex<KuudraPhase>> = LazyLock::new(|| Mutex::new(KuudraPhase::Other));

/// java: public class Kuudra
#[derive(Clone)]
pub struct Kuudra {}

impl Kuudra {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        DATA.get_mut().lock().borrow_mut().load();
        screen_events::AFTER_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            // `screen instanceof ContainerScreen genericContainerScreen`: the screen's kind (the vanilla `ContainerScreen`
            // object itself is not lent to the listener - its uses below stay with the TODOs)
            if Utils::is_on_skyblock() && matches!(compat::minecraft().gui.screen_kind, mcrust::net::minecraft::client::gui::gui::ScreenKind::Container) {
                let mut title: String = screen.get_title().to_string();
                let mut faction_shop_matcher: compat::regex::Matcher = FACTION_SHOP_PATTERN.matcher(&title);
                if true && faction_shop_matcher.matches() {
                    // TODO(fabric2seer): ScreenEvents.afterTick(..).register: no SeerClient equivalent yet
                    //   java: ScreenEvents.afterTick(screen).register(_ -> {
                    //   java: 							checkKuudraKeyShop(genericContainerScreen, factionShopMatcher);
                    //   java: 						});
                } else if true && title.ends_with("Pets") {
                    // TODO(fabric2seer): ScreenEvents.afterTick(..).register: no SeerClient equivalent yet
                    //   java: ScreenEvents.afterTick(screen).register(_ -> {
                    //   java: 							checkForKuudraPet(genericContainerScreen);
                    //   java: 						});
                } else {
                }
            }
        }));
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            Kuudra::reset();
        }));
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(Kuudra::onMessage);
    }

    /// java: public static KuudraProfileData getKuudraProfileData()
    pub fn get_kuudra_profile_data() -> KuudraProfileData {
        return (if ProfiledData::contains_key(DATA.get_mut()) { ProfiledData::get(DATA.get_mut()) } else { Some(kuudra_profile_data::EMPTY.clone()) }).unwrap();
    }

    /// java: private static void checkKuudraKeyShop(ContainerScreen screen, Matcher factionShopMatcher)
    pub fn check_kuudra_key_shop(screen: &ContainerScreen, faction_shop_matcher: &compat::regex::Matcher) {
        let mut faction: CrimsonFaction = CrimsonFaction::value_of(&faction_shop_matcher.group_named("faction").unwrap().to_uppercase());
        // TODO(fabric2seer): field `slots` of a value of unknown type
        //   java: List<ItemStack> kuudraKeyItems = screen.getMenu().slots.stream()
        //   java: 				.filter(Slot::hasItem)
        //   java: 				.map(Slot::getItem)
        //   java: 				.filter(stack -> KuudraProfileData.EMPTY.kuudraKeyPrices().containsKey(stack.getSkyblockId()))
        //   java: 				.toList();
        let mut kuudra_key_items: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = todo!("fabric2seer: the initialiser of `kuudraKeyItems` (TODO above)");
        let mut kuudra_key_prices: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
        kuudra_key_prices.extend(kuudra_profile_data::EMPTY.kuudra_key_prices().iter().map(|(k, v)| (k.clone(), v.clone())));
        for kuudra_key in kuudra_key_items.iter() {
            // TODO(fabric2seer): static `ChatFormatting.stripFormatting` has no Rust counterpart in the port
            // rust: let mut matcher: Option<compat::regex::Matcher> = Some(RegexListUtils::match_in_list_list_function_pattern(&kuudra_key.skyblocker_get_lore_strings(), Arc::new(move |__a0| ChatFormatting::strip_formatting(__a0.clone())), &chest_value::DUNGEON_CHEST_COIN_COST_PATTERN));
            let mut matcher: compat::regex::Matcher = todo!("fabric2seer: the initialiser of `matcher` (TODO above)");
            if true /* java: matcher != null: never null here */ {
                let mut found_string: String = compat::regex::replace_all(&matcher.group(1).unwrap(), "\\D", "");
                // TODO(fabric2seer): the condition: static call NumberUtils.isCreatable(..)
                //   java: !NumberUtils.isCreatable(foundString)
                if !compat::number_utils_is_creatable(&found_string) {
                    continue;
                }
                let mut amount: i32 = found_string.parse::<i32>().expect("NumberFormatException");
                kuudra_key_prices.insert(kuudra_key.get_skyblock_id(), amount);
            }
        }
        let mut stored_data: KuudraProfileData = Kuudra::get_kuudra_profile_data();
        if faction != stored_data.faction() || !(kuudra_key_prices == stored_data.kuudra_key_prices()) {
            let mut new_data: KuudraProfileData = KuudraProfileData::new(faction, stored_data.kuudra_pet(), kuudra_key_prices.clone());
            ProfiledData::put(DATA.get_mut(), new_data.clone());
            DATA.get_mut().lock().borrow().save();
        }
    }

    /// java: private static void checkForKuudraPet(ContainerScreen screen)
    pub fn check_for_kuudra_pet(screen: &ContainerScreen) {
        // TODO(fabric2seer): field `slots` of a value of unknown type
        //   java: for (Slot slot : screen.getMenu().slots) {
        //   java: 			if (slot.hasItem()) {
        //   java: 				PetInfo currentPet = slot.getItem().getPetInfo();
        //   java: 				KuudraProfileData storedData = getKuudraProfileData();
        //   java:
        //   java: 				// Ignore non-pet items
        //   java: 				if (currentPet.isEmpty()) continue;
        //   java:
        //   java: 				// If this is a Kuudra pet, the rarity of this pet is greater than or equal to the rarity of the stored one, and its exp is greater than
        //   java: 				// or equal to that of the stored one then we want to override the stored Kuudra pet data with this one.
        //   java: 				//
        //   java: 				// We also check if the stored pet is empty since in that case we want to update it.
        //   java: 				if (currentPet.type().equals("KUUDRA") && ((currentPet.tierIndex() >= storedData.kuudraPet().tierIndex() && currentPet.exp() >= storedData.kuudraPet().exp()) || storedData.kuudraPet().isEmpty())) {
        //   java: 					KuudraProfileData newData = new KuudraProfileData(storedData.faction(), currentPet, storedData.kuudraKeyPrices());
        //   java: 					DATA.put(newData);
        //   java: 					DATA.save();
        //   java:
        //   java: 					return;
        //   java: 				}
        //   java: 			}
        //   java: 		}
    }

    /// java: private static boolean onMessage(Component text, boolean overlay)
    pub fn on_message(text: &MutableComponent, mut overlay: bool) -> bool {
        if Utils::is_in_kuudra() && !overlay {
            let mut message: String = chat_formatting::strip_formatting(&text.get_string());
            if message == "[NPC] Elle: ARGH! All of the supplies fell into the lava! You need to retrieve them quickly!" {
                compat::set_state(&PHASE, KuudraPhase::RetrieveSupplies);
            }
            if message == "[NPC] Elle: Phew! The Ballista is finally ready! It should be strong enough to tank Kuudra's blows now!" {
                compat::set_state(&PHASE, KuudraPhase::Dps);
            }
            if message == "[NPC] Elle: POW! SURELY THAT'S IT! I don't think he has any more in him!" {
                compat::set_state(&PHASE, KuudraPhase::Other);
            }
            if message == "[NPC] Elle: What just happened!? Is this Kuudra's real lair?" {
                compat::set_state(&PHASE, KuudraPhase::KuudraLair);
            }
        }
        return true;
    }

    /// java: private static void reset()
    pub fn reset() {
        compat::set_state(&PHASE, KuudraPhase::Other);
    }

}
