//! Ported from de.hysky.skyblocker.skyblock.item.slottext.adders.HotmPerkLevelAdder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::heart_of_the_x_adder::HeartOfTheXAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::heart_of_the_x_adder::HeartOfTheXAdderBase;
use crate::de::hysky::skyblocker::skyblock::item::slottext::simple_slot_text_adder::SimpleSlotTextAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::simple_slot_text_adder::SimpleSlotTextAdderBase;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text::SlotText;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::slot_text_adder::ConfigInformation;
use crate::de::hysky::skyblocker::utils::container::slot_text_adder::SlotTextAdder;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::world::item::items;

/// java: private static final ConfigInformation CONFIG_INFORMATION = new ConfigInformation( "hotm_perk_level", "skyblocker.config.uiAndVisuals.slotText.hotmPerkLevel");
pub static CONFIG_INFORMATION: compat::MainThread<ConfigInformation> = compat::MainThread::new(|| ConfigInformation::new2("hotm_perk_level".to_string(), "skyblocker.config.uiAndVisuals.slotText.hotmPerkLevel".to_string()));

/// java: public class HotmPerkLevelAdder extends HeartOfTheXAdder
#[derive(Clone)]
pub struct HotmPerkLevelAdder {
    /// java: extends HeartOfTheXAdder
    pub base: HeartOfTheXAdderBase,
}

impl std::ops::Deref for HotmPerkLevelAdder {
    type Target = HeartOfTheXAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for HotmPerkLevelAdder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl HotmPerkLevelAdder {
    /// java: public HotmPerkLevelAdder()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = HeartOfTheXAdderBase::new_in("^Heart of the Mountain$".to_string(), Some(CONFIG_INFORMATION.get_mut().clone()), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn HeartOfTheXAdder>), __h)
        });
        __this
    }

}

impl HeartOfTheXAdder for HotmPerkLevelAdder {
    fn heart_of_the_x_adder_base(&self) -> &HeartOfTheXAdderBase {
        &self.base
    }
    fn heart_of_the_x_adder_base_mut(&mut self) -> &mut HeartOfTheXAdderBase {
        &mut self.base
    }

    /// java: @Override protected boolean isNonLeveledItem(ItemStack stack) (dispatch: the body for this class)
    fn is_non_leveled_item(&self) -> fn(&compat::Shared<dyn HeartOfTheXAdder>, &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        HotmPerkLevelAdder::is_non_leveled_item_dyn
    }

}

impl SimpleSlotTextAdder for HotmPerkLevelAdder {
    fn simple_slot_text_adder_base(&self) -> &SimpleSlotTextAdderBase {
        &self.base.base
    }
    fn simple_slot_text_adder_base_mut(&mut self) -> &mut SimpleSlotTextAdderBase {
        &mut self.base.base
    }

}

impl RegexContainerMatcher for HotmPerkLevelAdder {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base.base
    }

}

impl ContainerMatcher for HotmPerkLevelAdder {
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

impl SlotTextAdder for HotmPerkLevelAdder {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId) (dispatch: the body for this class)
    fn get_text(&self) -> fn(&compat::Shared<dyn SlotTextAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32) -> Vec<SlotText> {
        <dyn HeartOfTheXAdder>::get_text_from::<Self>
    }

    /// java: @Override public @Nullable ConfigInformation getConfigInformation() (dispatch: the body for this class)
    fn get_config_information(&self) -> fn(&compat::Shared<dyn SlotTextAdder>) -> Option<ConfigInformation> {
        <dyn SimpleSlotTextAdder>::get_config_information_from::<Self>
    }

}

impl HotmPerkLevelAdder {
    /// java: @Override protected boolean isNonLeveledItem(ItemStack stack)
    pub fn is_non_leveled_item_impl(__this: &compat::Shared<Self>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        return stack.is_item(&items::items().get("coal"));
    }

    /// HotmPerkLevelAdder.isNonLeveledItem as the dispatch of HeartOfTheXAdder.isNonLeveledItem
    pub fn is_non_leveled_item_dyn(__this: &compat::Shared<dyn HeartOfTheXAdder>, __p0: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        Self::is_non_leveled_item_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
