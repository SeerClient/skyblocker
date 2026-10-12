//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.name.visitor.GetClickedPositionVisitor by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::style::Style;
use mcrust::net::minecraft::util::formatted_char_sequence::FormattedCharSequence;
use mcrust::net::minecraft::util::formatted_char_sequence::Seq;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class GetClickedPositionVisitor implements FormattedText.StyledContentConsumer<Void>
pub struct GetClickedPositionVisitor {
    /// java: private final MutableComponent text = Component.empty();
    pub text: MutableComponent,
    /// java: private final Font textRenderer;
    pub text_renderer: Option<Font>,
    /// java: private int position = -1;
    pub position: i32,
    /// java: private final int x;
    pub x: i32,
}

impl GetClickedPositionVisitor {
    /// java: public GetClickedPositionVisitor(Font textRenderer, int x)
    pub fn new(mut text_renderer: Font, mut x: i32) -> Self {
        let __init_x = x;
        let __init_text_renderer = Some(compat::font());
        let mut this = Self {
            text: component::empty(),
            text_renderer: __init_text_renderer,
            position: -1,
            x: __init_x,
        };
        this
    }

    /// java: public GetClickedPositionVisitor(int x)
    pub fn new_1(mut x: i32) -> Self {
        let mut this = Self::new(compat::font(), x);
        this
    }

    /// java: protected void visit(Style style, String asString)
    pub fn visit(&mut self, style: &Style, as_string: &str) {
        if self.position >= 0 {
            return;
        }
        if as_string.is_empty() {
            return;
        }
        let mut text1: MutableComponent = component::literal(as_string).set_style(style.clone());
        let mut original_width: i32 = self.text_renderer.as_ref().unwrap().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(self.text.clone()));
        if original_width.wrapping_add(self.text_renderer.as_ref().unwrap().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text1.clone()))) < self.x {
            self.text.append(text1.clone());
            return;
        }
        let mut current_width: i32 = 0;
        let mut atomic_integer: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
        let mut ordered_text: Seq = Arc::new(move |visitor| {
            // TODO(fabric2seer): `.accept(..)` on a value of unknown type
            //   java: visitor.accept(0, style, asString.codePointAt(atomicInteger.get()));
            return true;
        });
        while atomic_integer.load(std::sync::atomic::Ordering::SeqCst) < compat::str_len(&as_string) && (original_width.wrapping_add(current_width)).wrapping_add((self.text_renderer.as_ref().unwrap().width_of_sequence(&ordered_text)).wrapping_div(2)) <= self.x {
            current_width = current_width.wrapping_add(self.text_renderer.as_ref().unwrap().width_of_sequence(&ordered_text));
            (atomic_integer.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1);
        }
        self.position = ((compat::str_len(&self.text.get_string())).wrapping_add(atomic_integer.load(std::sync::atomic::Ordering::SeqCst))).max(0);
    }

    /// java: public int getPosition()
    pub fn get_position(&self) -> i32 {
        return self.position;
    }

    /// java: @Override public Optional<Void> accept(Style style, String asString)
    pub fn accept(&mut self, style: &Style, as_string: &str) -> Option<()> {
        self.visit(style, as_string);
        return None;
    }

}
