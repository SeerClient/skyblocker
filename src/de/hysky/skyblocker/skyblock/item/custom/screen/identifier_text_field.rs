//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.IdentifierTextField by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::edit_box::EditBox;
use mcrust::net::minecraft::client::gui::components::edit_box::Responder;
use mcrust::net::minecraft::client::gui::components::edit_box::ResponderLambda;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::formatted_char_sequence;

/// java: class IdentifierTextField extends EditBox
#[derive(Clone)]
pub struct IdentifierTextField {
    /// java: extends net.minecraft.client.gui.components.EditBox
    pub base: EditBox,
    /// java: private final Consumer<@Nullable Identifier> callback;
    pub callback: Option<Arc<dyn Fn(Option<Identifier>)>>,
    /// java: private String lastValid = "";
    pub last_valid: String,
    /// java: private boolean valid = false;
    pub valid: bool,
}

impl std::ops::Deref for IdentifierTextField {
    type Target = EditBox;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for IdentifierTextField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl IdentifierTextField {
    /// java: IdentifierTextField(int width, int height, Consumer<@Nullable Identifier> callback)
    pub fn new(mut width: i32, mut height: i32, callback: Arc<dyn Fn(Option<Identifier>)>) -> compat::Shared<Self> {
        // TODO(fabric2seer): the port's `new` ports this Java method with other parameters (splitter, x, y, width, height for Java's width, height, narration)
        //   java: super(Minecraft.getInstance().font, width, height, Component.empty());
        let __this: compat::Shared<Self> = todo!("constructor");
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        { let (__q0,) = (Responder::Lambda(ResponderLambda::new(move |__a0| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { IdentifierTextField::on_changed(&__this_1, __a0.to_string()); } })),); __this.lock().borrow_mut().base.set_responder(__q0) };
        __this.lock().borrow_mut().set_max_length(100);
        { let __v = Some(callback.clone()); compat::write(&__this, |__t| __t.callback = __v); }
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.client.gui.components.EditBox.TextFormatter`)
        // rust: let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        // rust: { let (__q0,) = (|string, _| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; formatted_char_sequence::forward(&string, (if compat::read(&__this_2, |__t| __t.valid) { mcrust::net::minecraft::network::chat::style::Style::EMPTY } else { mcrust::net::minecraft::network::chat::style::Style::EMPTY.apply_format(ChatFormatting::Red) })) },); __this.lock().borrow_mut().add_formatter(__q0) };
        __this
    }

    /// java: private void onChanged(String s)
    pub fn on_changed(__this: &compat::Shared<Self>, mut s: String) {
        let mut identifier: Option<Identifier> = Identifier::try_parse(&s);
        { let __v = true; compat::write(__this, |__t| __t.valid = __v); }
        if compat::is_blank(&s) {
            ((compat::read(__this, |__t| __t.callback.clone()).unwrap()))(None);
            { let __v = "".to_string(); compat::write(__this, |__t| __t.last_valid = __v); }
        } else if identifier.is_some() {
            ((compat::read(__this, |__t| __t.callback.clone()).unwrap()))(identifier.clone());
            { let __v = s.clone(); compat::write(__this, |__t| __t.last_valid = __v); }
        } else {
            { let __v = false; compat::write(__this, |__t| __t.valid = __v); }
        }
    }

    /// java: @Override public void setFocused(boolean focused)
    pub fn set_focused(__this: &compat::Shared<Self>, mut focused: bool) {
        // TODO(fabric2seer): `set_focused_at` takes 2 argument(s) in the port (`focused: bool, millis: i64`), Java passes 1
        // rust: __this.lock().borrow_mut().base.set_focused_at(focused);
        if !focused && !(compat::read(__this, |__t| __t.last_valid.clone()) == __this.lock().borrow().get_value()) {
            // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
            // rust: let __r0 = compat::read(__this, |__t| __t.last_valid.clone());
            // rust: { let (__q0,) = (&__r0,); __this.lock().borrow_mut().set_value(__q0) };
        }
    }

}

// NOTE(fabric2seer): setFocused override(s) of EditBox - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
