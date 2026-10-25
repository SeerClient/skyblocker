//! Ported from de.hysky.skyblocker.skyblock.fishing.FishingHudWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::helper_config::FishingHookDisplay;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::fishing::fishing_helper;
use crate::de::hysky::skyblocker::skyblock::fishing::fishing_hook_display_helper;
use crate::de::hysky::skyblocker::skyblock::fishing::sea_creature_tracker;
use crate::de::hysky::skyblocker::skyblock::fishing::sea_creature_tracker::SeaCreatureTracker;
use crate::de::hysky::skyblocker::skyblock::item::pet_info::PetInfo;
use crate::de::hysky::skyblocker::skyblock::pet_cache::PetCache;
use crate::de::hysky::skyblocker::skyblock::profileviewer::utils::level_finder::LevelFinder;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element_collector::ElementCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::elements::Elements;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::Information;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::time::skyblock_time::SkyblockTime;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::cycle_button;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::entity::Entity as _;

/// java: private static final Vec3 BARN_LOCATION = new Vec3(108, 89, -252);
pub static BARN_LOCATION: compat::MainThread<Vec3> = compat::MainThread::new(|| Vec3::new((108) as f64, (89) as f64, (-252) as f64));
/// java: private static @Nullable FishingHudWidget instance;
pub static INSTANCE: compat::MainThread<Option<compat::Shared<FishingHudWidget>>> = compat::MainThread::new(|| None);

/// java: @RegisterWidget public class FishingHudWidget extends ElementBasedWidget
#[derive(Clone)]
pub struct FishingHudWidget {
    /// java: extends ElementBasedWidget
    pub base: ElementBasedWidgetBase,
    /// java: private boolean showCreatureCounter = true;
    pub show_creature_counter: bool,
    /// java: private boolean showFishingTimer = true;
    pub show_fishing_timer: bool,
    /// java: private boolean onlyShowHudInBarn = true;
    pub only_show_hud_in_barn: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for FishingHudWidget {
    type Target = ElementBasedWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for FishingHudWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FishingHudWidget {
    /// java: public FishingHudWidget()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = ElementBasedWidgetBase::new_in(component::literal("Fishing").with_style_formats(&[ChatFormatting::DarkAqua, ChatFormatting::Bold]), mcrust::net::minecraft::network::chat::text_color::TextColor::DARK_AQUA.get_value(), Information::new(Some("hud_fishing".to_string()), Some(component::literal("Fishing HUD"))), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                self_ref: std::sync::Weak::new(),
                show_creature_counter: true,
                show_fishing_timer: true,
                only_show_hud_in_barn: true,
            });
            compat::write(&__h, |__t| __t.self_ref = std::sync::Arc::downgrade(&__h));
            ((__h.clone() as compat::Shared<dyn ElementBasedWidget>), __h)
        });
        *INSTANCE.get_mut() = Some(__this.clone());
        __this
    }

    /// java: public static FishingHudWidget getInstance()
    pub fn get_instance() -> compat::Shared<FishingHudWidget> {
        return INSTANCE.get_mut().clone().unwrap();
    }

    /// java: private static boolean isBarnFishing()
    pub fn is_barn_fishing() -> bool {
        return (mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.distance_to_sqr_vec(BARN_LOCATION.get_mut())).unwrap() < (2500 as f64));
    }

}

impl ElementBasedWidget for FishingHudWidget {
    fn element_based_widget_base(&self) -> &ElementBasedWidgetBase {
        &self.base
    }
    fn element_based_widget_base_mut(&mut self) -> &mut ElementBasedWidgetBase {
        &mut self.base
    }

    /// java: @Override public boolean shouldUpdateBeforeRendering() (dispatch: the body for this class)
    fn should_update_before_rendering(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) -> bool {
        FishingHudWidget::should_update_before_rendering_dyn
    }

    /// java: @Override public void updateContent() (dispatch: the body for this class)
    fn update_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) {
        FishingHudWidget::update_content_dyn
    }

    /// java: @Override protected void updateConfigContent(ElementCollector collector) (dispatch: the body for this class)
    fn update_config_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>, &compat::Shared<dyn ElementCollector>) {
        FishingHudWidget::update_config_content_dyn
    }

}

impl HudWidget for FishingHudWidget {
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
        FishingHudWidget::should_render_dyn
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector) (dispatch: the body for this class)
    fn get_option_widgets(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut OptionWidgetCollector) {
        FishingHudWidget::get_option_widgets_dyn
    }

    /// java: @Override public void load(JsonValueInput input) (dispatch: the body for this class)
    fn load(&self) -> fn(&compat::Shared<dyn HudWidget>, &JsonValueInput) {
        FishingHudWidget::load_dyn
    }

    /// java: @Override public void save(JsonValueOutput output) (dispatch: the body for this class)
    fn save(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut JsonValueOutput) {
        FishingHudWidget::save_dyn
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

impl ElementCollector for FishingHudWidget {
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

impl FishingHudWidget {
    /// java: @Override public boolean shouldUpdateBeforeRendering()
    pub fn should_update_before_rendering_impl(__this: &compat::Shared<Self>) -> bool {
        return true;
    }

    /// FishingHudWidget.shouldUpdateBeforeRendering as the dispatch of ElementBasedWidget.shouldUpdateBeforeRendering
    pub fn should_update_before_rendering_dyn(__this: &compat::Shared<dyn ElementBasedWidget>) -> bool {
        Self::should_update_before_rendering_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void updateContent()
    pub fn update_content_impl(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.show_creature_counter) && SeaCreatureTracker::is_creatures_alive() {
            let mut timer: (MutableComponent, f32) = SeaCreatureTracker::get_timer_text(SeaCreatureTracker::get_oldest_sea_creature_age());
            let mut sea_creature_cap: i32 = sea_creature_tracker::SEA_CREATURE_CAP;
            let mut sea_creature_percent: f32 = ((((SeaCreatureTracker::sea_creature_count()) as f32) / (sea_creature_cap as f32)) * (100 as f32));
            { let (__a0,) = (Elements::progress_component_5(Some(ico::TROPICAL_FISH_BUCKET.get_mut().clone()), &component::null_to_empty(Some("Alive Creatures")), &component::null_to_empty(Some(&format!("{}/{}", SeaCreatureTracker::sea_creature_count(), sea_creature_cap))), sea_creature_percent, ColorUtils::percent_to_color(((100 as f32) - sea_creature_percent))),); __this.lock().borrow_mut().add_element(__a0) };
            // java: addElement(Elements.progressComponent(Ico.CLOCK, Component.nullToEmpty("Time Left"), timer.left(), timer.rightFloat()));
            { let (__a0,) = (Elements::progress_component_flexible_item_stack_component_component_float(Some(ico::CLOCK.get_mut().clone()), &component::null_to_empty(Some("Time Left")), &timer.0, timer.1),); __this.lock().borrow_mut().add_element(__a0) };
        }
        if compat::read(__this, |__t| __t.show_fishing_timer) && compat::get_state(&fishing_helper::START_TIME) != (0 as i64) {
            let mut time: f32 = ((compat::java_round_f32((((compat::millis()).wrapping_sub(compat::get_state(&fishing_helper::START_TIME)) as f32) / 1000.0f32))) as f32) as f32;
            let mut max_time: f32;
            let mut pet: Option<PetInfo> = PetCache::get_current_pet();
            if pet.is_some() && (pet.as_ref().unwrap().r#type()).contains("SLUG") {
                let mut level: i32 = compat::read(&LevelFinder::get_level_info(&format!("PET_{}", compat::jstring(&pet.as_ref().unwrap().tier())), ((pet.as_ref().unwrap().exp()) as i64)), |__t| __t.level);
                max_time = ((20 as f32) * ((1 as f32) - ((level as f32) / 200.0f32)));
            } else {
                max_time = ((20) as f32) as f32;
            }
            time = time.clamp((0 as f32), max_time);
            { let (__a0,) = (Elements::progress_component_flexible_item_stack_component_component_float(Some(ico::CLOCK.get_mut().clone()), &component::null_to_empty(Some("Bobber Time")), &SkyblockTime::format_time(((max_time - time)) as f64), ((100 as f32) - ((time / max_time) * (100 as f32)))),); __this.lock().borrow_mut().add_element(__a0) };
        }
        if SkyblockerConfigManager::get().helpers.fishing.fishing_hook_display == FishingHookDisplay::Hud && fishing_hook_display_helper::FISHING_HOOK_ARMOR_STAND.get_mut().is_some() {
            let mut rod_reel_timer: String = fishing_hook_display_helper::FishingHookDisplayHelper::armor_stand_name().get_string();
            ({ let __f = compat::dispatch(__this, |__t| __t.add_simple_ico_text_flexible_item_stack_string_chat_formatting_string()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ElementCollector>), Some(ico::CLOCK.get_mut().clone()), "Reel Timer: ", &(if rod_reel_timer == "!!!" { ChatFormatting::Red } else { ChatFormatting::Yellow }), &rod_reel_timer) });
        }
    }

    /// FishingHudWidget.updateContent as the dispatch of ElementBasedWidget.updateContent
    pub fn update_content_dyn(__this: &compat::Shared<dyn ElementBasedWidget>) {
        Self::update_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected void updateConfigContent(ElementCollector collector)
    pub fn update_config_content_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn ElementCollector>) {
        if compat::read(__this, |__t| __t.show_creature_counter) {
            { let (__a0,) = (Elements::progress_component_5(Some(ico::SALMON_BUCKET.get_mut().clone()), &component::null_to_empty(Some("Alive Creatures")), &component::null_to_empty(Some("3/5")), ((60) as f32) as f32, ColorUtils::percent_to_color(((40) as f32) as f32)),); collector.lock().borrow_mut().add_element(__a0) };
            { let (__a0,) = (Elements::progress_component_flexible_item_stack_component_component_float(Some(ico::CLOCK.get_mut().clone()), &component::null_to_empty(Some("Time Left")), &component::null_to_empty(Some("1m")), ((60.0f32 / (SkyblockerConfigManager::get().helpers.fishing.timer_length as f32)) * (100 as f32))),); collector.lock().borrow_mut().add_element(__a0) };
        }
        if compat::read(__this, |__t| __t.show_fishing_timer) {
            { let (__a0,) = (Elements::progress_component_flexible_item_stack_component_component_float(Some(ico::CLOCK.get_mut().clone()), &component::null_to_empty(Some("Bobber Time")), &SkyblockTime::format_time(((15) as f64) as f64), 75 as f32),); collector.lock().borrow_mut().add_element(__a0) };
        }
        if SkyblockerConfigManager::get().helpers.fishing.fishing_hook_display == FishingHookDisplay::Hud {
            ({ let __f = compat::dispatch(collector, |__t| __t.add_simple_ico_text_flexible_item_stack_string_chat_formatting_string()); __f(collector, Some(ico::CLOCK.get_mut().clone()), "Reel Timer: ", &ChatFormatting::Yellow, "???") });
        }
    }

    /// FishingHudWidget.updateConfigContent as the dispatch of ElementBasedWidget.updateConfigContent
    pub fn update_config_content_dyn(__this: &compat::Shared<dyn ElementBasedWidget>, __p0: &compat::Shared<dyn ElementCollector>) {
        Self::update_config_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean shouldRender()
    pub fn should_render_impl(__this: &compat::Shared<Self>) -> bool {
        if compat::read(__this, |__t| __t.show_creature_counter) && SeaCreatureTracker::is_creatures_alive() {
            if Utils::get_location() == Location::Hub && compat::read(__this, |__t| __t.only_show_hud_in_barn) {
                return FishingHudWidget::is_barn_fishing();
            }
            return true;
        }
        if compat::read(__this, |__t| __t.show_fishing_timer) && compat::get_state(&fishing_helper::START_TIME) != (0 as i64) {
            return true;
        }
        if SkyblockerConfigManager::get().helpers.fishing.fishing_hook_display == FishingHookDisplay::Hud && fishing_hook_display_helper::FISHING_HOOK_ARMOR_STAND.get_mut().is_some() {
            return true;
        }
        return false;
    }

    /// FishingHudWidget.shouldRender as the dispatch of HudWidget.shouldRender
    pub fn should_render_dyn(__this: &compat::Shared<dyn HudWidget>) -> bool {
        Self::should_render_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector)
    pub fn get_option_widgets_impl(__this: &compat::Shared<Self>, collector: &mut OptionWidgetCollector) {
        <dyn ElementBasedWidget>::get_option_widgets_body(&compat::view(__this, |a| a as compat::Shared<dyn ElementBasedWidget>), collector);
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        collector.yes_no_button(&component::translatable("skyblocker.config.helpers.fishing.hud.enableSeaCreatureCounter"), Arc::new(move |b| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { { let __v = b; compat::write((&__this_1), |__t| __t.show_creature_counter = __v); } } }), compat::read(__this, |__t| __t.show_creature_counter), Some(component::translatable("skyblocker.config.helpers.fishing.hud.enableSeaCreatureCounter.@Tooltip")));
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        collector.yes_no_button(&component::translatable("skyblocker.config.helpers.fishing.enableFishingTimer"), Arc::new(move |b| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; { { let __v = b; compat::write((&__this_2), |__t| __t.show_fishing_timer = __v); } } }), compat::read(__this, |__t| __t.show_fishing_timer), Some(component::translatable("skyblocker.config.helpers.fishing.enableFishingTimer.@Tooltip")));
        // java: collector.addWidget(CycleButton.builder(d -> Component.literal(d.toString()), SkyblockerConfigManager.get().helpers.fishing.fishingHookDisplay)
        //         .withTooltip(_ -> Tooltip.create(Component.translatable("skyblocker.config.helpers.fishing.fishingHookDisplay.@Tooltip")))
        //         .withValues(FishingHookDisplay.values())
        //         .create(Component.translatable("skyblocker.config.helpers.fishing.fishingHookDisplay"),
        //                 (_, value) -> SkyblockerConfigManager.updateOnly(config -> config.helpers.fishing.fishingHookDisplay = value)));
        // (`create(name, ..)` is CycleButton's 150x20 at (0, 0))
        // (the port's cycle button holds each value's LABEL and answers the chosen INDEX, which maps back to the value)
        {
            let values: Vec<FishingHookDisplay> = FishingHookDisplay::values();
            let builder: cycle_button::Builder = cycle_button::builder(component::literal(&compat::jstring(&SkyblockerConfigManager::get().helpers.fishing.fishing_hook_display)).get_string())
                .with_tooltip(cycle_button::TooltipSupplier::Constant(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.config.helpers.fishing.fishingHookDisplay.@Tooltip"))))
                .with_values(values.iter().map(|d| component::literal(&compat::jstring(d)).get_string()).collect());
            collector.add_widget(compat::cycle_button_widget(0, 0, 150, 20, &component::translatable("skyblocker.config.helpers.fishing.fishingHookDisplay"), builder, move |index: usize| {
                let value = values[index];
                SkyblockerConfigManager::update_only(Arc::new(move |config: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig| {
                    config.helpers.fishing.fishing_hook_display = value;
                }));
            }));
        }
        if collector.editing_for() == Location::Hub {
            let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            collector.yes_no_button(&component::translatable("skyblocker.config.helpers.fishing.hud.onlyShowHudInBarn"), Arc::new(move |b| { let Some(__this_3) = __this_w3.get().upgrade() else { return Default::default(); }; { { let __v = b; compat::write((&__this_3), |__t| __t.only_show_hud_in_barn = __v); } } }), compat::read(__this, |__t| __t.only_show_hud_in_barn), Some(component::translatable("skyblocker.config.helpers.fishing.hud.onlyShowHudInBarn.@Tooltip")));
        }
    }

    /// FishingHudWidget.getOptionWidgets as the dispatch of HudWidget.getOptionWidgets
    pub fn get_option_widgets_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &mut OptionWidgetCollector) {
        Self::get_option_widgets_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void load(JsonValueInput input)
    pub fn load_impl(__this: &compat::Shared<Self>, input: &JsonValueInput) {
        <dyn ElementBasedWidget>::load_body(&compat::view(__this, |a| a as compat::Shared<dyn ElementBasedWidget>), input);
        { let __v = input.read_boolean_or("creature_counter", true); compat::write(__this, |__t| __t.show_creature_counter = __v); }
        { let __v = input.read_boolean_or("fishing_timer", true); compat::write(__this, |__t| __t.show_fishing_timer = __v); }
        { let __v = input.read_boolean_or("only_barn", true); compat::write(__this, |__t| __t.only_show_hud_in_barn = __v); }
    }

    /// FishingHudWidget.load as the dispatch of HudWidget.load
    pub fn load_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &JsonValueInput) {
        Self::load_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void save(JsonValueOutput output)
    pub fn save_impl(__this: &compat::Shared<Self>, output: &mut JsonValueOutput) {
        <dyn ElementBasedWidget>::save_body(&compat::view(__this, |a| a as compat::Shared<dyn ElementBasedWidget>), output);
        output.write_bool("creature_counter", compat::read(__this, |__t| __t.show_creature_counter));
        output.write_bool("fishing_timer", compat::read(__this, |__t| __t.show_fishing_timer));
        output.write_bool("only_barn", compat::read(__this, |__t| __t.only_show_hud_in_barn));
    }

    /// FishingHudWidget.save as the dispatch of HudWidget.save
    pub fn save_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &mut JsonValueOutput) {
        Self::save_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
