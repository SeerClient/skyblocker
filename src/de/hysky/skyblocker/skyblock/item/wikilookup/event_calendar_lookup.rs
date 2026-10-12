//! Ported from de.hysky.skyblocker.skyblock.item.wikilookup.EventCalendarLookup by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::wiki_lookup;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::wiki_lookup::WikiLookup;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::wiki_lookup_manager::WikiLookupManager;
use mcrust::com::mojang::datafixers::util::either::Either;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern CALENDAR_EVENT_NAME = Pattern.compile("^[0-9a-z ]*(?<event>.+)$");
pub static CALENDAR_EVENT_NAME: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^[0-9a-z ]*(?<event>.+)$"));
/// java: public static final EventCalendarLookup INSTANCE = new EventCalendarLookup();
pub static INSTANCE: LazyLock<compat::Shared<EventCalendarLookup>> = LazyLock::new(|| EventCalendarLookup::new());

/// java: public class EventCalendarLookup implements WikiLookup
#[derive(Clone)]
pub struct EventCalendarLookup {}

impl EventCalendarLookup {
    /// java: private EventCalendarLookup()
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
        });
        __this
    }

}

impl WikiLookup for EventCalendarLookup {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void open(ItemStack itemStack, Player player) (dispatch: the body for this class)
    fn open(&self) -> fn(&compat::Shared<dyn WikiLookup>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &dyn Player) {
        EventCalendarLookup::open_dyn
    }

    /// java: @Override public boolean canSearch(@Nullable String title, Either<Slot, ItemStack> either) (dispatch: the body for this class)
    fn can_search(&self) -> fn(&compat::Shared<dyn WikiLookup>, Option<String>, &Either<Slot, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> bool {
        EventCalendarLookup::can_search_dyn
    }

}

impl EventCalendarLookup {
    /// java: @Override public void open(ItemStack itemStack, Player player)
    pub fn open_impl(__this: &compat::Shared<Self>, item_stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, player: &dyn Player) {
        let mut matcher: compat::regex::Matcher = CALENDAR_EVENT_NAME.matcher(&compat::component_of(item_stack.get_hover_name()).get_string());
        if matcher.matches() {
            let mut event_name: String = compat::trim(&matcher.group_named("event").unwrap());
            if event_name == "Election Over!" || event_name == "Election Booth Opens" {
                event_name = "Mayors".to_string();
            } else if event_name.starts_with("Bonus") {
                event_name = compat::substring(&event_name, 6);
            }
            let mut formatted_event: String = (wiki_lookup::REPLACING_FUNCTION.get_mut())(event_name.clone());
            WikiLookupManager::open_wiki_link_name(&formatted_event, player);
        }
    }

    /// EventCalendarLookup.open as the dispatch of WikiLookup.open
    pub fn open_dyn(__this: &compat::Shared<dyn WikiLookup>, __p0: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p1: &dyn Player) {
        Self::open_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public boolean canSearch(@Nullable String title, Either<Slot, ItemStack> either)
    pub fn can_search_impl(__this: &compat::Shared<Self>, mut title: Option<String>, either: &Either<Slot, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> bool {
        let mut optional: Option<Slot> = either.left_value().unwrap();
        if optional.is_none() {
            return false;
        }
        let mut slot: Slot = optional.clone().unwrap();
        if slot.index <= 9 || slot.index >= 26 {
            return false;
        }
        // TODO(fabric2seer): `get_item` takes 1 argument(s) in the port (`containers: &'a Containers<'_>`), Java passes 0
        if slot.get_item().is_item(mcrust::net::minecraft::world::item::items::stained_glass_pane().black()) {
            return false;
        }
        // TODO(fabric2seer): static call StringUtils.isNotEmpty(..)
        //   java: return StringUtils.isNotEmpty(title) && title.matches("^Calendar and Events$");
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// EventCalendarLookup.canSearch as the dispatch of WikiLookup.canSearch
    pub fn can_search_dyn(__this: &compat::Shared<dyn WikiLookup>, __p0: Option<String>, __p1: &Either<Slot, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> bool {
        Self::can_search_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}
