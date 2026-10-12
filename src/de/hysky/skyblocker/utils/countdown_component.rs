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
    pub time_unit: crate::unresolved::TimeUnit,
    /// java: private final Component prefix;
    pub prefix: MutableComponent,
    /// java: private final Style style;
    pub style: Style,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl CountdownComponent {
    /// java: public CountdownComponent(long duration, TimeUnit timeUnit, Component prefix, Style style)
    pub fn new(mut duration: i64, mut time_unit: crate::unresolved::TimeUnit, mut prefix: MutableComponent, mut style: Style) -> compat::Shared<Self> {
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
        // TODO(fabric2seer): `.toMillis(..)` on a TimeUnit
        //   java: long end = this.start + this.timeUnit.toMillis(this.duration);
        let mut end: i64 = todo!("fabric2seer: the initialiser of `end` (TODO above)");
        // TODO(fabric2seer): `.convert(..)` on a TimeUnit
        //   java: long timeLeft = this.timeUnit.convert(end - System.currentTimeMillis(), TimeUnit.MILLISECONDS);
        let mut time_left: i64 = todo!("fabric2seer: the initialiser of `timeLeft` (TODO above)");
        return (if time_left > (0 as i64) { PlainTextContents::create(&format!(" ({})", time_left)) } else { PlainTextContents::Empty });
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
