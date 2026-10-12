//! Ported from de.hysky.skyblocker.skyblock.dwarven.PickobulusHudWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dwarven::pickobulus_helper::MiningDrop;
use crate::de::hysky::skyblocker::skyblock::dwarven::pickobulus_helper::PickobulusHelper;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element_collector::ElementCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::plain_text_element::PlainTextElement;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::Information;
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use crate::de::hysky::skyblocker::utils::location::Location;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final MutableComponent TITLE = Component.literal("Pickobulus").withStyle(ChatFormatting.BLUE, ChatFormatting.BOLD);
pub static TITLE: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("Pickobulus").with_style_formats(&[ChatFormatting::Blue, ChatFormatting::Bold]));
/// java: private static final Set<Location> AVAILABLE_LOCATIONS = EnumSet.of(Location.GOLD_MINE, Location.DEEP_CAVERNS, Location.DWARVEN_MINES, Location.CRYSTAL_HOLLOWS, Location.GLACITE_MINESHAFTS);
pub static AVAILABLE_LOCATIONS: LazyLock<std::collections::HashSet<Location>> = LazyLock::new(|| std::collections::HashSet::from([Location::GoldMine, Location::DeepCaverns, Location::DwarvenMines, Location::CrystalHollows, Location::GlaciteMineshafts]));
/// java: private static @Nullable PickobulusHudWidget instance;
pub static INSTANCE: compat::MainThread<Option<compat::Shared<PickobulusHudWidget>>> = compat::MainThread::new(|| None);

/// java: @RegisterWidget public class PickobulusHudWidget extends ElementBasedWidget
#[derive(Clone)]
pub struct PickobulusHudWidget {
    /// java: extends ElementBasedWidget
    pub base: ElementBasedWidgetBase,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for PickobulusHudWidget {
    type Target = ElementBasedWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PickobulusHudWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PickobulusHudWidget {
    /// java: public PickobulusHudWidget()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = ElementBasedWidgetBase::new_in(TITLE.get_mut().clone(), mcrust::net::minecraft::network::chat::text_color::TextColor::BLUE.get_value(), Information::new2("hud_pickobulus".to_string(), component::literal("Pickobulus HUD"), AVAILABLE_LOCATIONS.clone()), move |__base| {
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

    /// java: @SuppressWarnings("unused") public static PickobulusHudWidget getInstance()
    pub fn get_instance() -> compat::Shared<PickobulusHudWidget> {
        if INSTANCE.get_mut().is_none() {
            PickobulusHudWidget::new();
        }
        return INSTANCE.get_mut().clone().unwrap();
    }

}

impl ElementBasedWidget for PickobulusHudWidget {
    fn element_based_widget_base(&self) -> &ElementBasedWidgetBase {
        &self.base
    }
    fn element_based_widget_base_mut(&mut self) -> &mut ElementBasedWidgetBase {
        &mut self.base
    }

    /// java: @Override public boolean shouldUpdateBeforeRendering() (dispatch: the body for this class)
    fn should_update_before_rendering(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) -> bool {
        PickobulusHudWidget::should_update_before_rendering_dyn
    }

    /// java: @Override public void updateContent() (dispatch: the body for this class)
    fn update_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) {
        PickobulusHudWidget::update_content_dyn
    }

    /// java: @Override protected void updateConfigContent(ElementCollector collector) (dispatch: the body for this class)
    fn update_config_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>, &compat::Shared<dyn ElementCollector>) {
        PickobulusHudWidget::update_config_content_dyn
    }

}

impl HudWidget for PickobulusHudWidget {
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
        PickobulusHudWidget::should_render_dyn
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

impl ElementCollector for PickobulusHudWidget {
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

impl PickobulusHudWidget {
    /// java: @Override public boolean shouldUpdateBeforeRendering()
    pub fn should_update_before_rendering_impl(__this: &compat::Shared<Self>) -> bool {
        return true;
    }

    /// PickobulusHudWidget.shouldUpdateBeforeRendering as the dispatch of ElementBasedWidget.shouldUpdateBeforeRendering
    pub fn should_update_before_rendering_dyn(__this: &compat::Shared<dyn ElementBasedWidget>) -> bool {
        Self::should_update_before_rendering_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void updateContent()
    pub fn update_content_impl(__this: &compat::Shared<Self>) {
        let mut error_message: Option<MutableComponent> = PickobulusHelper::get_error_message();
        if let Some(error_message) = &error_message {
            { let (__a0,) = ((PlainTextElement::new(Some(error_message.clone())) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
            return;
        }
        { let (__a0,) = ((PlainTextElement::new(Some(component::literal(&format!("Total Blocks: {}", PickobulusHelper::get_total_blocks())))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
        let mut drops: Vec<i32> = PickobulusHelper::get_drops();
        for drop in MiningDrop::values() {
            let mut count: i32 = drops[(drop.ordinal()) as usize];
            if count > 0 {
                { let (__a0,) = ((PlainTextElement::new(Some(component::literal(&format!("{}: {}", drop.friendly_name(), count)))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
            }
        }
    }

    /// PickobulusHudWidget.updateContent as the dispatch of ElementBasedWidget.updateContent
    pub fn update_content_dyn(__this: &compat::Shared<dyn ElementBasedWidget>) {
        Self::update_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected void updateConfigContent(ElementCollector collector)
    pub fn update_config_content_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn ElementCollector>) {
        { let (__a0,) = ((PlainTextElement::new(Some(component::literal("Total Blocks: ???"))) as compat::Shared<dyn Element>),); collector.lock().borrow_mut().add_element(__a0) };
        { let (__a0,) = ((PlainTextElement::new(Some(component::literal("Drop 1: ???"))) as compat::Shared<dyn Element>),); collector.lock().borrow_mut().add_element(__a0) };
        { let (__a0,) = ((PlainTextElement::new(Some(component::literal("Drop 2: ???"))) as compat::Shared<dyn Element>),); collector.lock().borrow_mut().add_element(__a0) };
    }

    /// PickobulusHudWidget.updateConfigContent as the dispatch of ElementBasedWidget.updateConfigContent
    pub fn update_config_content_dyn(__this: &compat::Shared<dyn ElementBasedWidget>, __p0: &compat::Shared<dyn ElementCollector>) {
        Self::update_config_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean shouldRender()
    pub fn should_render_impl(__this: &compat::Shared<Self>) -> bool {
        return PickobulusHelper::should_render();
    }

    /// PickobulusHudWidget.shouldRender as the dispatch of HudWidget.shouldRender
    pub fn should_render_dyn(__this: &compat::Shared<dyn HudWidget>) -> bool {
        Self::should_render_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
