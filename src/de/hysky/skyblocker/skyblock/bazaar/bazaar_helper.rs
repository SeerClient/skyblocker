//! Ported from de.hysky.skyblocker.skyblock.bazaar.BazaarHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::bazaar::bazaar_order_tracker;
use crate::de::hysky::skyblocker::skyblock::bazaar::bazaar_order_tracker::BazaarOrderTracker;
use crate::de::hysky::skyblocker::skyblock::item::slottext::simple_slot_text_adder::SimpleSlotTextAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::simple_slot_text_adder::SimpleSlotTextAdderBase;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text::SlotText;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::slot_text_adder::ConfigInformation;
use crate::de::hysky::skyblocker::utils::container::slot_text_adder::SlotTextAdder;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::world::container::Container as _;

/// java: private static final Pattern FILLED_PATTERN = Pattern.compile("Filled: \\S+ \\(?([\\d.]+)%\\)?!?");
pub static FILLED_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Filled: \\S+ \\(?([\\d.]+)%\\)?!?"));
/// java: private static final int RED = 0xE60B1E;
pub const RED: i32 = 0xE60B1E;
/// java: private static final int YELLOW = 0xE6BA0B;
pub const YELLOW: i32 = 0xE6BA0B;
/// java: private static final int GREEN = 0x1EE60B;
pub const GREEN: i32 = 0x1EE60B;

/// java: public class BazaarHelper extends SimpleSlotTextAdder
#[derive(Clone)]
pub struct BazaarHelper {
    /// java: extends SimpleSlotTextAdder
    pub base: SimpleSlotTextAdderBase,
}

impl std::ops::Deref for BazaarHelper {
    type Target = SimpleSlotTextAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for BazaarHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl BazaarHelper {
    /// java: public BazaarHelper()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleSlotTextAdderBase::new_in("(?:Co-op|Your) Bazaar Orders".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleSlotTextAdder>), __h)
        });
        __this
    }

    /// java: public static MutableComponent getExpiredIcon()
    pub fn get_expired_icon() -> MutableComponent {
        return (component::literal("⏰")).with_color(RED).with_style_format(ChatFormatting::Bold);
    }

    /// java: public static MutableComponent getExpiringIcon()
    pub fn get_expiring_icon() -> MutableComponent {
        return (component::literal("⏰")).with_color(YELLOW).with_style_format(ChatFormatting::Bold);
    }

    /// java: public static MutableComponent getFilledIcon(int filled)
    pub fn get_filled_icon(mut filled: i32) -> MutableComponent {
        if filled < 100 {
            return (component::literal("%")).with_color(YELLOW).with_style_format(ChatFormatting::Bold);
        }
        return (component::literal("✅")).with_color(GREEN).with_style_format(ChatFormatting::Bold);
    }

}

impl SimpleSlotTextAdder for BazaarHelper {
    fn simple_slot_text_adder_base(&self) -> &SimpleSlotTextAdderBase {
        &self.base
    }
    fn simple_slot_text_adder_base_mut(&mut self) -> &mut SimpleSlotTextAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for BazaarHelper {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

    /// java: @Override public boolean test(String title) (dispatch: the body for this class)
    fn test_string(&self) -> fn(&compat::Shared<dyn RegexContainerMatcher>, &str) -> bool {
        BazaarHelper::test_string_dyn
    }

}

impl ContainerMatcher for BazaarHelper {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        BazaarHelper::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl SlotTextAdder for BazaarHelper {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId) (dispatch: the body for this class)
    fn get_text(&self) -> fn(&compat::Shared<dyn SlotTextAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32) -> Vec<SlotText> {
        BazaarHelper::get_text_dyn
    }

    /// java: @Override public @Nullable ConfigInformation getConfigInformation() (dispatch: the body for this class)
    fn get_config_information(&self) -> fn(&compat::Shared<dyn SlotTextAdder>) -> Option<ConfigInformation> {
        <dyn SimpleSlotTextAdder>::get_config_information_from::<Self>
    }

}

impl BazaarHelper {
    /// java: @Override public boolean test(String title)
    pub fn test_string_impl(__this: &compat::Shared<Self>, title: &str) -> bool {
        if <dyn RegexContainerMatcher>::test_string_body(&compat::view(__this, |a| a as compat::Shared<dyn RegexContainerMatcher>), title) {
            BazaarOrderTracker::clear_orders(bazaar_order_tracker::INSTANCE.get_mut());
            return true;
        }
        return false;
    }

    /// BazaarHelper.test as the dispatch of RegexContainerMatcher.test
    pub fn test_string_dyn(__this: &compat::Shared<dyn RegexContainerMatcher>, __p0: &str) -> bool {
        Self::test_string_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return SkyblockerConfigManager::get().helpers.bazaar.enable_bazaar_helper;
    }

    /// BazaarHelper.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId)
    pub fn get_text_impl(__this: &compat::Shared<Self>, mut slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot_id: i32) -> Vec<SlotText> {
        let Some(mut slot) = slot.clone() else {
            return vec![];
        };
        if slot_id < 10 {
            return vec![];
        }
        // java: slot.container.getContainerSize() - the open menu's container the slot names
        let __container_size: i32 = compat::with_open_menu(|_, containers| containers.get(slot.container).get_container_size()).unwrap_or(0);
        if slot_id > __container_size.wrapping_sub(11) {
            return vec![];
        }
        let mut column: i32 = slot_id.wrapping_rem(9);
        if column == 0 || column == 8 {
            return vec![];
        }
        // java: ItemStack item = slot.getItem();
        let mut item: mcrust::net::minecraft::world::item::item_stack::ItemStack = compat::slot_item(&slot);
        if item.is_empty() {
            return vec![];
        }
        BazaarOrderTracker::process_order(bazaar_order_tracker::INSTANCE.get_mut(), &item, slot_id);
        let mut matcher: Option<compat::regex::Matcher> = ItemUtils::get_lore_line_if_match(&item, &FILLED_PATTERN);
        if let Some(matcher) = &matcher {
            let mut lore: Vec<String> = item.skyblocker_get_lore_strings();
            if !lore.is_empty() && lore[lore.len() - 1] == "Click to claim!" {
                // java: int filled = NumberUtils.toInt(matcher.group(1));
                let mut filled: i32 = compat::number_utils_to_int(&matcher.group(1).unwrap_or_default(), 0);
                return SlotText::top_left_list(&BazaarHelper::get_filled_icon(filled));
            }
        }
        if ItemUtils::get_lore_line_if(&item, Arc::new(move |str| (str == "Expired!"))).is_some() {
            return SlotText::top_left_list(&BazaarHelper::get_expired_icon());
        } else if ItemUtils::get_lore_line_if(&item, Arc::new(move |str| str.starts_with("Expires in"))).is_some() {
            return SlotText::top_left_list(&BazaarHelper::get_expiring_icon());
        }
        return vec![];
    }

    /// BazaarHelper.getText as the dispatch of SlotTextAdder.getText
    pub fn get_text_dyn(__this: &compat::Shared<dyn SlotTextAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32) -> Vec<SlotText> {
        Self::get_text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}
