//! Ported from de.hysky.skyblocker.skyblock.hunting.AttributeLevelHelper by fabric2seer.

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
use crate::de::hysky::skyblocker::utils::roman_numerals::RomanNumerals;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final ConfigInformation CONFIG_INFORMATION = new ConfigInformation("attribute_levels", "skyblocker.config.uiAndVisuals.slotText.attributeLevels", "skyblocker.config.uiAndVisuals.slotTex
pub static CONFIG_INFORMATION: compat::MainThread<ConfigInformation> = compat::MainThread::new(|| ConfigInformation::new3("attribute_levels".to_string(), "skyblocker.config.uiAndVisuals.slotText.attributeLevels".to_string(), vec!["skyblocker.config.uiAndVisuals.slotText.attributeLevels.@Tooltip".to_string()]));
/// java: public static final AttributeLevelHelper INSTANCE = new AttributeLevelHelper();
pub static INSTANCE: compat::MainThread<compat::Shared<AttributeLevelHelper>> = compat::MainThread::new(|| AttributeLevelHelper::new());

/// java: public class AttributeLevelHelper extends SimpleSlotTextAdder
#[derive(Clone)]
pub struct AttributeLevelHelper {
    /// java: extends SimpleSlotTextAdder
    pub base: SimpleSlotTextAdderBase,
}

impl std::ops::Deref for AttributeLevelHelper {
    type Target = SimpleSlotTextAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AttributeLevelHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AttributeLevelHelper {
    /// java: private AttributeLevelHelper()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleSlotTextAdderBase::new4_in("^(?:\\(\\d+\\/\\d+\\) )?Attribute Menu$".to_string(), Some(CONFIG_INFORMATION.get_mut().clone()), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleSlotTextAdder>), __h)
        });
        __this
    }

}

impl SimpleSlotTextAdder for AttributeLevelHelper {
    fn simple_slot_text_adder_base(&self) -> &SimpleSlotTextAdderBase {
        &self.base
    }
    fn simple_slot_text_adder_base_mut(&mut self) -> &mut SimpleSlotTextAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for AttributeLevelHelper {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for AttributeLevelHelper {
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

impl SlotTextAdder for AttributeLevelHelper {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId) (dispatch: the body for this class)
    fn get_text(&self) -> fn(&compat::Shared<dyn SlotTextAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32) -> Vec<SlotText> {
        AttributeLevelHelper::get_text_dyn
    }

    /// java: @Override public @Nullable ConfigInformation getConfigInformation() (dispatch: the body for this class)
    fn get_config_information(&self) -> fn(&compat::Shared<dyn SlotTextAdder>) -> Option<ConfigInformation> {
        <dyn SimpleSlotTextAdder>::get_config_information_from::<Self>
    }

}

impl AttributeLevelHelper {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId)
    pub fn get_text_impl(__this: &compat::Shared<Self>, mut slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot_id: i32) -> Vec<SlotText> {
        if slot.is_none() || stack.is_empty() {
            return vec![];
        }
        if slot.as_ref().unwrap().index <= 9 || slot.as_ref().unwrap().index >= 44 {
            return vec![];
        }
        if stack.is_item(mcrust::net::minecraft::world::item::items::stained_glass_pane().black()) {
            return vec![];
        }
        let Some(mut custom_name) = stack.get_custom_name() else {
            return vec![];
        };
        let mut item_name: String = custom_name.get_string();
        //   java: String levelText = StringUtils.substringAfterLast(itemName, " ");
        // (commons-lang: the text after the last separator; "" when there is none or it ends the string)
        let mut level_text: String = match item_name.rfind(' ') {
            Some(__i) => item_name[__i + ' '.len_utf8()..].to_string(),
            None => String::new(),
        };
        if level_text.is_empty() || !(RomanNumerals::is_valid_roman_numeral(Some(level_text.clone()))) {
            return vec![];
        }
        let mut level: i32 = RomanNumerals::roman_to_decimal(Some(level_text.clone()));
        if level < 1 || level > 10 {
            return vec![];
        }
        return SlotText::bottom_right_list(&component::literal(&level.to_string()).with_color((if level == 10 { slot_text::GOLD } else { slot_text::CREAM })));
    }

    /// AttributeLevelHelper.getText as the dispatch of SlotTextAdder.getText
    pub fn get_text_dyn(__this: &compat::Shared<dyn SlotTextAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32) -> Vec<SlotText> {
        Self::get_text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}
