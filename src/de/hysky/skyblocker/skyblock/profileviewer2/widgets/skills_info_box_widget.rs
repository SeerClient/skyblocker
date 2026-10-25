//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.widgets.SkillsInfoBoxWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::api_profile::ApiProfile;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::profile_member::ProfileMember;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::primitive_type_utils::PrimitiveTypeUtils;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::basic_info_box_widget::BasicInfoBoxWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::basic_info_box_widget::BasicInfoBoxWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::data::constants::constant_data::ConstantData;
use crate::de::hysky::skyblocker::utils::data::constants::emblem_constants::Emblem;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::java::time::format::date_time_formatter;
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

/// java: public final class SkillsInfoBoxWidget extends BasicInfoBoxWidget
#[derive(Clone)]
pub struct SkillsInfoBoxWidget {
    /// java: extends BasicInfoBoxWidget
    pub base: BasicInfoBoxWidgetBase,
    /// java: private final LoadingInformation info;
    pub info: LoadingInformation,
}

impl std::ops::Deref for SkillsInfoBoxWidget {
    type Target = BasicInfoBoxWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SkillsInfoBoxWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SkillsInfoBoxWidget {
    /// java: public SkillsInfoBoxWidget(int width, int height, LoadingInformation info)
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

}

impl BasicInfoBoxWidget for SkillsInfoBoxWidget {
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
        SkillsInfoBoxWidget::extract_widget_render_state_dyn
    }

}

impl SkillsInfoBoxWidget {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        <dyn BasicInfoBoxWidget>::extract_widget_render_state_body(&compat::view(__this, |a| a as compat::Shared<dyn BasicInfoBoxWidget>), graphics, mouse_x, mouse_y, a);
        let mut font: Font = compat::font();
        let mut profile: compat::Shared<ApiProfile> = __this.lock().borrow().info.profile();
        let mut member: compat::Shared<ProfileMember> = __this.lock().borrow().info.member();
        let mut x: i32 = (__this.lock().borrow().get_x()).wrapping_add(INFO_OFFSET);
        let mut y: i32 = (__this.lock().borrow().get_y()).wrapping_add(INFO_OFFSET);
        let mut text_y_step: i32 = (mcrust::net::minecraft::client::gui::font::LINE_HEIGHT).wrapping_add(1);
        let mut tooltip: Option<Vec<MutableComponent>> = None;
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((x) as f32, (y.wrapping_sub(1)) as f32);
        // java: graphics.pose().scale(0.75f);
        gui_graphics_extractor::pose_scale(0.75f32, 0.75f32);
        graphics.fake_item(&FlexibleItemStack::get_stack_or_throw(ico::PAINTING.get_mut()), 0, 0);
        gui_graphics_extractor::pop_pose(__pose_1);
        let mut profile_text: MutableComponent = (component::literal(&compat::read(&profile, |__t| __t.cute_name.clone()))).with_style_format(ChatFormatting::Underline);
        gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), (x.wrapping_add(12)).wrapping_add(2), y, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(profile_text.clone()), mcrust::net::minecraft::util::common_colors::WHITE);
        y = y.wrapping_add(1);
        let mut first_join: mcrust::java::time::instant::Instant = mcrust::java::time::instant::Instant::of_epoch_milli(member.lock().borrow().profile.first_join);
        let mut first_join_year: i32 = mcrust::java::time::zoned_date_time::ZonedDateTime::of_instant(mcrust::java::time::instant::Instant::of_epoch_milli(member.lock().borrow().profile.first_join), mcrust::java::time::zone_id::ZoneId::system_default()).get_year();
        let mut joined_text: MutableComponent = component::empty().append(component::literal("Joined: ").with_style_format(ChatFormatting::Green)).append_str(&first_join_year.to_string());
        y = y.wrapping_add(text_y_step);
        gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x, y, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(joined_text.clone()), mcrust::net::minecraft::util::common_colors::WHITE);
        if GuiHelper::point_is_in_area((mouse_x) as f64, (mouse_y) as f64, (x) as f64, (y) as f64, ((x.wrapping_add(font.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(joined_text.clone())))) as f64) as f64, ((y.wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)) as f64) as f64) {
            // java: Formatters.DATE_FORMATTER.format(firstJoin) - the formatter carries its zone (`withZone(getTimeZone())`)
            tooltip = Some(vec![component::literal(&formatters::DATE_FORMATTER.get_mut().format(mcrust::java::time::zoned_date_time::ZonedDateTime::of_instant(first_join, formatters::Formatters::get_time_zone())))]);
        }
        y = y.wrapping_add(text_y_step);
        let mut emblem: Option<Emblem> = ConstantData::get_emblem_constants().from_id(&member.lock().borrow().levelling.selected_emblem);
        let mut emblem_text: MutableComponent = component::empty()
            .append(component::literal("Emblem: ").with_style_format(ChatFormatting::Green))
            .append(emblem.as_ref().map(|e| e.display()).unwrap_or_else(|| component::literal("None")));
        gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x, y, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(emblem_text.clone()), mcrust::net::minecraft::util::common_colors::WHITE);
        if emblem.is_some() && GuiHelper::point_is_in_area((mouse_x) as f64, (mouse_y) as f64, (x) as f64, (y) as f64, ((x.wrapping_add(font.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(emblem_text.clone())))) as f64) as f64, ((y.wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)) as f64) as f64) {
            tooltip = Some(vec![component::literal(&emblem.clone().unwrap().name())]);
        }
        y = y.wrapping_add(text_y_step);
        let mut skill_average_text: MutableComponent = component::empty().append(component::literal("Skill Avg: ").with_style_format(ChatFormatting::Green)).append_str(&formatters::FLOAT_NUMBERS.get_mut().format(member.lock().borrow().player_data.get_skill_average(&compat::read(__this, |__t| __t.info.clone())) as f64));
        gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x, y, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(skill_average_text.clone()), mcrust::net::minecraft::util::common_colors::WHITE);
        y = y.wrapping_add(text_y_step);
        let mut purse_text: MutableComponent = component::empty().append(component::literal("Purse: ").with_style_format(ChatFormatting::Gold)).append_str(&formatters::SHORT_DOUBLE_NUMBERS.get_mut().format(member.lock().borrow().currencies.coins_in_purse));
        gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x, y, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(purse_text.clone()), mcrust::net::minecraft::util::common_colors::WHITE);
        y = y.wrapping_add(text_y_step);
        // (the `@Nullable Double` field, boxed as Java hands it to `Object`)
        let personal_bank_account: Option<compat::Object> = compat::read(&member, |__t| __t.profile.personal_bank_account).map(|d| std::sync::Arc::new(d) as compat::Object);
        let mut bank: f64 = (profile.lock().borrow().banking.balance + PrimitiveTypeUtils::coerce_double(personal_bank_account.clone()));
        let mut bank_text: MutableComponent = component::empty().append(component::literal("Bank: ").with_style_format(ChatFormatting::Gold)).append_str(&formatters::SHORT_DOUBLE_NUMBERS.get_mut().format(bank));
        gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), x, y, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(bank_text.clone()), mcrust::net::minecraft::util::common_colors::WHITE);
        if GuiHelper::point_is_in_area((mouse_x) as f64, (mouse_y) as f64, (x) as f64, (y) as f64, ((x.wrapping_add(font.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(bank_text.clone())))) as f64) as f64, ((y.wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)) as f64) as f64) && member.lock().borrow().profile.personal_bank_account.is_some() {
            let mut bank_tooltip: Vec<MutableComponent> = Vec::new();
            bank_tooltip.push((component::literal(&format!("Personal: {}", formatters::INTEGER_NUMBERS.get_mut().format(PrimitiveTypeUtils::coerce_double(personal_bank_account.clone()))))).with_style_format(ChatFormatting::Red));
            bank_tooltip.push((component::literal(&format!("Co-op: {}", formatters::INTEGER_NUMBERS.get_mut().format(profile.lock().borrow().banking.balance)))).with_style_format(ChatFormatting::Gold));
            bank_tooltip.push((component::literal(&format!("Total: {}", formatters::INTEGER_NUMBERS.get_mut().format(bank)))).with_style_format(ChatFormatting::Yellow));
            tooltip = Some(bank_tooltip.clone());
        }
        if let Some(tooltip) = &tooltip {
            // java: graphics.setComponentTooltipForNextFrame(font, tooltip, mouseX, mouseY);
            compat::batch_s_set_tooltip_for_next_frame(tooltip.iter().cloned().map(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from).collect());
        }
    }

    /// SkillsInfoBoxWidget.extractWidgetRenderState as the dispatch of BasicInfoBoxWidget.extractWidgetRenderState
    pub fn extract_widget_render_state_dyn(__this: &compat::Shared<dyn BasicInfoBoxWidget>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32) {
        Self::extract_widget_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

}

/// SkillsInfoBoxWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for SkillsInfoBoxWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base.base
    }
}

/// SkillsInfoBoxWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<SkillsInfoBoxWidget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_widget_render_state()); __f(&compat::view(__h, |a| a as compat::Shared<dyn BasicInfoBoxWidget>), &mut context.graphics, mouse_x, mouse_y, a) }))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.update_widget_narration()); __f(&compat::view(__h, |a| a as compat::Shared<dyn BasicInfoBoxWidget>), output) }))
    }

}
