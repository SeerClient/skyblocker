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
    /// java: public FilteredEditBox(Font font, int width, int height, Component narration) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut font: Font, mut width: i32, mut height: i32, mut narration: MutableComponent, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn FilteredEditBox>, __R)) -> (compat::Shared<dyn FilteredEditBox>, __R) {
        // TODO(fabric2seer): the port's `new` ports this Java method with other parameters (splitter, x, y, width, height for Java's width, height, narration)
        //   java: super(font, width, height, narration);
        let (__this, __r): (compat::Shared<dyn FilteredEditBox>, __R) = todo!("constructor");
        (__this, __r)
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
        // TODO(fabric2seer): the port's `new` ports this Java method with other parameters (splitter, x, y, width, height for Java's x, y, width, height, old_box, narration)
        //   java: super(font, x, y, width, height, null, narration);
        let (__this, __r): (compat::Shared<dyn FilteredEditBox>, __R) = todo!("constructor");
        (__this, __r)
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
        // TODO(fabric2seer): the port's `new` ports this Java method with other parameters (splitter, x, y, width, height for Java's x, y, width, height, old_box, narration)
        //   java: super(font, x, y, width, height, oldBox, narration);
        let (__this, __r): (compat::Shared<dyn FilteredEditBox>, __R) = todo!("constructor");
        (__this, __r)
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
            // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
            // rust: __this.lock().borrow_mut().filtered_edit_box_base_mut().base.set_value(value);
        }
    }

    /// java: @Override public void insertText(final String input)
    pub fn insert_text_body(__this: &compat::Shared<dyn FilteredEditBox>, input: &str) {
        // java: EditBoxAccessor accessor = (EditBoxAccessor) this; - the accessor is this edit box itself; its
        // getHighlightPos/getMaxLength/invokeOnValueChange reach `EditBox.highlight_pos` and `onValueChange`, which
        // the port keeps private (port gap, GAPS_batchA.md)
        let accessor: &compat::Shared<dyn FilteredEditBox> = __this;
        // TODO(fabric2seer): `.getHighlightPos(..)` on a EditBoxAccessor
        //   java: int start = Math.min(this.getCursorPosition(), accessor.getHighlightPos());
        let mut start: i32 = todo!("fabric2seer: the initialiser of `start` (TODO above)");
        // TODO(fabric2seer): `.getHighlightPos(..)` on a EditBoxAccessor
        //   java: int end = Math.max(this.getCursorPosition(), accessor.getHighlightPos());
        let mut end: i32 = todo!("fabric2seer: the initialiser of `end` (TODO above)");
        // TODO(fabric2seer): `.getMaxLength(..)` on a EditBoxAccessor
        //   java: int maxInsertionLength = accessor.getMaxLength() - this.getValue().length() - (start - end);
        let mut max_insertion_length: i32 = todo!("fabric2seer: the initialiser of `maxInsertionLength` (TODO above)");
        if max_insertion_length > 0 {
            let mut text: String = string_util::filter_text(input);
            let mut insertion_length: i32 = compat::str_len(&text);
            if max_insertion_length < insertion_length {
                // TODO(fabric2seer): the condition: Character.isHighSurrogate
                //   java: Character.isHighSurrogate(..)
                if todo!("fabric2seer: the condition (TODO above)") {
                    max_insertion_length -= 1;
                }
                text = compat::substring2(&text, 0, max_insertion_length);
                insertion_length = max_insertion_length;
            }
            // TODO(fabric2seer): StringBuilder.replace
            //   java: String newValue = new StringBuilder(this.getValue()).replace(start, end, text).toString();
            let mut new_value: String = todo!("fabric2seer: the initialiser of `newValue` (TODO above)");
            if ((compat::read(__this, |__t| __t.filtered_edit_box_base().filter.clone())))(input.to_string()) {
                ({ let __f = compat::dispatch(__this, |__t| __t.set_value()); __f(__this, &new_value) });
                // TODO(fabric2seer): `set_cursor_position` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, pos: i32`), Java passes 1
                // rust: { let (__q0,) = (&start.wrapping_add(insertion_length),); __this.lock().borrow_mut().filtered_edit_box_base_mut().set_cursor_position(__q0) };
                // TODO(fabric2seer): `set_highlight_pos` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, pos: i32`), Java passes 1
                // rust: { let (__q0,) = (&__this.lock().borrow().filtered_edit_box_base().get_cursor_position(),); __this.lock().borrow_mut().filtered_edit_box_base_mut().set_highlight_pos(__q0) };
                // TODO(fabric2seer): `.invokeOnValueChange(..)` on a EditBoxAccessor
                //   java: accessor.invokeOnValueChange(this.getValue());
            }
        }
    }

    /// java: @Override public void deleteCharsToPos(final int pos)
    pub fn delete_chars_to_pos_body(__this: &compat::Shared<dyn FilteredEditBox>, mut pos: i32) {
        // java: EditBoxAccessor accessor = (EditBoxAccessor) this; - the accessor is this edit box itself; its
        // getHighlightPos/getMaxLength/invokeOnValueChange reach `EditBox.highlight_pos` and `onValueChange`, which
        // the port keeps private (port gap, GAPS_batchA.md)
        let accessor: &compat::Shared<dyn FilteredEditBox> = __this;
        if !(__this.lock().borrow().filtered_edit_box_base().get_value().is_empty()) {
            // TODO(fabric2seer): the condition: `.getHighlightPos(..)` on a EditBoxAccessor
            //   java: accessor.getHighlightPos() != this.getCursorPosition()
            if todo!("fabric2seer: the condition (TODO above)") {
                ({ let __f = compat::dispatch(__this, |__t| __t.insert_text()); __f(__this, "") });
            } else {
                let mut start: i32 = pos.min(__this.lock().borrow().filtered_edit_box_base().get_cursor_position());
                let mut end: i32 = pos.max(__this.lock().borrow().filtered_edit_box_base().get_cursor_position());
                if start != end {
                    // TODO(fabric2seer): StringBuilder.delete
                    //   java: String newValue = new StringBuilder(this.getValue()).delete(start, end).toString();
                    let mut new_value: String = todo!("fabric2seer: the initialiser of `newValue` (TODO above)");
                    if ((compat::read(__this, |__t| __t.filtered_edit_box_base().filter.clone())))(new_value.clone()) {
                        ({ let __f = compat::dispatch(__this, |__t| __t.set_value()); __f(__this, &new_value) });
                        // TODO(fabric2seer): `set_cursor_position` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, pos: i32`), Java passes 1
                        // rust: __this.lock().borrow_mut().filtered_edit_box_base_mut().set_cursor_position(&start);
                        // TODO(fabric2seer): `.invokeOnValueChange(..)` on a EditBoxAccessor
                        //   java: accessor.invokeOnValueChange(this.getValue());
                        // TODO(fabric2seer): `move_cursor_to` takes 3 argument(s) in the port (`splitter: &StringSplitter<W>, pos: i32, extend_selection: bool`), Java passes 2
                        // rust: __this.lock().borrow_mut().filtered_edit_box_base_mut().move_cursor_to(&start, false);
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
