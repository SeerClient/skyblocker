//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.adders.ObtainedDateTooltip by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type::TooltipInfoType;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::simple_tooltip_adder::SimpleTooltipAdder;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::simple_tooltip_adder::SimpleTooltipAdderBase;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::tooltip_adder::TooltipAdder;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::render::text::grid_component::GridComponent;
use mcrust::java::time::format::date_time_formatter::DateTimeFormatter;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::nbt::tag::Tag;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::slot::Slot;

// TODO(fabric2seer): `.withZone(..)` on a value of unknown type
//   java: private static final DateTimeFormatter OBTAINED_DATE_FORMATTER = DateTimeFormatter.ofPattern("MMMM d, yyyy").withZone(ZoneId.systemDefault()).localizedBy(Locale.ENGLISH);
/// java: private static final DateTimeFormatter OBTAINED_DATE_FORMATTER = DateTimeFormatter.ofPattern("MMMM d, yyyy").withZone(ZoneId.systemDefault()).localizedBy(Locale.ENGLISH);
pub static OBTAINED_DATE_FORMATTER: compat::MainThread<DateTimeFormatter> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `OBTAINED_DATE_FORMATTER` - see the TODO above"));
// TODO(fabric2seer): `.withZone(..)` on a value of unknown type
//   java: private static final DateTimeFormatter OLD_OBTAINED_DATE_FORMAT = DateTimeFormatter.ofPattern("M/d/yy h:m a").withZone(ZoneId.of("UTC")).localizedBy(Locale.ENGLISH);
/// java: private static final DateTimeFormatter OLD_OBTAINED_DATE_FORMAT = DateTimeFormatter.ofPattern("M/d/yy h:m a").withZone(ZoneId.of("UTC")).localizedBy(Locale.ENGLISH);
pub static OLD_OBTAINED_DATE_FORMAT: compat::MainThread<DateTimeFormatter> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `OLD_OBTAINED_DATE_FORMAT` - see the TODO above"));

/// java: public class ObtainedDateTooltip extends SimpleTooltipAdder
#[derive(Clone)]
pub struct ObtainedDateTooltip {
    /// java: extends SimpleTooltipAdder
    pub base: SimpleTooltipAdderBase,
}

impl std::ops::Deref for ObtainedDateTooltip {
    type Target = SimpleTooltipAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ObtainedDateTooltip {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ObtainedDateTooltip {
    /// java: public ObtainedDateTooltip(int priority)
    pub fn new(mut priority: i32) -> compat::Shared<Self> {
        let (_, __this) = SimpleTooltipAdderBase::new_1_in(priority, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleTooltipAdder>), __h)
        });
        __this
    }

    /// java: private static TemporalAccessor getTimestampInternal(ItemStack stack)
    pub fn get_timestamp_internal(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> Option<crate::unresolved::TemporalAccessor> {
        let mut custom_data: Tag = ItemUtils::get_custom_data(stack);
        // TODO(fabric2seer): not translated (expected ')' at '(' (offset 2118))
        //   java: if (customData != null && customData.get("timestamp") instanceof LongTag(long value)) {
        //   java: 			return Instant.ofEpochMilli(value);
        //   java: 		}
        // TODO(fabric2seer): not translated (expected ')' at '(' (offset 2254))
        //   java: if (customData != null && customData.get("timestamp") instanceof StringTag(String value)) {
        //   java: 			try {
        //   java: 				return OLD_OBTAINED_DATE_FORMAT.parse(value);
        //   java: 			} catch (DateTimeParseException e) {
        //   java: 				LOGGER.error("[Skyblocker ObtainedDateTooltip] Failed to parse date: {}", value, e);
        //   java: 			}
        //   java: 		}
        return None;
    }

    /// java: public static String getTimestamp(ItemStack stack)
    pub fn get_timestamp(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> String {
        let mut accessor: Option<crate::unresolved::TemporalAccessor> = ObtainedDateTooltip::get_timestamp_internal(stack);
        return (if accessor.is_some() { OBTAINED_DATE_FORMATTER.get_mut().format(accessor.clone()) } else { "".to_string() });
    }

    /// java: public static long getLongTimestamp(ItemStack stack)
    pub fn get_long_timestamp(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> i64 {
        let mut accessor: Option<crate::unresolved::TemporalAccessor> = ObtainedDateTooltip::get_timestamp_internal(stack);
        // TODO(fabric2seer): `.toEpochMilli(..)` on a value of unknown type
        //   java: return accessor != null ? Instant.from(accessor).toEpochMilli() : 0;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl SimpleTooltipAdder for ObtainedDateTooltip {
    fn simple_tooltip_adder_base(&self) -> &SimpleTooltipAdderBase {
        &self.base
    }
    fn simple_tooltip_adder_base_mut(&mut self) -> &mut SimpleTooltipAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for ObtainedDateTooltip {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for ObtainedDateTooltip {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        ObtainedDateTooltip::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl TooltipAdder for ObtainedDateTooltip {
    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines) (dispatch: the body for this class)
    fn add_to_tooltip(&self) -> fn(&compat::Shared<dyn TooltipAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &mut Vec<MutableComponent>) {
        ObtainedDateTooltip::add_to_tooltip_dyn
    }

    /// java: @Override public int getPriority() (dispatch: the body for this class)
    fn get_priority(&self) -> fn(&compat::Shared<dyn TooltipAdder>) -> i32 {
        <dyn SimpleTooltipAdder>::get_priority_from::<Self>
    }

}

impl ObtainedDateTooltip {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return ({ let __r = tooltip_info_type::OBTAINED.get_mut(); let __f = compat::dispatch(__r, |__t| __t.is_tooltip_enabled()); __f(__r) });
    }

    /// ObtainedDateTooltip.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines)
    pub fn add_to_tooltip_impl(__this: &compat::Shared<Self>, mut focused_slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, lines: &mut Vec<MutableComponent>) {
        let mut timestamp: String = ObtainedDateTooltip::get_timestamp(stack);
        if !timestamp.is_empty() {
            lines.push(GridComponent::of_2(&compat::jstring(&component::literal("Obtained:").with_style_format(ChatFormatting::LightPurple)), &vec![component::literal(&timestamp).with_style_format(ChatFormatting::Red)]));
        }
    }

    /// ObtainedDateTooltip.addToTooltip as the dispatch of TooltipAdder.addToTooltip
    pub fn add_to_tooltip_dyn(__this: &compat::Shared<dyn TooltipAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: &mut Vec<MutableComponent>) {
        Self::add_to_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}
