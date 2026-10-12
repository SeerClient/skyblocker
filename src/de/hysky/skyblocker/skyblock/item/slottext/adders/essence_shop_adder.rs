//! Ported from de.hysky.skyblocker.skyblock.item.slottext.adders.EssenceShopAdder by fabric2seer.

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
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::roman_numerals::RomanNumerals;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern ESSENCELEVEL = Pattern.compile("^[\\w ]+ (?<level>[IVXLCDM]+)$");
pub static ESSENCELEVEL: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^[\\w ]+ (?<level>[IVXLCDM]+)$"));
/// java: private static final Pattern UNLOCKED = Pattern.compile("UNLOCKED");
pub static UNLOCKED: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("UNLOCKED"));
/// java: private static final Pattern ESSENCE = Pattern.compile("Your \\w+ Essence: (?<essence>[\\d,]+)");
pub static ESSENCE: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Your \\w+ Essence: (?<essence>[\\d,]+)"));
/// java: private static final ConfigInformation CONFIG_INFORMATION = new ConfigInformation( "essence_shop", "skyblocker.config.uiAndVisuals.slotText.essenceShop", "skyblocker.config.uiAndVisuals.slotText.essen
pub static CONFIG_INFORMATION: compat::MainThread<ConfigInformation> = compat::MainThread::new(|| ConfigInformation::new3("essence_shop".to_string(), "skyblocker.config.uiAndVisuals.slotText.essenceShop".to_string(), vec!["skyblocker.config.uiAndVisuals.slotText.essenceShop.@Tooltip".to_string()]));

/// java: public class EssenceShopAdder extends SimpleSlotTextAdder
#[derive(Clone)]
pub struct EssenceShopAdder {
    /// java: extends SimpleSlotTextAdder
    pub base: SimpleSlotTextAdderBase,
}

impl std::ops::Deref for EssenceShopAdder {
    type Target = SimpleSlotTextAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for EssenceShopAdder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EssenceShopAdder {
    /// java: public EssenceShopAdder()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleSlotTextAdderBase::new4_in(".*Essence Shop".to_string(), Some(CONFIG_INFORMATION.get_mut().clone()), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleSlotTextAdder>), __h)
        });
        __this
    }

}

impl SimpleSlotTextAdder for EssenceShopAdder {
    fn simple_slot_text_adder_base(&self) -> &SimpleSlotTextAdderBase {
        &self.base
    }
    fn simple_slot_text_adder_base_mut(&mut self) -> &mut SimpleSlotTextAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for EssenceShopAdder {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for EssenceShopAdder {
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

impl SlotTextAdder for EssenceShopAdder {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId) (dispatch: the body for this class)
    fn get_text(&self) -> fn(&compat::Shared<dyn SlotTextAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32) -> Vec<SlotText> {
        EssenceShopAdder::get_text_dyn
    }

    /// java: @Override public @Nullable ConfigInformation getConfigInformation() (dispatch: the body for this class)
    fn get_config_information(&self) -> fn(&compat::Shared<dyn SlotTextAdder>) -> Option<ConfigInformation> {
        <dyn SimpleSlotTextAdder>::get_config_information_from::<Self>
    }

}

impl EssenceShopAdder {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId)
    pub fn get_text_impl(__this: &compat::Shared<Self>, mut slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot_id: i32) -> Vec<SlotText> {
        if slot_id > 53 {
            return vec![];
        }
        let mut essence_level_matcher: compat::regex::Matcher = ESSENCELEVEL.matcher(&compat::component_of(stack.get_hover_name()).get_string());
        let mut essence_amount_matcher: Option<compat::regex::Matcher> = ItemUtils::get_lore_line_if_match(stack, &ESSENCE);
        if essence_level_matcher.matches() {
            let mut level: i32 = RomanNumerals::roman_to_decimal(essence_level_matcher.group_named("level"));
            let mut unlocked_matcher: Option<compat::regex::Matcher> = ItemUtils::get_lore_line_if_match(stack, &UNLOCKED);
            if let Some(unlocked_matcher) = &unlocked_matcher {
            } else {
                level = level.wrapping_sub(1);
            }
            return SlotText::bottom_right_list(&component::literal(&level.to_string()).with_color(slot_text::CREAM));
        }
        let Some(mut essence_amount_matcher) = essence_amount_matcher.clone() else {
            return vec![];
        };
        let mut essence_amount: String = essence_amount_matcher.group_named("essence").unwrap().replace(",", "");
        if !(compat::regex::Pattern::matches("-?\\d+", &essence_amount)) {
            return vec![];
        }
        return SlotText::bottom_right_list(&(component::literal(&formatters::SHORT_FLOAT_NUMBERS.get_mut().format((essence_amount.parse::<i32>().expect("NumberFormatException")) as f64))).with_color(slot_text::CREAM));
    }

    /// EssenceShopAdder.getText as the dispatch of SlotTextAdder.getText
    pub fn get_text_dyn(__this: &compat::Shared<dyn SlotTextAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32) -> Vec<SlotText> {
        Self::get_text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}
