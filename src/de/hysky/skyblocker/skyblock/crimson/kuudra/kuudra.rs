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

/// The `ScreenEvents.afterTick` listener the current container screen's AFTER_INIT registered.
#[derive(Clone)]
enum AfterTick {
    /// `checkKuudraKeyShop(genericContainerScreen, factionShopMatcher)` (the matched title)
    KeyShop(String),
    /// `checkForKuudraPet(genericContainerScreen)`
    Pets,
}

thread_local! {
    static AFTER_TICK: std::cell::RefCell<Option<AfterTick>> = const { std::cell::RefCell::new(None) };
}

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
            // object itself is not lent to the listener - the menu is read at the after-tick hook)
            if Utils::is_on_skyblock() && matches!(compat::minecraft().gui.screen_kind, mcrust::net::minecraft::client::gui::gui::ScreenKind::Container) {
                let mut title: String = screen.get_title().to_string();
                let mut faction_shop_matcher: compat::regex::Matcher = FACTION_SHOP_PATTERN.matcher(&title);
                // (Fabric's per-screen `afterTick` is the end of the container screen's `containerTick()`: the listener
                // belongs to this screen, so a new screen's AFTER_INIT replaces it - see `install_after_tick`)
                AFTER_TICK.with(|t| *t.borrow_mut() = None);
                if faction_shop_matcher.matches() {
                    // java: ScreenEvents.afterTick(screen).register(_ -> checkKuudraKeyShop(genericContainerScreen, factionShopMatcher));
                    AFTER_TICK.with(|t| *t.borrow_mut() = Some(AfterTick::KeyShop(title.clone())));
                } else if title.ends_with("Pets") {
                    // java: ScreenEvents.afterTick(screen).register(_ -> checkForKuudraPet(genericContainerScreen));
                    AFTER_TICK.with(|t| *t.borrow_mut() = Some(AfterTick::Pets));
                }
            } else {
                AFTER_TICK.with(|t| *t.borrow_mut() = None);
            }
        }));
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            Kuudra::reset();
        }));
        // java: ClientReceiveMessageEvents.ALLOW_GAME.register(Kuudra::onMessage);
        compat::batch_w_allow_game_register(Arc::new(|text: &MutableComponent, overlay: bool| Kuudra::on_message(text, overlay)));
        Kuudra::install_after_tick();
    }

    /// java: the two `ScreenEvents.afterTick(screen)` listeners `init` registers: run at the RETURN of the open container
    /// screen's `containerTick()` (the end of its `tick`), while that screen's AFTER_INIT chose one.
    fn install_after_tick() {
        let result = mcrust::mcrust::modding::hook::inject(
            "net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#containerTick",
            mcrust::mcrust::modding::hook::At::Return,
            |_ci: &mut mcrust::mcrust::modding::hook::CallbackInfo<'_>| {
                let Some(listener) = AFTER_TICK.with(|t| t.borrow().clone()) else {
                    return;
                };
                if !matches!(compat::minecraft().gui.screen_kind, mcrust::net::minecraft::client::gui::gui::ScreenKind::Container) {
                    return;
                }
                // java: screen.getMenu().slots - the items of the open menu's slots
                let Some(items) = compat::with_open_menu(|menu, containers| menu.slots.iter().filter(|slot| slot.has_item(containers)).map(|slot| slot.get_item(containers).duplicate()).collect::<Vec<_>>()) else {
                    return;
                };
                match listener {
                    AfterTick::KeyShop(title) => {
                        // (the captured matcher, matched against the same title)
                        let mut matcher = FACTION_SHOP_PATTERN.matcher(&title);
                        if matcher.matches() {
                            Kuudra::check_kuudra_key_shop(&items, &matcher);
                        }
                    }
                    AfterTick::Pets => Kuudra::check_for_kuudra_pet(&items),
                }
            },
        );
        if let Err(e) = result {
            mcrust::mcrust::modding::log::warn_once("skyblocker-kuudra-after-tick", &format!("{e:?}"));
        }
    }

    /// java: public static KuudraProfileData getKuudraProfileData()
    pub fn get_kuudra_profile_data() -> KuudraProfileData {
        return (if ProfiledData::contains_key(DATA.get_mut()) { ProfiledData::get(DATA.get_mut()) } else { Some(kuudra_profile_data::EMPTY.clone()) }).unwrap();
    }

    /// java: private static void checkKuudraKeyShop(ContainerScreen screen, Matcher factionShopMatcher)
    /// (`screen.getMenu().slots`' items that `Slot::hasItem` - the open container's - are handed in)
    pub fn check_kuudra_key_shop(slot_items: &Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>, faction_shop_matcher: &compat::regex::Matcher) {
        let mut faction: CrimsonFaction = CrimsonFaction::value_of(&faction_shop_matcher.group_named("faction").unwrap().to_uppercase());
        let mut kuudra_key_items: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = slot_items.iter().filter(|stack| kuudra_profile_data::EMPTY.kuudra_key_prices().contains_key(&stack.get_skyblock_id())).cloned().collect();
        let mut kuudra_key_prices: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
        kuudra_key_prices.extend(kuudra_profile_data::EMPTY.kuudra_key_prices().iter().map(|(k, v)| (k.clone(), v.clone())));
        for kuudra_key in kuudra_key_items.iter() {
            let Some(matcher) = RegexListUtils::match_in_list_list_function_pattern(&kuudra_key.skyblocker_get_lore_strings(), Arc::new(move |__a0: String| chat_formatting::strip_formatting(&__a0)), &chest_value::DUNGEON_CHEST_COIN_COST_PATTERN) else {
                continue;
            };
            {
                let mut found_string: String = compat::regex::replace_all(&matcher.group(1).unwrap(), "\\D", "");
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
    /// (the items of `screen.getMenu().slots` that `slot.hasItem()` are handed in)
    pub fn check_for_kuudra_pet(slot_items: &Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>) {
        for item in slot_items.iter() {
            let current_pet = item.get_pet_info();
            let stored_data: KuudraProfileData = Kuudra::get_kuudra_profile_data();
            // Ignore non-pet items
            if current_pet.is_empty() {
                continue;
            }
            if current_pet.r#type() == "KUUDRA" && ((current_pet.tier_index() >= stored_data.kuudra_pet().tier_index() && current_pet.exp() >= stored_data.kuudra_pet().exp()) || stored_data.kuudra_pet().is_empty()) {
                let new_data: KuudraProfileData = KuudraProfileData::new(stored_data.faction(), current_pet, stored_data.kuudra_key_prices().clone());
                ProfiledData::put(DATA.get_mut(), new_data);
                DATA.get_mut().lock().borrow().save();
                return;
            }
        }
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
