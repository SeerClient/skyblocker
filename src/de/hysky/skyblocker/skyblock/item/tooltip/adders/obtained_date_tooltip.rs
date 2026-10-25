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

/// java: private static final DateTimeFormatter OBTAINED_DATE_FORMATTER = DateTimeFormatter.ofPattern("MMMM d, yyyy").withZone(ZoneId.systemDefault()).localizedBy(Locale.ENGLISH);
/// The port's `DateTimeFormatter` is a pattern only (English names, no zone): `withZone(ZoneId.systemDefault())` is
/// applied where it formats (`getTimestamp`) - RustSrc gap, GAPS_batchB.md.
pub static OBTAINED_DATE_FORMATTER: compat::MainThread<DateTimeFormatter> = compat::MainThread::new(|| DateTimeFormatter::of_pattern("MMMM d, yyyy"));
/// java: private static final DateTimeFormatter OLD_OBTAINED_DATE_FORMAT = DateTimeFormatter.ofPattern("M/d/yy h:m a").withZone(ZoneId.of("UTC")).localizedBy(Locale.ENGLISH);
/// (its `parse` is not in the port's `DateTimeFormatter`: see `getTimestampInternal`)
pub static OLD_OBTAINED_DATE_FORMAT: compat::MainThread<DateTimeFormatter> = compat::MainThread::new(|| DateTimeFormatter::of_pattern("M/d/yy h:m a"));

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
    /// The accessor is an `Instant` (both of Java's answers are instants: `Instant.ofEpochMilli`, and a parse with a
    /// zoned formatter).
    pub fn get_timestamp_internal(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> Option<mcrust::java::time::instant::Instant> {
        let mut custom_data: Tag = ItemUtils::get_custom_data(stack);
        // if (customData != null && customData.get("timestamp") instanceof LongTag(long value)) return Instant.ofEpochMilli(value);
        if let Some(Tag::Long(value)) = custom_data.get("timestamp") {
            return Some(mcrust::java::time::instant::Instant::of_epoch_milli(*value));
        }
        // if (customData != null && customData.get("timestamp") instanceof StringTag(String value)) {
        //     try { return OLD_OBTAINED_DATE_FORMAT.parse(value); }
        //     catch (DateTimeParseException e) { LOGGER.error("[Skyblocker ObtainedDateTooltip] Failed to parse date: {}", value, e); }
        // }
        if let Some(Tag::String(value)) = custom_data.get("timestamp") {
            match parse_old_obtained_date(value) {
                Ok(instant) => return Some(instant),
                Err(e) => logger().error(&format!("[Skyblocker ObtainedDateTooltip] Failed to parse date: {} {}", value, e)),
            }
        }
        return None;
    }

    /// java: public static String getTimestamp(ItemStack stack)
    pub fn get_timestamp(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> String {
        let mut accessor: Option<mcrust::java::time::instant::Instant> = ObtainedDateTooltip::get_timestamp_internal(stack);
        // OBTAINED_DATE_FORMATTER.format(accessor) - in the formatter's zone (ZoneId.systemDefault())
        return (if let Some(accessor) = accessor { OBTAINED_DATE_FORMATTER.get().format(mcrust::java::time::zoned_date_time::ZonedDateTime::of_instant(accessor, mcrust::java::time::zone_id::ZoneId::system_default())) } else { "".to_string() });
    }

    /// java: public static long getLongTimestamp(ItemStack stack)
    pub fn get_long_timestamp(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> i64 {
        let mut accessor: Option<mcrust::java::time::instant::Instant> = ObtainedDateTooltip::get_timestamp_internal(stack);
        // return accessor != null ? Instant.from(accessor).toEpochMilli() : 0;
        return accessor.map(|accessor| accessor.to_epoch_milli()).unwrap_or(0);
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
            lines.push(GridComponent::of(&vec![component::literal("Obtained:").with_style_format(ChatFormatting::LightPurple), component::literal(&timestamp).with_style_format(ChatFormatting::Red)]));
        }
    }

    /// ObtainedDateTooltip.addToTooltip as the dispatch of TooltipAdder.addToTooltip
    pub fn add_to_tooltip_dyn(__this: &compat::Shared<dyn TooltipAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: &mut Vec<MutableComponent>) {
        Self::add_to_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}

/// `OLD_OBTAINED_DATE_FORMAT.parse(value)` - `DateTimeFormatter.ofPattern("M/d/yy h:m a").withZone(ZoneId.of("UTC"))
/// .localizedBy(Locale.ENGLISH)`, resolved to an instant. The port's `DateTimeFormatter` only formats, so the pattern's
/// parse is written out: `M`, `d`, `h`, `m` take one or two digits, `yy` two (the base year 2000), `a` is `AM`/`PM`
/// (English, exact case: the formatter parses case-sensitively), all of it in UTC. `Err` is the
/// `DateTimeParseException` message.
fn parse_old_obtained_date(text: &str) -> Result<mcrust::java::time::instant::Instant, String> {
    let fail = || format!("Text '{}' could not be parsed", text);
    let (date, rest) = text.split_once(' ').ok_or_else(fail)?;
    let (time, am_pm) = rest.split_once(' ').ok_or_else(fail)?;
    let mut date_parts = date.split('/');
    let num = |part: Option<&str>, min_len: usize, max_len: usize| -> Result<i64, String> {
        let part = part.ok_or_else(fail)?;
        if part.len() < min_len || part.len() > max_len || !part.bytes().all(|b| b.is_ascii_digit()) {
            return Err(fail());
        }
        part.parse::<i64>().map_err(|_| fail())
    };
    let month = num(date_parts.next(), 1, 2)?;
    let day = num(date_parts.next(), 1, 2)?;
    let year = 2000 + num(date_parts.next(), 2, 2)?;
    if date_parts.next().is_some() {
        return Err(fail());
    }
    let mut time_parts = time.split(':');
    let hour_of_am_pm = num(time_parts.next(), 1, 2)?;
    let minute = num(time_parts.next(), 1, 2)?;
    if time_parts.next().is_some() {
        return Err(fail());
    }
    let pm = match am_pm {
        "AM" => false,
        "PM" => true,
        _ => return Err(fail()),
    };
    if !(1..=12).contains(&month) || !(1..=12).contains(&hour_of_am_pm) || !(0..=59).contains(&minute) {
        return Err(fail());
    }
    let days_in_month = match month {
        2 => if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 { 29 } else { 28 },
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };
    if day < 1 || day > days_in_month {
        return Err(fail());
    }
    // `clock-hour-of-am-pm` 12 is hour 0 of its half
    let hour = (hour_of_am_pm % 12) + if pm { 12 } else { 0 };
    // days from 1970-01-01 (`LocalDate.toEpochDay`)
    let (y, m) = if month <= 2 { (year - 1, month + 9) } else { (year, month - 3) };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let doy = (153 * m + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let epoch_day = era * 146097 + doe - 719468;
    Ok(mcrust::java::time::instant::Instant::of_epoch_milli(((epoch_day * 24 + hour) * 60 + minute) * 60_000))
}
