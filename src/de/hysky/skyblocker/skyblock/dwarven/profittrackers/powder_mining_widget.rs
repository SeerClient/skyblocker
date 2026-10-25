//! Ported from de.hysky.skyblocker.skyblock.dwarven.profittrackers.PowderMiningWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::screens::powdertracker::powder_filter_config_screen::PowderFilterConfigScreen;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::powder_mining_tracker::PowderMiningTracker;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::Information;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::location::Location;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public static PowderMiningWidget INSTANCE;
pub static INSTANCE: compat::MainThread<Option<compat::Shared<PowderMiningWidget>>> = compat::MainThread::new(|| None);

/// java: @RegisterWidget public class PowderMiningWidget extends HudWidget
#[derive(Clone)]
pub struct PowderMiningWidget {
    /// java: extends HudWidget
    pub base: HudWidgetBase,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for PowderMiningWidget {
    type Target = HudWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PowderMiningWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PowderMiningWidget {
    /// java: public PowderMiningWidget()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = HudWidgetBase::new_in(Information::new3("powder_mining_tracker".to_string(), component::translatable("skyblocker.powderTracker"), Location::CrystalHollows), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                self_ref: std::sync::Weak::new(),
            });
            compat::write(&__h, |__t| __t.self_ref = std::sync::Arc::downgrade(&__h));
            ((__h.clone() as compat::Shared<dyn HudWidget>), __h)
        });
        *INSTANCE.get_mut() = Some(__this.clone());
        __this
    }

}

impl HudWidget for PowderMiningWidget {
    fn hud_widget_base(&self) -> &HudWidgetBase {
        &self.base
    }
    fn hud_widget_base_mut(&mut self) -> &mut HudWidgetBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        PowderMiningWidget::extract_widget_render_state_dyn
    }

    /// java: @Override protected void extractWidgetRenderStateForConfig(GuiGraphicsExtractor graphics, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state_for_config(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        PowderMiningWidget::extract_widget_render_state_for_config_dyn
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector) (dispatch: the body for this class)
    fn get_option_widgets(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut OptionWidgetCollector) {
        PowderMiningWidget::get_option_widgets_dyn
    }

}

impl PowderMiningWidget {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, float delta)
    pub fn extract_widget_render_state_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut delta: f32) {
        let mut set = PowderMiningTracker::get_shown_rewards().iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>();
        if set.is_empty() {
            // java: w = h = 0;
            compat::write(__this, |__t| { __t.h = 0; __t.w = 0; });
            return;
        }
        let mut temp_y: i32 = 0;
        let mut max_width: i32 = 0;
        for entry in set.iter() {
            let mut price: MutableComponent = (component::literal(&formatters::INTEGER_NUMBERS.get_mut().format((entry.1) as f64))).with_color(mcrust::net::minecraft::util::common_colors::WHITE);
            let mut text: MutableComponent = entry.0.copy().append_str(" ").append(price.clone());
            gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), 0, temp_y, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone()), mcrust::net::minecraft::util::common_colors::WHITE);
            temp_y = temp_y.wrapping_add(10);
            let mut width: i32 = compat::font().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone()));
            if width > max_width {
                max_width = width;
            }
        }
        temp_y = temp_y.wrapping_add(10);
        gui_graphics_extractor::draw_formatted_char_sequence(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from((component::translatable_args("skyblocker.powderTracker.profit", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(formatters::DOUBLE_NUMBERS.get_mut().format(PowderMiningTracker::get_profit()))])).with_style_format(ChatFormatting::Gold)), 0, temp_y, mcrust::net::minecraft::util::common_colors::WHITE);
        { let __v = max_width; compat::write(__this, |__t| __t.w = __v); }
        { let __v = temp_y.wrapping_add(10); compat::write(__this, |__t| __t.h = __v); }
    }

    /// PowderMiningWidget.extractWidgetRenderState as the dispatch of HudWidget.extractWidgetRenderState
    pub fn extract_widget_render_state_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &mut GuiGraphicsExtractor, __p1: f32) {
        Self::extract_widget_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override protected void extractWidgetRenderStateForConfig(GuiGraphicsExtractor graphics, float delta)
    pub fn extract_widget_render_state_for_config_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut delta: f32) {
        ({ let __f = compat::dispatch(__this, |__t| __t.extract_widget_render_state()); __f(&compat::view(__this, |a| a as compat::Shared<dyn HudWidget>), graphics, delta) });
    }

    /// PowderMiningWidget.extractWidgetRenderStateForConfig as the dispatch of HudWidget.extractWidgetRenderStateForConfig
    pub fn extract_widget_render_state_for_config_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &mut GuiGraphicsExtractor, __p1: f32) {
        Self::extract_widget_render_state_for_config_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector)
    pub fn get_option_widgets_impl(__this: &compat::Shared<Self>, collector: &mut OptionWidgetCollector) {
        <dyn HudWidget>::get_option_widgets_body(&compat::view(__this, |a| a as compat::Shared<dyn HudWidget>), collector);
        // (`collector.configScreen()` - the configuration screen that is up - is the parent the game holds:
        // `set_screen_keeping_parent`, and the filter screen's `onClose` returns to it)
        //   java: new PowderFilterConfigScreen(collector.configScreen(), ...)
        collector.add_widget((button::builder(component::translatable("skyblocker.config.mining.crystalHollows.powderTrackerFilter").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(PowderFilterConfigScreen::new(PowderMiningTracker::get_name2_id_map().keys().cloned().collect::<Vec<_>>())))); })))).tooltip_text(Some((Tooltip::create((component::translatable("skyblocker.config.mining.crystalHollows.powderTrackerFilter.@Tooltip")).get_string())).message.clone())).build());
    }

    /// PowderMiningWidget.getOptionWidgets as the dispatch of HudWidget.getOptionWidgets
    pub fn get_option_widgets_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &mut OptionWidgetCollector) {
        Self::get_option_widgets_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
