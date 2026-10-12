//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.widgets.DungeonRunsWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::generic_catacombs::AggregateStat;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::profile_member::ProfileMember;
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
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: private static final int INFO_OFFSET = 2;
pub const INFO_OFFSET: i32 = 2;
/// java: private static final Identifier RUNNING_MAN = SkyblockerMod.id("profile_viewer2/running_man");
pub static RUNNING_MAN: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("profile_viewer2/running_man"));

/// java: public final class DungeonRunsWidget extends BasicInfoBoxWidget
#[derive(Clone)]
pub struct DungeonRunsWidget {
    /// java: extends BasicInfoBoxWidget
    pub base: BasicInfoBoxWidgetBase,
    /// java: private final LoadingInformation info;
    pub info: LoadingInformation,
}

impl std::ops::Deref for DungeonRunsWidget {
    type Target = BasicInfoBoxWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DungeonRunsWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DungeonRunsWidget {
    /// java: public DungeonRunsWidget(int width, LoadingInformation info)
    pub fn new(mut width: i32, mut info: LoadingInformation) -> compat::Shared<Self> {
        let __init_info = info.clone();
        let (_, __this) = BasicInfoBoxWidgetBase::new_in(width, 22, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                info: __init_info,
            });
            ((__h.clone() as compat::Shared<dyn BasicInfoBoxWidget>), __h)
        });
        __this
    }

}

impl BasicInfoBoxWidget for DungeonRunsWidget {
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
        DungeonRunsWidget::extract_widget_render_state_dyn
    }

}

impl DungeonRunsWidget {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        <dyn BasicInfoBoxWidget>::extract_widget_render_state_body(&compat::view(__this, |a| a as compat::Shared<dyn BasicInfoBoxWidget>), graphics, mouse_x, mouse_y, a);
        let mut font: Font = compat::font();
        let mut member: compat::Shared<ProfileMember> = __this.lock().borrow().info.member();
        let mut x: i32 = (__this.lock().borrow().get_x()).wrapping_add(INFO_OFFSET);
        let mut y: i32 = (__this.lock().borrow().get_y()).wrapping_add(INFO_OFFSET);
        let mut text_y_step: i32 = (mcrust::net::minecraft::client::gui::font::LINE_HEIGHT).wrapping_add(1);
        let mut icon_size: i32 = mcrust::net::minecraft::client::gui::render::gui_renderer::DEFAULT_ITEM_SIZE;
        let mut icon_area_size: i32 = (__this.lock().borrow().get_height()).wrapping_sub(INFO_OFFSET.wrapping_mul(2));
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), RUNNING_MAN.get_mut(), x, y.wrapping_add((icon_area_size.wrapping_sub(icon_size)).wrapping_div(2)), icon_size, icon_size);
        x = x.wrapping_add((mcrust::net::minecraft::client::gui::render::gui_renderer::DEFAULT_ITEM_SIZE).wrapping_add(2));
        let mut normal_runs_text: MutableComponent = component::empty().append(component::literal("Normal ").with_style_format(ChatFormatting::Green)).append_str(&formatters::INTEGER_NUMBERS.get_mut().format(AggregateStat::get_manually_calculated_total(&compat::read(&member.lock().borrow().dungeons.dungeon_types.catacombs, |__t| __t.tier_completions.clone()))));
        gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x, y, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(normal_runs_text.clone()), mcrust::net::minecraft::util::common_colors::WHITE);
        y = y.wrapping_add(text_y_step);
        let mut master_runs_text: MutableComponent = component::empty().append(component::literal("Master ").with_style_format(ChatFormatting::Red)).append_str(&formatters::INTEGER_NUMBERS.get_mut().format(AggregateStat::get_manually_calculated_total(&compat::read(&member.lock().borrow().dungeons.dungeon_types.master_mode_catacombs, |__t| __t.generic_catacombs_base().tier_completions.clone()))));
        gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x, y, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(master_runs_text.clone()), mcrust::net::minecraft::util::common_colors::WHITE);
        y = y.wrapping_add(text_y_step);
    }

    /// DungeonRunsWidget.extractWidgetRenderState as the dispatch of BasicInfoBoxWidget.extractWidgetRenderState
    pub fn extract_widget_render_state_dyn(__this: &compat::Shared<dyn BasicInfoBoxWidget>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32) {
        Self::extract_widget_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

}

/// DungeonRunsWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for DungeonRunsWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base.base
    }
}

/// DungeonRunsWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<DungeonRunsWidget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_widget_render_state()); __f(&compat::view(__h, |a| a as compat::Shared<dyn BasicInfoBoxWidget>), &mut context.graphics, mouse_x, mouse_y, a) }))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.update_widget_narration()); __f(&compat::view(__h, |a| a as compat::Shared<dyn BasicInfoBoxWidget>), output) }))
    }

}
