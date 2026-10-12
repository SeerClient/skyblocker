//! Ported from de.hysky.skyblocker.utils.CountdownComponent by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::locale::language;
use mcrust::net::minecraft::network::chat::component_contents::ComponentContents;
use mcrust::net::minecraft::network::chat::contents::plain_text_contents::PlainTextContents;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::style::Style;
use mcrust::net::minecraft::util::formatted_char_sequence::FormattedCharSequence;
use mcrust::net::minecraft::util::formatted_char_sequence::Seq;

/// java: public final class CountdownComponent implements Component
pub struct CountdownComponent {
    /// java: private final long start;
    pub start: i64,
    /// java: private final long duration;
    pub duration: i64,
    /// java: private final TimeUnit timeUnit;
    pub time_unit: compat::TimeUnit,
    /// java: private final Component prefix;
    pub prefix: MutableComponent,
    /// java: private final Style style;
    pub style: Style,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl CountdownComponent {
    /// java: public CountdownComponent(long duration, TimeUnit timeUnit, Component prefix, Style style)
    pub fn new(mut duration: i64, mut time_unit: compat::TimeUnit, mut prefix: MutableComponent, mut style: Style) -> compat::Shared<Self> {
        let __init_start = compat::millis();
        let __init_duration = { duration += 1; duration };
        let __init_time_unit = time_unit.clone();
        let __init_prefix = prefix.clone();
        let __init_style = style.clone();
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            start: __init_start,
            duration: __init_duration,
            time_unit: __init_time_unit,
            prefix: __init_prefix,
            style: __init_style,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: @Override public Style getStyle()
    pub fn get_style(__this: &compat::Shared<Self>) -> Style {
        return compat::read(__this, |__t| __t.style.clone());
    }

    /// java: @Override public ComponentContents getContents()
    pub fn get_contents(__this: &compat::Shared<Self>) -> ComponentContents {
        // java: long end = this.start + this.timeUnit.toMillis(this.duration);
        let (start, duration, time_unit) = compat::read(__this, |__t| (__t.start, __t.duration, __t.time_unit));
        let mut end: i64 = start.wrapping_add(time_unit_convert(compat::TimeUnit::Milliseconds, duration, time_unit));
        // java: long timeLeft = this.timeUnit.convert(end - System.currentTimeMillis(), TimeUnit.MILLISECONDS);
        let mut time_left: i64 = time_unit_convert(time_unit, end.wrapping_sub(compat::millis()), compat::TimeUnit::Milliseconds);
        return (if time_left > (0 as i64) { ComponentContents::Text(PlainTextContents::create(&format!(" ({})", time_left))) } else { ComponentContents::Text(PlainTextContents::Empty) });
    }

    /// java: @Override public List<Component> getSiblings()
    pub fn get_siblings(__this: &compat::Shared<Self>) -> Vec<MutableComponent> {
        return Vec::new();
    }

    /// java: @Override public FormattedCharSequence getVisualOrderText()
    pub fn get_visual_order_text(__this: &compat::Shared<Self>) -> Seq {
        // TODO(fabric2seer): `Language.getVisualOrder` has no Rust method in the port (looked for `get_visual_order` in Language and its supertypes)
        // rust: return language::get_instance().get_visual_order(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::composite(vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(__this, |__t| __t.prefix.clone())), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(compat::jstring(&__this))]));
        return todo!("fabric2seer: see the TODO above");
    }

}

/// `TimeUnit.convert(sourceDuration, sourceUnit)` into `unit` (`toMillis(d)` is `MILLISECONDS.convert(d, this)`):
/// truncating toward zero, saturating at `Long.MIN_VALUE` / `Long.MAX_VALUE` as Java's does.
fn time_unit_convert(unit: compat::TimeUnit, source_duration: i64, source_unit: compat::TimeUnit) -> i64 {
    fn nanos(unit: compat::TimeUnit) -> i128 {
        match unit {
            compat::TimeUnit::Nanoseconds => 1,
            compat::TimeUnit::Microseconds => 1_000,
            compat::TimeUnit::Milliseconds => 1_000_000,
            compat::TimeUnit::Seconds => 1_000_000_000,
            compat::TimeUnit::Minutes => 60_000_000_000,
            compat::TimeUnit::Hours => 3_600_000_000_000,
            compat::TimeUnit::Days => 86_400_000_000_000,
        }
    }
    let value: i128 = (source_duration as i128) * nanos(source_unit) / nanos(unit);
    value.clamp(i64::MIN as i128, i64::MAX as i128) as i64
}
