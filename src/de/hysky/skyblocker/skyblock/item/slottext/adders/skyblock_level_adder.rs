//! Ported from de.hysky.skyblocker.skyblock.item.slottext.adders.SkyblockLevelAdder by fabric2seer.

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
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final ConfigInformation CONFIG_INFORMATION = new ConfigInformation( "skyblock_level", "skyblocker.config.uiAndVisuals.slotText.skyblockLevel");
pub static CONFIG_INFORMATION: compat::MainThread<ConfigInformation> = compat::MainThread::new(|| ConfigInformation::new2("skyblock_level".to_string(), "skyblocker.config.uiAndVisuals.slotText.skyblockLevel".to_string()));

/// java: public class SkyblockLevelAdder extends SimpleSlotTextAdder
#[derive(Clone)]
pub struct SkyblockLevelAdder {
    /// java: extends SimpleSlotTextAdder
    pub base: SimpleSlotTextAdderBase,
}

impl std::ops::Deref for SkyblockLevelAdder {
    type Target = SimpleSlotTextAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SkyblockLevelAdder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SkyblockLevelAdder {
    /// java: public SkyblockLevelAdder()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleSlotTextAdderBase::new4_in("^SkyBlock Menu".to_string(), Some(CONFIG_INFORMATION.get_mut().clone()), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleSlotTextAdder>), __h)
        });
        __this
    }

}

impl SimpleSlotTextAdder for SkyblockLevelAdder {
    fn simple_slot_text_adder_base(&self) -> &SimpleSlotTextAdderBase {
        &self.base
    }
    fn simple_slot_text_adder_base_mut(&mut self) -> &mut SimpleSlotTextAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for SkyblockLevelAdder {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for SkyblockLevelAdder {
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

impl SlotTextAdder for SkyblockLevelAdder {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId) (dispatch: the body for this class)
    fn get_text(&self) -> fn(&compat::Shared<dyn SlotTextAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32) -> Vec<SlotText> {
        SkyblockLevelAdder::get_text_dyn
    }

    /// java: @Override public @Nullable ConfigInformation getConfigInformation() (dispatch: the body for this class)
    fn get_config_information(&self) -> fn(&compat::Shared<dyn SlotTextAdder>) -> Option<ConfigInformation> {
        <dyn SimpleSlotTextAdder>::get_config_information_from::<Self>
    }

}

impl SkyblockLevelAdder {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId)
    pub fn get_text_impl(__this: &compat::Shared<Self>, mut slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot_id: i32) -> Vec<SlotText> {
        if slot_id != 22 {
            return vec![];
        }
        let mut lore: Vec<MutableComponent> = ItemUtils::get_lore(stack);
        if lore.is_empty() {
            return vec![];
        }
        let siblings = lore[0].get_siblings();
        if (siblings.len() as i32) < 3 {
            return vec![];
        }
        let mut level_text: String = (siblings[(2) as usize]).get_string();
        //   java: !NumberUtils.isDigits(levelText)
        if !(!level_text.is_empty() && level_text.chars().all(char::is_numeric)) {
            return vec![];
        }
        return SlotText::bottom_left_list(&component::literal(&level_text).with_color(slot_text::CREAM));
    }

    /// SkyblockLevelAdder.getText as the dispatch of SlotTextAdder.getText
    pub fn get_text_dyn(__this: &compat::Shared<dyn SlotTextAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32) -> Vec<SlotText> {
        Self::get_text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}
