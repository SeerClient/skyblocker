//! Ported from de.hysky.skyblocker.utils.config.DurationControllerWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class DurationControllerWidget extends StringControllerElement
pub struct DurationControllerWidget {
    /// java: extends dev.isxander.yacl3.gui.controllers.string.StringControllerElement
    pub base: crate::unresolved::StringControllerElement,
}

impl std::ops::Deref for DurationControllerWidget {
    type Target = crate::unresolved::StringControllerElement;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DurationControllerWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DurationControllerWidget {
    /// java: public DurationControllerWidget(IStringController<?> control, YACLScreen screen, Dimension<Integer> dim)
    pub fn new(mut control: crate::unresolved::IStringController, mut screen: crate::unresolved::YACLScreen, mut dim: crate::unresolved::Dimension) -> Self {
        // TODO(fabric2seer): `new StringControllerElement(..)`: the port has no `StringControllerElement::new`
        //   java: super(control, screen, dim, false);
        let mut this: Self = todo!("constructor");
        this
    }

    /// java: @Override public void unfocus()
    pub fn unfocus(&self) {
        // TODO(fabric2seer): the condition: unknown name `control`
        //   java: control.isInputValid(..)
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `StringControllerElement.unfocus` has no Rust method in the port (looked for `unfocus` in StringControllerElement and JavaSrc)
            // rust: self.base.unfocus();
        } else {
            // TODO(fabric2seer): unknown name `control`
            //   java: modifyInput(stringBuilder -> stringBuilder.replace(0, stringBuilder.length(), control.getString()));
        }
    }

    /// java: @Override public boolean modifyInput(Consumer<StringBuilder> consumer)
    pub fn modify_input(&self, consumer: Arc<dyn Fn(String)>) -> bool {
        // TODO(fabric2seer): unknown name `inputField`
        //   java: StringBuilder temp = new StringBuilder(inputField);
        let mut temp: String = todo!("fabric2seer: the initialiser of `temp` (TODO above)");
        (consumer)(temp.clone());
        // TODO(fabric2seer): assignment to unknown `inputField`
        //   java: inputField = temp.toString();
        return true;
    }

    /// java: @Override protected Component getValueText()
    pub fn get_value_text(&self) -> MutableComponent {
        // TODO(fabric2seer): `StringControllerElement.getValueText` has no Rust method in the port (looked for `get_value_text` in StringControllerElement and JavaSrc)
        // rust: let mut value_text: MutableComponent = self.base.get_value_text();
        let mut value_text: MutableComponent = todo!("fabric2seer: the initialiser of `valueText` (TODO above)");
        // TODO(fabric2seer): unknown name `control`
        //   java: boolean inputValid = control.isInputValid(valueText.getString());
        let mut input_valid: bool = todo!("fabric2seer: the initialiser of `inputValid` (TODO above)");
        return value_text.copy().with_style_format((if input_valid { ChatFormatting::White } else { ChatFormatting::Red }));
    }

}

// NOTE(fabric2seer): getValueText, modifyInput, unfocus override(s) of StringControllerElement - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
