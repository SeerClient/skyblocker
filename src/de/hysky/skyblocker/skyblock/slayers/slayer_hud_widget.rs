//! Ported from de.hysky.skyblocker.skyblock.slayers.SlayerHudWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerQuest;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_tier::SlayerTier;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_type::SlayerType;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element_collector::ElementCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::elements::Elements;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::texture_text_element::TextureTextElement;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::Information;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use crate::de::hysky::skyblocker::utils::location::Location;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final Set<Location> AVAILABLE_LOCATIONS = EnumSet.of(Location.CRIMSON_ISLE, Location.HUB, Location.SPIDERS_DEN, Location.THE_END, Location.THE_PARK, Location.THE_RIFT);
pub static AVAILABLE_LOCATIONS: LazyLock<std::collections::HashSet<Location>> = LazyLock::new(|| std::collections::HashSet::from([Location::CrimsonIsle, Location::Hub, Location::SpidersDen, Location::TheEnd, Location::ThePark, Location::TheRift]));
/// java: private static final int TEXTURE_SIZE = 16;
pub const TEXTURE_SIZE: i32 = 16;
/// java: private static @Nullable SlayerHudWidget instance;
pub static INSTANCE: compat::MainThread<Option<compat::Shared<SlayerHudWidget>>> = compat::MainThread::new(|| None);

/// java: @RegisterWidget public class SlayerHudWidget extends ElementBasedWidget
#[derive(Clone)]
pub struct SlayerHudWidget {
    /// java: extends ElementBasedWidget
    pub base: ElementBasedWidgetBase,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for SlayerHudWidget {
    type Target = ElementBasedWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SlayerHudWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SlayerHudWidget {
    /// java: public SlayerHudWidget()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = ElementBasedWidgetBase::new_in(component::literal("Slayer").with_style_formats(&[ChatFormatting::DarkPurple, ChatFormatting::Bold]), mcrust::net::minecraft::network::chat::text_color::TextColor::DARK_PURPLE.get_value(), Information::new2("hud_slayer".to_string(), component::literal("Slayer HUD"), AVAILABLE_LOCATIONS.clone()), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                self_ref: std::sync::Weak::new(),
            });
            compat::write(&__h, |__t| __t.self_ref = std::sync::Arc::downgrade(&__h));
            ((__h.clone() as compat::Shared<dyn ElementBasedWidget>), __h)
        });
        *INSTANCE.get_mut() = Some(__this.clone());
        ({ let __f = compat::dispatch(&__this, |__t| __t.update()); __f(&compat::view(&__this, |a| a as compat::Shared<dyn ElementBasedWidget>)) });
        __this
    }

    /// java: public static SlayerHudWidget getInstance()
    pub fn get_instance() -> compat::Shared<SlayerHudWidget> {
        return INSTANCE.get_mut().clone().unwrap();
    }

}

impl ElementBasedWidget for SlayerHudWidget {
    fn element_based_widget_base(&self) -> &ElementBasedWidgetBase {
        &self.base
    }
    fn element_based_widget_base_mut(&mut self) -> &mut ElementBasedWidgetBase {
        &mut self.base
    }

    /// java: @Override public boolean shouldUpdateBeforeRendering() (dispatch: the body for this class)
    fn should_update_before_rendering(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) -> bool {
        SlayerHudWidget::should_update_before_rendering_dyn
    }

    /// java: @Override public void updateContent() (dispatch: the body for this class)
    fn update_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) {
        SlayerHudWidget::update_content_dyn
    }

    /// java: @Override protected void updateConfigContent(ElementCollector collector) (dispatch: the body for this class)
    fn update_config_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>, &compat::Shared<dyn ElementCollector>) {
        SlayerHudWidget::update_config_content_dyn
    }

}

impl HudWidget for SlayerHudWidget {
    fn hud_widget_base(&self) -> &HudWidgetBase {
        &self.base.base
    }
    fn hud_widget_base_mut(&mut self) -> &mut HudWidgetBase {
        &mut self.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean shouldRender() (dispatch: the body for this class)
    fn should_render(&self) -> fn(&compat::Shared<dyn HudWidget>) -> bool {
        SlayerHudWidget::should_render_dyn
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
        <dyn ElementBasedWidget>::load_from::<Self>
    }

    /// java: @Override public void save(JsonValueOutput output) (dispatch: the body for this class)
    fn save(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut JsonValueOutput) {
        <dyn ElementBasedWidget>::save_from::<Self>
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector) (dispatch: the body for this class)
    fn get_option_widgets(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut OptionWidgetCollector) {
        <dyn ElementBasedWidget>::get_option_widgets_from::<Self>
    }

    /// java: @Override public void updateConfigPreview() (dispatch: the body for this class)
    fn update_config_preview(&self) -> fn(&compat::Shared<dyn HudWidget>) {
        <dyn ElementBasedWidget>::update_config_preview_from::<Self>
    }

}

impl ElementCollector for SlayerHudWidget {
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

impl SlayerHudWidget {
    /// java: @Override public boolean shouldUpdateBeforeRendering()
    pub fn should_update_before_rendering_impl(__this: &compat::Shared<Self>) -> bool {
        return true;
    }

    /// SlayerHudWidget.shouldUpdateBeforeRendering as the dispatch of ElementBasedWidget.shouldUpdateBeforeRendering
    pub fn should_update_before_rendering_dyn(__this: &compat::Shared<dyn ElementBasedWidget>) -> bool {
        Self::should_update_before_rendering_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void updateContent()
    pub fn update_content_impl(__this: &compat::Shared<Self>) {
        let mut slayer_quest: Option<compat::Shared<SlayerQuest>> = SlayerManager::get_slayer_quest();
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || slayer_quest.is_none() {
            return;
        }
        let mut slayer_type: SlayerType = compat::read(slayer_quest.as_ref().unwrap(), |__t| __t.slayer_type);
        let mut slayer_tier: SlayerTier = compat::read(slayer_quest.as_ref().unwrap(), |__t| __t.slayer_tier);
        let mut level: i32 = compat::read(slayer_quest.as_ref().unwrap(), |__t| __t.level);
        let mut bosses_needed: i32 = compat::read(slayer_quest.as_ref().unwrap(), |__t| __t.bosses_needed);
        let mut slayer_name: MutableComponent = (component::literal(&format!("{} {}", slayer_type.fields().boss_name, compat::jstring(&slayer_tier)))).with_style_format(slayer_tier.fields().color);
        { let (__a0,) = ((TextureTextElement::new(slayer_name.clone(), slayer_type.fields().texture.clone(), TEXTURE_SIZE, TEXTURE_SIZE) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
        if level == slayer_type.fields().max_level {
            { let (__a0,) = (Elements::icon_text_component_2(Some(ico::EXPERIENCE_BOTTLE.get_mut().clone()), Some(component::literal("XP: ").append(component::translatable("skyblocker.slayer.hud.levelMaxed").with_style_format(ChatFormatting::Green)))),); __this.lock().borrow_mut().add_element(__a0) };
        } else if level >= 0 {
            let mut next_milestone: i32 = slayer_type.fields().level_milestones[(level) as usize];
            let mut current_xp: i32 = next_milestone.wrapping_sub(compat::read(slayer_quest.as_ref().unwrap(), |__t| __t.xp_remaining));
            ({ let __f = compat::dispatch(__this, |__t| __t.add_simple_ico_text_flexible_item_stack_string_chat_formatting_string()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ElementCollector>), Some(ico::EXPERIENCE_BOTTLE.get_mut().clone()), "XP: ", &ChatFormatting::LightPurple, &format!("{}/{}", formatters::INTEGER_NUMBERS.get_mut().format((current_xp) as f64), formatters::INTEGER_NUMBERS.get_mut().format((next_milestone) as f64))) });
            if bosses_needed > 0 {
                { let (__a0,) = (Elements::icon_text_component_2(Some(ico::NETHER_STAR.get_mut().clone()), Some(component::translatable_args("skyblocker.slayer.hud.levelUpIn", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from((component::literal(&formatters::INTEGER_NUMBERS.get_mut().format((bosses_needed) as f64))).with_style_format(ChatFormatting::LightPurple))]))),); __this.lock().borrow_mut().add_element(__a0) };
            }
        }
    }

    /// SlayerHudWidget.updateContent as the dispatch of ElementBasedWidget.updateContent
    pub fn update_content_dyn(__this: &compat::Shared<dyn ElementBasedWidget>) {
        Self::update_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected void updateConfigContent(ElementCollector collector)
    pub fn update_config_content_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn ElementCollector>) {
        let mut slayer_type: SlayerType = SlayerType::Revenant;
        let mut slayer_tier: SlayerTier = SlayerTier::V;
        let mut slayer_name: MutableComponent = (component::literal(&format!("{} {}", slayer_type.fields().boss_name, compat::jstring(&slayer_tier)))).with_style_format(slayer_tier.fields().color);
        { let (__a0,) = ((TextureTextElement::new(slayer_name.clone(), slayer_type.fields().texture.clone(), TEXTURE_SIZE, TEXTURE_SIZE) as compat::Shared<dyn Element>),); collector.lock().borrow_mut().add_element(__a0) };
        ({ let __f = compat::dispatch(collector, |__t| __t.add_simple_ico_text_flexible_item_stack_string_chat_formatting_string()); __f(collector, Some(ico::EXPERIENCE_BOTTLE.get_mut().clone()), "XP: ", &ChatFormatting::LightPurple, "100,000/400,000") });
        { let (__a0,) = (Elements::icon_text_component_2(Some(ico::NETHER_STAR.get_mut().clone()), Some(component::translatable_args("skyblocker.slayer.hud.levelUpIn", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal("200").with_style_format(ChatFormatting::LightPurple))]))),); collector.lock().borrow_mut().add_element(__a0) };
    }

    /// SlayerHudWidget.updateConfigContent as the dispatch of ElementBasedWidget.updateConfigContent
    pub fn update_config_content_dyn(__this: &compat::Shared<dyn ElementBasedWidget>, __p0: &compat::Shared<dyn ElementCollector>) {
        Self::update_config_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean shouldRender()
    pub fn should_render_impl(__this: &compat::Shared<Self>) -> bool {
        return SlayerManager::is_in_slayer_quest();
    }

    /// SlayerHudWidget.shouldRender as the dispatch of HudWidget.shouldRender
    pub fn should_render_dyn(__this: &compat::Shared<dyn HudWidget>) -> bool {
        Self::should_render_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
