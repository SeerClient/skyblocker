//! Ported from de.hysky.skyblocker.skyblock.item.slottext.adders.ChipLevelAdder by fabric2seer.

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
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;

/// java: private static final Pattern LEVEL = Pattern.compile("Level (?<level>\\d+)/?(?<max>\\d+)?");
pub static LEVEL: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Level (?<level>\\d+)/?(?<max>\\d+)?"));
/// java: private static final ConfigInformation CONFIG_INFORMATION = new ConfigInformation( "chip_level", "skyblocker.config.uiAndVisuals.slotText.chipLevel", "skyblocker.config.uiAndVisuals.slotText.chipLevel
pub static CONFIG_INFORMATION: compat::MainThread<ConfigInformation> = compat::MainThread::new(|| ConfigInformation::new3("chip_level".to_string(), "skyblocker.config.uiAndVisuals.slotText.chipLevel".to_string(), vec!["skyblocker.config.uiAndVisuals.slotText.chipLevel.@tooltip".to_string()]));

/// java: public class ChipLevelAdder extends SimpleSlotTextAdder
#[derive(Clone)]
pub struct ChipLevelAdder {
    /// java: extends SimpleSlotTextAdder
    pub base: SimpleSlotTextAdderBase,
}

impl std::ops::Deref for ChipLevelAdder {
    type Target = SimpleSlotTextAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ChipLevelAdder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ChipLevelAdder {
    /// java: public ChipLevelAdder()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleSlotTextAdderBase::new4_in("^Manage Chips$".to_string(), Some(CONFIG_INFORMATION.get_mut().clone()), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleSlotTextAdder>), __h)
        });
        __this
    }

}

impl SimpleSlotTextAdder for ChipLevelAdder {
    fn simple_slot_text_adder_base(&self) -> &SimpleSlotTextAdderBase {
        &self.base
    }
    fn simple_slot_text_adder_base_mut(&mut self) -> &mut SimpleSlotTextAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for ChipLevelAdder {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for ChipLevelAdder {
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

impl SlotTextAdder for ChipLevelAdder {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId) (dispatch: the body for this class)
    fn get_text(&self) -> fn(&compat::Shared<dyn SlotTextAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32) -> Vec<SlotText> {
        ChipLevelAdder::get_text_dyn
    }

    /// java: @Override public @Nullable ConfigInformation getConfigInformation() (dispatch: the body for this class)
    fn get_config_information(&self) -> fn(&compat::Shared<dyn SlotTextAdder>) -> Option<ConfigInformation> {
        <dyn SimpleSlotTextAdder>::get_config_information_from::<Self>
    }

}

impl ChipLevelAdder {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId)
    pub fn get_text_impl(__this: &compat::Shared<Self>, mut slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot_id: i32) -> Vec<SlotText> {
        if slot_id < 18 || slot_id > 35 || stack.is_item(mcrust::net::minecraft::world::item::items::dye().gray()) {
            return vec![];
        }
        let mut lore: Vec<String> = stack.skyblocker_get_lore_strings();
        if lore.is_empty() {
            return vec![];
        }
        let mut level_line: String = lore[0].clone();
        let mut matcher: compat::regex::Matcher = LEVEL.matcher(&level_line);
        if !matcher.matches() {
            return vec![];
        }
        let mut level: String = matcher.group_named("level").unwrap();
        let mut max: String = matcher.group_named("max").unwrap();
        let mut color: i32 = slot_text::CREAM;
        if level == max {
            match max.as_str() {
                "10" => {
                    color = 0x1D8DE8;
                }
                "15" => {
                    color = 0xA40BE2;
                }
                "20" => {
                    color = slot_text::GOLD;
                }
                _ => {}
            }
        }
        return SlotText::bottom_right_list(&component::literal(&level).with_color(color));
    }

    /// ChipLevelAdder.getText as the dispatch of SlotTextAdder.getText
    pub fn get_text_dyn(__this: &compat::Shared<dyn SlotTextAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32) -> Vec<SlotText> {
        Self::get_text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}
