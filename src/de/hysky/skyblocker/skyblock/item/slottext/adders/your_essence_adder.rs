//! Ported from de.hysky.skyblocker.skyblock.item.slottext.adders.YourEssenceAdder by fabric2seer.

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
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern YOUR_ESSENCE = Pattern.compile("You currently own (?<essence>[\\d,]+)");
pub static YOUR_ESSENCE: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("You currently own (?<essence>[\\d,]+)"));
/// java: private static final Pattern ESSENCE_GUIDE = Pattern.compile("Your \\w+ Essence: (?<essence>[\\d,]+)");
pub static ESSENCE_GUIDE: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Your \\w+ Essence: (?<essence>[\\d,]+)"));
/// java: private static final ConfigInformation CONFIG_INFORMATION = new ConfigInformation( "your_essence", "skyblocker.config.uiAndVisuals.slotText.yourEssence");
pub static CONFIG_INFORMATION: compat::MainThread<ConfigInformation> = compat::MainThread::new(|| ConfigInformation::new2("your_essence".to_string(), "skyblocker.config.uiAndVisuals.slotText.yourEssence".to_string()));

/// java: public class YourEssenceAdder extends SimpleSlotTextAdder
#[derive(Clone)]
pub struct YourEssenceAdder {
    /// java: extends SimpleSlotTextAdder
    pub base: SimpleSlotTextAdderBase,
}

impl std::ops::Deref for YourEssenceAdder {
    type Target = SimpleSlotTextAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for YourEssenceAdder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl YourEssenceAdder {
    /// java: public YourEssenceAdder()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleSlotTextAdderBase::new4_in("^(?:Your Essence|Essence Guide)".to_string(), Some(CONFIG_INFORMATION.get_mut().clone()), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleSlotTextAdder>), __h)
        });
        __this
    }

    /// java: private Optional<Matcher> essenceAmountMatcher(List<String> lore)
    pub fn essence_amount_matcher(__this: &compat::Shared<Self>, lore: &Vec<String>) -> Option<compat::regex::Matcher> {
        if lore.is_empty() {
            return None;
        }
        let mut essence_amount_matcher: compat::regex::Matcher = YOUR_ESSENCE.matcher(&lore[0]);
        if essence_amount_matcher.find() {
            return Some(essence_amount_matcher.clone());
        }
        if (lore.len() as i32) < 3 {
            return None;
        }
        essence_amount_matcher = ESSENCE_GUIDE.matcher(&lore[((lore.len() as i32).wrapping_sub(3)) as usize]);
        if essence_amount_matcher.find() {
            return Some(essence_amount_matcher.clone());
        }
        return None;
    }

}

impl SimpleSlotTextAdder for YourEssenceAdder {
    fn simple_slot_text_adder_base(&self) -> &SimpleSlotTextAdderBase {
        &self.base
    }
    fn simple_slot_text_adder_base_mut(&mut self) -> &mut SimpleSlotTextAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for YourEssenceAdder {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for YourEssenceAdder {
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

impl SlotTextAdder for YourEssenceAdder {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId) (dispatch: the body for this class)
    fn get_text(&self) -> fn(&compat::Shared<dyn SlotTextAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32) -> Vec<SlotText> {
        YourEssenceAdder::get_text_dyn
    }

    /// java: @Override public @Nullable ConfigInformation getConfigInformation() (dispatch: the body for this class)
    fn get_config_information(&self) -> fn(&compat::Shared<dyn SlotTextAdder>) -> Option<ConfigInformation> {
        <dyn SimpleSlotTextAdder>::get_config_information_from::<Self>
    }

}

impl YourEssenceAdder {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId)
    pub fn get_text_impl(__this: &compat::Shared<Self>, mut slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot_id: i32) -> Vec<SlotText> {
        if compat::component_of(stack.get_hover_name()).get_string().contains("Essence") {
            return YourEssenceAdder::essence_amount_matcher(__this, &stack.skyblocker_get_lore_strings()).map(|essence_amount_matcher| {
                let mut essence_amount: String = essence_amount_matcher.group_named("essence").unwrap().replace(",", "");
                if !(compat::regex::Pattern::matches("-?\\d+", &essence_amount)) {
                    return vec![];
                }
                return SlotText::bottom_right_list(&(component::literal(&formatters::SHORT_FLOAT_NUMBERS.get_mut().format((essence_amount.parse::<i32>().expect("NumberFormatException")) as f64))).with_color(slot_text::CREAM));
            }).unwrap_or(vec![]);
        }
        return vec![];
    }

    /// YourEssenceAdder.getText as the dispatch of SlotTextAdder.getText
    pub fn get_text_dyn(__this: &compat::Shared<dyn SlotTextAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32) -> Vec<SlotText> {
        Self::get_text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}
