//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.CropMilestonesWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
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
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::jacobs_contest_widget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::tab_hud_widget::TabHudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::tab_hud_widget::TabHudWidgetBase;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use crate::de::hysky::skyblocker::utils::location::Location;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern PATTERN = Pattern.compile("(?<crop>[a-zA-Z ]+) (?<level>\\d+): (?<percentage>[0-9.]+%|MAX)");
pub static PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(?<crop>[a-zA-Z ]+) (?<level>\\d+): (?<percentage>[0-9.]+%|MAX)"));

/// java: @RegisterWidget public class CropMilestonesWidget extends TabHudWidget
#[derive(Clone)]
pub struct CropMilestonesWidget {
    /// java: extends TabHudWidget
    pub base: TabHudWidgetBase,
    /// java: private boolean showIcon;
    pub show_icon: bool,
    /// java: private boolean showProgressBar;
    pub show_progress_bar: bool,
}

impl std::ops::Deref for CropMilestonesWidget {
    type Target = TabHudWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CropMilestonesWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CropMilestonesWidget {
    /// java: public CropMilestonesWidget()
    pub fn new() -> compat::Shared<Self> {
        // NOTE(fabric2seer): `showIcon` is read by `updateContent`, which TabHudWidget's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `showProgressBar` is read by `updateContent`, which TabHudWidget's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        let (_, __this) = TabHudWidgetBase::new3_in("Crop Milestones".to_string(), component::literal("Crop Milestones"), mcrust::net::minecraft::network::chat::text_color::TextColor::AQUA.get_value(), Location::Garden, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                show_icon: false,
                show_progress_bar: false,
            });
            ((__h.clone() as compat::Shared<dyn TabHudWidget>), __h)
        });
        { let __v = true; compat::write(&__this, |__t| __t.hide_if_missing_by_default = __v); }
        __this
    }

}

impl TabHudWidget for CropMilestonesWidget {
    fn tab_hud_widget_base(&self) -> &TabHudWidgetBase {
        &self.base
    }
    fn tab_hud_widget_base_mut(&mut self) -> &mut TabHudWidgetBase {
        &mut self.base
    }

    /// java: @Override protected void updateContent(PlayerListManager.Widget widget) (dispatch: the body for this class)
    fn update_content_1(&self) -> fn(&compat::Shared<dyn TabHudWidget>, &Widget) {
        CropMilestonesWidget::update_content_1_dyn
    }

    /// java: @Override protected void updateConfigContentTab(ElementCollector collector) (dispatch: the body for this class)
    fn update_config_content_tab(&self) -> fn(&compat::Shared<dyn TabHudWidget>, &compat::Shared<dyn ElementCollector>) {
        CropMilestonesWidget::update_config_content_tab_dyn
    }

}

impl ElementBasedWidget for CropMilestonesWidget {
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

impl HudWidget for CropMilestonesWidget {
    fn hud_widget_base(&self) -> &HudWidgetBase {
        &self.base.base.base
    }
    fn hud_widget_base_mut(&mut self) -> &mut HudWidgetBase {
        &mut self.base.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector) (dispatch: the body for this class)
    fn get_option_widgets(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut OptionWidgetCollector) {
        CropMilestonesWidget::get_option_widgets_dyn
    }

    /// java: @Override public void load(JsonValueInput input) (dispatch: the body for this class)
    fn load(&self) -> fn(&compat::Shared<dyn HudWidget>, &JsonValueInput) {
        CropMilestonesWidget::load_dyn
    }

    /// java: @Override public void save(JsonValueOutput output) (dispatch: the body for this class)
    fn save(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut JsonValueOutput) {
        CropMilestonesWidget::save_dyn
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

    /// java: @Override public void updateConfigPreview() (dispatch: the body for this class)
    fn update_config_preview(&self) -> fn(&compat::Shared<dyn HudWidget>) {
        <dyn TabHudWidget>::update_config_preview_from::<Self>
    }

}

impl ElementCollector for CropMilestonesWidget {
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

impl CropMilestonesWidget {
    /// java: @Override protected void updateContent(PlayerListManager.Widget widget)
    pub fn update_content_1_impl(__this: &compat::Shared<Self>, widget: &Widget) {
        let line = &widget.lines()[0];
        let mut matcher: compat::regex::Matcher = PATTERN.matcher(&line.get_string());
        if !matcher.find() {
            { let (__a0,) = ((PlainTextElement::new(Some(line.clone())) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
            return;
        }
        let mut item_name: String = compat::trim(&matcher.group_named("crop").unwrap());
        let mut level: i32 = matcher.group_named("level").unwrap().parse::<i32>().expect("NumberFormatException");
        let mut progress: String = compat::trim(&matcher.group_named("percentage").unwrap());
        let mut icon: Option<compat::Shared<FlexibleItemStack>> = (if compat::read(__this, |__t| __t.show_icon) { jacobs_contest_widget::FARM_DATA.get_mut().get(&item_name).cloned() } else { None });
        if progress == "MAX" || !(compat::read(__this, |__t| __t.show_progress_bar)) {
            if let Some(icon) = &icon {
                { let (__a0,) = (Elements::icon_text_component_2(Some(icon.clone()), Some(line.clone())),); __this.lock().borrow_mut().add_element(__a0) };
            } else {
                { let (__a0,) = ((PlainTextElement::new(Some(line.clone())) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
            }
        } else {
            let mut percentage: f32 = (compat::substring2(&progress, 0, (compat::str_len(&progress)).wrapping_sub(1))).trim().parse::<f32>().expect("NumberFormatException");
            { let (__a0,) = (Elements::progress_component_3(icon.clone(), &component::literal(&format!("{} {}", item_name, level)), percentage),); __this.lock().borrow_mut().add_element(__a0) };
        }
    }

    /// CropMilestonesWidget.updateContent as the dispatch of TabHudWidget.updateContent
    pub fn update_content_1_dyn(__this: &compat::Shared<dyn TabHudWidget>, __p0: &Widget) {
        Self::update_content_1_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override protected void updateConfigContentTab(ElementCollector collector)
    pub fn update_config_content_tab_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn ElementCollector>) {
        if !(compat::read(__this, |__t| __t.show_progress_bar)) {
            let mut text: MutableComponent = component::literal("Wheat 32: ").append((component::literal("33.3%")).with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::GREEN));
            if compat::read(__this, |__t| __t.show_icon) {
                { let (__a0,) = (Elements::icon_text_component_2(Some(ico::WHEAT.get_mut().clone()), Some(text.clone())),); collector.lock().borrow_mut().add_element(__a0) };
            } else {
                { let (__a0,) = ((PlainTextElement::new(Some(text.clone())) as compat::Shared<dyn Element>),); collector.lock().borrow_mut().add_element(__a0) };
            }
        } else {
            let __r0 = compat::read(__this, |__t| __t.show_icon);
            { let (__a0,) = (Elements::progress_component_flexible_item_stack_component_float_int((if __r0 { Some(ico::WHEAT.get_mut().clone()) } else { None }), &component::literal("Wheat 32"), 33.3f32, mcrust::net::minecraft::network::chat::text_color::TextColor::DARK_AQUA.get_value()),); collector.lock().borrow_mut().add_element(__a0) };
        }
    }

    /// CropMilestonesWidget.updateConfigContentTab as the dispatch of TabHudWidget.updateConfigContentTab
    pub fn update_config_content_tab_dyn(__this: &compat::Shared<dyn TabHudWidget>, __p0: &compat::Shared<dyn ElementCollector>) {
        Self::update_config_content_tab_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector)
    pub fn get_option_widgets_impl(__this: &compat::Shared<Self>, collector: &mut OptionWidgetCollector) {
        <dyn TabHudWidget>::get_option_widgets_body(&compat::view(__this, |a| a as compat::Shared<dyn TabHudWidget>), collector);
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        collector.yes_no_button_3(&component::translatable("skyblocker.config.hud.cropMilestones.showIcon"), Arc::new(move |b| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { { let __v = b; compat::write((&__this_1), |__t| __t.show_icon = __v); } } }), compat::read(__this, |__t| __t.show_icon));
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        collector.yes_no_button_3(&component::translatable("skyblocker.config.hud.cropMilestones.showProgressBar"), Arc::new(move |b| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; { { let __v = b; compat::write((&__this_2), |__t| __t.show_progress_bar = __v); } } }), compat::read(__this, |__t| __t.show_progress_bar));
    }

    /// CropMilestonesWidget.getOptionWidgets as the dispatch of HudWidget.getOptionWidgets
    pub fn get_option_widgets_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &mut OptionWidgetCollector) {
        Self::get_option_widgets_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void load(JsonValueInput input)
    pub fn load_impl(__this: &compat::Shared<Self>, input: &JsonValueInput) {
        <dyn TabHudWidget>::load_body(&compat::view(__this, |a| a as compat::Shared<dyn TabHudWidget>), input);
        { let __v = input.read_boolean_or("icon", true); compat::write(__this, |__t| __t.show_icon = __v); }
        { let __v = input.read_boolean_or("progress_bar", true); compat::write(__this, |__t| __t.show_progress_bar = __v); }
    }

    /// CropMilestonesWidget.load as the dispatch of HudWidget.load
    pub fn load_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &JsonValueInput) {
        Self::load_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void save(JsonValueOutput output)
    pub fn save_impl(__this: &compat::Shared<Self>, output: &mut JsonValueOutput) {
        <dyn TabHudWidget>::save_body(&compat::view(__this, |a| a as compat::Shared<dyn TabHudWidget>), output);
        output.write_bool("icon", compat::read(__this, |__t| __t.show_icon));
        output.write_bool("progress_bar", compat::read(__this, |__t| __t.show_progress_bar));
    }

    /// CropMilestonesWidget.save as the dispatch of HudWidget.save
    pub fn save_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &mut JsonValueOutput) {
        Self::save_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
