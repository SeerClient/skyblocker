//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.widgets.DailyRunsWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::dungeons;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::dungeons::DailyRuns;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::profile_member::ProfileMember;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::time_format_utils::TimeFormatUtils;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::basic_info_box_widget::BasicInfoBoxWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::basic_info_box_widget::BasicInfoBoxWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
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

/// java: public final class DailyRunsWidget extends BasicInfoBoxWidget
#[derive(Clone)]
pub struct DailyRunsWidget {
    /// java: extends BasicInfoBoxWidget
    pub base: BasicInfoBoxWidgetBase,
    /// java: private final LoadingInformation info;
    pub info: LoadingInformation,
}

impl std::ops::Deref for DailyRunsWidget {
    type Target = BasicInfoBoxWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DailyRunsWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DailyRunsWidget {
    /// java: public DailyRunsWidget(int width, LoadingInformation info)
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

impl BasicInfoBoxWidget for DailyRunsWidget {
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
        DailyRunsWidget::extract_widget_render_state_dyn
    }

}

impl DailyRunsWidget {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        <dyn BasicInfoBoxWidget>::extract_widget_render_state_body(&compat::view(__this, |a| a as compat::Shared<dyn BasicInfoBoxWidget>), graphics, mouse_x, mouse_y, a);
        let mut font: Font = compat::font();
        let mut member: compat::Shared<ProfileMember> = __this.lock().borrow().info.member();
        let __member_guard = member.lock();
        let __member_ref = __member_guard.borrow();
        let daily_runs = &__member_ref.dungeons.daily_runs;
        let mut x: i32 = (__this.lock().borrow().get_x()).wrapping_add(INFO_OFFSET);
        let mut y: i32 = (__this.lock().borrow().get_y()).wrapping_add(INFO_OFFSET);
        let mut text_y_step: i32 = (mcrust::net::minecraft::client::gui::font::LINE_HEIGHT).wrapping_add(1);
        let mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack = FlexibleItemStack::get_stack_or_throw(&ItemRepository::get_item_stack_2("ARCHITECT_FIRST_DRAFT", ico::BARRIER.get_mut()));
        let mut icon_area_size: i32 = (__this.lock().borrow().get_height()).wrapping_sub(INFO_OFFSET.wrapping_mul(2));
        graphics.fake_item(&icon, x, y.wrapping_add((icon_area_size.wrapping_sub(mcrust::net::minecraft::client::gui::render::gui_renderer::DEFAULT_ITEM_SIZE)).wrapping_div(2)));
        x = x.wrapping_add((mcrust::net::minecraft::client::gui::render::gui_renderer::DEFAULT_ITEM_SIZE).wrapping_add(2));
        let mut dailies_completed: i32 = daily_runs.get_daily_runs_completed();
        let mut daily_runs_test: MutableComponent = component::empty().append(component::literal("Daily Runs ").with_style_format(ChatFormatting::DarkRed)).append((component::literal(&format!("{}/5", dailies_completed))).with_style_format((if dailies_completed == dungeons::DAILY_RUNS_MAX_DAILIES { ChatFormatting::Green } else { ChatFormatting::Yellow })));
        gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x, y, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(daily_runs_test.clone()), mcrust::net::minecraft::util::common_colors::WHITE);
        y = y.wrapping_add(text_y_step);
        let mut resets_in_text: MutableComponent = component::empty().append(component::literal("Resets In ").with_style_format(ChatFormatting::DarkPurple)).append_str(&TimeFormatUtils::get_shortest_reasonable_unit(&daily_runs.time_until_reset()));
        gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x, y, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(resets_in_text.clone()), mcrust::net::minecraft::util::common_colors::WHITE);
        y = y.wrapping_add(text_y_step);
    }

    /// DailyRunsWidget.extractWidgetRenderState as the dispatch of BasicInfoBoxWidget.extractWidgetRenderState
    pub fn extract_widget_render_state_dyn(__this: &compat::Shared<dyn BasicInfoBoxWidget>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32) {
        Self::extract_widget_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

}

/// DailyRunsWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for DailyRunsWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base.base
    }
}

/// DailyRunsWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<DailyRunsWidget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_widget_render_state()); __f(&compat::view(__h, |a| a as compat::Shared<dyn BasicInfoBoxWidget>), &mut context.graphics, mouse_x, mouse_y, a) }))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.update_widget_narration()); __f(&compat::view(__h, |a| a as compat::Shared<dyn BasicInfoBoxWidget>), output) }))
    }

}
