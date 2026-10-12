//! Ported from de.hysky.skyblocker.skyblock.item.slottext.adders.HotfPerkLevelAdder by fabric2seer.

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
use mcrust::net::minecraft::references::block_item_ids::BlockItemIds;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::core::component::data_component_map::DataComponentGetter as _;

/// java: private static final ConfigInformation CONFIG_INFORMATION = new ConfigInformation( "hotf_perk_level", "skyblocker.config.uiAndVisuals.slotText.hotfPerkLevel");
pub static CONFIG_INFORMATION: compat::MainThread<ConfigInformation> = compat::MainThread::new(|| ConfigInformation::new2("hotf_perk_level".to_string(), "skyblocker.config.uiAndVisuals.slotText.hotfPerkLevel".to_string()));

/// java: public class HotfPerkLevelAdder extends HeartOfTheXAdder
#[derive(Clone)]
pub struct HotfPerkLevelAdder {
    /// java: extends HeartOfTheXAdder
    pub base: HeartOfTheXAdderBase,
}

impl std::ops::Deref for HotfPerkLevelAdder {
    type Target = HeartOfTheXAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for HotfPerkLevelAdder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl HotfPerkLevelAdder {
    /// java: public HotfPerkLevelAdder()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = HeartOfTheXAdderBase::new_in("^Heart of the Forest$".to_string(), Some(CONFIG_INFORMATION.get_mut().clone()), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn HeartOfTheXAdder>), __h)
        });
        __this
    }

}

impl HeartOfTheXAdder for HotfPerkLevelAdder {
    fn heart_of_the_x_adder_base(&self) -> &HeartOfTheXAdderBase {
        &self.base
    }
    fn heart_of_the_x_adder_base_mut(&mut self) -> &mut HeartOfTheXAdderBase {
        &mut self.base
    }

    /// java: @Override protected boolean isNonLeveledItem(ItemStack stack) (dispatch: the body for this class)
    fn is_non_leveled_item(&self) -> fn(&compat::Shared<dyn HeartOfTheXAdder>, &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        HotfPerkLevelAdder::is_non_leveled_item_dyn
    }

}

impl SimpleSlotTextAdder for HotfPerkLevelAdder {
    fn simple_slot_text_adder_base(&self) -> &SimpleSlotTextAdderBase {
        &self.base.base
    }
    fn simple_slot_text_adder_base_mut(&mut self) -> &mut SimpleSlotTextAdderBase {
        &mut self.base.base
    }

}

impl RegexContainerMatcher for HotfPerkLevelAdder {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base.base
    }

}

impl ContainerMatcher for HotfPerkLevelAdder {
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

impl SlotTextAdder for HotfPerkLevelAdder {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId) (dispatch: the body for this class)
    fn get_text(&self) -> fn(&compat::Shared<dyn SlotTextAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32) -> Vec<SlotText> {
        <dyn HeartOfTheXAdder>::get_text_from::<Self>
    }

    /// java: @Override public @Nullable ConfigInformation getConfigInformation() (dispatch: the body for this class)
    fn get_config_information(&self) -> fn(&compat::Shared<dyn SlotTextAdder>) -> Option<ConfigInformation> {
        <dyn SimpleSlotTextAdder>::get_config_information_from::<Self>
    }

}

impl HotfPerkLevelAdder {
    /// java: @Override protected boolean isNonLeveledItem(ItemStack stack)
    pub fn is_non_leveled_item_impl(__this: &compat::Shared<Self>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        let mut item_model: Identifier = stack.get(mcrust::net::minecraft::core::component::data_components::item_model()).unwrap();
        // TODO(fabric2seer): static `BlockItemIds.PALE_OAK_BUTTON` has no Rust counterpart in the port's BlockItemIds file
        // rust: return (BlockItemIds::PALE_OAK_BUTTON.item().identifier() == item_model);
        return todo!("fabric2seer: see the TODO above");
    }

    /// HotfPerkLevelAdder.isNonLeveledItem as the dispatch of HeartOfTheXAdder.isNonLeveledItem
    pub fn is_non_leveled_item_dyn(__this: &compat::Shared<dyn HeartOfTheXAdder>, __p0: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        Self::is_non_leveled_item_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
