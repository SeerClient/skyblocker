//! Ported from de.hysky.skyblocker.utils.config.DurationControllerWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;
use yacl::dev::isxander::yacl3::api::utils::dimension::Dimension;
use yacl::dev::isxander::yacl3::gui::controllers::string::i_string_controller::IStringController;
use yacl::dev::isxander::yacl3::gui::controllers::string::string_controller_element::{self, StringControllerElement, StringControllerElementClass};
use yacl::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;

/// java: public class DurationControllerWidget extends StringControllerElement
pub struct DurationControllerWidget {
    /// java: extends dev.isxander.yacl3.gui.controllers.string.StringControllerElement
    pub base: StringControllerElement,
}

impl std::ops::Deref for DurationControllerWidget {
    type Target = StringControllerElement;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DurationControllerWidget {
    /// java: public DurationControllerWidget(IStringController<?> control, YACLScreen screen, Dimension<Integer> dim)
    pub fn new<T: Clone + 'static, C: IStringController<T> + ?Sized + 'static>(control: Arc<C>, screen: &Arc<YACLScreen>, dim: Arc<dyn Dimension<i32>>) -> Self {
        // java: super(control, screen, dim, false);
        let this = Self { base: StringControllerElement::new(control, screen, dim.clone(), false) };
        // java: the super constructor's `setDimension(dim)` dispatches to this class (its `getValueText` override sizes the
        // input field) - StringControllerElement's note: a subclass runs it after building itself
        string_controller_element::set_dimension(&this, dim);
        this
    }
}

impl StringControllerElementClass for DurationControllerWidget {
    fn string_element(&self) -> &StringControllerElement {
        &self.base
    }

    /// java: @Override public void unfocus()
    fn unfocus(&self) {
        if self.base.control.is_input_valid(&self.base.input_field()) {
            // java: super.unfocus();
            string_controller_element::unfocus(self);
        } else {
            // java: modifyInput(stringBuilder -> stringBuilder.replace(0, stringBuilder.length(), control.getString()));
            let value = self.base.control.get_string();
            self.modify_input(&|string_builder: &mut String| {
                let length = yacl::compat::str_len(string_builder);
                string_controller_element::sb_replace(string_builder, 0, length, &value);
            });
        }
    }

    /// java: @Override public boolean modifyInput(Consumer<StringBuilder> consumer)
    fn modify_input(&self, consumer: &dyn Fn(&mut String)) -> bool {
        // java: StringBuilder temp = new StringBuilder(inputField);
        let mut temp: String = self.base.input_field();
        consumer(&mut temp);
        // java: inputField = temp.toString();
        self.base.set_input_field(temp);
        true
    }

    /// java: @Override protected Component getValueText()
    fn get_value_text(&self) -> MutableComponent {
        // java: Component valueText = super.getValueText();
        let value_text: MutableComponent = string_controller_element::get_value_text(self);
        // java: boolean inputValid = control.isInputValid(valueText.getString());
        let input_valid: bool = self.base.control.is_input_valid(&value_text.get_string());
        value_text.copy().with_style_format(if input_valid { ChatFormatting::White } else { ChatFormatting::Red })
    }
}
