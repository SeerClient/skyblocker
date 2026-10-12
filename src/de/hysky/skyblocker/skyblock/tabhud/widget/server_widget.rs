//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.ServerWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::Widget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element_collector::ElementCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::elements::Elements;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::plain_text_element::PlainTextElement;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::tab_hud_widget::TabHudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::tab_hud_widget::TabHudWidgetBase;
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final MutableComponent TITLE = Component.literal("Server Info").withStyle(ChatFormatting.DARK_AQUA, ChatFormatting.BOLD);
pub static TITLE: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("Server Info").with_style_formats(&[ChatFormatting::DarkAqua, ChatFormatting::Bold]));

/// java: @RegisterWidget public class ServerWidget extends TabHudWidget
#[derive(Clone)]
pub struct ServerWidget {
    /// java: extends TabHudWidget
    pub base: TabHudWidgetBase,
}

impl std::ops::Deref for ServerWidget {
    type Target = TabHudWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ServerWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ServerWidget {
    /// java: public ServerWidget()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = TabHudWidgetBase::new_3_in("Area".to_string(), TITLE.get_mut().clone(), mcrust::net::minecraft::network::chat::text_color::TextColor::DARK_AQUA.get_value(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn TabHudWidget>), __h)
        });
        __this
    }

}

impl TabHudWidget for ServerWidget {
    fn tab_hud_widget_base(&self) -> &TabHudWidgetBase {
        &self.base
    }
    fn tab_hud_widget_base_mut(&mut self) -> &mut TabHudWidgetBase {
        &mut self.base
    }

    /// java: @Override public void updateContent(PlayerListManager.Widget widget) (dispatch: the body for this class)
    fn update_content_1(&self) -> fn(&compat::Shared<dyn TabHudWidget>, &Widget) {
        ServerWidget::update_content_1_dyn
    }

}

impl ElementBasedWidget for ServerWidget {
    fn element_based_widget_base(&self) -> &ElementBasedWidgetBase {
        &self.base.base
    }
    fn element_based_widget_base_mut(&mut self) -> &mut ElementBasedWidgetBase {
        &mut self.base.base
    }

    /// java: @Override public void updateContent() (dispatch: the body for this class)
    fn update_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) {
        <dyn TabHudWidget>::update_content_from::<Self>
    }

    /// java: @Override protected final void updateConfigContent(ElementCollector collector) (dispatch: the body for this class)
    fn update_config_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>, &compat::Shared<dyn ElementCollector>) {
        <dyn TabHudWidget>::update_config_content_from::<Self>
    }

}

impl HudWidget for ServerWidget {
    fn hud_widget_base(&self) -> &HudWidgetBase {
        &self.base.base.base
    }
    fn hud_widget_base_mut(&mut self) -> &mut HudWidgetBase {
        &mut self.base.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected final void extractWidgetRenderState(GuiGraphicsExtractor context, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        <dyn ElementBasedWidget>::extract_widget_render_state_from::<Self>
    }

    /// java: @Override protected void extractWidgetRenderStateForConfig(GuiGraphicsExtractor graphics, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state_for_config(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        <dyn ElementBasedWidget>::extract_widget_render_state_for_config_from::<Self>
    }

    /// java: @Override public boolean shouldRender() (dispatch: the body for this class)
    fn should_render(&self) -> fn(&compat::Shared<dyn HudWidget>) -> bool {
        <dyn TabHudWidget>::should_render_from::<Self>
    }

    /// java: @Override public void load(JsonValueInput input) (dispatch: the body for this class)
    fn load(&self) -> fn(&compat::Shared<dyn HudWidget>, &JsonValueInput) {
        <dyn TabHudWidget>::load_from::<Self>
    }

    /// java: @Override public void save(JsonValueOutput output) (dispatch: the body for this class)
    fn save(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut JsonValueOutput) {
        <dyn TabHudWidget>::save_from::<Self>
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector) (dispatch: the body for this class)
    fn get_option_widgets(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut OptionWidgetCollector) {
        <dyn TabHudWidget>::get_option_widgets_from::<Self>
    }

    /// java: @Override public void updateConfigPreview() (dispatch: the body for this class)
    fn update_config_preview(&self) -> fn(&compat::Shared<dyn HudWidget>) {
        <dyn TabHudWidget>::update_config_preview_from::<Self>
    }

}

impl ElementCollector for ServerWidget {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: public <T extends Element> T addElement(T c)
    fn add_element(&mut self, mut c: compat::Shared<dyn Element>) -> compat::Shared<dyn Element> {
        ({ let __f = compat::dispatch(&c, |__t| __t.set_parent()); __f(&c, self.clone()) });
        self.elements.push(c.clone());
        return c;
    }

}

impl ServerWidget {
    /// java: @Override public void updateContent(PlayerListManager.Widget widget)
    pub fn update_content_1_impl(__this: &compat::Shared<Self>, widget: &Widget) {
        { let (__a0,) = (Elements::icon_text_component_2(Some(ico::MAP.get_mut().clone()), Some(component::literal("Area: ").append(widget.detail().copy().with_style_format(ChatFormatting::DarkAqua)))),); __this.lock().borrow_mut().add_element(__a0) };
        for text in widget.lines() {
            let mut string: String = text.get_string();
            if true && string.to_lowercase().contains("server") {
                ({ let __f = compat::dispatch(__this, |__t| __t.add_simple_ico_text_flexible_item_stack_string_chat_formatting_string()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ElementCollector>), Some(ico::NTAG.get_mut().clone()), "Server ID:", &ChatFormatting::Gray, &(compat::regex::split_limit(&string, ":", 2))[(1) as usize]) });
            } else if true && string.to_lowercase().contains("gems") {
                { let (__a0,) = (Elements::icon_text_component_2(Some(ico::EMERALD.get_mut().clone()), Some(text.clone())),); __this.lock().borrow_mut().add_element(__a0) };
            } else if true && string.to_lowercase().contains("crystals") {
                { let (__a0,) = (Elements::icon_text_component_2(Some(ico::EMERALD.get_mut().clone()), Some(text.clone())),); __this.lock().borrow_mut().add_element(__a0) };
            } else if true && string.to_lowercase().contains("copper") {
                { let (__a0,) = (Elements::icon_text_component_2(Some(ico::COPPER.get_mut().clone()), Some(text.clone())),); __this.lock().borrow_mut().add_element(__a0) };
            } else if true && string.to_lowercase().contains("garden") {
                { let (__a0,) = (Elements::icon_text_component_2(Some(ico::EXPERIENCE_BOTTLE.get_mut().clone()), Some(text.clone())),); __this.lock().borrow_mut().add_element(__a0) };
            } else if true && string.to_lowercase().contains("fairy") {
                { let (__a0,) = (Elements::icon_text_component_2(Some(ItemRepository::get_item_stack_2("PLACEABLE_FAIRY_SOUL_RIFT", ico::FAIRY_SOUL.get_mut())), Some(text.clone())),); __this.lock().borrow_mut().add_element(__a0) };
            } else if true && string.to_lowercase().contains("rain") {
                { let (__a0,) = (Elements::icon_text_component_2(Some(ico::WATER.get_mut().clone()), Some(text.clone())),); __this.lock().borrow_mut().add_element(__a0) };
            } else if true && string.to_lowercase().contains("brood") {
                { let (__a0,) = (Elements::icon_text_component_2(Some(ico::SPIDER_EYE.get_mut().clone()), Some(text.clone())),); __this.lock().borrow_mut().add_element(__a0) };
            } else {
                { let (__a0,) = ((PlainTextElement::new(Some(text.clone())) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
            }
        }
    }

    /// ServerWidget.updateContent as the dispatch of TabHudWidget.updateContent
    pub fn update_content_1_dyn(__this: &compat::Shared<dyn TabHudWidget>, __p0: &Widget) {
        Self::update_content_1_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
