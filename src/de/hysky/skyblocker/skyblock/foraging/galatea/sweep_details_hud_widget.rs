//! Ported from de.hysky.skyblocker.skyblock.foraging.galatea.SweepDetailsHudWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::foraging::galatea::sweep_details_listener;
use crate::de::hysky::skyblocker::skyblock::foraging::galatea::sweep_details_listener::SweepDetailsListener;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element_collector::ElementCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::elements::Elements;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::plain_text_element::PlainTextElement;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::Information;
use crate::de::hysky::skyblocker::utils::area::Hub;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::utils;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::item::items;

/// java: private static final Map<String, FlexibleItemStack> LOG_TO_ITEM = Map.of( "Fig", new FlexibleItemStack(Items.STRIPPED_SPRUCE_LOG), "Mangrove", new FlexibleItemStack(Items.MANGROVE_LOG), "Helix", new F
pub static LOG_TO_ITEM: compat::MainThread<std::collections::HashMap<String, compat::Shared<FlexibleItemStack>>> = compat::MainThread::new(|| std::collections::HashMap::from([("Fig".to_string(), FlexibleItemStack::new2(items::items().get("stripped_spruce_log"))), ("Mangrove".to_string(), FlexibleItemStack::new2(items::items().get("mangrove_log"))), ("Helix".to_string(), FlexibleItemStack::new2(items::items().get("stripped_mangrove_wood"))), ("Jungle".to_string(), FlexibleItemStack::new2(items::items().get("jungle_log"))), ("Acacia".to_string(), FlexibleItemStack::new2(items::items().get("acacia_log"))), ("Dark Oak".to_string(), FlexibleItemStack::new2(items::items().get("dark_oak_log"))), ("Spruce".to_string(), FlexibleItemStack::new2(items::items().get("spruce_log"))), ("Birch".to_string(), FlexibleItemStack::new2(items::items().get("birch_log"))), ("Oak".to_string(), FlexibleItemStack::new2(items::items().get("oak_log")))]));
/// java: public static final Set<Location> LOCATIONS = EnumSet.of(Location.GALATEA, Location.HUB, Location.THE_PARK, Location.GARDEN, Location.TORRHUS_CANYON);
pub static LOCATIONS: LazyLock<std::collections::HashSet<Location>> = LazyLock::new(|| std::collections::HashSet::from([Location::Galatea, Location::Hub, Location::ThePark, Location::Garden, Location::TorrhusCanyon]));
/// java: public static SweepDetailsHudWidget INSTANCE;
pub static INSTANCE: compat::MainThread<Option<compat::Shared<SweepDetailsHudWidget>>> = compat::MainThread::new(|| None);

/// java: @RegisterWidget public class SweepDetailsHudWidget extends ElementBasedWidget
#[derive(Clone)]
pub struct SweepDetailsHudWidget {
    /// java: extends ElementBasedWidget
    pub base: ElementBasedWidgetBase,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for SweepDetailsHudWidget {
    type Target = ElementBasedWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SweepDetailsHudWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SweepDetailsHudWidget {
    /// java: public SweepDetailsHudWidget()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = ElementBasedWidgetBase::new_in(component::translatable("skyblocker.galatea.hud.sweepDetails"), 0xFF6E37CCu32 as i32, Information::new2("sweep_details".to_string(), component::literal("Sweep Details"), LOCATIONS.clone()), move |__base| {
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

}

impl ElementBasedWidget for SweepDetailsHudWidget {
    fn element_based_widget_base(&self) -> &ElementBasedWidgetBase {
        &self.base
    }
    fn element_based_widget_base_mut(&mut self) -> &mut ElementBasedWidgetBase {
        &mut self.base
    }

    /// java: @Override public void updateContent() (dispatch: the body for this class)
    fn update_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) {
        SweepDetailsHudWidget::update_content_dyn
    }

    /// java: @Override protected void updateConfigContent(ElementCollector collector) (dispatch: the body for this class)
    fn update_config_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>, &compat::Shared<dyn ElementCollector>) {
        SweepDetailsHudWidget::update_config_content_dyn
    }

    /// java: @Override public boolean shouldUpdateBeforeRendering() (dispatch: the body for this class)
    fn should_update_before_rendering(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) -> bool {
        SweepDetailsHudWidget::should_update_before_rendering_dyn
    }

}

impl HudWidget for SweepDetailsHudWidget {
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
        SweepDetailsHudWidget::should_render_dyn
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

impl ElementCollector for SweepDetailsHudWidget {
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

impl SweepDetailsHudWidget {
    /// java: @Override public void updateContent()
    pub fn update_content_impl(__this: &compat::Shared<Self>) {
        if !(compat::get_state(&sweep_details_listener::ACTIVE)) || compat::millis() > (compat::get_state(&sweep_details_listener::LAST_MATCH)).wrapping_add(sweep_details_listener::TIMEOUT_MS) {
            SweepDetailsListener::reset_stats();
            let mut axe_icon: compat::Shared<FlexibleItemStack> = match Utils::get_location() {
                Location::Hub => ItemRepository::get_item_stack_2("SWEET_AXE", ico::IRON_AXE.get_mut()),
                Location::ThePark => ItemRepository::get_item_stack_2("TREECAPITATOR_AXE", ico::GOLDEN_AXE.get_mut()),
                Location::Galatea => ItemRepository::get_item_stack_2("FIGSTONE_AXE", ico::STONE_AXE.get_mut()),
                Location::TorrhusCanyon => ItemRepository::get_item_stack_2("HELIX_CHOPPER", ico::GOLDEN_AXE.get_mut()),
                _ => ico::RED_CONCRETE.get_mut().clone(),
            };
            { let (__a0,) = (Elements::icon_text_component_2(Some(axe_icon.clone()), Some(component::translatable("skyblocker.galatea.hud.sweepDetails.inactive"))),); __this.lock().borrow_mut().add_element(__a0) };
            return;
        }
        let mut log_item: compat::Shared<FlexibleItemStack> = LOG_TO_ITEM.get_mut().get(&compat::get_state(&sweep_details_listener::LAST_TREE_TYPE)).cloned().unwrap_or_else(|| ico::RED_CONCRETE.get_mut().clone());
        { let (__a0,) = (Elements::icon_text_component_2(Some(log_item.clone()), Some(component::translatable_args("skyblocker.galatea.hud.sweepDetails.treeType", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(compat::get_state(&sweep_details_listener::LAST_TREE_TYPE))]))),); __this.lock().borrow_mut().add_element(__a0) };
        { let (__a0,) = ((PlainTextElement::new(Some(component::translatable_args("skyblocker.galatea.hud.sweepDetails.toughness", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(compat::get_state(&sweep_details_listener::TOUGHNESS))]))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
        let mut sweep_amount: MutableComponent;
        if compat::get_state(&sweep_details_listener::MAX_SWEEP) > compat::get_state(&sweep_details_listener::LAST_SWEEP) {
            let mut last_sweep: MutableComponent = component::literal(&formatters::DOUBLE_NUMBERS.get_mut().format(compat::get_state(&sweep_details_listener::LAST_SWEEP) as f64)).with_color(mcrust::net::minecraft::util::common_colors::SOFT_RED);
            let mut this_sweep: MutableComponent = component::literal(&formatters::DOUBLE_NUMBERS.get_mut().format(compat::get_state(&sweep_details_listener::MAX_SWEEP) as f64)).with_color(mcrust::net::minecraft::util::common_colors::GREEN);
            sweep_amount = last_sweep.append(component::literal(" (").with_color(mcrust::net::minecraft::util::common_colors::WHITE)).append(this_sweep.clone()).append(component::literal(")").with_color(mcrust::net::minecraft::util::common_colors::WHITE));
        } else {
            sweep_amount = component::literal(&formatters::DOUBLE_NUMBERS.get_mut().format(compat::get_state(&sweep_details_listener::MAX_SWEEP) as f64)).with_color(mcrust::net::minecraft::util::common_colors::GREEN);
        }
        { let (__a0,) = ((PlainTextElement::new(Some(component::translatable_args("skyblocker.galatea.hud.sweepDetails.sweep", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(sweep_amount.clone())]))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
        { let (__a0,) = ((PlainTextElement::new(Some(component::translatable_args("skyblocker.galatea.hud.sweepDetails.logs", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&compat::get_state(&sweep_details_listener::LOGS)).with_color(mcrust::net::minecraft::util::common_colors::GREEN))]))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
        if compat::get_state(&sweep_details_listener::AXE_PENALTY) {
            { let (__a0,) = (Elements::icon_text_component_2(Some(ico::BARRIER.get_mut().clone()), Some(component::translatable_args("skyblocker.galatea.hud.sweepDetails.throwPenalty", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(format!("{}%", compat::fstr(compat::get_state(&sweep_details_listener::AXE_PENALTY_AMOUNT) as f64)))]))),); __this.lock().borrow_mut().add_element(__a0) };
        }
        if compat::get_state(&sweep_details_listener::STYLE_PENALTY) {
            { let (__a0,) = (Elements::icon_text_component_2(Some(ico::BARRIER.get_mut().clone()), Some(component::translatable_args("skyblocker.galatea.hud.sweepDetails.stylePenalty", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(format!("{}%", compat::fstr(compat::get_state(&sweep_details_listener::STYLE_PENALTY_AMOUNT) as f64)))]))),); __this.lock().borrow_mut().add_element(__a0) };
            { let (__a0,) = ((PlainTextElement::new(Some(component::translatable_args("skyblocker.galatea.hud.sweepDetails.correctStyle", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(compat::get_state(&sweep_details_listener::CORRECT_STYLE))]))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
        }
    }

    /// SweepDetailsHudWidget.updateContent as the dispatch of ElementBasedWidget.updateContent
    pub fn update_content_dyn(__this: &compat::Shared<dyn ElementBasedWidget>) {
        Self::update_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected void updateConfigContent(ElementCollector collector)
    pub fn update_config_content_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn ElementCollector>) {
        { let (__a0,) = (Elements::icon_text_component_2(Some(FlexibleItemStack::new2(items::items().get("stripped_spruce_log"))), Some(component::translatable_args("skyblocker.galatea.hud.sweepDetails.treeType", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of("Fig".to_string())]))),); collector.lock().borrow_mut().add_element(__a0) };
        { let (__a0,) = ((PlainTextElement::new(Some(component::translatable_args("skyblocker.galatea.hud.sweepDetails.toughness", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(compat::fstr(3.5 as f64))]))) as compat::Shared<dyn Element>),); collector.lock().borrow_mut().add_element(__a0) };
        { let (__a0,) = ((PlainTextElement::new(Some(component::translatable_args("skyblocker.galatea.hud.sweepDetails.sweep", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(compat::fstr(314.15 as f64))]))) as compat::Shared<dyn Element>),); collector.lock().borrow_mut().add_element(__a0) };
    }

    /// SweepDetailsHudWidget.updateConfigContent as the dispatch of ElementBasedWidget.updateConfigContent
    pub fn update_config_content_dyn(__this: &compat::Shared<dyn ElementBasedWidget>, __p0: &compat::Shared<dyn ElementCollector>) {
        Self::update_config_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean shouldUpdateBeforeRendering()
    pub fn should_update_before_rendering_impl(__this: &compat::Shared<Self>) -> bool {
        return true;
    }

    /// SweepDetailsHudWidget.shouldUpdateBeforeRendering as the dispatch of ElementBasedWidget.shouldUpdateBeforeRendering
    pub fn should_update_before_rendering_dyn(__this: &compat::Shared<dyn ElementBasedWidget>) -> bool {
        Self::should_update_before_rendering_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public boolean shouldRender()
    pub fn should_render_impl(__this: &compat::Shared<Self>) -> bool {
        return (!(Utils::get_location() == Location::Hub) || compat::read(&Utils::get_area(), |__t| compat::DynAny::dyn_any(__t).downcast_ref::<Hub>().is_some_and(|__e| *__e == Hub::Forest)) || compat::read(&Utils::get_area(), |__t| compat::DynAny::dyn_any(__t).downcast_ref::<Hub>().is_some_and(|__e| *__e == Hub::ForagingCamp)) && !(Utils::get_location() == Location::Garden) || compat::lock(&utils::STRING_SCOREBOARD).iter().cloned().any(|s| s.contains("Cleanup")));
    }

    /// SweepDetailsHudWidget.shouldRender as the dispatch of HudWidget.shouldRender
    pub fn should_render_dyn(__this: &compat::Shared<dyn HudWidget>) -> bool {
        Self::should_render_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
