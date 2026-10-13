//! Ported from de.hysky.skyblocker.utils.render.gui.FilteredEditBox by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::components::edit_box::EditBox;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::util::string_util;

/// java: public class FilteredEditBox extends EditBox
#[derive(Clone)]
pub struct FilteredEditBoxBase {
    /// java: extends net.minecraft.client.gui.components.EditBox
    pub base: EditBox,
    /// java: private Predicate<String> filter = Objects::nonNull;
    pub filter: Arc<dyn Fn(String) -> bool>,
}

impl std::ops::Deref for FilteredEditBoxBase {
    type Target = EditBox;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for FilteredEditBoxBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FilteredEditBoxBase {
    /// (The game's `EditBox` widget runs its own `insertText` / `deleteCharsToPos` / `setValue`: these overrides run
    /// where the mod calls them. rust: port gap - the port's `EditBox` has no `filter` and a mod `EditBox` subclass's
    /// overrides are not dispatched by the game's widget (GAPS_N #2).)
    /// java: public FilteredEditBox(Font font, int width, int height, Component narration) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut font: Font, mut width: i32, mut height: i32, mut narration: MutableComponent, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn FilteredEditBox>, __R)) -> (compat::Shared<dyn FilteredEditBox>, __R) {
        // java: super(font, width, height, narration); - `EditBox(font, 0, 0, width, height, narration)`
        Self::new_7_in(font, 0, 0, width, height, None, narration, __wrap)
    }

    /// java: public FilteredEditBox(Font font, int width, int height, Component narration)
    pub fn new(mut font: Font, mut width: i32, mut height: i32, mut narration: MutableComponent) -> compat::Shared<Self> {
        Self::new_in(font, width, height, narration, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn FilteredEditBox>), __h)
        }).1
    }

    /// java: public FilteredEditBox(Font font, int x, int y, int width, int height, Component narration) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_6_in<__R>(mut font: Font, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut narration: MutableComponent, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn FilteredEditBox>, __R)) -> (compat::Shared<dyn FilteredEditBox>, __R) {
        // java: super(font, x, y, width, height, null, narration);
        Self::new_7_in(font, x, y, width, height, None, narration, __wrap)
    }

    /// java: public FilteredEditBox(Font font, int x, int y, int width, int height, Component narration)
    pub fn new_6(mut font: Font, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut narration: MutableComponent) -> compat::Shared<Self> {
        Self::new_6_in(font, x, y, width, height, narration, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn FilteredEditBox>), __h)
        }).1
    }

    /// java: public FilteredEditBox(Font font, int x, int y, int width, int height, @Nullable EditBox oldBox, Component narration) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_7_in<__R>(mut font: Font, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut old_box: Option<EditBox>, mut narration: MutableComponent, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn FilteredEditBox>, __R)) -> (compat::Shared<dyn FilteredEditBox>, __R) {
        // java: super(font, x, y, width, height, oldBox, narration); - `EditBox(..):95-104`: the box over the font's
        // splitter, then `if (oldBox != null) setValue(oldBox.getValue())` (the narration is the widget's message, which
        // the game's `AbstractWidget` around the box keeps)
        let _ = narration;
        let splitter = font.splitter();
        let mut edit_box = EditBox::new(&splitter, x, y, width, height);
        if let Some(old_box) = &old_box {
            edit_box.set_value(&splitter, &old_box.get_value());
        }
        // java: private Predicate<String> filter = Objects::nonNull; - a `String` is never null here
        let this = Self { base: edit_box, filter: Arc::new(|_: String| true) };
        __wrap(this)
    }

    /// java: public FilteredEditBox(Font font, int x, int y, int width, int height, @Nullable EditBox oldBox, Component narration)
    pub fn new_7(mut font: Font, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut old_box: Option<EditBox>, mut narration: MutableComponent) -> compat::Shared<Self> {
        Self::new_7_in(font, x, y, width, height, old_box, narration, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn FilteredEditBox>), __h)
        }).1
    }

}

/// java: the virtual methods of FilteredEditBox (implemented by FilteredEditBoxBase and every subclass)
pub trait FilteredEditBox: compat::DynAny {
    fn filtered_edit_box_base(&self) -> &FilteredEditBoxBase;
    fn filtered_edit_box_base_mut(&mut self) -> &mut FilteredEditBoxBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public void setFilter(Predicate<String> filter) (dispatch: the body for this class)
    fn set_filter(&self) -> fn(&compat::Shared<dyn FilteredEditBox>, Arc<dyn Fn(String) -> bool>) {
        <dyn FilteredEditBox>::set_filter_body
    }

    /// java: @Override public void setValue(String value) (dispatch: the body for this class)
    fn set_value(&self) -> fn(&compat::Shared<dyn FilteredEditBox>, &str) {
        <dyn FilteredEditBox>::set_value_body
    }

    /// java: @Override public void insertText(final String input) (dispatch: the body for this class)
    fn insert_text(&self) -> fn(&compat::Shared<dyn FilteredEditBox>, &str) {
        <dyn FilteredEditBox>::insert_text_body
    }

    /// java: @Override public void deleteCharsToPos(final int pos) (dispatch: the body for this class)
    fn delete_chars_to_pos(&self) -> fn(&compat::Shared<dyn FilteredEditBox>, i32) {
        <dyn FilteredEditBox>::delete_chars_to_pos_body
    }

}

impl dyn FilteredEditBox {
    /// java: public void setFilter(Predicate<String> filter)
    pub fn set_filter_body(__this: &compat::Shared<dyn FilteredEditBox>, filter: Arc<dyn Fn(String) -> bool>) {
        { let __v = filter.clone(); compat::write(__this, |__t| __t.filtered_edit_box_base_mut().filter = __v); }
    }

    /// java: @Override public void setValue(String value)
    pub fn set_value_body(__this: &compat::Shared<dyn FilteredEditBox>, value: &str) {
        if ((compat::read(__this, |__t| __t.filtered_edit_box_base().filter.clone())))(value.to_string()) {
            // java: super.setValue(value);
            let __font = compat::font();
            __this.lock().borrow_mut().filtered_edit_box_base_mut().base.set_value(&__font.splitter(), value);
        }
    }

    /// java: @Override public void insertText(final String input)
    pub fn insert_text_body(__this: &compat::Shared<dyn FilteredEditBox>, input: &str) {
        // java: EditBoxAccessor accessor = (EditBoxAccessor) this; - the accessor is this edit box itself; its
        // getHighlightPos/getMaxLength/invokeOnValueChange reach `EditBox.highlight_pos` and `onValueChange`, which
        // the port keeps private (port gap, GAPS_batchA.md)
        let accessor: &compat::Shared<dyn FilteredEditBox> = __this;
        let _ = accessor;
        let (__cursor, __highlight, __max_length, __value) = compat::read(__this, |__t| {
            let b = &__t.filtered_edit_box_base().base;
            (b.get_cursor_position(), b.get_highlight_pos(), b.get_max_length(), b.get_value())
        });
        // java: int start = Math.min(this.getCursorPosition(), accessor.getHighlightPos());
        let start: i32 = __cursor.min(__highlight);
        // java: int end = Math.max(this.getCursorPosition(), accessor.getHighlightPos());
        let end: i32 = __cursor.max(__highlight);
        // java: int maxInsertionLength = accessor.getMaxLength() - this.getValue().length() - (start - end);
        let mut max_insertion_length: i32 = __max_length - compat::str_len(&__value) - (start - end);
        if max_insertion_length > 0 {
            let mut text: String = string_util::filter_text(input);
            let mut insertion_length: i32 = compat::str_len(&text);
            if max_insertion_length < insertion_length {
                // java: if (Character.isHighSurrogate(text.charAt(maxInsertionLength - 1)))
                if text.encode_utf16().nth((max_insertion_length - 1) as usize).is_some_and(|__u| (0xD800..=0xDBFF).contains(&__u)) {
                    max_insertion_length -= 1;
                }
                text = compat::substring2(&text, 0, max_insertion_length);
                insertion_length = max_insertion_length;
            }
            // java: String newValue = new StringBuilder(this.getValue()).replace(start, end, text).toString();
            let new_value: String = format!("{}{}{}", compat::substring2(&__value, 0, start), text, compat::substring(&__value, end));
            if ((compat::read(__this, |__t| __t.filtered_edit_box_base().filter.clone())))(input.to_string()) {
                ({ let __f = compat::dispatch(__this, |__t| __t.set_value()); __f(__this, &new_value) });
                let __font = compat::font();
                let __splitter = __font.splitter();
                compat::write(__this, |__t| {
                    let b = &mut __t.filtered_edit_box_base_mut().base;
                    // java: this.setCursorPosition(start + insertionLength);
                    b.set_cursor_position(&__splitter, start.wrapping_add(insertion_length));
                    // java: this.setHighlightPos(this.getCursorPosition());
                    let __pos = b.get_cursor_position();
                    b.set_highlight_pos(&__splitter, __pos);
                    // java: accessor.invokeOnValueChange(this.getValue());
                    let __v = b.get_value();
                    b.on_value_change(&__splitter, &__v);
                });
            }
        }
    }

    /// java: @Override public void deleteCharsToPos(final int pos)
    pub fn delete_chars_to_pos_body(__this: &compat::Shared<dyn FilteredEditBox>, mut pos: i32) {
        // java: EditBoxAccessor accessor = (EditBoxAccessor) this; - the accessor is this edit box itself; its
        // getHighlightPos/getMaxLength/invokeOnValueChange reach `EditBox.highlight_pos` and `onValueChange`, which
        // the port keeps private (port gap, GAPS_batchA.md)
        let accessor: &compat::Shared<dyn FilteredEditBox> = __this;
        let _ = accessor;
        if !(__this.lock().borrow().filtered_edit_box_base().get_value().is_empty()) {
            // java: if (accessor.getHighlightPos() != this.getCursorPosition())
            if compat::read(__this, |__t| __t.filtered_edit_box_base().base.get_highlight_pos() != __t.filtered_edit_box_base().base.get_cursor_position()) {
                ({ let __f = compat::dispatch(__this, |__t| __t.insert_text()); __f(__this, "") });
            } else {
                let mut start: i32 = pos.min(__this.lock().borrow().filtered_edit_box_base().get_cursor_position());
                let mut end: i32 = pos.max(__this.lock().borrow().filtered_edit_box_base().get_cursor_position());
                if start != end {
                    // java: String newValue = new StringBuilder(this.getValue()).delete(start, end).toString();
                    let __value: String = __this.lock().borrow().filtered_edit_box_base().get_value();
                    let new_value: String = format!("{}{}", compat::substring2(&__value, 0, start), compat::substring(&__value, end));
                    if ((compat::read(__this, |__t| __t.filtered_edit_box_base().filter.clone())))(new_value.clone()) {
                        ({ let __f = compat::dispatch(__this, |__t| __t.set_value()); __f(__this, &new_value) });
                        let __font = compat::font();
                        let __splitter = __font.splitter();
                        compat::write(__this, |__t| {
                            let b = &mut __t.filtered_edit_box_base_mut().base;
                            // java: this.setCursorPosition(start);
                            b.set_cursor_position(&__splitter, start);
                            // java: accessor.invokeOnValueChange(this.getValue());
                            let __v = b.get_value();
                            b.on_value_change(&__splitter, &__v);
                            // java: this.moveCursorTo(start, false);
                            b.move_cursor_to(&__splitter, start, false);
                        });
                    }
                }
            }
        }
    }

}

impl FilteredEditBox for FilteredEditBoxBase {
    fn filtered_edit_box_base(&self) -> &FilteredEditBoxBase {
        self
    }
    fn filtered_edit_box_base_mut(&mut self) -> &mut FilteredEditBoxBase {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}
