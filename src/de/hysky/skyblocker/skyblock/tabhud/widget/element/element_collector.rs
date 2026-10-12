//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.element.ElementCollector by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::PlayerListManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::elements::Elements;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public interface ElementCollector
pub trait ElementCollector: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: <E extends Element> E addElement(E element);
    fn add_element(&mut self, element: compat::Shared<dyn Element>) -> compat::Shared<dyn Element>;

    /// java: default void addSimpleIcoText(@Nullable FlexibleItemStack ico, String string, ChatFormatting fmt, int idx) (dispatch: the body for this class)
    fn add_simple_ico_text(&self) -> fn(&compat::Shared<dyn ElementCollector>, Option<compat::Shared<FlexibleItemStack>>, &str, &ChatFormatting, i32) {
        <dyn ElementCollector>::add_simple_ico_text_body
    }

    /// java: default void addSimpleIcoText(@Nullable FlexibleItemStack ico, String string, ChatFormatting fmt, String content) (dispatch: the body for this class)
    fn add_simple_ico_text_flexible_item_stack_string_chat_formatting_string(&self) -> fn(&compat::Shared<dyn ElementCollector>, Option<compat::Shared<FlexibleItemStack>>, &str, &ChatFormatting, &str) {
        <dyn ElementCollector>::add_simple_ico_text_flexible_item_stack_string_chat_formatting_string_body
    }

    /// java: default void addSimpleIconTranslatableText(@Nullable FlexibleItemStack icon, @Translatable String translationKey, ChatFormatting formatting, String content) (dispatch: the body for this class)
    fn add_simple_icon_translatable_text(&self) -> fn(&compat::Shared<dyn ElementCollector>, Option<compat::Shared<FlexibleItemStack>>, &str, &ChatFormatting, &str) {
        <dyn ElementCollector>::add_simple_icon_translatable_text_body
    }

    /// java: default void addSimpleIconTranslatableText(FlexibleItemStack icon, @Translatable String translationKey, ChatFormatting formatting, Component content) (dispatch: the body for this class)
    fn add_simple_icon_translatable_text_flexible_item_stack_string_chat_formatting_component(&self) -> fn(&compat::Shared<dyn ElementCollector>, &compat::Shared<FlexibleItemStack>, &str, &ChatFormatting, &MutableComponent) {
        <dyn ElementCollector>::add_simple_icon_translatable_text_flexible_item_stack_string_chat_formatting_component_body
    }

}

impl dyn ElementCollector {
    /// java: default void addSimpleIcoText(@Nullable FlexibleItemStack ico, String string, ChatFormatting fmt, int idx)
    pub fn add_simple_ico_text_body(__this: &compat::Shared<dyn ElementCollector>, mut ico: Option<compat::Shared<FlexibleItemStack>>, string: &str, fmt: &ChatFormatting, mut idx: i32) {
        let mut txt: MutableComponent = simple_entry_text(idx, string, fmt).unwrap();
        { let (__a0,) = (Elements::icon_text_component_2(ico.clone(), Some(txt.clone())),); __this.lock().borrow_mut().add_element(__a0) };
    }

    /// java: default void addSimpleIcoText(@Nullable FlexibleItemStack ico, String string, ChatFormatting fmt, String content)
    pub fn add_simple_ico_text_flexible_item_stack_string_chat_formatting_string_body(__this: &compat::Shared<dyn ElementCollector>, mut ico: Option<compat::Shared<FlexibleItemStack>>, string: &str, fmt: &ChatFormatting, content: &str) {
        let mut txt: MutableComponent = simple_entry_text_string_string_chat_formatting(content, string, fmt);
        { let (__a0,) = (Elements::icon_text_component_2(ico.clone(), Some(txt.clone())),); __this.lock().borrow_mut().add_element(__a0) };
    }

    /// java: default void addSimpleIconTranslatableText(@Nullable FlexibleItemStack icon, @Translatable String translationKey, ChatFormatting formatting, String content)
    pub fn add_simple_icon_translatable_text_body(__this: &compat::Shared<dyn ElementCollector>, mut icon: Option<compat::Shared<FlexibleItemStack>>, translation_key: &str, formatting: &ChatFormatting, content: &str) {
        let mut text: MutableComponent = simple_entry_translatable_text(translation_key, content, formatting);
        { let (__a0,) = (Elements::icon_text_component_2(icon.clone(), Some(text.clone())),); __this.lock().borrow_mut().add_element(__a0) };
    }

    /// java: default void addSimpleIconTranslatableText(FlexibleItemStack icon, @Translatable String translationKey, ChatFormatting formatting, Component content)
    pub fn add_simple_icon_translatable_text_flexible_item_stack_string_chat_formatting_component_body(__this: &compat::Shared<dyn ElementCollector>, icon: &compat::Shared<FlexibleItemStack>, translation_key: &str, formatting: &ChatFormatting, content: &MutableComponent) {
        let mut text: MutableComponent = simple_entry_translatable_text_string_component_chat_formatting(translation_key, content, formatting);
        { let (__a0,) = (Elements::icon_text_component_2(Some(icon.clone()), Some(text.clone())),); __this.lock().borrow_mut().add_element(__a0) };
    }

}

/// java: static @Nullable Component simpleEntryText(int idx, String entryName, ChatFormatting contentFmt)
pub fn simple_entry_text(mut idx: i32, entry_name: &str, content_fmt: &ChatFormatting) -> Option<MutableComponent> {
    let Some(mut src) = PlayerListManager::str_at(idx) else {
        return None;
    };
    let mut cidx: i32 = compat::index_of(&src, ':');
    if cidx == -1 {
        return None;
    }
    src = compat::substring(&src, (compat::index_of(&src, ':')).wrapping_add(1));
    return Some(simple_entry_text_string_string_chat_formatting(&src, entry_name, content_fmt));
}

/// java: static Component simpleEntryText(String entryContent, String entryName, ChatFormatting contentFmt)
pub fn simple_entry_text_string_string_chat_formatting(entry_content: &str, entry_name: &str, content_fmt: &ChatFormatting) -> MutableComponent {
    return component::literal(entry_name).append(component::literal(entry_content).with_style_format(*content_fmt));
}

/// java: static Component simpleEntryTranslatableText(String translationKey, String content, ChatFormatting contentFormatting)
pub fn simple_entry_translatable_text(translation_key: &str, content: &str, content_formatting: &ChatFormatting) -> MutableComponent {
    return component::translatable_args(translation_key, vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(content).with_style_format(*content_formatting))]);
}

/// java: static Component simpleEntryTranslatableText(String translationKey, Component content, ChatFormatting contentFormatting)
pub fn simple_entry_translatable_text_string_component_chat_formatting(translation_key: &str, content: &MutableComponent, content_formatting: &ChatFormatting) -> MutableComponent {
    return component::translatable_args(translation_key, vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(content.copy().with_style_format(*content_formatting))]);
}

/// java: class ElementCollection implements ElementCollector
#[derive(Clone)]
pub struct ElementCollection {
    /// java: private final List<Element> elements = new ArrayList<>();
    pub elements: Vec<compat::Shared<dyn Element>>,
}

impl ElementCollection {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            elements: Vec::new(),
        });
        __this
    }

    /// java: public List<Element> getElements()
    pub fn get_elements(__this: &compat::Shared<Self>) -> Vec<compat::Shared<dyn Element>> {
        return compat::read(__this, |__t| __t.elements.clone());
    }

}

impl ElementCollector for ElementCollection {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public <E extends Element> E addElement(E element)
    fn add_element(&mut self, mut element: compat::Shared<dyn Element>) -> compat::Shared<dyn Element> {
        self.elements.push(element.clone());
        return element;
    }

}
