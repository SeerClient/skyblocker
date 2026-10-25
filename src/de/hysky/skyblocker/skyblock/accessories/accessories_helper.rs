//! Ported from de.hysky.skyblocker.skyblock.accessories.AccessoriesHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::accessories::accessories_helper_widget::AccessoriesHelperWidget;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type::TooltipInfoType;
use crate::de::hysky::skyblocker::utils::data::profiled_data::ProfiledData;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::net::minecraft::client::gui::screens::inventory::container_screen::ContainerScreen;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData as _;

thread_local! {
    /// The page of the accessory bag screen whose `ScreenEvents.afterTick` listener is registered (`None`: the open
    /// screen registered none).
    static AFTER_TICK_PAGE: std::cell::Cell<Option<i32>> = const { std::cell::Cell::new(None) };
}

/// java: private static final ObjectOpenHashSet<String> EMPTY = new ObjectOpenHashSet<>(0);
pub static EMPTY: LazyLock<std::collections::HashSet<String>> = LazyLock::new(|| std::collections::HashSet::with_capacity((0) as usize));
/// java: private static final Path FILE = SkyblockerMod.CONFIG_DIR.resolve("collected_accessories.json");
pub static FILE: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("collected_accessories.json"));
/// java: static final Pattern ACCESSORY_BAG_TITLE = Pattern.compile("Accessory Bag(?: \\((?<page>\\d+)/\\d+\\))?");
pub static ACCESSORY_BAG_TITLE: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Accessory Bag(?: \\((?<page>\\d+)/\\d+\\))?"));
/// java: private static final ProfiledData<ProfileAccessoryData> COLLECTED_ACCESSORIES = new ProfiledData<>(FILE, ProfileAccessoryData.CODEC, true);
pub static COLLECTED_ACCESSORIES: compat::MainThread<compat::Shared<ProfiledData<ProfileAccessoryData>>> = compat::MainThread::new(|| ProfiledData::new_3(FILE.get_mut().clone(), PROFILE_ACCESSORY_DATA_CODEC.get_mut().clone(), true));
/// java: private static final Predicate<String> NON_EMPTY = s -> !s.isEmpty();
pub static NON_EMPTY: compat::MainThread<Arc<dyn Fn(String) -> bool>> = compat::MainThread::new(|| Arc::new(move |s| !s.is_empty()));
/// java: private static final Predicate<Accessory> HAS_FAMILY = Accessory::hasFamily;
pub static HAS_FAMILY: compat::MainThread<Arc<dyn Fn(Accessory) -> bool>> = compat::MainThread::new(|| Arc::new(move |__a0| __a0.has_family()));
/// java: private static final ToIntFunction<Accessory> ACCESSORY_TIER = Accessory::tier;
pub static ACCESSORY_TIER: compat::MainThread<Arc<dyn Fn(Accessory) -> i32>> = compat::MainThread::new(|| Arc::new(move |__a0| __a0.tier()));
/// java: public static Map<String, Accessory> ACCESSORY_DATA = new Object2ObjectOpenHashMap<>();
pub static ACCESSORY_DATA: LazyLock<Mutex<std::collections::HashMap<String, Accessory>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));
//   java: private static final Codec<ProfileAccessoryData> CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 				Codec.unboundedMap(Codec.INT, Codec.STRING.listOf().xmap(ObjectOpenHashSet::new, ObjectArrayList::new))
//   java: 						.xmap(Int2ObjectOpenHashMap::new, Int2ObjectOpenHashMap::new).fieldOf("pages").forGetter(ProfileAccessoryData::pages),
//   java: 				Codec.STRING.listOf().optionalFieldOf("recombobulatedAccessories", List.of()).xmap(ObjectOpenHashSet::new, List::copyOf).forGetter(ProfileAccessoryData::recombobulatedAccessories)
//   java: 		).apply(instance, ProfileAccessoryData::new));
/// java: private static final Codec<ProfileAccessoryData> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.unboundedMap(Codec.INT, Codec.STRING.listOf().xmap(ObjectOpenHashSet::new, ObjectAr
pub static PROFILE_ACCESSORY_DATA_CODEC: compat::MainThread<CodecRef<ProfileAccessoryData>> = compat::MainThread::new(|| {
    // Codec.STRING.listOf().xmap(ObjectOpenHashSet::new, ObjectArrayList::new / List::copyOf)
    let string_set = || codec::xmap(codec::list(codec::string_codec()), |list: Vec<String>| list.into_iter().collect::<std::collections::HashSet<String>>(), |set: &std::collections::HashSet<String>| set.iter().cloned().collect::<Vec<String>>());
    record_codec_builder::create(record_codec_builder::apply2(
        // Codec.unboundedMap(Codec.INT, ..).xmap(Int2ObjectOpenHashMap::new, Int2ObjectOpenHashMap::new).fieldOf("pages")
        record_codec_builder::of(|o: &ProfileAccessoryData| o.pages(), codec::field_of(codec::xmap(codec::unbounded_map(codec::int_codec(), string_set()), |entries: Vec<(i32, std::collections::HashSet<String>)>| entries.into_iter().collect::<std::collections::HashMap<_, _>>(), |map: &std::collections::HashMap<i32, std::collections::HashSet<String>>| map.iter().map(|(k, v)| (*k, v.clone())).collect::<Vec<_>>()), "pages")),
        // .optionalFieldOf("recombobulatedAccessories", List.of()).xmap(ObjectOpenHashSet::new, List::copyOf): the empty
        // default maps to the empty set, so the xmap is applied to the element codec instead
        record_codec_builder::of(|o: &ProfileAccessoryData| o.recombobulated_accessories(), codec::optional_field_of(string_set(), "recombobulatedAccessories", std::collections::HashSet::new())),
        move |__a0, __a1| ProfileAccessoryData::new(__a0.clone(), __a1.clone()),
    ))
});
/// java: private static final Codec<Accessory> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("id").forGetter(Accessory::id), Codec.STRING.optionalFieldOf("family").forGette
pub static ACCESSORY_CODEC: compat::MainThread<CodecRef<Accessory>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply6(record_codec_builder::of(|o: &Accessory| o.id(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "id")), record_codec_builder::of(|o: &Accessory| o.family(), codec::optional_field_of_opt(mcrust::com::mojang::serialization::codec::string_codec(), "family")), record_codec_builder::of(|o: &Accessory| o.tier(), codec::optional_field_of(mcrust::com::mojang::serialization::codec::int_codec(), "tier", 0)), record_codec_builder::of(|o: &Accessory| o.origin(), codec::optional_field_of_opt(mcrust::com::mojang::serialization::codec::string_codec(), "origin")), record_codec_builder::of(|o: &Accessory| o.enrichable(), codec::optional_field_of(mcrust::com::mojang::serialization::codec::bool_codec(), "enrichable", true)), record_codec_builder::of(|o: &Accessory| o.recombobulatable(), codec::optional_field_of(mcrust::com::mojang::serialization::codec::bool_codec(), "recombobulatable", true)), move |__a0, __a1, __a2, __a3, __a4, __a5| Accessory::new(__a0.clone(), __a1.clone(), __a2.clone(), __a3.clone(), __a4.clone(), __a5.clone()))));
/// java: public static final Codec<Map<String, Accessory>> MAP_CODEC = Codec.unboundedMap(Codec.STRING, CODEC);
///
/// The port's `unboundedMap` answers the entries as a list; the map is built over it.
pub static ACCESSORY_MAP_CODEC: compat::MainThread<CodecRef<std::collections::HashMap<String, Accessory>>> = compat::MainThread::new(|| codec::xmap(codec::unbounded_map(mcrust::com::mojang::serialization::codec::string_codec(), ACCESSORY_CODEC.get_mut().clone()), |entries: Vec<(String, Accessory)>| entries.into_iter().collect::<std::collections::HashMap<_, _>>(), |map: &std::collections::HashMap<String, Accessory>| map.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>()));

/// java: public class AccessoriesHelper
#[derive(Clone)]
pub struct AccessoriesHelper {}

impl AccessoriesHelper {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        COLLECTED_ACCESSORIES.get_mut().lock().borrow_mut().init();
        AccessoriesHelper::install_after_tick();
        AccessoriesHelperWidget::install_screen_hooks();
        screen_events::AFTER_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            // `ScreenEvents.afterTick(screen)` and `attachToScreen(screen)` belong to the screen being initialised: a
            // new screen (or a re-init) starts with neither, as a new Java screen has no listeners of its own
            AFTER_TICK_PAGE.with(|p| p.set(None));
            AccessoriesHelperWidget::cancel_attach();
            if Utils::is_on_skyblock() && { let __r = tooltip_info_type::ACCESSORIES.get_mut(); let __f = compat::dispatch(__r, |__t| __t.is_tooltip_enabled()); __f(&compat::view(__r, |a| a as compat::Shared<dyn TooltipInfoType>)) } && matches!(compat::minecraft().gui.screen_kind, mcrust::net::minecraft::client::gui::gui::ScreenKind::Container) && !(crate::entry::catharsis_compatibility::is_gui_element_hidden("skyblocker:accessoriesHelper")) {
                // `screen instanceof ContainerScreen genericContainerScreen`: the screen's kind; its title is the
                // `Screen` half the listener is handed
                let mut matcher: compat::regex::Matcher = ACCESSORY_BAG_TITLE.matcher(&screen.get_title().to_string());
                if matcher.matches() {
                    // java: ScreenEvents.afterTick(screen).register(_ -> {
                    // java: 	ChestMenu handler = genericContainerScreen.getMenu();
                    // java: 	int page = matcher.group("page") != null ? Integer.parseInt(matcher.group("page")) : 1;
                    // java: 	collectAccessories(handler.slots.subList(0, handler.getRowCount() * 9), page);
                    // java: });
                    // (Fabric's per-screen `afterTick` is the end of `Screen.tick`, whose last statement for a container
                    // screen is `containerTick()`: the listener is this screen's page, run by `install_after_tick`'s
                    // RETURN injector there. The matcher is evaluated once, as the captured Java matcher answers the same
                    // group on every tick.)
                    let page: i32 = match matcher.group_named("page") {
                        Some(page) => page.parse::<i32>().unwrap_or_else(|_| panic!("NumberFormatException: For input string: \"{page}\"")),
                        None => 1,
                    };
                    AFTER_TICK_PAGE.with(|p| p.set(Some(page)));
                    // java: AccessoriesHelperWidget.attachToScreen(genericContainerScreen);
                    // (the listener is handed the `Screen` half; the container screen around it - `leftPos`, its widget
                    // list - is attached to as `AbstractContainerScreen.init` returns, which is where this listener runs)
                    AccessoriesHelperWidget::request_attach();
                }
            }
        }));
    }

    /// java: `ScreenEvents.afterTick(screen).register(..)` for the accessory bag screen (see `init`): the end of the open
    /// container screen's tick (`AbstractContainerScreen.tick:547`'s `containerTick()`), while that screen's AFTER_INIT
    /// recorded a page.
    fn install_after_tick() {
        let result = mcrust::mcrust::modding::hook::inject(
            "net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#containerTick",
            mcrust::mcrust::modding::hook::At::Return,
            |_ci: &mut mcrust::mcrust::modding::hook::CallbackInfo<'_>| {
                let Some(page) = AFTER_TICK_PAGE.with(|p| p.get()) else {
                    return;
                };
                // java: ChestMenu handler = genericContainerScreen.getMenu();
                // java: collectAccessories(handler.slots.subList(0, handler.getRowCount() * 9), page);
                let Some(rows) = mcrust::mcrust::modding::game::with_chest_menu(|handler| handler.get_row_count()) else {
                    return;
                };
                let count: usize = (rows.wrapping_mul(9)) as usize;
                let Some(slots) = compat::with_open_menu(|menu, _| menu.slots[0..count].to_vec()) else {
                    return;
                };
                AccessoriesHelper::collect_accessories(&slots, page);
            },
        );
        if let Err(e) = result {
            mcrust::mcrust::modding::log::warn_once("skyblocker-accessories-after-tick", &format!("{e:?}"));
        }
    }

    /// java: private static void collectAccessories(List<Slot> slots, int page)
    pub fn collect_accessories(slots: &Vec<Slot>, mut page: i32) {
        if !(COLLECTED_ACCESSORIES.get_mut().lock().borrow().is_loaded()) {
            return;
        }
        // `Slot::getItem` of the open menu's slots
        let items: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = slots.iter().map(|slot| compat::slot_item(slot)).collect();
        let mut accessory_ids: Vec<String> = items.iter().map(|stack| stack.get_skyblock_id()).filter(|s| (NON_EMPTY.get())(s.clone())).collect();
        let mut recombobulated: Vec<String> = items.iter().filter(|stack| ItemUtils::get_custom_data(*stack).get_int_or("rarity_upgrades", 0) > 0).map(|stack| stack.get_skyblock_id()).filter(|s| (NON_EMPTY.get())(s.clone())).collect();
        let mut data: ProfileAccessoryData = ProfiledData::compute_if_absent(COLLECTED_ACCESSORIES.get_mut(), Arc::new(move || ProfileAccessoryData::create_default()));
        { for __x in data.pages.get(&page).cloned().unwrap_or_else(|| EMPTY.clone()).into_iter() { data.recombobulated_accessories.remove(&__x); } };
        data.recombobulated_accessories.extend(recombobulated.iter().cloned());
        data.pages.insert(page, accessory_ids.iter().cloned().collect::<std::collections::HashSet<_>>());
        // the record's sets ARE the stored data in Java; the port's `computeIfAbsent` answers a copy, so it is put back
        ProfiledData::put(COLLECTED_ACCESSORIES.get_mut(), data);
    }

    /// java: public static Pair<AccessoryReport, String> calculateReport4Accessory(String accessoryId)
    pub fn calculate_report4_accessory(accessory_id: &str) -> (AccessoryReport, String) {
        if !(compat::lock(&ACCESSORY_DATA).contains_key(accessory_id)) || Utils::get_profile_id().is_empty() {
            return (AccessoryReport::Ineligible, "".to_string());
        }
        let mut accessory: Accessory = compat::lock(&ACCESSORY_DATA).get(accessory_id).cloned().unwrap();
        if accessory.origin().unwrap_or("".to_string()) == "RIFT" {
            return (AccessoryReport::Ineligible, "".to_string());
        }
        let mut collected_accessories: std::collections::HashSet<Accessory> = AccessoriesHelper::get_collected_accessories();
        if accessory.family().is_none() {
            return (if collected_accessories.contains(&accessory) { (AccessoryReport::HasHighestTier, "".to_string()) } else { (AccessoryReport::Missing, "".to_string()) });
        }
        let mut report: FamilyReport = AccessoriesHelper::calculate_family_report(&accessory, &collected_accessories);
        let mut highest_tier_in_family: i32 = report.highest_in_family().tier();
        if report.highest_collected_in_family().is_none() {
            return (AccessoryReport::Missing, format!("({}/{})", accessory.tier(), highest_tier_in_family));
        }
        let mut highest_tier_collected_in_family: i32 = report.highest_collected_in_family().unwrap().tier();
        if accessory.tier() == highest_tier_in_family && highest_tier_collected_in_family == highest_tier_in_family {
            return (AccessoryReport::HasHighestTier, "".to_string());
        }
        if accessory.tier() > highest_tier_collected_in_family {
            return (AccessoryReport::IsGreaterTier, format!("({}→{}/{})", highest_tier_collected_in_family, accessory.tier(), highest_tier_in_family));
        }
        if accessory.tier() < highest_tier_collected_in_family {
            return (AccessoryReport::OwnsBetterTier, format!("({}→{}/{})", highest_tier_collected_in_family, accessory.tier(), highest_tier_in_family));
        }
        if accessory.tier() < highest_tier_in_family {
            return (AccessoryReport::HasGreaterTier, format!("({}/{})", highest_tier_collected_in_family, highest_tier_in_family));
        }
        return (AccessoryReport::Missing, format!("({}/{})", accessory.tier(), highest_tier_in_family));
    }

    /// java: public static FamilyReport calculateFamilyReport(Accessory accessory, Set<Accessory> collectedAccessories)
    pub fn calculate_family_report(accessory: &Accessory, collected_accessories: &std::collections::HashSet<Accessory>) -> FamilyReport {
        if accessory.family().is_none() {
            panic!("IllegalArgumentException: {}", "accessory family cannot be empty");
        }
        let __accessory: Accessory = accessory.clone();
        let mut has_same_family: Arc<dyn Fn(Accessory) -> bool> = Arc::new(move |__a0| __accessory.has_same_family(&__a0));
        //   java: return new FamilyReport(
        //   java: 				ACCESSORY_DATA.values().stream()
        //   java: 						.filter(HAS_FAMILY)
        //   java: 						.filter(hasSameFamily)
        //   java: 						.max(Comparator.comparingInt(ACCESSORY_TIER))
        //   java: 						.orElse(accessory),
        //   java: 				collectedAccessories.stream()
        //   java: 						.filter(HAS_FAMILY)
        //   java: 						.filter(hasSameFamily)
        //   java: 						.max(Comparator.comparingInt(ACCESSORY_TIER))
        //   java: 		);
        // (`Stream.max(comparator)` is `reduce(BinaryOperator.maxBy(comparator))`, which keeps the FIRST of equal
        // elements - `Iterator::max_by_key` keeps the last)
        let max_by_tier = |best: Option<Accessory>, candidate: Accessory| -> Option<Accessory> {
            match best {
                Some(best) if (ACCESSORY_TIER.get())(best.clone()) >= (ACCESSORY_TIER.get())(candidate.clone()) => Some(best),
                _ => Some(candidate),
            }
        };
        let highest_in_family: Accessory = compat::lock(&ACCESSORY_DATA).values().cloned()
            .filter(|a| (HAS_FAMILY.get())(a.clone()))
            .filter(|a| (has_same_family)(a.clone()))
            .fold(None, max_by_tier)
            .unwrap_or_else(|| accessory.clone());
        let highest_collected_in_family: Option<Accessory> = collected_accessories.iter().cloned()
            .filter(|a| (HAS_FAMILY.get())(a.clone()))
            .filter(|a| (has_same_family)(a.clone()))
            .fold(None, max_by_tier);
        return FamilyReport::new(highest_in_family, highest_collected_in_family);
    }

    /// java: public static Set<Accessory> getCollectedAccessories()
    pub fn get_collected_accessories() -> std::collections::HashSet<Accessory> {
        let data: ProfileAccessoryData = ProfiledData::compute_if_absent(COLLECTED_ACCESSORIES.get_mut(), Arc::new(move || ProfileAccessoryData::create_default()));
        let accessory_data = compat::lock(&ACCESSORY_DATA);
        return data.pages.values().flat_map(|set| set.iter()).filter_map(|id| accessory_data.get(id).cloned()).collect::<std::collections::HashSet<Accessory>>();
    }

    /// java: public static boolean hasAccessory(String accessoryId)
    pub fn has_accessory(accessory_id: &str) -> bool {
        let data: ProfileAccessoryData = ProfiledData::compute_if_absent(COLLECTED_ACCESSORIES.get_mut(), Arc::new(move || ProfileAccessoryData::create_default()));
        return data.pages.values().any(|set| set.contains(accessory_id));
    }

    /// java: public static boolean isRecombobulated(String accessoryId)
    pub fn is_recombobulated(accessory_id: &str) -> bool {
        return AccessoriesHelper::has_accessory(accessory_id) && ProfiledData::compute_if_absent(COLLECTED_ACCESSORIES.get_mut(), Arc::new(move || ProfileAccessoryData::create_default())).recombobulated_accessories.contains(accessory_id);
    }

    /// java: public static void refreshData(Map<String, Accessory> data)
    pub fn refresh_data(mut data: std::collections::HashMap<String, Accessory>) {
        compat::set_state(&ACCESSORY_DATA, data.clone());
    }

}

/// java: private record ProfileAccessoryData(Int2ObjectOpenHashMap<ObjectOpenHashSet<String>> pages, ObjectOpenHashSet<String> recombobulatedAccessories)
#[derive(Clone, Debug, PartialEq)]
pub struct ProfileAccessoryData {
    /// java: Int2ObjectOpenHashMap<ObjectOpenHashSet<String>> pages
    pub pages: std::collections::HashMap<i32, std::collections::HashSet<String>>,
    /// java: ObjectOpenHashSet<String> recombobulatedAccessories
    pub recombobulated_accessories: std::collections::HashSet<String>,
}

impl ProfileAccessoryData {
    /// java: the canonical record constructor
    pub fn new(mut pages: std::collections::HashMap<i32, std::collections::HashSet<String>>, mut recombobulated_accessories: std::collections::HashSet<String>) -> Self {
        Self { pages, recombobulated_accessories }
    }

    /// java: the record accessor `pages()`
    pub fn pages(&self) -> std::collections::HashMap<i32, std::collections::HashSet<String>> {
        self.pages.clone()
    }

    /// java: the record accessor `recombobulatedAccessories()`
    pub fn recombobulated_accessories(&self) -> std::collections::HashSet<String> {
        self.recombobulated_accessories.clone()
    }

    /// java: private static ProfileAccessoryData createDefault()
    pub fn create_default() -> ProfileAccessoryData {
        return ProfileAccessoryData::new(std::collections::HashMap::new(), std::collections::HashSet::new());
    }

}

/// java: public record Accessory(String id, Optional<String> family, int tier, Optional<String> origin, boolean enrichable, boolean recombobulatable)
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Accessory {
    /// java: String id
    pub id: String,
    /// java: Optional<String> family
    pub family: Option<String>,
    /// java: int tier
    pub tier: i32,
    /// java: Optional<String> origin
    pub origin: Option<String>,
    /// java: boolean enrichable
    pub enrichable: bool,
    /// java: boolean recombobulatable
    pub recombobulatable: bool,
}

impl Accessory {
    /// java: the canonical record constructor
    pub fn new(mut id: String, mut family: Option<String>, mut tier: i32, mut origin: Option<String>, mut enrichable: bool, mut recombobulatable: bool) -> Self {
        Self { id, family, tier, origin, enrichable, recombobulatable }
    }

    /// java: the record accessor `id()`
    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// java: the record accessor `family()`
    pub fn family(&self) -> Option<String> {
        self.family.clone()
    }

    /// java: the record accessor `tier()`
    pub fn tier(&self) -> i32 {
        self.tier
    }

    /// java: the record accessor `origin()`
    pub fn origin(&self) -> Option<String> {
        self.origin.clone()
    }

    /// java: the record accessor `enrichable()`
    pub fn enrichable(&self) -> bool {
        self.enrichable
    }

    /// java: the record accessor `recombobulatable()`
    pub fn recombobulatable(&self) -> bool {
        self.recombobulatable
    }

    /// java: public boolean hasFamily()
    pub fn has_family(&self) -> bool {
        return self.family.is_some();
    }

    /// java: public boolean hasSameFamily(Accessory other)
    pub fn has_same_family(&self, other: &Accessory) -> bool {
        // java: return other.family().equals(this.family); - `Optional.equals`
        return other.family() == self.family;
    }

}

/// java: public record FamilyReport(Accessory highestInFamily, Optional<Accessory> highestCollectedInFamily)
#[derive(Clone, Debug, PartialEq)]
pub struct FamilyReport {
    /// java: Accessory highestInFamily
    pub highest_in_family: Accessory,
    /// java: Optional<Accessory> highestCollectedInFamily
    pub highest_collected_in_family: Option<Accessory>,
}

impl FamilyReport {
    /// java: the canonical record constructor
    pub fn new(mut highest_in_family: Accessory, mut highest_collected_in_family: Option<Accessory>) -> Self {
        Self { highest_in_family, highest_collected_in_family }
    }

    /// java: the record accessor `highestInFamily()`
    pub fn highest_in_family(&self) -> Accessory {
        self.highest_in_family.clone()
    }

    /// java: the record accessor `highestCollectedInFamily()`
    pub fn highest_collected_in_family(&self) -> Option<Accessory> {
        self.highest_collected_in_family.clone()
    }

}

/// java: public enum AccessoryReport
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AccessoryReport {
    HasHighestTier,
    IsGreaterTier,
    HasGreaterTier,
    OwnsBetterTier,
    Missing,
    Ineligible,
}

impl AccessoryReport {
    pub const VALUES: [AccessoryReport; 6] = [AccessoryReport::HasHighestTier, AccessoryReport::IsGreaterTier, AccessoryReport::HasGreaterTier, AccessoryReport::OwnsBetterTier, AccessoryReport::Missing, AccessoryReport::Ineligible];

    /// java: values()
    pub fn values() -> Vec<AccessoryReport> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            AccessoryReport::HasHighestTier => "HAS_HIGHEST_TIER",
            AccessoryReport::IsGreaterTier => "IS_GREATER_TIER",
            AccessoryReport::HasGreaterTier => "HAS_GREATER_TIER",
            AccessoryReport::OwnsBetterTier => "OWNS_BETTER_TIER",
            AccessoryReport::Missing => "MISSING",
            AccessoryReport::Ineligible => "INELIGIBLE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> AccessoryReport {
        match name {
            "HAS_HIGHEST_TIER" => AccessoryReport::HasHighestTier,
            "IS_GREATER_TIER" => AccessoryReport::IsGreaterTier,
            "HAS_GREATER_TIER" => AccessoryReport::HasGreaterTier,
            "OWNS_BETTER_TIER" => AccessoryReport::OwnsBetterTier,
            "MISSING" => AccessoryReport::Missing,
            "INELIGIBLE" => AccessoryReport::Ineligible,
            _ => panic!("IllegalArgumentException: No enum constant AccessoryReport.{name}"),
        }
    }

}

impl std::fmt::Display for AccessoryReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
