//! Ported from de.hysky.skyblocker.skyblock.item.slottext.adders.PrehistoricEggAdder by fabric2seer.

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
use mcrust::net::minecraft::nbt::tag::Tag;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;

/// java: private static final ConfigInformation CONFIG_INFORMATION = new ConfigInformation( "prehistoric_egg", "skyblocker.config.uiAndVisuals.slotText.prehistoricEgg", "skyblocker.config.uiAndVisuals.slotText
pub static CONFIG_INFORMATION: compat::MainThread<ConfigInformation> = compat::MainThread::new(|| ConfigInformation::new3("prehistoric_egg".to_string(), "skyblocker.config.uiAndVisuals.slotText.prehistoricEgg".to_string(), vec!["skyblocker.config.uiAndVisuals.slotText.prehistoricEgg.@Tooltip".to_string()]));

/// java: public class PrehistoricEggAdder extends SimpleSlotTextAdder
#[derive(Clone)]
pub struct PrehistoricEggAdder {
    /// java: extends SimpleSlotTextAdder
    pub base: SimpleSlotTextAdderBase,
}

impl std::ops::Deref for PrehistoricEggAdder {
    type Target = SimpleSlotTextAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PrehistoricEggAdder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PrehistoricEggAdder {
    /// java: public PrehistoricEggAdder()
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

impl SimpleSlotTextAdder for PrehistoricEggAdder {
    fn simple_slot_text_adder_base(&self) -> &SimpleSlotTextAdderBase {
        &self.base
    }
    fn simple_slot_text_adder_base_mut(&mut self) -> &mut SimpleSlotTextAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for PrehistoricEggAdder {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for PrehistoricEggAdder {
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

impl SlotTextAdder for PrehistoricEggAdder {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId) (dispatch: the body for this class)
    fn get_text(&self) -> fn(&compat::Shared<dyn SlotTextAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32) -> Vec<SlotText> {
        PrehistoricEggAdder::get_text_dyn
    }

    /// java: @Override public @Nullable ConfigInformation getConfigInformation() (dispatch: the body for this class)
    fn get_config_information(&self) -> fn(&compat::Shared<dyn SlotTextAdder>) -> Option<ConfigInformation> {
        <dyn SimpleSlotTextAdder>::get_config_information_from::<Self>
    }

}

impl PrehistoricEggAdder {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId)
    pub fn get_text_impl(__this: &compat::Shared<Self>, mut slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot_id: i32) -> Vec<SlotText> {
        if !(stack.get_skyblock_id() == "PREHISTORIC_EGG") {
            return vec![];
        }
        let __nbt = ItemUtils::custom_data_component(stack);
        let nbt: &Tag = ItemUtils::custom_data_tag(&__nbt);
        if !(nbt.contains("blocks_walked")) {
            return vec![];
        }
        let mut walked: i32 = nbt.get_int_or("blocks_walked", 0);
        let mut walked_str: String;
        if walked < 1000 {
            walked_str = walked.to_string();
        } else if walked < 10000 {
            walked_str = compat::string_format("%.1fk", &[((walked as f32) / 1000.0f32).into()]);
        } else {
            walked_str = format!("{}k", walked.wrapping_div(1000));
        }
        return SlotText::bottom_left_list(&component::literal(&walked_str).with_color(slot_text::CREAM));
    }

    /// PrehistoricEggAdder.getText as the dispatch of SlotTextAdder.getText
    pub fn get_text_dyn(__this: &compat::Shared<dyn SlotTextAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32) -> Vec<SlotText> {
        Self::get_text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}
