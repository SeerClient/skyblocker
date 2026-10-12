//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.widgets.TuningSlotWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::accessory_bag_storage::TuningSlot;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::accessory_power_calculator::AccessoryPowerCalculator;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::basic_info_box_widget::BasicInfoBoxWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::basic_info_box_widget::BasicInfoBoxWidgetBase;
use crate::de::hysky::skyblocker::utils::collection_utils::CollectionUtils;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final int OFFSET = 2;
pub const OFFSET: i32 = 2;
/// java: private static final int TEXT_EXTRA_SPACING = 1;
pub const TEXT_EXTRA_SPACING: i32 = 1;
/// java: private static final int TITLE_BOTTOM_PADDING = 2;
pub const TITLE_BOTTOM_PADDING: i32 = 2;

/// java: public final class TuningSlotWidget extends BasicInfoBoxWidget
#[derive(Clone)]
pub struct TuningSlotWidget {
    /// java: extends BasicInfoBoxWidget
    pub base: BasicInfoBoxWidgetBase,
    /// java: private final Component title;
    pub title: Option<MutableComponent>,
    /// java: private final Pair<List<Component>, List<Component>> halves;
    pub halves: Option<(Vec<MutableComponent>, Vec<MutableComponent>)>,
}

impl std::ops::Deref for TuningSlotWidget {
    type Target = BasicInfoBoxWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TuningSlotWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TuningSlotWidget {
    /// java: public TuningSlotWidget(int width, TuningSlot slot)
    pub fn new(mut width: i32, mut slot: compat::Shared<dyn TuningSlot>) -> compat::Shared<Self> {
        let mut lines: Vec<MutableComponent> = TuningSlotWidget::build_lines(&slot);
        let mut halves: (Vec<MutableComponent>, Vec<MutableComponent>) = CollectionUtils::halve(&lines);
        let mut line_height: i32 = mcrust::net::minecraft::client::gui::font::LINE_HEIGHT;
        let mut title_height: i32 = (line_height.wrapping_add(TEXT_EXTRA_SPACING)).wrapping_add(TITLE_BOTTOM_PADDING);
        let mut lines_height: i32 = (line_height.wrapping_add(TEXT_EXTRA_SPACING)).wrapping_mul((halves.0.len() as i32).max((halves.1.len() as i32)));
        let mut height: i32 = ((OFFSET.wrapping_mul(2)).wrapping_add(title_height)).wrapping_add(lines_height);
        // TODO(fabric2seer): super(..) with 0 argument(s)
        //   java:
        let __this: compat::Shared<Self> = todo!("constructor");
        // TODO(fabric2seer): super statement
        //   java: super(width, height);
        { let __v = Some(component::literal("Tuning Stats").with_style_format(ChatFormatting::Bold)); compat::write(&__this, |__t| __t.title = __v); }
        { let __v = Some(halves.clone()); compat::write(&__this, |__t| __t.halves = __v); }
        __this
    }

    /// java: private static List<Component> buildLines(TuningSlot slot)
    pub fn build_lines(slot: &compat::Shared<dyn TuningSlot>) -> Vec<MutableComponent> {
        let mut lines: Vec<MutableComponent> = Vec::new();
        if compat::read(slot, |__t| __t.tuning_slot_base().health) > 0 {
            lines.push(AccessoryPowerCalculator::format_health(((compat::read(slot, |__t| __t.tuning_slot_base().health) as f64) * 5.0)));
        }
        if compat::read(slot, |__t| __t.tuning_slot_base().defence) > 0 {
            lines.push(AccessoryPowerCalculator::format_defence(((compat::read(slot, |__t| __t.tuning_slot_base().defence)) as f64) as f64));
        }
        if compat::read(slot, |__t| __t.tuning_slot_base().strength) > 0 {
            lines.push(AccessoryPowerCalculator::format_strength(((compat::read(slot, |__t| __t.tuning_slot_base().strength)) as f64) as f64));
        }
        if compat::read(slot, |__t| __t.tuning_slot_base().critical_chance) > 0 {
            lines.push(AccessoryPowerCalculator::format_crit_chance(((compat::read(slot, |__t| __t.tuning_slot_base().critical_chance) as f64) * 0.2)));
        }
        if compat::read(slot, |__t| __t.tuning_slot_base().critical_damage) > 0 {
            lines.push(AccessoryPowerCalculator::format_crit_damage(((compat::read(slot, |__t| __t.tuning_slot_base().critical_damage)) as f64) as f64));
        }
        if compat::read(slot, |__t| __t.tuning_slot_base().attack_speed) > 0 {
            lines.push(AccessoryPowerCalculator::format_attack_speed(((compat::read(slot, |__t| __t.tuning_slot_base().attack_speed) as f64) * 0.3)));
        }
        if compat::read(slot, |__t| __t.tuning_slot_base().intelligence) > 0 {
            lines.push(AccessoryPowerCalculator::format_intelligence(((compat::read(slot, |__t| __t.tuning_slot_base().intelligence) as f64) * 2.0)));
        }
        if compat::read(slot, |__t| __t.tuning_slot_base().walk_speed) > 0 {
            lines.push(AccessoryPowerCalculator::format_walk_speed(((compat::read(slot, |__t| __t.tuning_slot_base().walk_speed) as f64) * 1.5)));
        }
        if lines.is_empty() {
            lines.push(component::literal("Unconfigured...").with_style_formats(&[ChatFormatting::Gray, ChatFormatting::Italic]));
        }
        return lines.iter().cloned().collect::<Vec<_>>();
    }

}

impl BasicInfoBoxWidget for TuningSlotWidget {
    fn basic_info_box_widget_base(&self) -> &BasicInfoBoxWidgetBase {
        &self.base
    }
    fn basic_info_box_widget_base_mut(&mut self) -> &mut BasicInfoBoxWidgetBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn BasicInfoBoxWidget>, &mut GuiGraphicsExtractor, i32, i32, f32) {
        TuningSlotWidget::extract_widget_render_state_dyn
    }

}

impl TuningSlotWidget {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        <dyn BasicInfoBoxWidget>::extract_widget_render_state_body(&compat::view(__this, |a| a as compat::Shared<dyn BasicInfoBoxWidget>), graphics, mouse_x, mouse_y, a);
        let mut font: Font = compat::font();
        let mut x: i32 = (__this.lock().borrow().get_x()).wrapping_add(OFFSET);
        let mut y: i32 = (__this.lock().borrow().get_y()).wrapping_add(OFFSET);
        let mut text_y_step: i32 = (mcrust::net::minecraft::client::gui::font::LINE_HEIGHT).wrapping_add(TEXT_EXTRA_SPACING);
        gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x, y, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(__this, |__t| __t.title.clone()).unwrap()), mcrust::net::minecraft::util::common_colors::WHITE);
        // TODO(fabric2seer): an assignment used as a value
        //   java: final int titleY = y += textYStep + TITLE_BOTTOM_PADDING;
        let mut title_y: i32 = todo!("fabric2seer: the initialiser of `titleY` (TODO above)");
        for line in (compat::read(__this, |__t| __t.halves.clone()).0).iter() {
            gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x, y, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(line.clone()), mcrust::net::minecraft::util::common_colors::WHITE);
            y = y.wrapping_add(text_y_step);
        }
        x = (__this.lock().borrow().get_x()).wrapping_add((__this.lock().borrow().get_width()).wrapping_div(2));
        y = title_y;
        for line in (compat::read(__this, |__t| __t.halves.clone()).1).iter() {
            gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x, y, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(line.clone()), mcrust::net::minecraft::util::common_colors::WHITE);
            y = y.wrapping_add(text_y_step);
        }
    }

    /// TuningSlotWidget.extractWidgetRenderState as the dispatch of BasicInfoBoxWidget.extractWidgetRenderState
    pub fn extract_widget_render_state_dyn(__this: &compat::Shared<dyn BasicInfoBoxWidget>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32) {
        Self::extract_widget_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

}

/// TuningSlotWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for TuningSlotWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base.base
    }
}

/// TuningSlotWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<TuningSlotWidget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_widget_render_state()); __f(&compat::view(__h, |a| a as compat::Shared<dyn BasicInfoBoxWidget>), &mut context.graphics, mouse_x, mouse_y, a) }))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.update_widget_narration()); __f(&compat::view(__h, |a| a as compat::Shared<dyn BasicInfoBoxWidget>), output) }))
    }

}
