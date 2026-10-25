//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.IdentifierTextField by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::edit_box::Responder;
use mcrust::net::minecraft::client::gui::components::edit_box::ResponderLambda;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// The text field's own fields (Java's `lastValid` / `valid`), which its responder (`this::onChanged`) writes as the
/// value changes.
#[derive(Clone, Debug, Default)]
pub struct IdentifierState {
    /// java: private String lastValid = "";
    pub last_valid: String,
    /// java: private boolean valid = false;
    pub valid: bool,
}

/// java: class IdentifierTextField extends EditBox
///
/// The box itself is the game's `EditBox` widget (`widget`); this is the subclass's state and callback. Java's
/// `setFocused` override is `IdentifierTextField::set_focused`, run by the owner where it sees the box's focus change.
pub struct IdentifierTextField {
    /// java: private final Consumer<@Nullable Identifier> callback;
    pub callback: compat::RenderThread<Arc<dyn Fn(Option<Identifier>)>>,
    /// `lastValid`, `valid`
    pub state: Arc<Mutex<IdentifierState>>,
}

impl IdentifierTextField {
    /// java: IdentifierTextField(int width, int height, Consumer<@Nullable Identifier> callback) - the object and its
    /// `EditBox` widget
    pub fn new(width: i32, height: i32, callback: Arc<dyn Fn(Option<Identifier>)>) -> (IdentifierTextField, AbstractWidget) {
        // java: super(Minecraft.getInstance().font, width, height, Component.empty());
        let mut widget = compat::edit_box(0, 0, width, height, component::empty());
        let state = Arc::new(Mutex::new(IdentifierState::default()));
        let callback = compat::RenderThread::new(callback);
        // java: super.setResponder(this::onChanged);
        let responder_state = state.clone();
        let responder_callback = compat::RenderThread::new(callback.get().clone());
        compat::with_edit_box(&mut widget, |e| {
            e.set_responder(Responder::Lambda(ResponderLambda::new(move |s| IdentifierTextField::on_changed(&responder_state, responder_callback.get(), s))));
            // java: setMaxLength(100);
            e.set_max_length(100);
        });
        // java: addFormatter((string, _) -> FormattedCharSequence.forward(string, valid ? Style.EMPTY : Style.EMPTY.applyFormat(ChatFormatting.RED)));
        let formatter_state = state.clone();
        compat::with_edit_box(&mut widget, |e| {
            e.add_formatter(mcrust::net::minecraft::client::gui::components::edit_box::TextFormatter::Mod(mcrust::net::minecraft::client::gui::components::edit_box::ModTextFormatter::new(move |string: &str, _offset: i32| {
                let valid: bool = compat::lock(&formatter_state).valid;
                let style = if valid { mcrust::net::minecraft::network::chat::style::Style::EMPTY } else { mcrust::net::minecraft::network::chat::style::Style::EMPTY.apply_format(mcrust::net::minecraft::chat_formatting::ChatFormatting::Red) };
                Some(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of_styled(string, style))
            })));
        });
        (IdentifierTextField { callback, state }, widget)
    }

    /// java: private void onChanged(String s)
    pub fn on_changed(state: &Mutex<IdentifierState>, callback: &Arc<dyn Fn(Option<Identifier>)>, s: &str) {
        let identifier: Option<Identifier> = Identifier::try_parse(s);
        compat::lock(state).valid = true;
        if compat::is_blank(s) {
            callback(None);
            compat::lock(state).last_valid = String::new();
        } else if identifier.is_some() {
            callback(identifier);
            compat::lock(state).last_valid = s.to_string();
        } else {
            compat::lock(state).valid = false;
        }
    }

    /// java: @Override public void setFocused(boolean focused) - after `super.setFocused(focused)` (the game's), an
    /// invalid value is put back to the last valid one when the box loses the focus.
    pub fn set_focused(&self, widget: &mut AbstractWidget, focused: bool) {
        let last_valid = compat::lock(&self.state).last_valid.clone();
        if !focused && last_valid != compat::edit_box_value(widget) {
            compat::set_edit_box_value(widget, &last_valid);
        }
    }
}
