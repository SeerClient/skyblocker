//! Ported from de.hysky.skyblocker.skyblock.item.slottext.adders.CatacombsLevelAdder by fabric2seer.

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

/// java: private static final SlotTextAdder.ConfigInformation CONFIG_INFORMATION = new SlotTextAdder.ConfigInformation( "catacombs_level", "skyblocker.config.uiAndVisuals.slotText.catacombsLevel");
pub static CONFIG_INFORMATION: compat::MainThread<ConfigInformation> = compat::MainThread::new(|| ConfigInformation::new2("catacombs_level".to_string(), "skyblocker.config.uiAndVisuals.slotText.catacombsLevel".to_string()));
/// java: private static final Pattern LEVEL_PATTERN = Pattern.compile(".*?(?:(?: (?<arabic>\\d+)| (?<roman>[IVXLC]+))(?: ✯)?)?");
pub static DUNGEONEERING_LEVEL_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(".*?(?:(?: (?<arabic>\\d+)| (?<roman>[IVXLC]+))(?: ✯)?)?"));

/// java: public class CatacombsLevelAdder
#[derive(Clone)]
pub struct CatacombsLevelAdder {}

impl CatacombsLevelAdder {
    /// java: private CatacombsLevelAdder()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static String getBracketedLevelFromName(ItemStack itemStack)
    pub fn get_bracketed_level_from_name(item_stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> Option<String> {
        let mut name: String = compat::component_of(item_stack.get_hover_name()).get_string();
        if !(name.starts_with("[Lvl ")) {
            return None;
        }
        let mut index: i32 = compat::index_of(&name, ']');
        if index == -1 {
            return None;
        }
        return Some(compat::substring2(&name, 5, index));
    }

}

/// java: public static class Dungeoneering extends SimpleSlotTextAdder
#[derive(Clone)]
pub struct Dungeoneering {
    /// java: extends SimpleSlotTextAdder
    pub base: SimpleSlotTextAdderBase,
}

impl std::ops::Deref for Dungeoneering {
    type Target = SimpleSlotTextAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for Dungeoneering {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Dungeoneering {
    /// java: public Dungeoneering()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleSlotTextAdderBase::new4_in("^Dungeoneering".to_string(), Some(CONFIG_INFORMATION.get_mut().clone()), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleSlotTextAdder>), __h)
        });
        __this
    }

}

impl SimpleSlotTextAdder for Dungeoneering {
    fn simple_slot_text_adder_base(&self) -> &SimpleSlotTextAdderBase {
        &self.base
    }
    fn simple_slot_text_adder_base_mut(&mut self) -> &mut SimpleSlotTextAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for Dungeoneering {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for Dungeoneering {
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

impl SlotTextAdder for Dungeoneering {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId) (dispatch: the body for this class)
    fn get_text(&self) -> fn(&compat::Shared<dyn SlotTextAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32) -> Vec<SlotText> {
        Dungeoneering::get_text_dyn
    }

    /// java: @Override public @Nullable ConfigInformation getConfigInformation() (dispatch: the body for this class)
    fn get_config_information(&self) -> fn(&compat::Shared<dyn SlotTextAdder>) -> Option<ConfigInformation> {
        <dyn SimpleSlotTextAdder>::get_config_information_from::<Self>
    }

}

impl Dungeoneering {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId)
    pub fn get_text_impl(__this: &compat::Shared<Self>, mut slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot_id: i32) -> Vec<SlotText> {
        match slot_id {
            12 | 29 | 30 | 31 | 32 | 33 => {
                let mut matcher: compat::regex::Matcher = DUNGEONEERING_LEVEL_PATTERN.matcher(&compat::component_of(stack.get_hover_name()).get_string());
                if !matcher.matches() {
                    return vec![];
                }
                let mut arabic: Option<String> = matcher.group_named("arabic");
                let mut roman: Option<String> = matcher.group_named("roman");
                if arabic.is_none() && roman.is_none() {
                    return SlotText::bottom_left_list(&component::literal("0").with_color(slot_text::CREAM));
                }
                let mut level: String;
                if let Some(arabic) = &arabic {
                    // TODO(fabric2seer): the condition: static call NumberUtils.isDigits(..)
                    //   java: !NumberUtils.isDigits(arabic)
                    if !(!arabic.is_empty() && arabic.chars().all(char::is_numeric)) {
                        return vec![];
                    }
                    level = arabic.to_string();
                } else {
                    if !(RomanNumerals::is_valid_roman_numeral(roman.clone())) {
                        return vec![];
                    }
                    level = RomanNumerals::roman_to_decimal(roman.clone()).to_string();
                }
                return SlotText::bottom_left_list(&component::literal(&level).with_color(slot_text::CREAM));
            }
            _ => {
                return vec![];
            }
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// Dungeoneering.getText as the dispatch of SlotTextAdder.getText
    pub fn get_text_dyn(__this: &compat::Shared<dyn SlotTextAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32) -> Vec<SlotText> {
        Self::get_text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}

/// java: public static class DungeonClasses extends SimpleSlotTextAdder
#[derive(Clone)]
pub struct DungeonClasses {
    /// java: extends SimpleSlotTextAdder
    pub base: SimpleSlotTextAdderBase,
}

impl std::ops::Deref for DungeonClasses {
    type Target = SimpleSlotTextAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DungeonClasses {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DungeonClasses {
    /// java: public DungeonClasses()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleSlotTextAdderBase::new4_in("^Dungeon Classes".to_string(), Some(CONFIG_INFORMATION.get_mut().clone()), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleSlotTextAdder>), __h)
        });
        __this
    }

}

impl SimpleSlotTextAdder for DungeonClasses {
    fn simple_slot_text_adder_base(&self) -> &SimpleSlotTextAdderBase {
        &self.base
    }
    fn simple_slot_text_adder_base_mut(&mut self) -> &mut SimpleSlotTextAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for DungeonClasses {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for DungeonClasses {
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

impl SlotTextAdder for DungeonClasses {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId) (dispatch: the body for this class)
    fn get_text(&self) -> fn(&compat::Shared<dyn SlotTextAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32) -> Vec<SlotText> {
        DungeonClasses::get_text_dyn
    }

    /// java: @Override public @Nullable ConfigInformation getConfigInformation() (dispatch: the body for this class)
    fn get_config_information(&self) -> fn(&compat::Shared<dyn SlotTextAdder>) -> Option<ConfigInformation> {
        <dyn SimpleSlotTextAdder>::get_config_information_from::<Self>
    }

}

impl DungeonClasses {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId)
    pub fn get_text_impl(__this: &compat::Shared<Self>, mut slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot_id: i32) -> Vec<SlotText> {
        match slot_id {
            11 | 12 | 13 | 14 | 15 => {
                let mut level: String = CatacombsLevelAdder::get_bracketed_level_from_name(stack).unwrap();
                // TODO(fabric2seer): the condition: static call NumberUtils.isDigits(..)
                //   java: !NumberUtils.isDigits(level)
                if !(!level.is_empty() && level.chars().all(char::is_numeric)) {
                    return vec![];
                }
                return SlotText::bottom_left_list(&component::literal(&level).with_color(slot_text::CREAM));
            }
            _ => {
                return vec![];
            }
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// DungeonClasses.getText as the dispatch of SlotTextAdder.getText
    pub fn get_text_dyn(__this: &compat::Shared<dyn SlotTextAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32) -> Vec<SlotText> {
        Self::get_text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}

/// java: public static class ReadyUp extends SimpleSlotTextAdder
#[derive(Clone)]
pub struct ReadyUp {
    /// java: extends SimpleSlotTextAdder
    pub base: SimpleSlotTextAdderBase,
}

impl std::ops::Deref for ReadyUp {
    type Target = SimpleSlotTextAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ReadyUp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ReadyUp {
    /// java: public ReadyUp()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleSlotTextAdderBase::new4_in("^Ready Up".to_string(), Some(CONFIG_INFORMATION.get_mut().clone()), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleSlotTextAdder>), __h)
        });
        __this
    }

}

impl SimpleSlotTextAdder for ReadyUp {
    fn simple_slot_text_adder_base(&self) -> &SimpleSlotTextAdderBase {
        &self.base
    }
    fn simple_slot_text_adder_base_mut(&mut self) -> &mut SimpleSlotTextAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for ReadyUp {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for ReadyUp {
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

impl SlotTextAdder for ReadyUp {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId) (dispatch: the body for this class)
    fn get_text(&self) -> fn(&compat::Shared<dyn SlotTextAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32) -> Vec<SlotText> {
        ReadyUp::get_text_dyn
    }

    /// java: @Override public @Nullable ConfigInformation getConfigInformation() (dispatch: the body for this class)
    fn get_config_information(&self) -> fn(&compat::Shared<dyn SlotTextAdder>) -> Option<ConfigInformation> {
        <dyn SimpleSlotTextAdder>::get_config_information_from::<Self>
    }

}

impl ReadyUp {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId)
    pub fn get_text_impl(__this: &compat::Shared<Self>, mut slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot_id: i32) -> Vec<SlotText> {
        match slot_id {
            29 | 30 | 31 | 32 | 33 => {
                let mut level: String = CatacombsLevelAdder::get_bracketed_level_from_name(stack).unwrap();
                // TODO(fabric2seer): the condition: static call NumberUtils.isDigits(..)
                //   java: !NumberUtils.isDigits(level)
                if !(!level.is_empty() && level.chars().all(char::is_numeric)) {
                    return vec![];
                }
                return SlotText::bottom_left_list(&component::literal(&level).with_color(slot_text::CREAM));
            }
            _ => {
                return vec![];
            }
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// ReadyUp.getText as the dispatch of SlotTextAdder.getText
    pub fn get_text_dyn(__this: &compat::Shared<dyn SlotTextAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32) -> Vec<SlotText> {
        Self::get_text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}
