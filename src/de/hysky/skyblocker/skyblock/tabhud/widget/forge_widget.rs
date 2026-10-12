//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.ForgeWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::Widget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element_collector::ElementCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::elements::Elements;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::tab_hud_widget::TabHudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::tab_hud_widget::TabHudWidgetBase;
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final MutableComponent TITLE = Component.literal("Forges").withStyle(ChatFormatting.DARK_AQUA, ChatFormatting.BOLD);
pub static TITLE: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("Forges").with_style_formats(&[ChatFormatting::DarkAqua, ChatFormatting::Bold]));

/// java: @RegisterWidget public class ForgeWidget extends TabHudWidget
#[derive(Clone)]
pub struct ForgeWidget {
    /// java: extends TabHudWidget
    pub base: TabHudWidgetBase,
    /// java: private boolean render;
    pub render: bool,
}

impl std::ops::Deref for ForgeWidget {
    type Target = TabHudWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ForgeWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ForgeWidget {
    /// java: public ForgeWidget()
    pub fn new() -> compat::Shared<Self> {
        // NOTE(fabric2seer): `render` is read by `updateContent`, which TabHudWidget's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        let (_, __this) = TabHudWidgetBase::new_3_in("Forges".to_string(), TITLE.get_mut().clone(), mcrust::net::minecraft::network::chat::text_color::TextColor::DARK_AQUA.get_value(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                render: false,
            });
            ((__h.clone() as compat::Shared<dyn TabHudWidget>), __h)
        });
        __this
    }

}

impl TabHudWidget for ForgeWidget {
    fn tab_hud_widget_base(&self) -> &TabHudWidgetBase {
        &self.base
    }
    fn tab_hud_widget_base_mut(&mut self) -> &mut TabHudWidgetBase {
        &mut self.base
    }

    /// java: @Override public void updateContent(PlayerListManager.Widget widget) (dispatch: the body for this class)
    fn update_content_1(&self) -> fn(&compat::Shared<dyn TabHudWidget>, &Widget) {
        ForgeWidget::update_content_1_dyn
    }

    /// java: @Override protected void updateConfigContentTab(ElementCollector collector) (dispatch: the body for this class)
    fn update_config_content_tab(&self) -> fn(&compat::Shared<dyn TabHudWidget>, &compat::Shared<dyn ElementCollector>) {
        ForgeWidget::update_config_content_tab_dyn
    }

}

impl ElementBasedWidget for ForgeWidget {
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

impl HudWidget for ForgeWidget {
    fn hud_widget_base(&self) -> &HudWidgetBase {
        &self.base.base.base
    }
    fn hud_widget_base_mut(&mut self) -> &mut HudWidgetBase {
        &mut self.base.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean shouldRender() (dispatch: the body for this class)
    fn should_render(&self) -> fn(&compat::Shared<dyn HudWidget>) -> bool {
        ForgeWidget::should_render_dyn
    }

    /// java: @Override protected final void extractWidgetRenderState(GuiGraphicsExtractor context, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        <dyn ElementBasedWidget>::extract_widget_render_state_from::<Self>
    }

    /// java: @Override protected void extractWidgetRenderStateForConfig(GuiGraphicsExtractor graphics, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state_for_config(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        <dyn ElementBasedWidget>::extract_widget_render_state_for_config_from::<Self>
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

impl ElementCollector for ForgeWidget {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: public <T extends Element> T addElement(T c)
    fn add_element(&mut self, mut c: compat::Shared<dyn Element>) -> compat::Shared<dyn Element> {
        ({ let __f = compat::dispatch(&c, |__t| __t.set_parent()); __f(&c, self.hud_widget_base().as_parent()) });
        self.elements.push(c.clone());
        return c;
    }

}

impl ForgeWidget {
    /// java: @Override public void updateContent(PlayerListManager.Widget widget)
    pub fn update_content_1_impl(__this: &compat::Shared<Self>, widget: &Widget) {
        let mut lines: Vec<MutableComponent> = widget.lines();
        { let __v = !lines.is_empty(); compat::write(__this, |__t| __t.render = __v); }
        {
            let mut i: i32 = 0;
            let mut slot: i32 = 1;
            while i < (lines.len() as i32) {
                let mut trim: String = compat::trim(&((lines[(i) as usize]).get_string()));
                let mut c: compat::Shared<dyn Element>;
                let mut l1: MutableComponent;
                let mut l2: MutableComponent;
                match (compat::substring(&trim, 3)).as_str() {
                    "LOCKED" => {
                        l1 = component::literal("Locked").with_style_format(ChatFormatting::Red);
                        l2 = match slot {
                                3 => component::literal("Needs HotM 3").with_style_format(ChatFormatting::Gray),
                                4 => component::literal("Needs HotM 4").with_style_format(ChatFormatting::Gray),
                                5 => component::literal("Needs HotM 5").with_style_format(ChatFormatting::Gray),
                                6 => component::literal("Needs HotM 6").with_style_format(ChatFormatting::Gray),
                                7 => component::literal("Needs HotM 7").with_style_format(ChatFormatting::Gray),
                                _ => component::literal("This message should not appear").with_style_formats(&[ChatFormatting::Red, ChatFormatting::Bold]),
                            };
                        c = Elements::icon_fat_text_component_3(Some(ico::BARRIER.get_mut().clone()), Some(l1.clone()), Some(l2.clone()));
                    }
                    "EMPTY" => {
                        l1 = component::literal("Empty").with_style_format(ChatFormatting::Gray);
                        c = Elements::icon_text_component_2(Some(ico::FURNACE.get_mut().clone()), Some(l1.clone()));
                    }
                    _ => {
                        let mut parts: Vec<String> = compat::regex::split(&trim, ": ");
                        if (parts.len() as i32) != 2 {
                            c = Elements::icon_fat_text_component();
                        } else {
                            l1 = (component::literal(&compat::substring(&(parts[(0) as usize]), 3))).with_style_format(ChatFormatting::Yellow);
                            if parts[(1) as usize] == "Ready!" {
                                l2 = (component::literal("Done!")).with_style_format(ChatFormatting::Green);
                            } else {
                                l2 = component::literal("Done in: ").with_style_format(ChatFormatting::Gray).append((component::literal(&parts[(1) as usize])).with_style_format(ChatFormatting::White));
                            }
                            c = Elements::icon_fat_text_component_3(Some(ico::CAMPFIRE.get_mut().clone()), Some(l1.clone()), Some(l2.clone()));
                        }
                    }
                }
                { let (__a0,) = (c.clone(),); __this.lock().borrow_mut().add_element(__a0) };
                i += 1;
                slot += 1;
            }
        }
    }

    /// ForgeWidget.updateContent as the dispatch of TabHudWidget.updateContent
    pub fn update_content_1_dyn(__this: &compat::Shared<dyn TabHudWidget>, __p0: &Widget) {
        Self::update_content_1_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override protected void updateConfigContentTab(ElementCollector collector)
    pub fn update_config_content_tab_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn ElementCollector>) {
        { let (__a0,) = (Elements::icon_fat_text_component_3(Some(ico::CAMPFIRE.get_mut().clone()), Some(component::literal("Thing 1").with_style_format(ChatFormatting::Yellow)), Some((component::literal("Done!")).with_style_format(ChatFormatting::Green))),); collector.lock().borrow_mut().add_element(__a0) };
        { let (__a0,) = (Elements::icon_fat_text_component_3(Some(ico::CAMPFIRE.get_mut().clone()), Some(component::literal("Thing 2").with_style_format(ChatFormatting::Yellow)), Some((component::literal("Done!")).with_style_format(ChatFormatting::Green))),); collector.lock().borrow_mut().add_element(__a0) };
        { let (__a0,) = (Elements::icon_fat_text_component_3(Some(ico::CAMPFIRE.get_mut().clone()), Some(component::literal("Thing 3").with_style_format(ChatFormatting::Yellow)), Some(component::literal("Done in: ").with_style_format(ChatFormatting::Gray).append((component::literal("???")).with_style_format(ChatFormatting::White)))),); collector.lock().borrow_mut().add_element(__a0) };
        { let (__a0,) = (Elements::icon_fat_text_component_3(Some(ico::CAMPFIRE.get_mut().clone()), Some(component::literal("LOCKED").with_style_format(ChatFormatting::Red)), Some((component::literal("Requires ???")).with_style_format(ChatFormatting::Gray))),); collector.lock().borrow_mut().add_element(__a0) };
        { let (__a0,) = (Elements::icon_fat_text_component_3(Some(ico::CAMPFIRE.get_mut().clone()), Some(component::literal("LOCKED").with_style_format(ChatFormatting::Red)), Some((component::literal("Requires ???")).with_style_format(ChatFormatting::Gray))),); collector.lock().borrow_mut().add_element(__a0) };
    }

    /// ForgeWidget.updateConfigContentTab as the dispatch of TabHudWidget.updateConfigContentTab
    pub fn update_config_content_tab_dyn(__this: &compat::Shared<dyn TabHudWidget>, __p0: &compat::Shared<dyn ElementCollector>) {
        Self::update_config_content_tab_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean shouldRender()
    pub fn should_render_impl(__this: &compat::Shared<Self>) -> bool {
        return (compat::read(__this, |__t| __t.render) && <dyn TabHudWidget>::should_render_body(&compat::view(__this, |a| a as compat::Shared<dyn TabHudWidget>)));
    }

    /// ForgeWidget.shouldRender as the dispatch of HudWidget.shouldRender
    pub fn should_render_dyn(__this: &compat::Shared<dyn HudWidget>) -> bool {
        Self::should_render_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
