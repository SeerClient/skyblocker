//! Ported from de.hysky.skyblocker.skyblock.item.slottext.adders.PetLevelAdder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::slottext::simple_slot_text_adder::SimpleSlotTextAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::simple_slot_text_adder::SimpleSlotTextAdderBase;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text::SlotText;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::slot_text_adder::ConfigInformation;
use crate::de::hysky::skyblocker::utils::container::slot_text_adder::SlotTextAdder;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::world::item::items;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern LEVEL_PATTERN = Pattern.compile("⭐? ?\\[Lvl (\\d+)].*");
pub static LEVEL_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("⭐? ?\\[Lvl (\\d+)].*"));
/// java: private static final ConfigInformation CONFIG_INFORMATION = new ConfigInformation( "pet_level", "skyblocker.config.uiAndVisuals.slotText.petLevel");
pub static CONFIG_INFORMATION: compat::MainThread<ConfigInformation> = compat::MainThread::new(|| ConfigInformation::new2("pet_level".to_string(), "skyblocker.config.uiAndVisuals.slotText.petLevel".to_string()));

/// java: public class PetLevelAdder extends SimpleSlotTextAdder
#[derive(Clone)]
pub struct PetLevelAdder {
    /// java: extends SimpleSlotTextAdder
    pub base: SimpleSlotTextAdderBase,
}

impl std::ops::Deref for PetLevelAdder {
    type Target = SimpleSlotTextAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PetLevelAdder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PetLevelAdder {
    /// java: public PetLevelAdder()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleSlotTextAdderBase::new6_in(Some(CONFIG_INFORMATION.get_mut().clone()), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleSlotTextAdder>), __h)
        });
        __this
    }

}

impl SimpleSlotTextAdder for PetLevelAdder {
    fn simple_slot_text_adder_base(&self) -> &SimpleSlotTextAdderBase {
        &self.base
    }
    fn simple_slot_text_adder_base_mut(&mut self) -> &mut SimpleSlotTextAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for PetLevelAdder {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for PetLevelAdder {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

    /// java: @Override default boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        <dyn SlotTextAdder>::is_enabled_from::<Self>
    }

}

impl SlotTextAdder for PetLevelAdder {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId) (dispatch: the body for this class)
    fn get_text(&self) -> fn(&compat::Shared<dyn SlotTextAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32) -> Vec<SlotText> {
        PetLevelAdder::get_text_dyn
    }

    /// java: @Override public @Nullable ConfigInformation getConfigInformation() (dispatch: the body for this class)
    fn get_config_information(&self) -> fn(&compat::Shared<dyn SlotTextAdder>) -> Option<ConfigInformation> {
        <dyn SimpleSlotTextAdder>::get_config_information_from::<Self>
    }

}

impl PetLevelAdder {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId)
    pub fn get_text_impl(__this: &compat::Shared<Self>, mut slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot_id: i32) -> Vec<SlotText> {
        if !(stack.is_item(&items::items().get("player_head"))) || !(stack.get_skyblock_id() == "PET") {
            return vec![];
        }
        let mut name: String = compat::component_of(stack.get_hover_name()).get_string();
        let mut matcher: compat::regex::Matcher = LEVEL_PATTERN.matcher(&name);
        if !matcher.matches() {
            return vec![];
        }
        let mut level: String = matcher.group(1).unwrap();
        // TODO(fabric2seer): the condition: static call NumberUtils.isDigits(..)
        //   java: !NumberUtils.isDigits(level) || "100".equals(level) && (!name.contains("Dragon") || name.contains("Ender Dragon")) || "200".equals(level)
        if !(!level.is_empty() && level.chars().all(char::is_numeric)) || (level == "100" && (!name.contains("Dragon") || name.contains("Ender Dragon"))) || level == "200" {
            return vec![];
        }
        return SlotText::top_left_list(&component::literal(&level).with_color(slot_text::CREAM));
    }

    /// PetLevelAdder.getText as the dispatch of SlotTextAdder.getText
    pub fn get_text_dyn(__this: &compat::Shared<dyn SlotTextAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32) -> Vec<SlotText> {
        Self::get_text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}
