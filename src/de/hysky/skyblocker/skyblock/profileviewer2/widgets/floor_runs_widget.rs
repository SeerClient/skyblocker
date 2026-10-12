//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.widgets.FloorRunsWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::generic_catacombs::GenericCatacombs;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::generic_catacombs::PerFloorDisambiguation;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::profile_member::ProfileMember;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::time_format_utils::TimeFormatUtils;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::basic_info_box_widget::BasicInfoBoxWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::basic_info_box_widget::BasicInfoBoxWidgetBase;
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

/// java: private static final int INFO_OFFSET = 2;
pub const INFO_OFFSET: i32 = 2;

/// java: public final class FloorRunsWidget extends BasicInfoBoxWidget
#[derive(Clone)]
pub struct FloorRunsWidget {
    /// java: extends BasicInfoBoxWidget
    pub base: BasicInfoBoxWidgetBase,
    /// java: private final LoadingInformation info;
    pub info: LoadingInformation,
}

impl std::ops::Deref for FloorRunsWidget {
    type Target = BasicInfoBoxWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for FloorRunsWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FloorRunsWidget {
    /// java: public FloorRunsWidget(int width, int height, LoadingInformation info)
    pub fn new(mut width: i32, mut height: i32, mut info: LoadingInformation) -> compat::Shared<Self> {
        let __init_info = info.clone();
        let (_, __this) = BasicInfoBoxWidgetBase::new_in(width, height, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                info: __init_info,
            });
            ((__h.clone() as compat::Shared<dyn BasicInfoBoxWidget>), __h)
        });
        __this
    }

    /// java: private static Component compileStatsLine(String floorName, int floorIndex, boolean master, GenericCatacombs stats)
    pub fn compile_stats_line(floor_name: &str, mut floor_index: i32, mut master: bool, stats: &compat::Shared<dyn GenericCatacombs>) -> MutableComponent {
        let mut s_plus_time: i64 = ((({ let __r = &compat::read(stats, |__t| __t.generic_catacombs_base().fastest_time_s_plus.clone()); let __f = compat::dispatch(__r, |__t| __t.get_value_or_zero()); __f(&compat::view(__r, |a| a as compat::Shared<dyn PerFloorDisambiguation>), floor_index) })) as i64);
        let mut s_time: i64 = ((({ let __r = &compat::read(stats, |__t| __t.generic_catacombs_base().fastest_time_s.clone()); let __f = compat::dispatch(__r, |__t| __t.get_value_or_zero()); __f(&compat::view(__r, |a| a as compat::Shared<dyn PerFloorDisambiguation>), floor_index) })) as i64);
        let mut pb_text: MutableComponent = component::literal("(None)").with_style_format(ChatFormatting::Gray);
        if s_plus_time != (0 as i64) {
            pb_text = (component::empty().append(component::literal("(").with_style_format(ChatFormatting::Gray)).append(component::literal(&TimeFormatUtils::get_duration_string(s_plus_time)).with_style_format(ChatFormatting::Gray)).append((component::literal(" S+")).with_style_formats(&[ChatFormatting::LightPurple, ChatFormatting::Bold]))).append(component::literal(")").with_style_format(ChatFormatting::Gray));
        } else if s_time != (0 as i64) {
            pb_text = component::empty().append(component::literal("(").with_style_format(ChatFormatting::Gray)).append(component::literal(&TimeFormatUtils::get_duration_string(s_time)).with_style_format(ChatFormatting::Gray)).append(component::literal(" S").with_style_format(ChatFormatting::LightPurple)).append(component::literal(")").with_style_format(ChatFormatting::Gray));
        }
        let mut floor_text: MutableComponent = component::literal(floor_name).with_style_format((if master { ChatFormatting::Red } else { ChatFormatting::Green }));
        let mut runs_text: MutableComponent = component::literal(&formatters::INTEGER_NUMBERS.get_mut().format(({ let __r = &compat::read(stats, |__t| __t.generic_catacombs_base().tier_completions.clone()); let __f = compat::dispatch(__r, |__t| __t.get_value_or_zero()); __f(&compat::view(__r, |a| a as compat::Shared<dyn PerFloorDisambiguation>), floor_index) })));
        let mut line: MutableComponent = component::empty().append(floor_text.clone()).append_str(" ").append(runs_text.clone()).append_str(" ").append(pb_text.clone());
        return line;
    }

}

impl BasicInfoBoxWidget for FloorRunsWidget {
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
        FloorRunsWidget::extract_widget_render_state_dyn
    }

}

impl FloorRunsWidget {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        <dyn BasicInfoBoxWidget>::extract_widget_render_state_body(&compat::view(__this, |a| a as compat::Shared<dyn BasicInfoBoxWidget>), graphics, mouse_x, mouse_y, a);
        let mut font: Font = compat::font();
        let mut member: compat::Shared<ProfileMember> = __this.lock().borrow().info.member();
        let mut normal: compat::Shared<dyn GenericCatacombs> = (member.lock().borrow().dungeons.dungeon_types.catacombs.clone() as compat::Shared<dyn GenericCatacombs>);
        let mut master_mode: compat::Shared<dyn GenericCatacombs> = member.lock().borrow().dungeons.dungeon_types.master_mode_catacombs.clone();
        let mut x: i32 = (__this.lock().borrow().get_x()).wrapping_add(INFO_OFFSET);
        let mut y: i32 = (__this.lock().borrow().get_y()).wrapping_add(INFO_OFFSET);
        let mut text_y_step: i32 = (mcrust::net::minecraft::client::gui::font::LINE_HEIGHT).wrapping_add(1);
        let mut title: MutableComponent = component::literal("Floor Runs").with_style_format(ChatFormatting::Bold);
        gui_graphics_extractor::centered_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(title.clone()), x.wrapping_add(((__this.lock().borrow().get_width()).wrapping_sub(INFO_OFFSET.wrapping_mul(2))).wrapping_div(2)), y, mcrust::net::minecraft::util::common_colors::WHITE);
        // TODO(fabric2seer): an assignment used as a value
        //   java: final int afterTitleY = (y += textYStep + 2);
        let mut after_title_y: i32 = todo!("fabric2seer: the initialiser of `afterTitleY` (TODO above)");
        let mut normal_stats: Vec<MutableComponent> = vec![FloorRunsWidget::compile_stats_line("E ", 0, false, &normal), FloorRunsWidget::compile_stats_line("F1", 1, false, &normal), FloorRunsWidget::compile_stats_line("F2", 2, false, &normal), FloorRunsWidget::compile_stats_line("F3", 3, false, &normal), FloorRunsWidget::compile_stats_line("F4", 4, false, &normal), FloorRunsWidget::compile_stats_line("F5", 5, false, &normal), FloorRunsWidget::compile_stats_line("F6", 6, false, &normal), FloorRunsWidget::compile_stats_line("F7", 7, false, &normal)];
        let mut master_mode_stats: Vec<MutableComponent> = vec![component::empty(), FloorRunsWidget::compile_stats_line("M1", 1, true, &master_mode), FloorRunsWidget::compile_stats_line("M2", 2, true, &master_mode), FloorRunsWidget::compile_stats_line("M3", 3, true, &master_mode), FloorRunsWidget::compile_stats_line("M4", 4, true, &master_mode), FloorRunsWidget::compile_stats_line("M5", 5, true, &master_mode), FloorRunsWidget::compile_stats_line("M6", 6, true, &master_mode), FloorRunsWidget::compile_stats_line("M7", 7, true, &master_mode)];
        for line in normal_stats.iter() {
            gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x, y, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(line.clone()), mcrust::net::minecraft::util::common_colors::WHITE);
            y = y.wrapping_add(text_y_step);
        }
        x = (__this.lock().borrow().get_x()).wrapping_add((__this.lock().borrow().get_width()).wrapping_div(2));
        y = after_title_y;
        for line in master_mode_stats.iter() {
            gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x, y, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(line.clone()), mcrust::net::minecraft::util::common_colors::WHITE);
            y = y.wrapping_add(text_y_step);
        }
    }

    /// FloorRunsWidget.extractWidgetRenderState as the dispatch of BasicInfoBoxWidget.extractWidgetRenderState
    pub fn extract_widget_render_state_dyn(__this: &compat::Shared<dyn BasicInfoBoxWidget>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32) {
        Self::extract_widget_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

}

/// FloorRunsWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for FloorRunsWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base.base
    }
}

/// FloorRunsWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<FloorRunsWidget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_widget_render_state()); __f(&compat::view(__h, |a| a as compat::Shared<dyn BasicInfoBoxWidget>), &mut context.graphics, mouse_x, mouse_y, a) }))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.update_widget_narration()); __f(&compat::view(__h, |a| a as compat::Shared<dyn BasicInfoBoxWidget>), output) }))
    }

}
