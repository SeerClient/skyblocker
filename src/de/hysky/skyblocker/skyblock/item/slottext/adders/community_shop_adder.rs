//! Ported from de.hysky.skyblocker.skyblock.item.slottext.adders.CommunityShopAdder by fabric2seer.

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
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final byte CATEGORIES_START = 10;
pub const CATEGORIES_START: i8 = (10) as i8;
/// java: private static final byte CATEGORIES_END = 14;
pub const CATEGORIES_END: i8 = (14) as i8;
/// java: private static final ConfigInformation CONFIG_INFORMATION = new ConfigInformation( "community_shop", "skyblocker.config.uiAndVisuals.slotText.communityShop", "skyblocker.config.uiAndVisuals.slotText.c
pub static CONFIG_INFORMATION: compat::MainThread<ConfigInformation> = compat::MainThread::new(|| ConfigInformation::new3("community_shop".to_string(), "skyblocker.config.uiAndVisuals.slotText.communityShop".to_string(), vec!["skyblocker.config.uiAndVisuals.slotText.communityShop.@Tooltip".to_string()]));
/// java: private static byte currentScreen = -1;
pub static CURRENT_SCREEN: LazyLock<Mutex<i8>> = LazyLock::new(|| Mutex::new((-1) as i8));

/// java: public class CommunityShopAdder extends SimpleSlotTextAdder
#[derive(Clone)]
pub struct CommunityShopAdder {
    /// java: extends SimpleSlotTextAdder
    pub base: SimpleSlotTextAdderBase,
}

impl std::ops::Deref for CommunityShopAdder {
    type Target = SimpleSlotTextAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CommunityShopAdder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CommunityShopAdder {
    /// java: public CommunityShopAdder()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleSlotTextAdderBase::new4_in("^Community Shop".to_string(), Some(CONFIG_INFORMATION.get_mut().clone()), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleSlotTextAdder>), __h)
        });
        __this
    }

    /// java: private static List<SlotText> getTextForUpgradesScreen(ItemStack stack, int slotId)
    pub fn get_text_for_upgrades_screen(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot_id: i32) -> Vec<SlotText> {
        if slot_id >= 18 && slot_id <= 44 {
            let mut name: String = compat::component_of(stack.get_hover_name()).get_string();
            let mut last_index: i32 = compat::last_index_of(&name, ' ');
            let mut roman: String = compat::substring(&name, last_index.wrapping_add(1));
            if !(RomanNumerals::is_valid_roman_numeral(Some(roman.clone()))) {
                return vec![];
            }
            let mut lore: Vec<String> = stack.skyblocker_get_lore_strings();
            if lore.is_empty() {
                return vec![];
            }
            let mut last_line: String = lore[lore.len() - 1].clone();
            return SlotText::bottom_left_list(&match last_line.as_str() {
                "Maxed out!" => component::literal("Max").with_color(slot_text::LIGHT_ORANGE),
                "Currently upgrading!" | "Click to instantly upgrade!" => (component::literal("⏰")).with_color(slot_text::LIGHT_YELLOW).with_style_format(ChatFormatting::Bold),
                "Click to claim!" => (component::literal("✅")).with_color(0xA6E3A1).with_style_format(ChatFormatting::Bold),
                _ => component::literal(&RomanNumerals::roman_to_decimal(Some(roman.clone())).to_string()).with_color(slot_text::LIGHT_PURPLE),
            });
        }
        return vec![];
    }

}

impl SimpleSlotTextAdder for CommunityShopAdder {
    fn simple_slot_text_adder_base(&self) -> &SimpleSlotTextAdderBase {
        &self.base
    }
    fn simple_slot_text_adder_base_mut(&mut self) -> &mut SimpleSlotTextAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for CommunityShopAdder {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for CommunityShopAdder {
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

impl SlotTextAdder for CommunityShopAdder {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId) (dispatch: the body for this class)
    fn get_text(&self) -> fn(&compat::Shared<dyn SlotTextAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32) -> Vec<SlotText> {
        CommunityShopAdder::get_text_dyn
    }

    /// java: @Override public @Nullable ConfigInformation getConfigInformation() (dispatch: the body for this class)
    fn get_config_information(&self) -> fn(&compat::Shared<dyn SlotTextAdder>) -> Option<ConfigInformation> {
        <dyn SimpleSlotTextAdder>::get_config_information_from::<Self>
    }

}

impl CommunityShopAdder {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId)
    pub fn get_text_impl(__this: &compat::Shared<Self>, mut slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot_id: i32) -> Vec<SlotText> {
        if slot_id >= (CATEGORIES_START as i32) && slot_id <= (CATEGORIES_END as i32) && stack.is_item(mcrust::net::minecraft::world::item::items::stained_glass_pane().lime()) {
            compat::set_state(&CURRENT_SCREEN, ((slot_id.wrapping_sub(CATEGORIES_START as i32)) as i8));
            return vec![];
        }
        return match compat::get_state(&CURRENT_SCREEN) {
            1 => CommunityShopAdder::get_text_for_upgrades_screen(stack, slot_id),
            _ => vec![],
        };
    }

    /// CommunityShopAdder.getText as the dispatch of SlotTextAdder.getText
    pub fn get_text_dyn(__this: &compat::Shared<dyn SlotTextAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32) -> Vec<SlotText> {
        Self::get_text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}
