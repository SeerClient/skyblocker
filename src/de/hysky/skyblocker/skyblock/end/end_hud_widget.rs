//! Ported from de.hysky.skyblocker.skyblock.end.EndHudWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::end::the_end;
use crate::de::hysky::skyblocker::skyblock::end::the_end::EndStats;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element_collector::ElementCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::elements::Elements;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::plain_text_element::PlainTextElement;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::Information;
use crate::de::hysky::skyblocker::utils::data::profiled_data::ProfiledData;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use crate::de::hysky::skyblocker::utils::location::Location;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::item::items;

/// java: private static final MutableComponent TITLE = Component.literal("The End").withStyle(ChatFormatting.LIGHT_PURPLE, ChatFormatting.BOLD);
pub static TITLE: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("The End").with_style_formats(&[ChatFormatting::LightPurple, ChatFormatting::Bold]));
/// java: private static @Nullable EndHudWidget instance = null;
pub static INSTANCE: compat::MainThread<Option<compat::Shared<EndHudWidget>>> = compat::MainThread::new(|| None);
// TODO(fabric2seer): `.set(..)` on a T
//   java: private static final FlexibleItemStack ENDERMAN_HEAD = Util.make(new FlexibleItemStack(Items.PLAYER_HEAD), stack -> stack.set(DataComponents.PROFILE, ResolvableProfile.createUnresolved("MHF_Enderman")));
/// java: private static final FlexibleItemStack ENDERMAN_HEAD = Util.make(new FlexibleItemStack(Items.PLAYER_HEAD), stack -> stack.set(DataComponents.PROFILE, ResolvableProfile.createUnresolved("MHF_Enderman")
pub static ENDERMAN_HEAD: compat::MainThread<compat::Shared<FlexibleItemStack>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `ENDERMAN_HEAD` - see the TODO above"));
// TODO(fabric2seer): `.set(..)` on a T
//   java: private static final FlexibleItemStack POPPY = Util.make(new FlexibleItemStack(Items.POPPY), stack -> stack.set(DataComponents.ENCHANTMENT_GLINT_OVERRIDE, true));
/// java: private static final FlexibleItemStack POPPY = Util.make(new FlexibleItemStack(Items.POPPY), stack -> stack.set(DataComponents.ENCHANTMENT_GLINT_OVERRIDE, true));
pub static POPPY: compat::MainThread<compat::Shared<FlexibleItemStack>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `POPPY` - see the TODO above"));

/// java: @RegisterWidget public class EndHudWidget extends ElementBasedWidget
#[derive(Clone)]
pub struct EndHudWidget {
    /// java: extends ElementBasedWidget
    pub base: ElementBasedWidgetBase,
    /// java: public boolean showZealotKills = true;
    pub show_zealot_kills: bool,
    /// java: private boolean showProtectorLocation = true;
    pub show_protector_location: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for EndHudWidget {
    type Target = ElementBasedWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for EndHudWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EndHudWidget {
    /// java: public EndHudWidget()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = ElementBasedWidgetBase::new_in(TITLE.get_mut().clone(), mcrust::net::minecraft::network::chat::text_color::TextColor::DARK_PURPLE.get_value(), Information::new3("hud_end".to_string(), component::literal("End Hud"), Location::TheEnd), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                self_ref: std::sync::Weak::new(),
                show_zealot_kills: true,
                show_protector_location: true,
            });
            compat::write(&__h, |__t| __t.self_ref = std::sync::Arc::downgrade(&__h));
            ((__h.clone() as compat::Shared<dyn ElementBasedWidget>), __h)
        });
        *INSTANCE.get_mut() = Some(__this.clone());
        ({ let __f = compat::dispatch(&__this, |__t| __t.update()); __f(&compat::view(&__this, |a| a as compat::Shared<dyn ElementBasedWidget>)) });
        __this
    }

    /// java: public static EndHudWidget getInstance()
    pub fn get_instance() -> compat::Shared<EndHudWidget> {
        return INSTANCE.get_mut().clone().unwrap();
    }

}

impl ElementBasedWidget for EndHudWidget {
    fn element_based_widget_base(&self) -> &ElementBasedWidgetBase {
        &self.base
    }
    fn element_based_widget_base_mut(&mut self) -> &mut ElementBasedWidgetBase {
        &mut self.base
    }

    /// java: @Override public void updateContent() (dispatch: the body for this class)
    fn update_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) {
        EndHudWidget::update_content_dyn
    }

}

impl HudWidget for EndHudWidget {
    fn hud_widget_base(&self) -> &HudWidgetBase {
        &self.base.base
    }
    fn hud_widget_base_mut(&mut self) -> &mut HudWidgetBase {
        &mut self.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector) (dispatch: the body for this class)
    fn get_option_widgets(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut OptionWidgetCollector) {
        EndHudWidget::get_option_widgets_dyn
    }

    /// java: @Override public void load(JsonValueInput input) (dispatch: the body for this class)
    fn load(&self) -> fn(&compat::Shared<dyn HudWidget>, &JsonValueInput) {
        EndHudWidget::load_dyn
    }

    /// java: @Override public void save(JsonValueOutput output) (dispatch: the body for this class)
    fn save(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut JsonValueOutput) {
        EndHudWidget::save_dyn
    }

    /// java: @Override protected final void extractWidgetRenderState(GuiGraphicsExtractor context, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        <dyn ElementBasedWidget>::extract_widget_render_state_from::<Self>
    }

    /// java: @Override protected void extractWidgetRenderStateForConfig(GuiGraphicsExtractor graphics, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state_for_config(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        <dyn ElementBasedWidget>::extract_widget_render_state_for_config_from::<Self>
    }

    /// java: @Override public void updateConfigPreview() (dispatch: the body for this class)
    fn update_config_preview(&self) -> fn(&compat::Shared<dyn HudWidget>) {
        <dyn ElementBasedWidget>::update_config_preview_from::<Self>
    }

}

impl ElementCollector for EndHudWidget {
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

impl EndHudWidget {
    /// java: @Override public void updateContent()
    pub fn update_content_impl(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.show_zealot_kills) {
            let mut end_stats: EndStats = ProfiledData::compute_if_absent(the_end::PROFILES_STATS.get_mut(), the_end::END_STATS_EMPTY.get_mut().clone());
            { let (__a0,) = (Elements::icon_text_component_2(Some(ENDERMAN_HEAD.get_mut().clone()), Some(component::literal("Zealots").with_style_format(ChatFormatting::Bold))),); __this.lock().borrow_mut().add_element(__a0) };
            { let (__a0,) = ((PlainTextElement::new(Some(component::translatable_args("skyblocker.end.hud.zealotsSinceLastEye", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(end_stats.zealots_since_last_eye().to_string())]))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
            { let (__a0,) = ((PlainTextElement::new(Some(component::translatable_args("skyblocker.end.hud.zealotsTotalKills", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(formatters::INTEGER_NUMBERS.get_mut().format((end_stats.total_zealot_kills()) as f64))]))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
            let mut avg: String = (if end_stats.eyes() == 0 { "???".to_string() } else { formatters::DOUBLE_NUMBERS.get_mut().format((((end_stats.total_zealot_kills()) as f32) / (end_stats.eyes() as f32))) });
            { let (__a0,) = ((PlainTextElement::new(Some(component::translatable_args("skyblocker.end.hud.avgKillsPerEye", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(avg.clone())]))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
        }
        if compat::read(__this, |__t| __t.show_protector_location) {
            { let (__a0,) = (Elements::icon_text_component_2(Some(POPPY.get_mut().clone()), Some(component::literal("End Stone Protector").with_style_format(ChatFormatting::Bold))),); __this.lock().borrow_mut().add_element(__a0) };
            if compat::get_state(&the_end::STAGE) == 5 {
                { let (__a0,) = ((PlainTextElement::new(Some(component::translatable_args("skyblocker.end.hud.stage", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of("IMMINENT".to_string())]))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
            } else {
                { let (__a0,) = ((PlainTextElement::new(Some(component::translatable_args("skyblocker.end.hud.stage", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(compat::get_state(&the_end::STAGE).to_string())]))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
            }
            if the_end::CURRENT_PROTECTOR_LOCATION.get_mut().is_none() {
                { let (__a0,) = ((PlainTextElement::new(Some(component::translatable_args("skyblocker.end.hud.location", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of("?".to_string())]))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
            } else {
                { let (__a0,) = ((PlainTextElement::new(Some(component::translatable_args("skyblocker.end.hud.location", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(the_end::CURRENT_PROTECTOR_LOCATION.get_mut().as_ref().unwrap().name())]))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
            }
        }
    }

    /// EndHudWidget.updateContent as the dispatch of ElementBasedWidget.updateContent
    pub fn update_content_dyn(__this: &compat::Shared<dyn ElementBasedWidget>) {
        Self::update_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector)
    pub fn get_option_widgets_impl(__this: &compat::Shared<Self>, collector: &mut OptionWidgetCollector) {
        <dyn ElementBasedWidget>::get_option_widgets_body(&compat::view(__this, |a| a as compat::Shared<dyn ElementBasedWidget>), collector);
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        collector.yes_no_button(&component::translatable("skyblocker.config.otherLocations.end.zealotKillsEnabled"), Arc::new(move |b| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { { let __v = b; compat::write((&__this_1), |__t| __t.show_zealot_kills = __v); } } }), compat::read(__this, |__t| __t.show_zealot_kills), Some(component::translatable("skyblocker.config.otherLocations.end.zealotKillsEnabled.@Tooltip")));
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        collector.yes_no_button_3(&component::translatable("skyblocker.config.otherLocations.end.protectorLocationEnable"), Arc::new(move |b| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; { { let __v = b; compat::write((&__this_2), |__t| __t.show_protector_location = __v); } } }), compat::read(__this, |__t| __t.show_protector_location));
    }

    /// EndHudWidget.getOptionWidgets as the dispatch of HudWidget.getOptionWidgets
    pub fn get_option_widgets_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &mut OptionWidgetCollector) {
        Self::get_option_widgets_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void load(JsonValueInput input)
    pub fn load_impl(__this: &compat::Shared<Self>, input: &JsonValueInput) {
        <dyn ElementBasedWidget>::load_body(&compat::view(__this, |a| a as compat::Shared<dyn ElementBasedWidget>), input);
        { let __v = input.read_boolean_or("zealot_kills", true); compat::write(__this, |__t| __t.show_zealot_kills = __v); }
        { let __v = input.read_boolean_or("protector_location", true); compat::write(__this, |__t| __t.show_protector_location = __v); }
    }

    /// EndHudWidget.load as the dispatch of HudWidget.load
    pub fn load_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &JsonValueInput) {
        Self::load_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void save(JsonValueOutput output)
    pub fn save_impl(__this: &compat::Shared<Self>, output: &mut JsonValueOutput) {
        <dyn ElementBasedWidget>::save_body(&compat::view(__this, |a| a as compat::Shared<dyn ElementBasedWidget>), output);
        output.write_bool("zealot_kills", compat::read(__this, |__t| __t.show_zealot_kills));
        output.write_bool("protector_location", compat::read(__this, |__t| __t.show_protector_location));
    }

    /// EndHudWidget.save as the dispatch of HudWidget.save
    pub fn save_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &mut JsonValueOutput) {
        Self::save_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
