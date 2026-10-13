//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.widgets.AccessoryPowerWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::accessory_power_calculator::AccessoryPowerCalculator;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::profile_item_storage::ProfileItemStorage;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::basic_info_box_widget::BasicInfoBoxWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::basic_info_box_widget::BasicInfoBoxWidgetBase;
use crate::de::hysky::skyblocker::utils::collection_utils::CollectionUtils;
use crate::de::hysky::skyblocker::utils::formatters;
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

/// java: public final class AccessoryPowerWidget extends BasicInfoBoxWidget
#[derive(Clone)]
pub struct AccessoryPowerWidget {
    /// java: extends BasicInfoBoxWidget
    pub base: BasicInfoBoxWidgetBase,
    /// java: private final List<Component> mainLines;
    pub main_lines: Vec<MutableComponent>,
    /// java: private final Pair<List<Component>, List<Component>> statsHalves;
    pub stats_halves: Option<(Vec<MutableComponent>, Vec<MutableComponent>)>,
}

impl std::ops::Deref for AccessoryPowerWidget {
    type Target = BasicInfoBoxWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AccessoryPowerWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AccessoryPowerWidget {
    /// java: public AccessoryPowerWidget(int width, LoadingInformation info, ProfileItemStorage itemStorage)
    pub fn new(mut width: i32, mut info: LoadingInformation, mut item_storage: ProfileItemStorage) -> compat::Shared<Self> {
        let mut accessory_power: i32 = AccessoryPowerCalculator::calculate_accessory_power(&info.member(), &item_storage.bags().accessories());
        let mut main_lines: Vec<MutableComponent> = AccessoryPowerWidget::build_lines(&info, accessory_power);
        let mut stat_lines: Vec<MutableComponent> = AccessoryPowerCalculator::get_power_stats(accessory_power, &info.member().lock().borrow().accessory_bag_storage.selected_power);
        let mut stats_halves: (Vec<MutableComponent>, Vec<MutableComponent>) = CollectionUtils::halve(&stat_lines);
        let mut line_height: i32 = mcrust::net::minecraft::client::gui::font::LINE_HEIGHT;
        let mut main_lines_height: i32 = (line_height.wrapping_add(TEXT_EXTRA_SPACING)).wrapping_mul(main_lines.len() as i32);
        let mut stat_lines_height: i32 = (line_height.wrapping_add(TEXT_EXTRA_SPACING)).wrapping_mul((stats_halves.0.len() as i32).max((stats_halves.1.len() as i32)));
        let mut height: i32 = ((OFFSET.wrapping_mul(2)).wrapping_add(main_lines_height)).wrapping_add(stat_lines_height);
        // TODO(fabric2seer): super(..) with 0 argument(s)
        //   java:
        let __this: compat::Shared<Self> = todo!("constructor");
        // TODO(fabric2seer): super statement
        //   java: super(width, height);
        { let __v = main_lines.clone(); compat::write(&__this, |__t| __t.main_lines = __v); }
        { let __v = Some(stats_halves.clone()); compat::write(&__this, |__t| __t.stats_halves = __v); }
        __this
    }

    /// java: private static List<Component> buildLines(LoadingInformation info, int accessoryPower)
    pub fn build_lines(info: &LoadingInformation, mut accessory_power: i32) -> Vec<MutableComponent> {
        let mut lines: Vec<MutableComponent> = Vec::new();
        let mut ap_text: MutableComponent = component::empty().append(component::literal("AP: ").with_style_format(ChatFormatting::Gold)).append_str(&formatters::INTEGER_NUMBERS.get_mut().format((accessory_power) as f64));
        lines.push(ap_text.clone());
        // TODO(fabric2seer): static call WordUtils.capitalizeFully(..)
        //   java: Component powerText = Component.empty()
        //   java: 				.append(Component.literal("Power: ").withStyle(ChatFormatting.GREEN))
        //   java: 				.append(WordUtils.capitalizeFully(info.member().accessoryBagStorage.selectedPower));
        let mut power_text: MutableComponent = component::empty().append(component::literal("Power: ").with_style_format(ChatFormatting::Green)).append_str(&compat::capitalize_fully(&info.member().lock().borrow().accessory_bag_storage.selected_power));
        lines.push(power_text.clone());
        lines.push(component::empty());
        lines.push(component::literal("Power Stats").with_style_format(ChatFormatting::Bold));
        return lines.iter().cloned().collect::<Vec<_>>();
    }

}

impl BasicInfoBoxWidget for AccessoryPowerWidget {
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
        AccessoryPowerWidget::extract_widget_render_state_dyn
    }

}

impl AccessoryPowerWidget {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        <dyn BasicInfoBoxWidget>::extract_widget_render_state_body(&compat::view(__this, |a| a as compat::Shared<dyn BasicInfoBoxWidget>), graphics, mouse_x, mouse_y, a);
        let mut font: Font = compat::font();
        let mut x: i32 = (__this.lock().borrow().get_x()).wrapping_add(OFFSET);
        let mut y: i32 = (__this.lock().borrow().get_y()).wrapping_add(OFFSET);
        let mut text_y_step: i32 = (mcrust::net::minecraft::client::gui::font::LINE_HEIGHT).wrapping_add(TEXT_EXTRA_SPACING);
        for line in compat::read(__this, |__t| __t.main_lines.clone()) {
            gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x, y, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(line.clone()), mcrust::net::minecraft::util::common_colors::WHITE);
            y = y.wrapping_add(text_y_step);
        }
        let mut stats_y: i32 = y;
        for line in compat::read(__this, |__t| __t.stats_halves.as_ref().expect("NullPointerException").0.clone()).iter() {
            gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x, y, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(line.clone()), mcrust::net::minecraft::util::common_colors::WHITE);
            y = y.wrapping_add(text_y_step);
        }
        x = (__this.lock().borrow().get_x()).wrapping_add((__this.lock().borrow().get_width()).wrapping_div(2));
        y = stats_y;
        for line in compat::read(__this, |__t| __t.stats_halves.as_ref().expect("NullPointerException").1.clone()).iter() {
            gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x, y, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(line.clone()), mcrust::net::minecraft::util::common_colors::WHITE);
            y = y.wrapping_add(text_y_step);
        }
    }

    /// AccessoryPowerWidget.extractWidgetRenderState as the dispatch of BasicInfoBoxWidget.extractWidgetRenderState
    pub fn extract_widget_render_state_dyn(__this: &compat::Shared<dyn BasicInfoBoxWidget>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32) {
        Self::extract_widget_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

}

/// AccessoryPowerWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for AccessoryPowerWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base.base
    }
}

/// AccessoryPowerWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<AccessoryPowerWidget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_widget_render_state()); __f(&compat::view(__h, |a| a as compat::Shared<dyn BasicInfoBoxWidget>), &mut context.graphics, mouse_x, mouse_y, a) }))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.update_widget_narration()); __f(&compat::view(__h, |a| a as compat::Shared<dyn BasicInfoBoxWidget>), output) }))
    }

}
